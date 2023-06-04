use bindings::{
    configurator_lib::*, 
    delay_lib::*, 
    demand_side_lib::*, 
    redemption_lib::*,
    state_transitions_lib::*,
    supply_side_lib::*,
    manager::*,
    set::*,
    set_factory::*,
    p_token::*
};
use simulate::{
    agent::Agent,
    environment::sim_environment::SimulationEnvironment,
    contract::sim_contract::{IsDeployed, SimulationContract},
    utils::recast_address
};
use revm::primitives::{create_address, B160, U256};

#[derive(Debug, Clone)]
pub struct ProtocolDeployParams {
    owner: B160,
    pauser: B160,
    delays: Delays,
    fees: Fees,
    allowed_markets_per_set: U256,
}

pub struct ProtocolDeployer {
    name: String,
    deploy_params: ProtocolDeployParams
}

impl ProtocolDeployer {
    fn new(name: String, deploy_params: ProtocolDeployParams) -> Self {
        Self { name , deploy_params }
    }
}

impl Agent for ProtocolDeployer {
    fn name(&self) -> Option<String> {
        Ok(self.name)
    }

    fn activation_step(&self, simulation_environment: &mut SimulationEnvironment) {
        // Deploy external libraries.
        self.deploy_libraries(simulation_environment);
        // Deploy core protocol.
        self.deploy_core_protocol(simulation_environment);
    }

    fn step(&self, simulation_environment: &mut SimulationEnvironment) {}
}

impl ProtocolDeployer {
    fn deploy_libraries(&self, simulation_environment: &mut SimulationEnvironment) {
        let configurator_lib = SimulationContract::new(
            CONFIGURATORLIB_ABI.clone(),
            CONFIGURATORLIB_BYTECODE.clone(),
        );
        let configurator_lib = self.deploy_contract(
            &mut simulation_environment,
            &configurator_lib,
            (),
        );
        println!("Configurator lib deployed at: {}.", configurator_lib.address);

        let delay_lib = SimulationContract::new(
            DELAYLIB_ABI.clone(),
            DELAYLIB_BYTECODE.clone(),
        );
        let delay_lib = self.deploy_contract(
            &mut simulation_environment,
            &delay_lib,
            (),
        );
        println!("Delay lib deployed at: {}.", configurator_lib.address);

        let demand_side_lib = SimulationContract::new(
            DEMANDSIDELIB_ABI.clone(),
            DEMANDSIDELIB_BYTECODE.clone(),
        );
        let demand_side_lib = self.deploy_contract(
            &mut simulation_environment,
            &demand_side_lib,
            (),
        );
        println!("Demand side lib deployed at: {}.", demand_side_lib.address);

        let redemption_lib = SimulationContract::new(
            REDEMPTIONLIB_ABI.clone(),
            REDEMPTIONLIB_BYTECODE.clone(),
        );
        let redemption_lib = self.deploy_contract(
            &mut simulation_environment,
            &redemption_lib,
            (),
        );
        println!("Redemption lib deployed at: {}.", redemption_lib.address);

        let state_transitions_lib = SimulationContract::new(
            STATETRANSITIONSLIB_ABI.clone(),
            STATETRANSITIONSLIB_BYTECODE.clone(),
        );
        let state_transitions_lib = self.deploy_contract(
            &mut simulation_environment,
            &state_transitions_lib,
            (),
        );
        println!("State transitions lib deployed at: {}.", state_transitions_lib.address);

        let supply_side_lib = SimulationContract::new(
            SUPPLYSIDELIB_ABI.clone(),
            SUPPLYSIDELIB_BYTECODE.clone(),
        );
        let supply_side_lib = self.deploy_contract(
            &mut simulation_environment,
            &supply_side_lib,
            (),
        );
        println!("Supply side lib deployed at: {}.", supply_side_lib.address);
    }

    fn link_set_logic_bytecode(&self) {
        
    }

    fn deploy_core_protocol(&self, simulation_environment: &mut SimulationEnvironment) {
        // Pre-compute Cozy protocol addresses
        let current_nonce = self.get_nonce(simulation_environment);
        let manager_address = create_address(self.address(), current_nonce);
        let set_logic_address = create_address(self.address(), current_nonce + 1);
        // current_nonce + 2 is initialization of the Set logic.
        let set_factory_address = create_address(self.address(), current_nonce + 3);
        let p_token_logic_address = create_address(self.address(), current_nonce + 4);
        // current_nonce + 5 is initialization of the PToken logic.
        let p_token_factory_address = create_address(self.address(), current_nonce + 6);
        let backstop_address = create_address(self.address(), current_nonce + 7);
        
        let manager = SimulationContract::new(
            MANAGER_ABI.clone(),
            MANAGER_BYTECODE.clone()
        );
        let manager = self.deploy_contract(
            &mut simulation_environment,
            &manager,
            (
                manager_address,
                set_factory_address,
                self.deploy_params.owner,
                self.deploy_params.pauser,
                self.deploy_params.delays,
                self.deploy_params.fees,
                self.deploy_params.allowed_markets_per_set,
            ).to_tokens(),
        );
        println!("Cozy manager deployed at: {}.", manager.address);



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
    
        let set_factory = SimulationContract::new(
            SETFACTORY_ABI.clone(),
            SETFACTORY_BYTECODE.clone(),
        );
        let set_factory = self.deploy_contract(
            &mut simulation_environment,
            &set_factory,
            (manager_address, set_logic_address).to_tokens()
        );
    
        let p_token_logic = SimulationContract::new(
            PTOKEN_ABI.clone(),
            PTOKEN_BYTECODE.clone(),
        );
        let p_token_logic =
            p_token_logic.deploy(
                &mut simulation_environment, 
                &p_token_logic, 
                (manager_address,).to_tokens()
            );
        println!("PToken logic deployed at: {}.", p_token_logic.address);
    
        self.call_contract(
            &mut simulation_environment,
            &p_token_logic,
            p_token_logic.encode_function("initialize", (B160::ZERO, B160::ZERO, 0_u8))?,
        );
        println!("Ptoken logic initialized.");
    

    }
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
