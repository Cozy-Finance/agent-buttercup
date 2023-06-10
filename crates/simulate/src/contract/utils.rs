use ethers_solc::artifacts::BytecodeObject;
use eyre::{Result, *};
use revm::primitives::{ExecutionResult, Output, TransactTo, TxEnv, U256};

use crate::agent::AgentTxGasSettings;
use crate::{EthersAddress, EvmAddress, EvmBytes};

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, EthersAddress)>,
) -> Result<EvmBytes> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links);
    match bytecode.resolve() {
        Some(b) => {
            return Ok(b.0.clone());
        }
        None => {
            return Err(eyre!("Could not link bytecode."));
        }
    }
}

pub fn build_deploy_contract_tx(
    address: &EvmAddress,
    bytecode: EvmBytes,
    value: Option<U256>,
    gas_settings: Option<AgentTxGasSettings>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: *address,
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
