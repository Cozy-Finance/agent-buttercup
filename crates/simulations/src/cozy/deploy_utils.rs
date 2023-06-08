use crate::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress},
};
use ethers::abi::{Contract as EthersContract, Tokenize};
use eyre::Result;
use simulate::agent::Agent;
use simulate::contract::sim_contract::SimContract;
use simulate::contract::utils as contract_utils;
use simulate::environment::sim_env::SimEnv;
use simulate::sim_env_data::SimEnvData;
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

pub fn create_and_deploy_sim_contract<T: Tokenize>(
    agent: &mut dyn Agent,
    sim_env: &mut SimEnv,
    sim_data: &mut SimEnvData,
    abi: EthersContract,
    bytecode: EthersBytes,
    name: &str,
    args: T,
) -> Result<()> {
    let contract = SimContract::new(abi, bytecode);
    let contract = agent.deploy_contract(sim_env, &contract, contract.encode_constructor(args)?)?;
    println!("{} deployed at address: {}.", name, contract.address);

    sim_data
        .contract_registry
        .insert(name.to_string(), contract);

    Ok(())
}

pub fn deploy_linked_contract_with_args<T: Tokenize>(
    agent: &mut dyn Agent,
    sim_env: &mut SimEnv,
    sim_data: &mut SimEnvData,
    contract_bindings: &BindingsWrapper,
    args: T,
) -> Result<()> {
    let abi = (*contract_bindings).abi.clone();
    let bytecode = (*contract_bindings)
        .bytecode
        .ok_or(DeploymentError::MissingLinkedBytecode)?
        .clone();
    let name = (*contract_bindings).name;

    create_and_deploy_sim_contract(agent, sim_env, sim_data, abi, bytecode, name, args)
}

pub fn deploy_unlinked_contract_with_args<T: Tokenize>(
    agent: &mut dyn Agent,
    sim_env: &mut SimEnv,
    sim_data: &mut SimEnvData,
    contract_bindings: &BindingsWrapper,
    libraries: Vec<&BindingsWrapper>,
    args: T,
) -> Result<()> {
    let mut links: Vec<(&str, &str, EthersAddress)> = vec![];
    for lib_binding in libraries.iter() {
        links.push((
            lib_binding.path,
            lib_binding.name,
            (*sim_data
                .contract_registry
                .get(lib_binding.name)
                .ok_or(DeploymentError::NonExistentLibraryAddr)?)
            .address,
        ));
    }

    let bytecode = contract_utils::build_linked_bytecode(
        (*contract_bindings)
            .unlinked_bytecode
            .ok_or(DeploymentError::MissingUnlinkedBytecode)?,
        links,
    ).map_err(|_| DeploymentError::LinkingBytecodeFailure)?;

    let abi = (*contract_bindings).abi.clone();
    let name = (*contract_bindings).name;

    create_and_deploy_sim_contract(agent, sim_env, sim_data, abi, bytecode.into(), name, args)
}
