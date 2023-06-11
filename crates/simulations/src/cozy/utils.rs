use crate::cozy::world_state::CozyWorldStateUpdate;
use crate::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress, EvmBytes},
};
use ethers::abi::{Contract as EthersContract, Tokenize};
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::agent::Agent;
use simulate::contract::sim_contract::SimContract;
use simulate::contract::utils as contract_utils;
use simulate::utils::build_deploy_contract_txenv;
use thiserror::Error;

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
    agent: &mut dyn Agent<CozyWorldStateUpdate>,
    contract_bindings: &BindingsWrapper,
    args: T,
) -> Result<TxEnv> {
    let abi = (*contract_bindings).abi.clone();
    let bytecode = (*contract_bindings)
        .bytecode
        .ok_or(DeploymentError::MissingLinkedBytecode)?
        .clone();
    let contract = SimContract::new(abi, bytecode);
    let bytecode = contract.encode_constructor(args)?;

    Ok(build_deploy_contract_txenv(
        agent.address(),
        bytecode,
        None,
        None,
    ))
}

pub fn build_unlinked_deploy_contract_tx<T: Tokenize>(
    agent: &mut dyn Agent<CozyWorldStateUpdate>,
    contract_bindings: &BindingsWrapper,
    args: T,
) -> Result<TxEnv> {
    let abi = (*contract_bindings).abi.clone();
    let bytecode = (*contract_bindings)
        .bytecode
        .ok_or(DeploymentError::MissingLinkedBytecode)?
        .clone();
    let contract = SimContract::new(abi, bytecode);
    let bytecode = contract.encode_constructor(args)?;

    Ok(build_deploy_contract_txenv(
        agent.address(),
        bytecode,
        None,
        None,
    ))
}
