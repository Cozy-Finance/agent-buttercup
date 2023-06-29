use std::{collections::HashMap, sync::Arc};

use ethers::abi::Tokenize;
use eyre::Result;
use revm::primitives::TxEnv;
use serde::Deserialize;
use simulate::{
    address::Address,
    contract::{sim_contract::SimContract, utils as contract_utils},
    utils::{build_call_contract_txenv, build_deploy_contract_txenv},
};
use thiserror::Error;

use super::types::CozyCostModelType;
use crate::cozy::{
    bindings_wrapper::*,
    types::{CozyPassiveBuyersParams, CozyTriggerType},
    world::CozyProtocolContract,
    EthersAddress, EthersBytes, EthersU256,
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
    agent_address: Address,
    contract_bindings: &BindingsWrapper,
    args: T,
) -> Result<(TxEnv, SimContract)> {
    let abi = contract_bindings.abi.clone();
    let bytecode = contract_bindings
        .bytecode
        .ok_or(DeploymentError::MissingLinkedBytecode)?
        .clone();
    let contract = SimContract::new(abi, bytecode);
    let bytecode = contract.encode_constructor(args)?;

    Ok((
        build_deploy_contract_txenv(agent_address, bytecode, None, None),
        contract,
    ))
}

pub fn build_unlinked_deploy_contract_tx<T: Tokenize>(
    agent_address: Address,
    contract_bindings: &BindingsWrapper,
    libraries: &HashMap<Address, &BindingsWrapper>,
    args: T,
) -> Result<(TxEnv, SimContract)> {
    let mut links: Vec<(&str, &str, EthersAddress)> = vec![];
    for (addr, lib_binding) in libraries.iter() {
        links.push((lib_binding.path, lib_binding.name, (*addr).into()));
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

    Ok((
        build_deploy_contract_txenv(agent_address, bytecode, None, None),
        contract,
    ))
}

pub fn build_call_protocol_contract_tx<T: Tokenize>(
    agent_address: Address,
    contract_data: &Arc<CozyProtocolContract>,
    func_name: &str,
    args: T,
) -> Result<TxEnv> {
    Ok(build_call_contract_txenv(
        agent_address,
        contract_data.as_ref().address.into(),
        contract_data
            .as_ref()
            .contract
            .encode_function(func_name, args)?,
        None,
        None,
    ))
}

pub struct Counter {
    count: u64,
}

impl Counter {
    pub fn new(start_value: u64) -> Counter {
        Counter { count: start_value }
    }

    pub fn get_and_increment_count(&mut self) -> u64 {
        let count: u64 = self.count;
        self.count += 1;
        count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
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

#[derive(Deserialize, Debug)]
pub struct Test {
    pub cost_models: Vec<(String, CozyCostModelType)>,
}

pub fn get_config(analysis_config_file: &str) -> Result<(), config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let configs_dir = base_path.join("src/cozy/configs");
    println!("{:?}", configs_dir);
    let settings = config::Config::builder()
        .add_source(config::File::from(configs_dir.join(analysis_config_file)))
        .build()?;
    println!("{:?}", settings);
    let x = settings.try_deserialize::<Test>();

    println!("{:?}", x);

    Ok(())
}
