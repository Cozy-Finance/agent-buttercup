use revm::primitives::AccountInfo;

use crate::{
    address::Address,
    agent::agent_channel::{AgentChannelReceiver, AgentChannelSender},
    state::{update::UpdateData, world::World, State},
};

pub mod agent_channel;

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent<U, W>: Sync + Send
where
    U: UpdateData,
    W: World<WorldUpdateData = U>,
{
    /// Returns the address of the agent.
    fn address(&self) -> Address;

    /// Returns the account info of the agent.
    fn account_info(&self) -> AccountInfo {
        AccountInfo::default()
    }

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&mut self, _state: &mut State<U, W>) {}

    /// Executes the agents actions against the simulation environment.
    fn step(&mut self, _state: &State<U, W>, _channel: &AgentChannelSender<U>) {}

    /// Executes the agents actions against the simulation environment.
    fn resolve_step(&mut self, _state: &State<U, W>, _channel: &AgentChannelReceiver<U>) {}
}
