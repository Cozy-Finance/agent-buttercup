use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers_solc::artifacts::BytecodeObject;
use revm::primitives::{TransactTo, TxEnv};

use super::errors::SimContractError;
use crate::{
    address::Address, agent::types::AgentTxGas, contract::sim_contract::SimContract, u256::U256,
    EthersAddress, EthersBytes, EvmBytes,
};

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, EthersAddress)>,
) -> Result<EvmBytes, SimContractError> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links.clone());
    match bytecode.resolve() {
        Some(b) => Ok(b.0.clone()),
        None => Err(SimContractError::BytecodeLinkingError(
            unlinked_bytecode_str.to_string(),
            links
                .into_iter()
                .map(|(a, b, c)| (a.to_string(), b.to_string(), c))
                .collect(),
        )),
    }
}

pub fn build_deploy_tx_and_contract<T: Tokenize>(
    sender_addr: Address,
    abi: &EthersContract,
    bytecode: &EthersBytes,
    args: T,
) -> Result<(TxEnv, SimContract), SimContractError> {
    let contract = SimContract::new(abi.clone(), bytecode.clone());
    let tx_bytecode = contract.encode_constructor(args)?;
    Ok((build_deploy_tx(sender_addr, tx_bytecode), contract))
}

pub fn build_unlinked_deploy_tx_and_contract<T: Tokenize>(
    sender_addr: Address,
    abi: &EthersContract,
    unlinked_bytecode: &str,
    links: Vec<(&str, &str, EthersAddress)>,
    args: T,
) -> Result<(TxEnv, SimContract), SimContractError> {
    let linked_bytecode = build_linked_bytecode(unlinked_bytecode, links)?;
    let contract = SimContract::new(abi.clone(), EthersBytes(linked_bytecode));
    let tx_bytecode = contract.encode_constructor(args)?;
    Ok((build_deploy_tx(sender_addr, tx_bytecode), contract))
}

pub fn build_deploy_tx(caller_address: Address, bytecode: EvmBytes) -> TxEnv {
    let tx_gas_settings = AgentTxGas::default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price.into(),
        gas_priority_fee: tx_gas_settings.gas_priority_fee.map(|x| x.into()),
        transact_to: TransactTo::create(),
        value: U256::zero().into(),
        data: bytecode,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}

pub fn build_deploy_tx_w_settings(
    caller_address: Address,
    bytecode: EvmBytes,
    value: Option<U256>,
    gas_settings: Option<AgentTxGas>,
) -> TxEnv {
    let tx_gas_settings = gas_settings.unwrap_or_default();
    TxEnv {
        caller: caller_address.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price.into(),
        gas_priority_fee: tx_gas_settings.gas_priority_fee.map(|x| x.into()),
        transact_to: TransactTo::create(),
        value: value.unwrap_or(U256::zero()).into(),
        data: bytecode,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    }
}
