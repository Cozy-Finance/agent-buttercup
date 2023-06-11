use eyre::Result;
use simulate::{agent::Agent, environment::sim_env::SimEnv, sim_env_data::SimEnvData};

use crate::cozy::{bindings_wrapper::*, deploy_utils, EvmAddress, world_state::CozyUpdate};

pub struct WethDeployer {
    name: String,
    address: Option<EvmAddress>,
}

impl WethDeployer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            address: None,
        }
    }
}

impl Agent<CozyUpdate> for WethDeployer {
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

impl WethDeployer {
    fn deploy_weth(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) -> Result<()> {
        deploy_utils::deploy_linked_contract_with_args(self, sim_env, sim_data, &WETH, ())?;
        Ok(())
    }
}
