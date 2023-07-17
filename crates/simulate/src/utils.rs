use bytes::Bytes;
<<<<<<< HEAD
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, U256 as EvmU256};

use crate::{address::Address, agent::types::AgentTxGas, EvmBytes};
=======
use ethers::prelude::{
    U256 as EthersU256,
};
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv};

use crate::{agent::types::AgentTxGas, EvmBytes, address::Address, u256::U256};
>>>>>>> 1f1e355 (init refac to custom u256 type)

#[derive(thiserror::Error, Debug)]
pub enum FailedExecutionError {
    #[error("{0}")]
    Halted(String),
    #[error("{0}")]
    Reverted(String, Bytes),
}

/// Takes an `ExecutionResult` and returns the raw bytes of the output that can then be decoded.
/// # Arguments
/// * `execution_result` - The `ExecutionResult` that we want to unpack.
/// # Returns
/// * `Ok(Bytes)` - The raw bytes of the output.
pub fn unpack_execution(execution_result: ExecutionResult) -> Result<Bytes, FailedExecutionError> {
    match execution_result {
        ExecutionResult::Success { output, .. } => match output {
            Output::Call(value) => Ok(value),
            Output::Create(value, _address) => Ok(value),
        },
        ExecutionResult::Halt { reason, gas_used } => Err(FailedExecutionError::Halted(format!(
            "This call halted for {:#?} and used {} gas.",
            reason, gas_used
        ))),
        ExecutionResult::Revert { output, gas_used } => Err(FailedExecutionError::Reverted(
            format!(
                "This call reverted with output {:#?} and used {} gas.",
                output, gas_used
            ),
            output,
        )),
    }
}

pub fn is_execution_success(execution_result: &ExecutionResult) -> bool {
    matches!(execution_result, ExecutionResult::Success { .. })
}

pub fn build_call_tx(
    caller_address: Address,
<<<<<<< HEAD
    receiver_address: Address,
    call_data: EvmBytes,
=======
    bytecode: EvmBytes,
    value: Option<U256>,
    gas_settings: Option<AgentTxGas>,
>>>>>>> 1f1e355 (init refac to custom u256 type)
) -> TxEnv {
    let tx_gas_settings = AgentTxGas::default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price.into(),
        gas_priority_fee: tx_gas_settings.gas_priority_fee,
<<<<<<< HEAD
        transact_to: TransactTo::Call(receiver_address.into()),
        value: EvmU256::ZERO,
        data: call_data,
=======
        transact_to: TransactTo::create(),
        value: value.unwrap_or(U256::ZERO).into(),
        data: bytecode,
>>>>>>> 1f1e355 (init refac to custom u256 type)
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}

pub fn build_call_tx_w_settings(
    caller_address: Address,
    receiver_address: Address,
    call_data: EvmBytes,
    value: Option<U256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price.into(),
        gas_priority_fee: tx_gas_settings.gas_priority_fee,
        transact_to: TransactTo::Call(receiver_address.into()),
        value: value.unwrap_or(U256::ZERO).into(),
        data: call_data,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}
