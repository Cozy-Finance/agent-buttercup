//! Module for utility functionality.
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bytes::Bytes;

use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, U256 as EvmU256};

use crate::{address::Address, agent::types::AgentTxGas, EvmBytes};

#[derive(Debug)]
/// Error type for the simulation manager.
/// # Fields
/// * `message` - Error message.
/// * `output` - Byte output of the error.
pub struct UnpackError {
    /// Error message.
    pub message: String,
    /// Byte output of the error.
    pub output: Option<Bytes>,
}

impl Error for UnpackError {}

impl Display for UnpackError {
    /// Display the error message.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

/// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
/// # Arguments
/// * `execution_result` - The `ExecutionResult` that we want to unpack.
/// # Returns
/// * `Ok(Bytes)` - The raw bytes of the output.
pub fn unpack_execution(execution_result: ExecutionResult) -> Result<Bytes, UnpackError> {
    match execution_result {
        ExecutionResult::Success { output, .. } => match output {
            Output::Call(value) => Ok(value),
            Output::Create(value, _address) => Ok(value),
        },
        ExecutionResult::Halt { reason, gas_used } => Err(UnpackError {
            message: format!(
                "This call halted for {:#?} and used {} gas.",
                reason, gas_used
            ),
            output: None,
        }),
        ExecutionResult::Revert { output, gas_used } => Err(UnpackError {
            message: format!(
                "This call reverted with output {:#?} and used {} gas.",
                output, gas_used
            ),
            output: Some(output),
        }),
    }
}

pub fn is_execution_success(execution_result: &ExecutionResult) -> bool {
    matches!(execution_result, ExecutionResult::Success { .. })
}

pub fn build_deploy_contract_txenv(
    caller_address: Address,
    bytecode: EvmBytes,
    value: Option<EvmU256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price,
        gas_priority_fee: tx_gas_settings.gas_priority_fee,
        transact_to: TransactTo::create(),
        value: value.unwrap_or(EvmU256::ZERO),
        data: bytecode,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}

pub fn build_call_contract_txenv(
    caller_address: Address,
    receiver_address: Address,
    call_data: EvmBytes,
    value: Option<EvmU256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price,
        gas_priority_fee: tx_gas_settings.gas_priority_fee,
        transact_to: TransactTo::Call(receiver_address.into()),
        value: value.unwrap_or(EvmU256::ZERO),
        data: call_data,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}
