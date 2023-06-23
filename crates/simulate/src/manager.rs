#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.
use std::{collections::HashMap, thread};

use crossbeam_channel::unbounded;
use eyre::Result;

use crate::{
    agent::{
        agent_channel::{AgentChannel, AgentSimUpdate},
        types::AgentId,
        Agent,
    },
    errors::*,
    state::{update::UpdateData, world::World, SimState},
    stepper::*,
    time_policy::TimePolicy,
};

/// Manages simulations.
/// # Fields
/// * `state` - The simulation state that the manager controls.
/// * `time_policy` - The time policy that the manager calls.
/// * `agents` - The agents that are currently running in the simulation environment.
/// * `rng` - Randomness generator.
pub struct SimManager<U: UpdateData, W: World<WorldUpdateData = U>> {
    pub time_policy: Box<dyn TimePolicy>,
    pub agents: HashMap<AgentId, Box<dyn Agent<U, W>>>,
    pub stepper: SimStepper<U, W>,
    pub stepper_read_factory: SimStepperReadHandleFactory<U, W>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> SimManager<U, W> {
    pub fn new(state: SimState<U, W>, time_policy: Box<dyn TimePolicy>) -> Self {
        let stepper: SimStepper<U, W> = SimStepper::new_from_current_state(state);
        let stepper_read_factory = stepper.factory();
        Self {
            time_policy,
            agents: HashMap::new(),
            stepper,
            stepper_read_factory,
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
                for (agent_id, agent) in &mut self.agents {
                    let channel: AgentChannel<U> = AgentChannel::new(&sender, agent_id);
                    s.spawn(|| agent.step(&self.stepper_read_factory.sim_state(), channel));
                }
                s.spawn(|| {
                    for update in receiver.iter() {
                        self.stepper.append_agent_sim_update(update);
                    }
                });
                drop(sender);
            });

            // Publish new state with all agent sim updates.
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
            self.stepper.update_time_env(self.time_policy.step());
        }
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(
        &mut self,
        mut new_agent: Box<dyn Agent<U, W>>,
    ) -> Result<(), SimManagerError> {
        // Register agent account info.
        let id = new_agent.id();
        if self.agents.contains_key(&id) {
            return Err(SimManagerError::AddressCollision(id));
        }

        self.stepper
            .insert_account_info(id.address, new_agent.account_info());

        // Runs the agent's activation step and queue updates.
        let (sender, receiver) = unbounded::<AgentSimUpdate<U>>();
        let channel = AgentChannel::new(&sender, &id);
        new_agent.activation_step(&self.stepper_read_factory.sim_state(), channel);

        // Execute queued updates.
        drop(sender);
        for update in receiver.iter() {
            self.stepper.append_agent_sim_update(update);
        }
        self.stepper.publish();

        // Resolve activation step.
        new_agent.resolve_activation_step(&self.stepper_read_factory.sim_state());

        // Clear all results.
        self.stepper.clear_all_results();

        // Adds agent to local data.
        self.agents.insert(id.clone(), new_agent);

        Ok(())
    }

    pub fn get_state(&self) -> SimState<U, W> {
        self.stepper_read_factory.sim_state()
    }
}
