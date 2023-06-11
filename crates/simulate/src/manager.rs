#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.
use std::{collections::HashMap, thread};

use crossbeam_channel::unbounded;
use eyre::Result;
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{AccountInfo};
use thiserror::Error;

use crate::{
    agent::{
        agent_channel::{AgentChannel, AgentSimUpdate},
        Agent,
    },
    state::{
        update::{Update},
        SimState,
    },
    stepper::*,
    time_policy::TimePolicy,
    EvmAddress,
};

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("Unknown manager error.")]
    UnknownError,
}

/// Manages simulations.
/// # Fields
/// * `state` - The simulation state that the manager controls.
/// * `time_policy` - The time policy that the manager calls.
/// * `agents` - The agents that are currently running in the simulation environment.
/// * `rng` - Randomness generator.
pub struct SimManager<U: Update + 'static> {
    pub time_policy: Box<dyn TimePolicy>,
    pub agents: HashMap<EvmAddress, Box<dyn Agent<U>>>,
    pub rng: StdRng,
    pub stepper: SimStepper<U>,
    pub stepper_read_factory: SimStepperReadHandleFactory<U>,
    pub agent_id_iter: u64,
}

impl<U: Update> SimManager<U> {
    pub fn new(state: SimState<U>, time_policy: Box<dyn TimePolicy>, rng_seed: u64) -> Self {
        let stepper: SimStepper<U> = SimStepper::new_from_current_state(state);
        let stepper_read_factory = stepper.factory();
        Self {
            time_policy,
            agents: HashMap::new(),
            rng: StdRng::seed_from_u64(rng_seed),
            stepper,
            stepper_read_factory,
            agent_id_iter: 0,
        }
    }

    /// Run the time policy and agents to update the simulation environment.
    pub fn run_sim(&mut self) {
        // Initiate block time policy.
        self.stepper
            .update_time_env(self.time_policy.current_time_env());

        while self.time_policy.is_active() {
            let (sender, receiver) = unbounded::<AgentSimUpdate<U>>();

            // Concurrently:
            //      1) Spawn agents to access immutable state via a read handle and queue updates.
            //      2) Append queued updates via the write handle.
            thread::scope(|s| {
                for (address, agent) in &mut self.agents {
                    let channel = AgentChannel {
                        address: *address,
                        sender: sender.clone(),
                    };
                    s.spawn(|| agent.step(&self.stepper_read_factory.sim_state(), channel));
                }
                s.spawn(|| {
                    for update in receiver.iter() {
                        self.stepper.append(update);
                    }
                });
                drop(sender);
            });

            // Publish new state.
            self.stepper.publish();

            // Let agents resolve the step.
            thread::scope(|t| {
                for (_, agent) in &mut self.agents {
                    t.spawn(|| agent.resolve_step(&self.stepper_read_factory.sim_state()));
                }
            });

            // Clear all results.
            self.stepper.clear_all_results();

            // Update time policy.
            self.time_policy.step();
            self.stepper
                .update_time_env(self.time_policy.current_time_env());
        }
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(&mut self, mut new_agent: Box<dyn Agent<U> + Sync>) -> Result<()> {
        // Generates an address for the agent.
        let new_agent_address = EvmAddress::random_using(&mut self.rng);
        self.stepper
            .insert_account_info(new_agent_address, AccountInfo::default());

        // Register address with agent.
        new_agent.register_address(&new_agent_address);

        // Runs the agent's activation step and queue updates.
        let (sender, receiver) = unbounded::<AgentSimUpdate<U>>();
        let channel = AgentChannel {
            address: new_agent_address,
            sender: sender.clone(),
        };
        new_agent.activation_step(&self.stepper_read_factory.sim_state(), channel);

        // Execute queued updates.
        drop(sender);
        for update in receiver.iter() {
            self.stepper.append(update);
        }
        self.stepper.publish();

        // Adds agent to local data.
        self.agents.insert(new_agent_address, new_agent);

        Ok(())
    }
}
