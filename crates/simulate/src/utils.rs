//! Module for utility functionality.
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use crate::{EthersAddress, EvmAddress, EvmBytes};
use bytes::Bytes;
use ethers::prelude::{Address, U256 as EthersU256};
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, B160, U256 as EvmU256};

use crate::agent::types::AgentTxGas;

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

/// Recast a B160 into an Address type
/// # Arguments
/// * `address` - B160 to recast. (B160)
/// # Returns
/// * `Address` - Recasted Address.
pub fn recast_address(address: B160) -> Address {
    let temp: [u8; 20] = address.as_bytes().try_into().unwrap();
    Address::from(temp)
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - Float to convert. (f64)
/// # Returns
/// * `U256` - Converted U256 number.
pub fn float_to_wad(x: f64) -> EthersU256 {
    EthersU256::from((x * 1e18) as u128)
}

/// Converts a float to a WAD fixed point prepared U256 number.
/// # Arguments
/// * `x` - WAD to convert. (U256)
/// # Returns
/// * `f64` - Converted f64 number.
pub fn wad_to_float(x: EthersU256) -> f64 {
    x.as_u128() as f64 / 1e18
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

pub fn is_execution_success(execution_result: ExecutionResult) -> bool {
    match execution_result {
        ExecutionResult::Success { .. } => true,
        _ => false,
    }
}

pub fn build_deploy_contract_txenv(
    caller_address: EvmAddress,
    bytecode: EvmBytes,
    value: Option<EvmU256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address,
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
    caller_address: EvmAddress,
    receiver_address: EvmAddress,
    call_data: EvmBytes,
    value: Option<EvmU256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address,
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
