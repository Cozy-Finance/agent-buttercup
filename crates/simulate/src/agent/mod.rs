use crate::{
    address::Address,
    agent::agent_channel::{AgentChannelReceiver, AgentChannelSender},
    state::{update::Update, State, World},
};

pub mod agent_channel;

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent<U, W>: Sync + Send
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    /// Returns the address of the agent.
    fn address(&self) -> Address;

    /// Executes actions against state as soon as the agent is activated.
    fn activation_step(&mut self, _state: &mut State<U, W>) {}

    /// Executes the agents actions against state in each step.
    fn step(&mut self, _state: &State<U, W>, _channel: &AgentChannelSender<U>) {}

    /// Allows agent to resolve step by reading state and receiving tx outcomes.
    fn resolve_step(&mut self, _state: &State<U, W>, _channel: &AgentChannelReceiver<U>) {}
}
