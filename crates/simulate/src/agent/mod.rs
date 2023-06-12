use agent_channel::AgentChannel;

use crate::{
    state::{update::UpdateData, world::World, SimState},
    EvmAddress,
};

pub mod agent_channel;
pub mod types;

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent<U: UpdateData, W: World<WorldUpdateData = U>>: Sync + Send {
    /// Returns the address of the agent.
    fn address(&self) -> EvmAddress;

    /// Allows manager to register address with the agent.
    fn register_address(&mut self, address: &EvmAddress);

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&mut self, state: &SimState<U, W>, channel: AgentChannel<U>);

    /// Executes actions against the simulation as soon as the agent is activated.
    fn resolve_activation_step(&mut self, state: &SimState<U, W>);

    /// Executes the agents actions against the simulation environment.
    fn step(&mut self, state: &SimState<U, W>, channel: AgentChannel<U>);

    /// Executes the agents actions against the simulation environment.
    fn resolve_step(&mut self, state: &SimState<U, W>);
}
