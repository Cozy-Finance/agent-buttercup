use std::{collections::BTreeMap, error::Error};

use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::U256 as EvmU256;
use simulate::{
    block_time_policy::FixedBlockTimePolicy, environment::sim_env::SimEnv, manager::SimManager,
};

pub use ethers::types::{Bytes as EthersBytes, H160 as EthersAddress};
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};

use agents::protocol_deployer::{ProtocolDeployer, ProtocolDeployerParams};

pub mod agents;
pub mod bindings_wrapper;
pub mod sim_types;

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let sim_env = SimEnv::new();
    let time_policy = Box::new(FixedBlockTimePolicy::new(
        EvmU256::from(0),
        EvmU256::from(1),
        12_u64,
        10_u64,
        Some(500_000_u64),
        None,
    )?);
    let mut sim_manager = SimManager::new(sim_env, time_policy, 99_u64);

    // Create and activate agents.
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

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn run_sim() {
        run();
    }
}
