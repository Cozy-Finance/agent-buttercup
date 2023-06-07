use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::Agent,
    contract::{
        sim_contract::{IsDeployed, SimContract},
        utils,
    },
    environment::sim_env::SimEnv,
    sim_env_data::SimEnvData,
};
use thiserror::Error;

use crate::simulator::cozy::deploy_utils;
use crate::simulator::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress},
};

pub struct AssetDeployer {
    name: String,
    address: Option<EvmAddress>,
}

impl AssetDeployer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            address: None,
        }
    }
}

impl Agent for AssetDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn name(&self) -> Option<String> {
        Option::Some(self.name.clone())
    }

    fn activation_step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) {
        // Deploy external libraries.
        self.deploy_weth(sim_env, sim_data);
    }

    fn step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) {}
}

impl AssetDeployer {
    fn deploy_weth(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) -> Result<()> {
        deploy_utils::deploy_linked_contract_with_args(self, sim_env, sim_data, &WETH, ())?;

        Ok(())
    }
}
