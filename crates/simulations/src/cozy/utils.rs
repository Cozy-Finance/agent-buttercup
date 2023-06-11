use ethers::abi::{Tokenize};
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::Agent,
    contract::{sim_contract::SimContract},
    utils::build_deploy_contract_txenv,
};
use thiserror::Error;

use crate::cozy::{
    bindings_wrapper::*, world_state::CozyWorldStateUpdate,
};

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
    let abi = contract_bindings.abi.clone();
    let bytecode = contract_bindings
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
    let abi = contract_bindings.abi.clone();
    let bytecode = contract_bindings
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
