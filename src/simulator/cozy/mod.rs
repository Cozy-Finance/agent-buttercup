use std::{collections::BTreeMap, error::Error};

use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::U256 as EvmU256;
use simulate::{
    block_time_policy::FixedBlockTimePolicy, environment::sim_env::SimEnv, manager::SimManager,
    sim_env_data::SimEnvData,
};

pub use ethers::types::{Bytes as EthersBytes, H160 as EthersAddress};
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};

use agents::protocol_deployer::{ProtocolDeployer, ProtocolDeployerParams};

use crate::simulator::cozy::agents::asset_deployer::AssetDeployer;

pub mod agents;
pub mod bindings_wrapper;
pub mod deploy_utils;
pub mod sim_types;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let sim_env = SimEnv::new();
    let sim_data = SimEnvData::new();
    let time_policy = Box::new(FixedBlockTimePolicy::new(
        EvmU256::from(0),
        EvmU256::from(1),
        12_u64,
        10_u64,
        Some(500_000_u64),
        None,
    )?);
    let mut sim_manager = SimManager::new(sim_env, sim_data, time_policy, 99_u64);

    // Create and activate agents.
    let asset_deployer = Box::new(AssetDeployer::new(
        "Asset deployer".to_owned(),
    ));
    sim_manager.activate_agent(asset_deployer);

    let deploy_params = ProtocolDeployerParams {
        delays: Delays {
            config_update_delay: EthersU256::from(172800),
            config_update_grace_period: EthersU256::from(259200),
            min_deposit_duration: EthersU256::from(86400),
            redemption_delay: EthersU256::from(43200),
            purchase_delay: EthersU256::from(57600),
        },
        fees: Fees {
            deposit_fee_reserves: 0_u16,
            deposit_fee_backstop: 0_u16,
            purchase_fee_reserves: 0_u16,
            purchase_fee_backstop: 0_u16,
            sale_fee_reserves: 0_u16,
            sale_fee_backstop: 0_u16,
        },
        allowed_markets_per_set: EthersU256::from(10),
    };
    let protocol_deployer = Box::new(ProtocolDeployer::new(
        "Protocol deployer".to_owned(),
        deploy_params,
    ));
    sim_manager.activate_agent(protocol_deployer);

    sim_manager.run_simulation();
    Ok(())
}