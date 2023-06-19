use std::{collections::HashMap, error::Error};

use agents::{
    protocol_deployer::{ProtocolDeployer, ProtocolDeployerParams},
    set_admin::{SetAdmin, SetAdminParams},
    token_deployer::TokenDeployer,
    weth_deployer::WethDeployer,
};
use bindings::cozy_protocol::shared_types::{Delays, Fees};
pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
pub use bindings_wrapper::*;
pub use ethers::types::{Bytes as EthersBytes, H160 as EthersAddress};
use ethers::types::{U128 as EthersU128, U256 as EthersU256};
use eyre::Result;
use rand::{rngs::StdRng, SeedableRng};
use revm::primitives::U256 as EvmU256;
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};
use simulate::{
    manager::SimManager, state::SimState, time_policy::FixedTimePolicy, utils::float_to_wad,
};
use world::CozyWorld;

use self::{
    agents::{
        cost_models_deployer::CostModelsDeployer,
        drip_decay_models_deployer::DripDecayModelsDeployer, passive_buyer::PassiveBuyer,
        passive_supplier::PassiveSupplier, triggers_deployer::TriggersDeployer,
    },
    bindings_wrapper::MANAGER,
    constants::{DUMMYTOKEN_DEPLOYER, PASSIVE_SUPPLIER, SET_ADMIN, WETH_DEPLOYER},
    types::{CozyCostModelType, CozyDripDecayModelType, CozyTokenDeployParams, CozyTriggerType},
};
use crate::cozy::constants::*;

pub mod agents;
pub mod bindings_wrapper;
pub mod constants;
pub mod types;
pub mod utils;
pub mod world;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut rng = StdRng::seed_from_u64(88_u64);

    // Create sim manager.
    let world_state = CozyWorld::new();
    let sim_state = SimState::new(world_state);

    let time_policy = Box::new(FixedTimePolicy::new(
        EvmU256::from(10),
        EvmU256::from(1200),
        12_u64,
        10_u64,
        Some(500_u64),
        None,
    )?);

    let mut sim_manager = SimManager::new(sim_state, time_policy);

    // Create and activate agents.

    // Weth deployer.
    let weth_deployer = Box::new(WethDeployer::new(
        Some(WETH_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
    ));
    sim_manager.activate_agent(weth_deployer);

    // Protocol deployer.
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
        allowed_markets_per_set: EthersU256::from(1000),
    };
    let protocol_deployer = Box::new(ProtocolDeployer::new(
        Some(PROTOCOL_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
        deploy_params,
    ));
    sim_manager.activate_agent(protocol_deployer);

    let protocol_contracts = sim_manager
        .stepper
        .sim_state_writer()
        .world
        .protocol_contracts;

    // Cost Models deployer
    let mut cost_models = HashMap::new();
    cost_models.insert(
        "Basic".into(),
        CozyCostModelType::JumpRate(cost_model_jump_rate_factory::DeployModelCall {
            kink: float_to_wad(0.8),
            cost_factor_at_full_utilization: float_to_wad(0.95),
            cost_factor_at_kink_utilization: float_to_wad(0.8),
            cost_factor_at_zero_utilization: float_to_wad(0.01),
        }),
    );
    let cost_models_deployer = Box::new(CostModelsDeployer::new(
        Some(COST_MODELS_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
        cost_models,
        protocol_contracts
            .get(COSTMODELJUMPRATEFACTORY.name)
            .unwrap(),
        protocol_contracts
            .get(COSTMODELDYNAMICLEVELFACTORY.name)
            .unwrap(),
    ));
    sim_manager.activate_agent(cost_models_deployer);

    // Drip decay models deployer
    let mut drip_decay_models = HashMap::new();
    drip_decay_models.insert(
        "Basic".into(),
        CozyDripDecayModelType::Constant(drip_decay_model_constant_factory::DeployModelCall {
            rate_per_second: float_to_wad(0.8),
        }),
    );

    let drip_decay_models_deployer = Box::new(DripDecayModelsDeployer::new(
        Some(DRIP_DECAY_MODELS_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
        drip_decay_models,
        protocol_contracts
            .get(DRIPDECAYMODELCONSTANTFACTORY.name)
            .unwrap(),
    ));
    sim_manager.activate_agent(drip_decay_models_deployer);

    // Triggers deployer
    let mut triggers = HashMap::new();
    triggers.insert("Basic".into(), CozyTriggerType::DummyTrigger);
    let triggers_deployer = Box::new(TriggersDeployer::new(
        Some(TRIGGERS_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
        triggers,
        protocol_contracts.get(UMATRIGGERFACTORY.name).unwrap(),
        protocol_contracts
            .get(CHAINLINKTRIGGERFACTORY.name)
            .unwrap(),
        protocol_contracts.get(MANAGER.name).unwrap(),
    ));
    sim_manager.activate_agent(triggers_deployer);

    let supplier_addr = EvmAddress::random_using(&mut rng);
    let supplier_addr2 = EvmAddress::random_using(&mut rng);
    let buyer_addr = EvmAddress::random_using(&mut rng);
    let buyer_addr2 = EvmAddress::random_using(&mut rng);

    let mut allocate_addrs = HashMap::new();
    allocate_addrs.insert(supplier_addr, EthersU256::from(88));
    allocate_addrs.insert(supplier_addr2, EthersU256::from(880000000));
    allocate_addrs.insert(buyer_addr, EthersU256::from(99999));
    allocate_addrs.insert(buyer_addr2, EthersU256::from(99999));
    let token_deployer = Box::new(TokenDeployer::new(
        Some(DUMMYTOKEN_DEPLOYER.into()),
        EvmAddress::random_using(&mut rng),
        CozyTokenDeployParams {
            name: "Random Dummy Token".to_string(),
            symbol: "RDM".to_string(),
            decimals: 16_u8,
        },
        allocate_addrs,
    ));
    sim_manager.activate_agent(token_deployer);

    let world = sim_manager.stepper.sim_state_writer().world;
    let protocol_contracts = sim_manager
        .stepper
        .sim_state_writer()
        .world
        .protocol_contracts;

    let trigger_addr = world.triggers.get("Basic").unwrap().address;
    let cost_model_addr = world.cost_models.get("Basic").unwrap().address;
    let drip_decay_model_addr = world.drip_decay_models.get("Basic").unwrap().address;
    let dummy_token_addr = protocol_contracts.get(DUMMYTOKEN.name).unwrap().address;

    let salt: Option<[u8; 32]> = Some(rand::random());
    let market_configs = vec![MarketConfig {
        trigger: trigger_addr.into(),
        cost_model: cost_model_addr.into(),
        drip_decay_model: drip_decay_model_addr.into(),
        weight: 10000_u16,
        purchase_fee: 0_u16,
        sale_fee: 0_u16,
    }];

    let set_params = SetAdminParams {
        asset: EthersAddress::from(*dummy_token_addr),
        set_config: SetConfig {
            leverage_factor: 10000_u32,
            deposit_fee: 0_u16,
        },
        market_configs,
        salt,
    };

    let set_admin = Box::new(SetAdmin::new(
        Some(SET_ADMIN.into()),
        EvmAddress::random_using(&mut rng),
        set_params,
        protocol_contracts.get(SET.name).unwrap(),
        protocol_contracts.get(MANAGER.name).unwrap(),
    ));
    sim_manager.activate_agent(set_admin);

    let passive_supplier = Box::new(PassiveSupplier::new(
        Some(PASSIVE_SUPPLIER.into()),
        supplier_addr,
        protocol_contracts.get(COZYROUTER.name).unwrap(),
        protocol_contracts.get(DUMMYTOKEN.name).unwrap(),
        EthersU256::from(90000),
    ));
    sim_manager.activate_agent(passive_supplier);

    let passive_supplier2 = Box::new(PassiveSupplier::new(
        Some((PASSIVE_SUPPLIER.to_owned() + "te").into()),
        supplier_addr2,
        protocol_contracts.get(COZYROUTER.name).unwrap(),
        protocol_contracts.get(DUMMYTOKEN.name).unwrap(),
        EthersU256::from(77),
    ));
    sim_manager.activate_agent(passive_supplier2);

    let passive_buyer = Box::new(PassiveBuyer::new(
        Some(PASSIVE_BUYER.into()),
        buyer_addr,
        protocol_contracts.get(COZYROUTER.name).unwrap(),
        protocol_contracts.get(DUMMYTOKEN.name).unwrap(),
        protocol_contracts.get(SET.name).unwrap(),
        vec![trigger_addr],
        vec![EthersU256::from(900)],
        EthersU256::from(900000000),
    ));
    sim_manager.activate_agent(passive_buyer);

    let passive_buyer2 = Box::new(PassiveBuyer::new(
        Some((PASSIVE_BUYER.to_owned() + "2").into()),
        buyer_addr2,
        protocol_contracts.get(COZYROUTER.name).unwrap(),
        protocol_contracts.get(DUMMYTOKEN.name).unwrap(),
        protocol_contracts.get(SET.name).unwrap(),
        vec![trigger_addr],
        vec![EthersU256::from(100)],
        EthersU256::from(100000000),
    ));
    sim_manager.activate_agent(passive_buyer2);

    sim_manager.run_sim();
    Ok(())
}
