#![warn(missing_docs)]
//! This module contains the [`SimulationContract`] struct that is used to wrap around the ethers `BaseContract` and add some additional information relevant for revm and the simulation.

use bytes::Bytes;
use ethers::{
    abi::{Contract, Detokenize, Token, Tokenize},
    contract::AbiError,
    prelude::BaseContract,
    types::{Bytes as EthersBytes, H256},
};
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, B160, B256, U256};

use crate::{
    agent::{Agent, AgentTxGasSettings},
    environment::sim_environment::SimulationEnvironment,
};

#[derive(Debug, Clone)]
/// A struct to indicate a lock on contracts that are not deployed.
pub struct NotDeployed;
#[derive(Debug)]
/// A struct to indicate an unlocked contract that is deployed.
pub struct IsDeployed;

/// Trait that is used to allow for different statuses of contract fields depending on whether a contract is deployed or not.
pub trait DeploymentStatus {
    /// The type of the address field.
    type Address;
    /// The type of the bytecode field used only before deployment.
    type Bytecode;
    /// The type of the constructor arguments field used only before deployment.
    type ConstructorArguments;
}

impl DeploymentStatus for NotDeployed {
    type Address = ();
    type Bytecode = Vec<u8>;
    type ConstructorArguments = ();
}

impl DeploymentStatus for IsDeployed {
    type Address = B160;
    type Bytecode = ();
    type ConstructorArguments = Vec<Token>;
}

#[derive(Debug, Clone)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
/// # Fields
/// * `address` - The address of the contract within the relevant [`SimulationEnvironment`].
/// * `base_contract` - The ethers [`BaseContract`] that holds the ABI.
/// * `bytecode` - The contract's deployed bytecode.
/// * `constructor_arguments` - The constructor arguments for the contract.
pub struct SimulationContract<DeployedState: DeploymentStatus> {
    /// The address of the contract within the relevant [`SimulationEnvironment`].
    pub address: DeployedState::Address,
    /// The ethers [`BaseContract`] that holds the ABI.
    pub base_contract: BaseContract,
    /// The contract's deployed bytecode.
    pub bytecode: DeployedState::Bytecode,
    /// The constructor arguments for the contract.
    pub constructor_arguments: DeployedState::ConstructorArguments,
}

impl SimulationContract<NotDeployed> {
    /// A constructor function for [`SimulationContract`] that takes a [`BaseContract`] and the deployment bytecode.
    pub fn new(contract: Contract, bytecode: EthersBytes) -> Self {
        Self {
            base_contract: BaseContract::from(contract),
            bytecode: bytecode.to_vec(),
            address: (),
            constructor_arguments: (),
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
    pub fn deploy<T: Tokenize>(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        deployer: Box<dyn Agent>,
        constructor_arguments: T,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode.
        let tokenized_args = constructor_arguments.into_tokens();
        let bytecode = match self.base_contract.abi().constructor.clone() {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(self.bytecode.clone(), &tokenized_args)
                    .unwrap(),
            ),
            None => Bytes::from(self.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        let deploy_txenv = Self::build_deploy_tx(deployer, bytecode, value, gas_settings);

        let execution_result = simulation_environment.execute(deploy_txenv);
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        SimulationContract {
            bytecode: (),
            address,
            base_contract: self.base_contract.clone(),
            constructor_arguments: tokenized_args,
        }
    }

    /// A constructor to build a `TxEnv` for a contract deployment.
    pub fn build_deploy_tx(
        deployer: Box<dyn Agent>,
        bytecode: Bytes,
        value: Option<U256>,
        gas_settings: Option<AgentTxGasSettings>,
    ) -> TxEnv {
        let tx_gas_settings = gas_settings.unwrap_or_default();
        TxEnv {
            caller: deployer.address(),
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

impl SimulationContract<IsDeployed> {
    /// Encodes the arguments for a function call for the [`SimulationContract`].
    /// # Arguments
    /// * `function_name` - The name of the function to encode.
    /// * `args` - The arguments to encode.
    /// # Returns
    /// * `Result<Bytes, AbiError>` - The encoded arguments.
    pub fn encode_function(
        &self,
        function_name: &str,
        args: impl Tokenize,
    ) -> Result<Bytes, AbiError> {
        match self.base_contract.encode(function_name, args) {
            Ok(encoded) => Ok(encoded.into_iter().collect()),
            Err(e) => Err(e),
        }
    }
    /// Decodes the output of a function call for the [`SimulationContract`].
    /// # Arguments
    /// * `function_name` - The name of the function to decode.
    /// * `value` - The bytecode to decode.
    /// # Returns
    /// * `Result<D, AbiError>` - The decoded output.
    pub fn decode_output<D: Detokenize>(
        &self,
        function_name: &str,
        value: Bytes,
    ) -> Result<D, AbiError> {
        self.base_contract
            .decode_output::<D, Bytes>(function_name, value)
    }
    /// Decodes the logs for an event with the [`SimulationContract`].
    /// # Arguments
    /// * `function_name` - The name of the event to decode.
    /// * `log_topics` - The topics of the log.
    /// * `log_data` - The data of the log.
    /// # Returns
    /// * `Result<D, AbiError>` - The decoded event logs.
    pub fn decode_event<D: Detokenize>(
        &self,
        function_name: &str,
        log_topics: Vec<B256>,
        log_data: Bytes,
    ) -> Result<D, AbiError> {
        let log_topics: Vec<H256> = log_topics
            .into_iter()
            .map(|topic| H256::from_slice(&topic.0))
            .collect();
        self.base_contract
            .decode_event(function_name, log_topics, log_data.into())
    }

    /// Decodes the error for an error with the [`SimulationContract`].
    /// # Arguments
    /// * `name` - The name of the error to decode.
    /// * `value` - The data of the error.
    /// # Returns
    /// * `Vec<Token>` - The raw decoded error.
    pub fn decode_error(&self, name: String, value: Bytes) -> Vec<Token> {
        let mut abi_errors = self.base_contract.abi().errors();
        let predicate = |error: &&ethers::abi::ethabi::AbiError| error.name == name;
        let error = abi_errors.find(predicate).unwrap();
        error.decode(&value).unwrap()
    }
}
