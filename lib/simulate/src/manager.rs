#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::collections::BTreeMap;

use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{AccountInfo, Address};
use thiserror::Error;

use crate::{
    agent::{Agent, AgentId},
    block_time_policy::BlockTimePolicy,
    environment::sim_environment::SimulationEnvironment,
};

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("Unknown manager error.")]
    UnknownError,
}

/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `time_policy` - The block time policy that the manager calls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager {
    /// [`SimulationEnvironment`] that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// Implements the [`BlockTimePolicy`] trait to manage time in the simulation.
    pub time_policy: Box<dyn BlockTimePolicy>,
    /// The agents that are currently running in the [`SimulationEnvironment`].
    pub agents: BTreeMap<AgentId, Box<dyn Agent>>,
    /// Rng seed used to generate reproducible random agent addresses.
    pub rng: StdRng,
}

impl SimulationManager {
    pub fn new(
        environment: SimulationEnvironment,
        time_policy: Box<dyn BlockTimePolicy>,
        agents: BTreeMap<AgentId, Box<dyn Agent>>,
        rng_seed: u64,
    ) -> Self {
        Self {
            environment,
            time_policy,
            agents,
            rng: StdRng::seed_from_u64(rng_seed),
        }
    }

    /// Run the time policy and agents to update the simulation environment.
    pub fn run_simulation(&mut self) {
        self.environment
            .update_block_time_env(self.time_policy.current_block_time_env());

        while self.time_policy.is_active() {
            for (_, agent) in &self.agents {
                agent.step(&mut self.environment);
            }
            self.environment
                .update_block_time_env(self.time_policy.step());
        }

        self.shut_down_simulation();
    }

    /// Stop the current simulation.
    pub fn shut_down_simulation(&mut self) {
        self.environment.event_senders.clear();
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(&mut self, new_agent: Box<dyn Agent>) -> Result<(), ManagerError> {
        let new_agent_address = Address::random_using(&mut self.rng);
        self.environment
            .add_account_info(new_agent_address, AccountInfo::default());
        new_agent.activation_step(&mut self.environment);
        self.agents.insert(
            AgentId {
                id_num: self.agents.len() as u64,
                name: new_agent.name(),
            },
            new_agent,
        );
        Ok(())
    }
}
