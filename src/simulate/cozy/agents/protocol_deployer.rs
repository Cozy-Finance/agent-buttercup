use bindings::{
    backstop, chainlink_trigger_factory, configurator_lib, cost_model_dynamic_level_factory,
    cost_model_jump_rate_factory, cozy_router, delay_lib, demand_side_lib,
    drip_decay_model_constant_factory, manager, p_token, p_token_factory, redemption_lib, set,
    set_factory,
    shared_types::{Delays, Fees, MarketConfig, SetConfig},
    state_transitions_lib, supply_side_lib, uma_trigger_factory, weth9,
};
use simulate::agent::{Agent, SimulationEnvironment};

pub struct ProtocolDeployer {
    name: String
}

impl ProtocolDeployer {
    fn new(name: String) -> Self {
        Self {name}
    }
}

impl Agent for ProtocolDeployer {

    fn name(&self) -> Option<String> {
        Ok(self.name)
    }

    fn activation_step(&self, simulation_environment: &mut SimulationEnvironment) {

    }

    fn deploy_

    fn step(&self, simulation_environment: &mut SimulationEnvironment) {}
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
