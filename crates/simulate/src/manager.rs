#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.
use std::thread;

use crate::stepper::*;
use crossbeam_channel::unbounded;
use eyre::Result;
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{AccountInfo, U256 as EvmU256};
use thiserror::Error;

use crate::{
    agent::agent::{Agent, AgentId},
    state::{
        update::{SimUpdate, Update},
        SimState,
    },
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
    pub state: SimState<U>,
    pub time_policy: Box<dyn TimePolicy>,
    pub agents: Vec<Box<dyn Agent<U>>>,
    pub rng: StdRng,
}

impl<U: Update> SimManager<U> {
    pub fn new(state: SimState<U>, time_policy: Box<dyn TimePolicy>, rng_seed: u64) -> Self {
        Self {
            state,
            time_policy,
            agents: Vec::new(),
            rng: StdRng::seed_from_u64(rng_seed),
        }
    }

    /// Run the time policy and agents to update the simulation environment.
    pub fn run_sim(&mut self) {
        // Initiate block time policy.
        self.state
            .update_time_env(self.time_policy.current_time_env());

        // Initiate stepper.
        let mut sim_stepper: SimStepper<U> = SimStepper::new();
        let sim_stepper_read_handle_factory = sim_stepper.factory();

        while self.time_policy.is_active() {
            let (tx, rx) = unbounded::<SimUpdate<U>>();

            // Concurrently:
            //      1) Spawn agents to access immutable state via a read handle and queue updates.
            //      2) Append queued updates via the write handle.
            thread::scope(|s| {
                for agent in &mut self.agents {
                    let tx_clone = tx.clone();
                    s.spawn(|| agent.step(&sim_stepper_read_handle_factory.sim_state(), tx_clone));
                }
                s.spawn(||
                    for update in rx.iter() {
                        sim_stepper.append(update);
                    }    
                );
                drop(tx);
            });

            // Publish new state.
            sim_stepper.publish();

            // Update time policy.
            self.time_policy.step();
            self.state
                .update_time_env(self.time_policy.current_time_env());
        }
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(&mut self, mut new_agent: Box<dyn Agent<U> + Sync>) -> Result<()> {
        // Generates an address for the agent.
        let new_agent_address = EvmAddress::random_using(&mut self.rng);
        self.state
            .insert_account_info(new_agent_address, AccountInfo::default())?;

        // Register address with agent.
        new_agent.register_address(&new_agent_address);

        // Runs the agent's activation step and queue updates.
        let (tx, rx) = unbounded::<SimUpdate<U>>();
        new_agent.activation_step(&self.state, tx);

        // Execute queued updates.
        for update in rx.iter() {
            self.state.execute(&update);
        }

        // Adds agent to local data.
        self.agents.push(new_agent);
        Ok(())
    }
}
