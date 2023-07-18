use std::error::Error;

use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory, dummy_trigger, manager,
    shared_types::{MarketConfig, SetConfig},
};
use ethers::{
    prelude::U256,
    types::{Sign, H160, I256},
};
use eyre::Result;
use rand;
use revm::primitives::{ruint::Uint, Address, B160};
use simulate::{
    agent::{chronos::Chronos, Agent, AgentType, IsActive, SimulationEventFilter},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

pub(crate) fn create_chronos<S: Into<String>>(
    manager: &mut SimulationManager,
    name: S,
    address_iter: &mut u64,
) {
    let address = B160::from_low_u64_be(address_iter.clone());
    *address_iter += 1;
    let event_filters = vec![];
    let chronos = Chronos::new(name, event_filters);
    manager
        .activate_agent(AgentType::Chronos(chronos), address)
        .unwrap();
    println!("Created a Chronos agent at address: {}.", address);
}

pub(crate) fn update_block_and_block_timestamp(
    manager: &mut SimulationManager,
) -> Result<(), Box<dyn Error>> {
    let chronos = manager.agents.get("chronos").unwrap();
    chronos.update_block_and_block_timestamp(
        &mut manager.environment,
        U256::from(1).into(),
        U256::from(12).into(),
    );
    Ok(())
}
