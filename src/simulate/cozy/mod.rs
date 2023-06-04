#![warn(missing_docs)]
use std::error::Error;

use ethers::types::{Address, H160, U256};
use eyre::Result;
use revm::primitives::B160;
use ruint::Uint;
use simulate::{
    agent::Agent,
    contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{float_to_wad, unpack_execution},
};

pub mod cozy_passive_buyer_actions;
pub mod cozy_passive_supplier_actions;
pub mod cozy_set_admin_actions;
pub mod startup;

/*
pub struct CozyProtocolParams {
    pub owner: Address,
    pub pauser: Address,
    pub delays: Delays,
    pub fees: Fees,
    pub allowed_markets_per_set: U256,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create a `SimulationManager` that runs simulations in their `SimulationEnvironment`.
    // let mut manager = SimulationManager::new();
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

    Ok(())
}

*/

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    use super::*;
}
