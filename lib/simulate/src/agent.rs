use eyre::Result;
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, U256};
use thiserror::Error;

use crate::{
    contract::sim_contract::{IsDeployed, NotDeployed, SimContract},
    environment::sim_env::SimEnv,
    sim_env_data::SimEnvData,
    EvmAddress, EvmBytes,
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

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("failed to deploy contract")]
    ContractDeploymentFailure,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent {
    /// Returns the address of the agent.
    fn address(&self) -> EvmAddress;

    /// Allows manager to register address with the agent.
    fn register_address(&mut self, address: &EvmAddress);

    /// Returns the address of the agent.
    fn name(&self) -> Option<String>;

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData);

    /// Executes the agents actions against the simulation environment.
    fn step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData);

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &self,
        sim_env: &mut SimEnv,
        contract: &SimContract<IsDeployed>,
        call_data: EvmBytes,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.into(), call_data, None, None);
        sim_env.execute(tx)
    }

    fn call_contract_with_value(
        &self,
        sim_env: &mut SimEnv,
        contract: &SimContract<IsDeployed>,
        call_data: EvmBytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.into(), call_data, Some(value), None);
        sim_env.execute(tx)
    }

    fn call_contract_with_gas_settings(
        &self,
        sim_env: &mut SimEnv,
        contract: &SimContract<IsDeployed>,
        call_data: EvmBytes,
        gas_settings: AgentTxGasSettings,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(
            contract.address.into(),
            call_data,
            None,
            Some(gas_settings),
        );
        sim_env.execute(tx)
    }

    fn call_contract_with_value_and_gas_settings(
        &self,
        sim_env: &mut SimEnv,
        contract: &SimContract<IsDeployed>,
        call_data: EvmBytes,
        value: U256,
        gas_settings: AgentTxGasSettings,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(
            contract.address.into(),
            call_data,
            Some(value),
            Some(gas_settings),
        );
        sim_env.execute(tx)
    }

    /// Used to allow agents to make a generic call a specific smart contract by address.
    fn call_contract_by_address(
        &self,
        sim_env: &mut SimEnv,
        contract_address: EvmAddress,
        call_data: EvmBytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract_address, call_data, value, gas_settings);
        sim_env.execute(tx)
    }

    /// A constructor to build a `TxEnv` for an agent contract call.
    fn build_call_transaction(
        &self,
        receiver_address: EvmAddress,
        call_data: EvmBytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> TxEnv {
        let tx_gas_settings = gas_settings.unwrap_or_default();
        TxEnv {
            caller: self.address(),
            gas_limit: tx_gas_settings.gas_limit,
            gas_price: tx_gas_settings.gas_price,
            gas_priority_fee: tx_gas_settings.gas_priority_fee,
            transact_to: TransactTo::Call(receiver_address.into()),
            value: value.unwrap_or(U256::ZERO),
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    /// Deploy a contract to the current simulation environment and return a new [`SimulationContract<IsDeployed>`].
    /// Does not consume the current [`SimulationContract<NotDeployed>`] so that more copies can be deployed later.
    /// # Arguments
    /// * `simulation_environment` - The [`SimulationEnvironment`] to deploy the contract to.
    /// * `deployer` - The [`AgentType`] that will deploy the contract.
    /// * `constructor_arguments` - The constructor arguments for the contract.
    /// # Returns
    /// * `SimulationContract<IsDeployed>` - The deployed contract.
    fn deploy_contract(
        &self,
        sim_env: &mut SimEnv,
        contract: &SimContract<NotDeployed>,
        call_data: EvmBytes,
    ) -> Result<SimContract<IsDeployed>, AgentError> {
        let tx = self.build_deploy_tx(call_data, None, None);
        let execution_result = sim_env.execute(tx);

        let address = match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Create(_, address) => address,
                _ => return Err(AgentError::ContractDeploymentFailure),
            },
            _ => return Err(AgentError::ContractDeploymentFailure),
        };

        Ok(SimContract {
            bytecode: (),
            address: address.unwrap().into(),
            base_contract: contract.base_contract.clone(),
        })
    }

    /// A constructor to build a `TxEnv` for a contract deployment.
    fn build_deploy_tx(
        &self,
        bytecode: EvmBytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> TxEnv {
        let tx_gas_settings = gas_settings.unwrap_or_default();
        TxEnv {
            caller: self.address(),
            gas_limit: tx_gas_settings.gas_limit,
            gas_price: tx_gas_settings.gas_price,
            gas_priority_fee: tx_gas_settings.gas_priority_fee,
            transact_to: TransactTo::create(),
            value: value.unwrap_or(U256::ZERO),
            data: bytecode,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }
}
