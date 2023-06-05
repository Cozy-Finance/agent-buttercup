#![warn(missing_docs)]
#![warn(unsafe_code)]

use bytes::Bytes;
use ethers::abi::Token;
use revm::primitives::B160;
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, U256};
use thiserror::Error;

use crate::{
    contract::sim_contract::{IsDeployed, NotDeployed, SimulationContract},
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

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Failed to deploy contract.")]
    ContractDeploymentFailure,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM and simulation.
pub trait Agent {
    /// Returns the address of the agent.
    fn address(&self) -> B160;

    /// Returns the name of the agent.
    fn name(&self) -> Option<String>;

    /// Executes actions against the simulation as soon as the agent is activated.
    fn activation_step(&self, simulation_environment: &mut SimulationEnvironment);

    /// Executes the agents actions against the simulation environment.
    fn step(&self, simulation_environment: &mut SimulationEnvironment);

    /// Gets the agents current nonce.
    fn get_nonce(&self, simulation_environment: &mut SimulationEnvironment) -> u64 {
        simulation_environment
            .evm
            .db()
            .unwrap()
            .load_account(self.address())
            .unwrap()
            .info
            .nonce
    }

    /// Used to allow agents to make a generic call a specific smart contract.
    fn call_contract(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.into(), call_data, None, None);
        simulation_environment.execute(tx)
    }

    fn call_contract_with_value(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.into(), call_data, Some(value), None);
        simulation_environment.execute(tx)
    }

    fn call_contract_with_gas_settings(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        gas_settings: AgentTxGasSettings,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(
            contract.address.into(),
            call_data,
            None,
            Some(gas_settings),
        );
        simulation_environment.execute(tx)
    }

    fn call_contract_with_value_and_gas_settings(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
        gas_settings: AgentTxGasSettings,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(
            contract.address.into(),
            call_data,
            Some(value),
            Some(gas_settings),
        );
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
            caller: self.address().into(),
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
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<NotDeployed>,
        call_data: Bytes,
    ) -> Result<SimulationContract<IsDeployed>, AgentError> {
        let tx = self.build_deploy_tx(call_data, None, None);
        let execution_result = simulation_environment.execute(tx);

        let address = match execution_result {
            ExecutionResult::Success { output, .. } => match output {
                Output::Create(_, address) => address,
                _ => return Err(AgentError::ContractDeploymentFailure),
            },
            _ => return Err(AgentError::ContractDeploymentFailure),
        };

        Ok(SimulationContract {
            bytecode: (),
            address: address.ok_or(return Err(AgentError::ContractDeploymentFailure))?,
            base_contract: contract.base_contract.clone(),
        })
    }

    /// A constructor to build a `TxEnv` for a contract deployment.
    fn build_deploy_tx(
        &self,
        bytecode: Bytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> TxEnv {
        let tx_gas_settings = gas_settings.unwrap_or_default();
        TxEnv {
            caller: self.address().into(),
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
