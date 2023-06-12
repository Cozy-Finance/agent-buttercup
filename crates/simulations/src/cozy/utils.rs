use std::collections::HashMap;

use ethers::abi::Tokenize;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    contract::{sim_contract::SimContract, utils as contract_utils},
    utils::build_deploy_contract_txenv,
};
use thiserror::Error;

use crate::cozy::{bindings_wrapper::*, EthersAddress, EthersBytes, EvmAddress};

#[derive(Error, Debug)]
pub enum DeploymentError {
    #[error("tried accessing a non-existent library addr")]
    NonExistentLibraryAddr,
    #[error("failed to link set bytecode")]
    LinkingBytecodeFailure,
    #[error("missing linked bytecode")]
    MissingLinkedBytecode,
    #[error("missing unlinked bytecode")]
    MissingUnlinkedBytecode,
}

pub fn build_deploy_contract_tx<T: Tokenize>(
    agent_address: EvmAddress,
    contract_bindings: &BindingsWrapper,
    args: T,
) -> Result<TxEnv> {
    let abi = contract_bindings.abi.clone();
    let bytecode = contract_bindings
        .bytecode
        .ok_or(DeploymentError::MissingLinkedBytecode)?
        .clone();
    let contract = SimContract::new(abi, bytecode);
    let bytecode = contract.encode_constructor(args)?;

    Ok(build_deploy_contract_txenv(
        agent_address,
        bytecode,
        None,
        None,
    ))
}

pub fn build_unlinked_deploy_contract_tx<T: Tokenize>(
    agent_address: EvmAddress,
    contract_bindings: &BindingsWrapper,
    libraries: &HashMap<EthersAddress, &BindingsWrapper>,
    args: T,
) -> Result<TxEnv> {
    let mut links: Vec<(&str, &str, EthersAddress)> = vec![];
    for (addr, lib_binding) in libraries.iter() {
        links.push((lib_binding.path, lib_binding.name, *addr));
    }
    let bytecode = contract_utils::build_linked_bytecode(
        (*contract_bindings)
            .unlinked_bytecode
            .ok_or(DeploymentError::MissingUnlinkedBytecode)?,
        links,
    )?;
    let abi = (*contract_bindings).abi.clone();
    let contract = SimContract::new(abi, EthersBytes(bytecode));
    let bytecode = contract.encode_constructor(args)?;

    Ok(build_deploy_contract_txenv(
        agent_address,
        bytecode,
        None,
        None,
    ))
}
