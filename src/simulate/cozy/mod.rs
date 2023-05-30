#![warn(missing_docs)]
use std::error::Error;

use bindings::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::types::{Address, H160, U256};
use eyre::Result;
use revm::primitives::B160;
use ruint::Uint;
use simulate::{
    agent::{Agent, AgentType},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{float_to_wad, unpack_execution},
};

pub mod chronos_actions;
pub mod cozy_set_admin_actions;
pub mod cozy_passive_buyer_actions;
pub mod cozy_passive_supplier_actions;
pub mod startup;

pub struct CozyProtocolParams {
    pub owner: Address,
    pub pauser: Address,
    pub delays: Delays,
    pub fees: Fees,
    pub allowed_markets_per_set: U256,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    let mut manager = SimulationManager::new();
    // Address iterator
    let mut address_iter: u64 = 99;
    // Define the Cozy protocol params
    let cozy_protocol_owner_address = B160::from_low_u64_be(address_iter);
    address_iter += 1;
    let cozy_protocol_params = CozyProtocolParams {
        owner: cozy_protocol_owner_address.into(),
        pauser: cozy_protocol_owner_address.into(),
        delays: Delays {
            config_update_delay: U256::from(172800),
            config_update_grace_period: U256::from(259200),
            min_deposit_duration: U256::from(86400),
            redemption_delay: U256::from(43200),
            purchase_delay: U256::from(57600),
        },
        fees: Fees {
            deposit_fee_reserves: 0_u16,
            deposit_fee_backstop: 0_u16,
            purchase_fee_reserves: 0_u16,
            purchase_fee_backstop: 0_u16,
            sale_fee_reserves: 0_u16,
            sale_fee_backstop: 0_u16,
        },
        allowed_markets_per_set: U256::from(10),
    };
    let (cozy_protocol_contracts, cozy_periphery_contracts) =
        startup::run(&mut manager, &cozy_protocol_params, &mut address_iter)?;

    // Set admin deploys a set
    let dummy_trigger_address = cozy_set_admin_actions::deploy_dummy_trigger(
        &mut manager,
        &cozy_protocol_contracts.cozy_manager,
    )?;
    let cost_model_address = cozy_set_admin_actions::deploy_cost_model_jump_rate(
        &mut manager,
        &cozy_periphery_contracts.cost_model_jump_rate_factory,
        float_to_wad(0.8),
        float_to_wad(0.01),
        float_to_wad(0.9),
        float_to_wad(0.95),
    )?;
    let drip_decay_model_address = cozy_set_admin_actions::deploy_drip_decay_model(
        &mut manager,
        &cozy_periphery_contracts.drip_decay_model_factory,
        float_to_wad(0.8),
    )?;

    let market_configs: Vec<MarketConfig> = vec![MarketConfig {
        trigger: dummy_trigger_address,
        cost_model: cost_model_address,
        drip_decay_model: drip_decay_model_address,
        weight: 10000_u16,
        purchase_fee: 0_u16,
        sale_fee: 0_u16,
    }];
    let salt: [u8; 32] = rand::random();
    let set_address = cozy_set_admin_actions::deploy_set(
        &mut manager,
        &cozy_protocol_contracts.cozy_manager,
        cozy_protocol_contracts.weth.address,
        SetConfig {
            leverage_factor: 10000_u32,
            deposit_fee: 0_u16,
        },
        market_configs,
        salt,
    )?;

    chronos_actions::update_block_and_block_timestamp(&mut manager)?;
    let market_configs: Vec<MarketConfig> = vec![MarketConfig {
        trigger: dummy_trigger_address,
        cost_model: cost_model_address,
        drip_decay_model: drip_decay_model_address,
        weight: 10000_u16,
        purchase_fee: 0_u16,
        sale_fee: 0_u16,
    }];

    cozy_set_admin_actions::deploy_set(
        &mut manager,
        &cozy_protocol_contracts.cozy_manager,
        cozy_protocol_contracts.weth.address,
        SetConfig {
            leverage_factor: 10000_u32,
            deposit_fee: 0_u16,
        },
        market_configs,
        salt,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;

    #[test]
    fn test_cozy_sim() -> Result<(), Box<dyn std::error::Error>> {
        run()?;
        Ok(())
    }
}
