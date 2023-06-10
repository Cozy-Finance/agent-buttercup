use eyre::Result;
use revm::primitives::U256 as EvmU256;
use thiserror::Error;

use crate::{EvmAddress, EvmBytes};

use crossbeam_channel::Sender;

use crate::state::{
    update::{SimUpdate, Update},
    SimState,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AgentId {
    pub id_num: u64,
    pub name: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AgentTxGasSettings {
    pub gas_limit: u64,
    pub gas_price: EvmU256,
    pub gas_priority_fee: Option<EvmU256>,
}

impl Default for AgentTxGasSettings {
    fn default() -> Self {
        Self {
            gas_limit: u64::MAX,
            gas_price: EvmU256::ZERO,
            gas_priority_fee: None,
        }
    }
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent<U: Update>: Sync + Send {
    /// Returns the address of the agent.
    fn address(&self) -> EvmAddress;

    /// Allows manager to register address with the agent.
    fn register_address(&mut self, address: &EvmAddress);

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&mut self, state: &SimState<U>, sender: Sender<SimUpdate<U>>);

    /// Executes the agents actions against the simulation environment.
    fn step(&mut self, state: &SimState<U>, sender: Sender<SimUpdate<U>>);
}
