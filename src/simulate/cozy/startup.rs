use std::error::Error;
use std::str::FromStr;

use bindings::{
    backstop, chainlink_trigger_factory, configurator_lib, cost_model_dynamic_level_factory,
    cost_model_jump_rate_factory, cozy_router, delay_lib, demand_side_lib,
    drip_decay_model_constant_factory, manager, p_token, p_token_factory, redemption_lib, set,
    set_factory,
    shared_types::{Delays, Fees, MarketConfig, SetConfig},
    state_transitions_lib, supply_side_lib, uma_trigger_factory, weth9,
};
use ethers::{
    prelude::{H160, U256},
    types::Address,
};
use eyre::Result;
use revm::primitives::{create_address, ruint::Uint, B160};
use simulate::{
    agent::{Agent, AgentType},
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::{recast_address, unpack_execution},
};

use super::CozyProtocolParams;
use crate::simulate::cozy::{self, chronos_actions, cozy_set_admin_actions};

pub(crate) struct CozyProtocolContracts {
    pub(crate) cozy_manager: SimulationContract<IsDeployed>,
    pub(crate) set_logic: SimulationContract<IsDeployed>,
    pub(crate) set_factory: SimulationContract<IsDeployed>,
    pub(crate) p_token_logic: SimulationContract<IsDeployed>,
    pub(crate) p_token_factory: SimulationContract<IsDeployed>,
    pub(crate) backstop: SimulationContract<IsDeployed>,
    pub(crate) weth: SimulationContract<IsDeployed>,
}

pub(crate) struct CozyPeripheryContracts {
    pub(crate) cost_model_jump_rate_factory: SimulationContract<IsDeployed>,
    pub(crate) cost_model_dynamic_level_factory: SimulationContract<IsDeployed>,
    pub(crate) drip_decay_model_factory: SimulationContract<IsDeployed>,
    pub(crate) uma_trigger_factory: SimulationContract<IsDeployed>,
    pub(crate) chainlink_trigger_factory: SimulationContract<IsDeployed>,
}

pub(crate) fn run(
    manager: &mut SimulationManager,
    cozy_protocol_params: &CozyProtocolParams,
    address_iter: &mut u64,
) -> Result<(CozyProtocolContracts, CozyPeripheryContracts), Box<dyn Error>> {
    println!("=======================================");
    println!("Starting the simulation ...");
    println!("=======================================");

    println!("Deploying Cozy protocol contracts ...");
    println!("---------------------------------------");
    let cozy_protocol_contracts = deploy_cozy_protocol_contracts(manager, cozy_protocol_params)?;
    println!("---------------------------------------");
    println!("Cozy protocol contracts deployed successfully!\n");

    println!("Deploying Cozy periphery contracts ...");
    println!("---------------------------------------");
    let cozy_periphery_contracts = deploy_cozy_periphery_contracts(
        manager,
        cozy_protocol_contracts.cozy_manager.address.into(),
    )?;
    println!("---------------------------------------");
    println!("Cozy periphery contracts deployed successfully!\n");

    println!("Creating the set admin ...");
    println!("---------------------------------------");
    cozy_set_admin_actions::create_set_admin(manager, "set admin", address_iter);
    println!("---------------------------------------");
    println!("Set admin created successfully!\n");

    println!("Creating chronos agent ...");
    println!("---------------------------------------");
    chronos_actions::create_chronos(manager, "chronos", address_iter);
    println!("---------------------------------------");
    println!("Set admin created successfully!\n");

    Ok((cozy_protocol_contracts, cozy_periphery_contracts))
}

fn deploy_cozy_periphery_contracts(
    manager: &mut SimulationManager,
    cozy_manager_address: Address,
) -> Result<CozyPeripheryContracts, Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();

    let cost_model_jump_rate_factory = SimulationContract::new(
        cost_model_jump_rate_factory::COSTMODELJUMPRATEFACTORY_ABI.clone(),
        cost_model_jump_rate_factory::COSTMODELJUMPRATEFACTORY_BYTECODE.clone(),
    );
    let cost_model_jump_rate_factory =
        cost_model_jump_rate_factory.deploy(&mut manager.environment, admin, ());
    println!(
        "Cost model (jump rate) factory deployed at: {}.",
        cost_model_jump_rate_factory.address
    );

    let cost_model_dynamic_level_factory = SimulationContract::new(
        cost_model_dynamic_level_factory::COSTMODELDYNAMICLEVELFACTORY_ABI.clone(),
        cost_model_dynamic_level_factory::COSTMODELDYNAMICLEVELFACTORY_BYTECODE.clone(),
    );
    let cost_model_dynamic_level_factory =
        cost_model_dynamic_level_factory.deploy(&mut manager.environment, admin, ());
    println!(
        "Cost model (dynamic level) factory deployed at: {}.",
        cost_model_dynamic_level_factory.address
    );

    let drip_decay_model_factory = SimulationContract::new(
        drip_decay_model_constant_factory::DRIPDECAYMODELCONSTANTFACTORY_ABI.clone(),
        drip_decay_model_constant_factory::DRIPDECAYMODELCONSTANTFACTORY_BYTECODE.clone(),
    );
    let drip_decay_model_factory =
        drip_decay_model_factory.deploy(&mut manager.environment, admin, ());
    println!(
        "Drip decay model (constant) factory deployed at: {}.",
        drip_decay_model_factory.address
    );

    let chainlink_trigger_factory = SimulationContract::new(
        chainlink_trigger_factory::CHAINLINKTRIGGERFACTORY_ABI.clone(),
        chainlink_trigger_factory::CHAINLINKTRIGGERFACTORY_BYTECODE.clone(),
    );
    let chainlink_trigger_factory =
        chainlink_trigger_factory.deploy(&mut manager.environment, admin, cozy_manager_address);
    println!(
        "Chainlink trigger factory deployed at: {}.",
        chainlink_trigger_factory.address
    );

    let uma_trigger_factory = SimulationContract::new(
        uma_trigger_factory::UMATRIGGERFACTORY_ABI.clone(),
        uma_trigger_factory::UMATRIGGERFACTORY_BYTECODE.clone(),
    );
    let uma_trigger_factory = uma_trigger_factory.deploy(
        &mut manager.environment,
        admin,
        (cozy_manager_address, cozy_manager_address),
    );
    println!(
        "UMA trigger factory deployed at: {}.",
        uma_trigger_factory.address
    );

    Ok((CozyPeripheryContracts {
        cost_model_jump_rate_factory,
        cost_model_dynamic_level_factory,
        drip_decay_model_factory,
        uma_trigger_factory,
        chainlink_trigger_factory,
    }))
}

fn deploy_cozy_protocol_contracts(
    manager: &mut SimulationManager,
    cozy_protocol_params: &CozyProtocolParams,
) -> Result<CozyProtocolContracts, Box<dyn Error>> {
    let admin = manager.agents.get("admin").unwrap();
    let admin_address = admin.inner().address();

    // Deploy wETH
    let weth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let weth = weth.deploy(&mut manager.environment, admin, ());
    println!("wETH deployed at: {}", weth.address);

    // Deploy stETH
    let steth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let steth = steth.deploy(&mut manager.environment, admin, ());
    println!("stETH deployed at: {}", steth.address);

    // Deploy wstETH
    let wsteth = SimulationContract::new(weth9::WETH9_ABI.clone(), weth9::WETH9_BYTECODE.clone());
    let wsteth = wsteth.deploy(&mut manager.environment, admin, ());
    println!("wstEth deployed at: {}", wsteth.address);

    // Deploy libraries
    let configurator_lib = SimulationContract::new(
        configurator_lib::CONFIGURATORLIB_ABI.clone(),
        configurator_lib::CONFIGURATORLIB_BYTECODE.clone(),
    );
    let configurator_lib = configurator_lib.deploy(&mut manager.environment, admin, ());
    println!("Configurator lib deployed at: {}", configurator_lib.address);

    let delay_lib = SimulationContract::new(
        delay_lib::DELAYLIB_ABI.clone(),
        delay_lib::DELAYLIB_BYTECODE.clone(),
    );
    let delay_lib = delay_lib.deploy(&mut manager.environment, admin, ());
    println!("Delay lib deployed at: {}", delay_lib.address);

    let demand_side_lib = SimulationContract::new(
        demand_side_lib::DEMANDSIDELIB_ABI.clone(),
        demand_side_lib::DEMANDSIDELIB_BYTECODE.clone(),
    );
    let demand_side_lib = demand_side_lib.deploy(&mut manager.environment, admin, ());
    println!("Demand side lib deployed at: {}", demand_side_lib.address);

    let redemption_lib = SimulationContract::new(
        redemption_lib::REDEMPTIONLIB_ABI.clone(),
        redemption_lib::REDEMPTIONLIB_BYTECODE.clone(),
    );
    let redemption_lib = redemption_lib.deploy(&mut manager.environment, admin, ());
    println!("Redemption lib deployed at: {}", redemption_lib.address);

    let state_transitions_lib = SimulationContract::new(
        state_transitions_lib::STATETRANSITIONSLIB_ABI.clone(),
        state_transitions_lib::STATETRANSITIONSLIB_BYTECODE.clone(),
    );
    let state_transitions_lib = state_transitions_lib.deploy(&mut manager.environment, admin, ());
    println!(
        "State transitions lib deployed at: {}",
        state_transitions_lib.address
    );

    let supply_side_lib = SimulationContract::new(
        supply_side_lib::SUPPLYSIDELIB_ABI.clone(),
        supply_side_lib::SUPPLYSIDELIB_BYTECODE.clone(),
    );
    let supply_side_lib = supply_side_lib.deploy(&mut manager.environment, admin, ());
    println!("Supply side lib deployed at: {}", supply_side_lib.address);

    let current_nonce = match admin {
        AgentType::User(inner) => 10,
        _ => 10,
    };
    println!("Current nonce: {}", current_nonce);
    // Pre-compute Cozy protocol addresses
    let computed_address_manager = recast_address(create_address(admin_address, current_nonce));
    let computed_address_set_logic =
        recast_address(create_address(admin_address, current_nonce + 1));
    // current_nonce + 2 is initialization of the Set logic.
    let computed_address_set_factory =
        recast_address(create_address(admin_address, current_nonce + 3));
    let computed_address_p_token_logic =
        recast_address(create_address(admin_address, current_nonce + 4));
    // current_nonce + 5 is initialization of the PToken logic.
    let computed_address_p_token_factory =
        recast_address(create_address(admin_address, current_nonce + 6));
    let computed_address_backstop =
        recast_address(create_address(admin_address, current_nonce + 7));

    // Deploy Cozy protocol contracts
    let cozy_manager = SimulationContract::new(
        manager::MANAGER_ABI.clone(),
        manager::MANAGER_BYTECODE.clone(),
    );
    let cozy_manager = cozy_manager.deploy(
        &mut manager.environment,
        admin,
        (
            computed_address_manager,
            computed_address_set_factory,
            cozy_protocol_params.owner,
            cozy_protocol_params.pauser,
            cozy_protocol_params.delays.clone(),
            cozy_protocol_params.fees.clone(),
            cozy_protocol_params.allowed_markets_per_set,
        ),
    );
    println!("Cozy manager deployed at: {}", cozy_manager.address);
    assert!(cozy_manager.address == computed_address_manager.into());

    let set_logic = SimulationContract::new(set::SET_ABI.clone(), set::SET_BYTECODE.clone());
    let set_logic = set_logic.deploy(
        &mut manager.environment,
        admin,
        (
            computed_address_manager,
            computed_address_p_token_factory,
            computed_address_backstop,
        ),
    );
    println!("Set logic deployed at: {}", set_logic.address);
    assert!(set_logic.address == computed_address_set_logic.into());

    let zero_address = H160::from_str("0x0000000000000000000000000000000000000000").unwrap();
    let empty_market_configs: Vec<MarketConfig> = Vec::new();
    admin.call_contract(
        &mut manager.environment,
        &set_logic,
        set_logic.encode_function(
            "initialize",
            (
                zero_address,
                zero_address,
                recast_address(weth.address),
                SetConfig {
                    deposit_fee: 0,
                    leverage_factor: 0,
                },
                empty_market_configs,
            ),
        )?,
        Uint::from(0),
    );
    println!("Set logic initialized");

    let set_factory = SimulationContract::new(
        set_factory::SETFACTORY_ABI.clone(),
        set_factory::SETFACTORY_BYTECODE.clone(),
    );
    let set_factory = set_factory.deploy(
        &mut manager.environment,
        admin,
        (computed_address_manager, computed_address_set_logic),
    );
    println!("Set factory deployed at: {}", set_factory.address);
    assert!(set_factory.address == computed_address_set_factory.into());

    let p_token_logic = SimulationContract::new(
        p_token::PTOKEN_ABI.clone(),
        p_token::PTOKEN_BYTECODE.clone(),
    );
    let p_token_logic =
        p_token_logic.deploy(&mut manager.environment, admin, (computed_address_manager,));
    println!("PToken logic deployed at: {}", p_token_logic.address);
    assert!(p_token_logic.address == computed_address_p_token_logic.into());

    admin.call_contract(
        &mut manager.environment,
        &p_token_logic,
        p_token_logic.encode_function("initialize", (zero_address, zero_address, 0_u8))?,
        Uint::from(0),
    );
    println!("Ptoken logic initialized");

    let p_token_factory = SimulationContract::new(
        p_token_factory::PTOKENFACTORY_ABI.clone(),
        p_token_factory::PTOKENFACTORY_BYTECODE.clone(),
    );
    let p_token_factory = p_token_factory.deploy(
        &mut manager.environment,
        admin,
        (computed_address_p_token_logic,),
    );
    println!("PToken factory deployed at: {}", p_token_factory.address);
    assert!(p_token_factory.address == computed_address_p_token_factory.into());

    let backstop = SimulationContract::new(
        backstop::BACKSTOP_ABI.clone(),
        backstop::BACKSTOP_BYTECODE.clone(),
    );
    let backstop = backstop.deploy(
        &mut manager.environment,
        admin,
        (computed_address_manager, recast_address(weth.address)),
    );
    println!("Backstop deployed at: {}", backstop.address);

    let cozy_router = SimulationContract::new(
        cozy_router::COZYROUTER_ABI.clone(),
        cozy_router::COZYROUTER_BYTECODE.clone(),
    );
    let cozy_router = cozy_router.deploy(
        &mut manager.environment,
        admin,
        (
            computed_address_manager,
            recast_address(weth.address),
            recast_address(steth.address),
            recast_address(wsteth.address),
        ),
    );
    println!("Cozy router deployed at: {}", cozy_router.address);

    Ok(CozyProtocolContracts {
        cozy_manager,
        set_logic,
        set_factory,
        p_token_logic,
        p_token_factory,
        backstop,
        weth,
    })
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use std::str::FromStr;

    use ethers::{abi::Address, prelude::BaseContract, types::H160, utils::parse_ether};
    use tokio::sync::mpsc::error;

    use super::*;

    #[test]
    fn create_pair() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = SimulationManager::new();
        let address = B160::from_low_u64_be(99);
        let address = H160::from(address);
        let cozy_protocol_params = CozyProtocolParams {
            owner: address,
            pauser: address,
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
        deploy_cozy_protocol_contracts(&mut manager, &cozy_protocol_params)?;
        Ok(())
    }
}
