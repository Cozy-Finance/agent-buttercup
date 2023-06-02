#![warn(missing_docs)]
#![warn(unsafe_code)]

use bytes::Bytes;
use revm::primitives::{Address, ExecutionResult, TransactTo, TxEnv, U256};

use crate::{
    contract::{IsDeployed, SimulationContract},
    environment::sim_environment::SimulationEnvironment,
};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AgentId {
    pub id_num: u64,
    pub name: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct AgentTxGasSettings {
    pub gas_limit: u64,
    pub gas_price: U256,
    pub gas_priority_fee: Option<U256>,
}

impl Default for AgentTxGasSettings {
    fn default() -> Self {
        Self {
            gas_limit: u64::MAX,
            gas_price: U256::ZERO,
            gas_priority_fee: None,
        }
    }
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent {
    /// Returns the address of the agent.
    fn address(&self) -> Address;

    /// Returns the name of the agent.
    fn name(&self) -> Option<String>;

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&self, simulation_environment: &mut SimulationEnvironment);

    /// Executes the agents actions against the simulation environment.
    fn step(&self, simulation_environment: &mut SimulationEnvironment);

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address, call_data, value, gas_settings);
        simulation_environment.execute(tx)
    }

    /// Used to allow agents to make a generic call a specific smart contract by address.
    fn call_contract_by_address(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract_address: Address,
        call_data: Bytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract_address, call_data, value, gas_settings);
        simulation_environment.execute(tx)
    }

    /// A constructor to build a `TxEnv` for an agent contract call.
    fn build_call_transaction(
        &self,
        receiver_address: Address,
        call_data: Bytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> TxEnv {
        let tx_gas_settings = gas_settings.unwrap_or_default();
        TxEnv {
            caller: self.address(),
            gas_limit: tx_gas_settings.gas_limit,
            gas_price: tx_gas_settings.gas_price,
            gas_priority_fee: tx_gas_settings.gas_priority_fee,
            transact_to: TransactTo::Call(receiver_address),
            value: value.unwrap_or(U256::ZERO),
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }
}
