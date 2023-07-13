use ethers::{
    abi::{Contract as EthersContract, Detokenize, Token, Tokenize},
    prelude::BaseContract as EthersBaseContract,
    types::H256 as EthersH256,
};
use revm::primitives::B256;

use crate::{contract::errors::SimContractError, EthersBytes, EvmBytes};

type SimContractResult<T> = Result<T, SimContractError>;

#[derive(Debug, Clone)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
/// # Fields
/// * `base_contract` - The ethers [`BaseContract`] that holds the ABI.
/// * `bytecode` - The contract's deployed bytecode.
pub struct SimContract {
    /// The ethers [`BaseContract`] that holds the ABI.
    pub base_contract: EthersBaseContract,
    /// The contract's deployed bytecode.
    pub bytecode: EvmBytes,
}

impl SimContract {
    /// A constructor function for [`SimulationContract`] that takes a [`BaseContract`] and the deployment bytecode.
    pub fn new(contract: EthersContract, bytecode: EthersBytes) -> Self {
        Self {
            base_contract: EthersBaseContract::from(contract),
            bytecode: bytecode.0,
        }
    }

    pub fn encode_constructor(&self, args: impl Tokenize) -> SimContractResult<EvmBytes> {
        match &self.base_contract.abi().constructor {
            Some(constructor) => {
                let encoded_vec = constructor
                    .encode_input(self.bytecode.to_vec(), &args.into_tokens())
                    .map_err(SimContractError::ConstructorEncodeError)?;
                Ok(EvmBytes::from(encoded_vec))
            }
            None => Ok(self.bytecode.clone()),
        }
    }

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
    ) -> SimContractResult<EvmBytes> {
        match self.base_contract.encode(function_name, args) {
            Ok(encoded) => Ok(encoded.into_iter().collect()),
            Err(e) => Err(SimContractError::AbiError(e)),
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
        value: EvmBytes,
    ) -> SimContractResult<D> {
        self.base_contract
            .decode_output::<D, EvmBytes>(function_name, value)
            .map_err(SimContractError::AbiError)
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
        log_data: EvmBytes,
    ) -> SimContractResult<D> {
        let log_topics: Vec<EthersH256> = log_topics
            .into_iter()
            .map(|topic| EthersH256::from_slice(&topic.0))
            .collect();
        self.base_contract
            .decode_event(function_name, log_topics, log_data.into())
            .map_err(SimContractError::AbiError)
    }

    /// Decodes the error for an error with the [`SimulationContract`].
    /// # Arguments
    /// * `name` - The name of the error to decode.
    /// * `value` - The data of the error.
    /// # Returns
    /// * `Vec<Token>` - The raw decoded error.
    pub fn decode_error(&self, name: String, value: EvmBytes) -> SimContractResult<Vec<Token>> {
        let mut abi_errors = self.base_contract.abi().errors();
        let error = abi_errors
            .find(|error| error.name == name)
            .ok_or(SimContractError::MissingErrorName(name))?;
        error
            .decode(&value)
            .map_err(|e| SimContractError::AbiError(e.into()))
    }
}
