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
    agent::{cozy_set_admin::CozySetAdmin, Agent, AgentType, IsActive, SimulationEventFilter},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

pub(crate) fn create_set_admin<S: Into<String>>(
    manager: &mut SimulationManager,
    name: S,
    address_iter: &mut u64,
) {
    let address = B160::from_low_u64_be(address_iter.clone());
    *address_iter += 1;
    let event_filters = vec![];
    let cozy_set_admin = CozySetAdmin::new(name, event_filters);
    manager
        .activate_agent(AgentType::CozySetAdmin(cozy_set_admin), address)
        .unwrap();
    println!("Created a Cozy set admin at address: {}.", address);
}

pub(crate) fn deploy_cost_model_jump_rate(
    manager: &mut SimulationManager,
    cost_model_jump_rate_factory: &SimulationContract<IsDeployed>,
    param_kink: U256,
    param_cost_factor_at_zero_utilization: U256,
    param_cost_factor_at_kink_utilization: U256,
    param_cost_factor_at_full_utilization: U256,
) -> Result<H160, Box<dyn Error>> {
    let set_admin = manager.agents.get("set admin").unwrap();
    let deploy_cost_model_args = cost_model_jump_rate_factory::DeployModelCall {
        kink: param_kink,
        cost_factor_at_zero_utilization: param_cost_factor_at_zero_utilization,
        cost_factor_at_kink_utilization: param_cost_factor_at_kink_utilization,
        cost_factor_at_full_utilization: param_cost_factor_at_full_utilization,
    };
    let deploy_cost_model_result = set_admin.call_contract(
        &mut manager.environment,
        cost_model_jump_rate_factory,
        cost_model_jump_rate_factory.encode_function("deployModel", deploy_cost_model_args)?,
        Uint::from(0),
    );
    let unpacked_deploy_cost_model_result = unpack_execution(deploy_cost_model_result)?;
    let deployed_cost_model_address: H160 = cost_model_jump_rate_factory
        .decode_output("deployModel", unpacked_deploy_cost_model_result)?;
    println!(
        "Cozy set admin \"{}\" deployed a cost model (jump rate) at address: {}.",
        set_admin.inner().name(),
        deployed_cost_model_address
    );

    Ok(deployed_cost_model_address)
}

pub(crate) fn deploy_cost_model_dynamic_level(
    manager: &mut SimulationManager,
    cost_model_dynamic_level_factory: &SimulationContract<IsDeployed>,
    param_u_low: U256,
    param_u_high: U256,
    param_cost_factor_at_zero_utilization: U256,
    param_cost_factor_at_full_utilization: U256,
    param_cost_factor_in_optimal_zone: U256,
    param_optimal_zone_rate: U256,
) -> Result<H160, Box<dyn Error>> {
    let set_admin = manager.agents.get("set admin").unwrap();
    let deploy_cost_model_args = cost_model_dynamic_level_factory::DeployModelCall {
        u_low: param_u_low,
        u_high: param_u_high,
        cost_factor_at_zero_utilization: param_cost_factor_at_zero_utilization,
        cost_factor_at_full_utilization: param_cost_factor_at_full_utilization,
        cost_factor_in_optimal_zone: param_cost_factor_in_optimal_zone,
        optimal_zone_rate: param_optimal_zone_rate,
    };
    let deploy_cost_model_result = set_admin.call_contract(
        &mut manager.environment,
        cost_model_dynamic_level_factory,
        cost_model_dynamic_level_factory.encode_function("deployModel", deploy_cost_model_args)?,
        Uint::from(0),
    );
    let unpacked_deploy_cost_model_result = unpack_execution(deploy_cost_model_result)?;
    let deployed_cost_model_address: H160 = cost_model_dynamic_level_factory
        .decode_output("deployModel", unpacked_deploy_cost_model_result)?;
    println!(
        "Cozy set admin \"{}\" deployed a cost model (dynamic level) at address: {}.",
        set_admin.inner().name(),
        deployed_cost_model_address
    );

    Ok(deployed_cost_model_address)
}

pub(crate) fn deploy_drip_decay_model(
    manager: &mut SimulationManager,
    drip_decay_model_factory: &SimulationContract<IsDeployed>,
    param_rate_per_second: U256,
) -> Result<H160, Box<dyn Error>> {
    let set_admin = manager.agents.get("set admin").unwrap();
    let deploy_drip_decay_model_args = drip_decay_model_constant_factory::DeployModelCall {
        rate_per_second: param_rate_per_second,
    };
    let deply_drip_decay_model_result = set_admin.call_contract(
        &mut manager.environment,
        drip_decay_model_factory,
        drip_decay_model_factory.encode_function("deployModel", deploy_drip_decay_model_args)?,
        Uint::from(0),
    );
    let unpacked_deploy_drip_decay_model_result = unpack_execution(deply_drip_decay_model_result)?;
    let deployed_drip_decay_model_address: H160 = drip_decay_model_factory
        .decode_output("deployModel", unpacked_deploy_drip_decay_model_result)?;
    println!(
        "Cozy set admin \"{}\" deployed a drip decay model (constant) at address: {}.",
        set_admin.inner().name(),
        deployed_drip_decay_model_address
    );

    Ok(deployed_drip_decay_model_address)
}

pub(crate) fn deploy_dummy_trigger(
    manager: &mut SimulationManager,
    cozy_manager: &SimulationContract<IsDeployed>,
) -> Result<H160, Box<dyn Error>> {
    let set_admin = manager.agents.get("set admin").unwrap();

    let dummy_trigger = SimulationContract::new(
        dummy_trigger::DUMMYTRIGGER_ABI.clone(),
        dummy_trigger::DUMMYTRIGGER_BYTECODE.clone(),
    );
    let dummy_trigger = dummy_trigger.deploy(
        &mut manager.environment,
        set_admin,
        recast_address(cozy_manager.address),
    );
    let dummy_trigger_address = recast_address(dummy_trigger.address);
    println!(
        "Cozy set admin \"{}\" deployed a dummy trigger at address: {}.",
        set_admin.inner().name(),
        dummy_trigger_address
    );

    Ok(dummy_trigger_address)
}

pub(crate) fn deploy_set(
    manager: &mut SimulationManager,
    cozy_manager: &SimulationContract<IsDeployed>,
    param_asset: Address,
    param_set_config: SetConfig,
    param_market_configs: Vec<MarketConfig>,
    param_salt: [u8; 32],
) -> Result<H160, Box<dyn Error>> {
    let set_admin = manager.agents.get("set admin").unwrap();
    let deploy_set_args = manager::CreateSetCall {
        owner: set_admin.address().into(),
        pauser: set_admin.address().into(),
        asset: param_asset.into(),
        set_config: param_set_config,
        market_configs: param_market_configs,
        salt: param_salt,
    };
    let deploy_set_result = set_admin.call_contract(
        &mut manager.environment,
        cozy_manager,
        cozy_manager.encode_function("createSet", deploy_set_args)?,
        Uint::from(0),
    );
    let unpacked_deploy_set_result = unpack_execution(deploy_set_result)?;
    let deployed_set_address: H160 =
        cozy_manager.decode_output("createSet", unpacked_deploy_set_result)?;
    println!(
        "Cozy set admin \"{}\" deployed a set at address: {}.",
        set_admin.inner().name(),
        deployed_set_address
    );

    Ok(deployed_set_address)
}
