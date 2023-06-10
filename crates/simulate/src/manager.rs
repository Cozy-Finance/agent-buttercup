#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.
use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_utils::thread;

use eyre::Result;
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::{AccountInfo, U256 as EvmU256};
use thiserror::Error;

use crate::{
    agent::{Agent, AgentId},
    state::{
        update::{SimUpdate, Update},
        SimState,
    },
    time_policy::BlockTimePolicy,
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
    pub time_policy: Box<dyn BlockTimePolicy>,
    pub agents: Vec<Box<dyn Agent<U>>>,
    pub rng: StdRng,
}

impl<U: Update> SimManager<U> {
    pub fn new(state: SimState<U>, time_policy: Box<dyn BlockTimePolicy>, rng_seed: u64) -> Self {
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
            .update_block_time_env(self.time_policy.current_block_time_env());

        while self.time_policy.is_active() {
            let (update_sender, update_receiver) = unbounded::<SimUpdate<U>>();

            // Spawn agents to read state immutablely and queue updates.
            thread::scope(|s| {
                for agent in &mut self.agents {
                    s.spawn(|_| agent.step(&self.state, &update_sender));
                }
            })
            .unwrap();

            // Execute queued updates.
            drop(update_sender);
            for update in update_receiver.iter().collect::<Vec<_>>() {
                self.state.execute(update);
            }

            // Update time policy.
            self.time_policy.step();
            self.state
                .update_block_time_env(self.time_policy.current_block_time_env());
        }
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(&mut self, mut new_agent: Box<dyn Agent<U> + Sync>) -> Result<()> {
        // Generates an address for the agent.
        let new_agent_address = EvmAddress::random_using(&mut self.rng);
        self.state
            .add_account_info(new_agent_address, AccountInfo::default())?;

        // Register address with agent.
        new_agent.register_address(&new_agent_address);

        // Runs the agent's activation step and queue updates.
        let (update_sender, update_receiver) = unbounded::<SimUpdate<U>>();
        new_agent.activation_step(&self.state, &update_sender);

        // Execute queued updates.
        drop(update_sender);
        for update in update_receiver.iter().collect::<Vec<_>>() {
            self.state.execute(update);
        }

        // Adds agent to local data.
        self.agents.push(new_agent);
        Ok(())
    }
}
