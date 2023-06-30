use agent_channel::AgentChannel;
use revm::primitives::AccountInfo;

use crate::{
    address::Address,
    agent::types::AgentId,
    state::{update::UpdateData, world::World, SimState},
};

pub mod agent_channel;
pub mod types;

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent<U: UpdateData, W: World<WorldUpdateData = U>>: Sync + Send {
    /// Returns the address of the agent.
    fn id(&self) -> AgentId {
        AgentId {
            address: Address::random(),
            name: None,
        }
    }

    fn account_info(&self) -> AccountInfo {
        AccountInfo::default()
    }

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&mut self, _state: &SimState<U, W>, _channel: AgentChannel<U>) {}

    /// Executes actions against the simulation as soon as the agent is activated.
    fn resolve_activation_step(&mut self, _state: &SimState<U, W>) {}

    /// Executes the agents actions against the simulation environment.
    fn step(&mut self, _state: &SimState<U, W>, _channel: AgentChannel<U>) {}

    /// Executes the agents actions against the simulation environment.
    fn resolve_step(&mut self, _state: &SimState<U, W>) {}
}
