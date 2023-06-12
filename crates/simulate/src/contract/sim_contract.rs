//! This module contains the [`SimulationContract`] struct that is used to wrap around the ethers `BaseContract` and add some additional information relevant for revm and the simulation.
use ethers::{
    abi::{Contract as EthersContract, Detokenize, Token, Tokenize},
    prelude::BaseContract as EthersBaseContract,
    types::H256 as EthersH256,
};
use eyre::Result;
use revm::primitives::B256;
use thiserror::Error;

use crate::{EthersBytes, EvmAddress, EvmBytes};

#[derive(Error, Debug)]
pub enum SimContractError {
    #[error("could not encode function")]
    EncodeFuncFailure,
    #[error("could not decode output")]
    DecodeOutputFailure,
    #[error("could not decode event")]
    DecodeEventFailure,
    #[error("could not decode error")]
    DecodeErrorFailure,
}

#[derive(Debug, Clone)]
/// A struct that wraps around the ethers `BaseContract` and adds some additional information relevant for revm and the simulation.
/// # Fields
/// * `address` - The address of the contract within the relevant [`SimulationEnvironment`].
/// * `base_contract` - The ethers [`BaseContract`] that holds the ABI.
/// * `bytecode` - The contract's deployed bytecode.
pub struct SimContract {
    /// The address of the contract within the relevant [`SimulationEnvironment`].
    pub address: Option<EvmAddress>,
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
            address: None,
        }
    }

    pub fn encode_constructor(&self, args: impl Tokenize) -> Result<EvmBytes, SimContractError> {
        match &self.base_contract.abi().constructor {
            Some(constructor) => {
                let encoded_vec = constructor
                    .encode_input(self.bytecode.to_vec(), &args.into_tokens())
                    .map_err(|_| SimContractError::EncodeFuncFailure)?;
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
    ) -> Result<EvmBytes, SimContractError> {
        match self.base_contract.encode(function_name, args) {
            Ok(encoded) => Ok(encoded.into_iter().collect()),
            _ => Err(SimContractError::EncodeFuncFailure),
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
    ) -> Result<D, SimContractError> {
        self.base_contract
            .decode_output::<D, EvmBytes>(function_name, value)
            .map_err(|_| SimContractError::DecodeOutputFailure)
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
    ) -> Result<D, SimContractError> {
        let log_topics: Vec<EthersH256> = log_topics
            .into_iter()
            .map(|topic| EthersH256::from_slice(&topic.0))
            .collect();
        self.base_contract
            .decode_event(function_name, log_topics, log_data.into())
            .map_err(|_| SimContractError::DecodeEventFailure)
    }

    /// Decodes the error for an error with the [`SimulationContract`].
    /// # Arguments
    /// * `name` - The name of the error to decode.
    /// * `value` - The data of the error.
    /// # Returns
    /// * `Vec<Token>` - The raw decoded error.
    pub fn decode_error(
        &self,
        name: String,
        value: EvmBytes,
    ) -> Result<Vec<Token>, SimContractError> {
        let mut abi_errors = self.base_contract.abi().errors();
        let predicate = |error: &&ethers::abi::ethabi::AbiError| error.name == name;
        let error = abi_errors
            .find(predicate)
            .ok_or(SimContractError::DecodeErrorFailure)?;
        error
            .decode(&value)
            .map_err(|_| SimContractError::DecodeErrorFailure)
    }
}
