use std::{borrow::Cow, collections::HashMap, sync::Arc};

use bindings::{
    chainlink_trigger_factory::*,
    cozy_models::{
        cost_model_dynamic_level::*, cost_model_dynamic_level_factory::*, cost_model_jump_rate::*,
        cost_model_jump_rate_factory::*, drip_decay_model_constant::*,
        drip_decay_model_constant_factory::*,
    },
    cozy_protocol::{
        backstop::*,
        configurator_lib::*,
        cozy_router::*,
        delay_lib::*,
        demand_side_lib::*,
        manager::*,
        metadata::*,
        p_token::*,
        p_token_factory::*,
        redemption_lib::*,
        set::*,
        set_factory::*,
        shared_types::{MarketConfig, SetConfig},
        state_transitions_lib::*,
        supply_side_lib::*,
    },
    dummy_token::*,
    dummy_trigger::*,
    uma_trigger_factory::*,
    weth::weth9::*,
};
use nalgebra::DVector;
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::Distribution;
use revm::primitives::create_address;
use serde::Deserialize;
use simulate::{
    address::Address,
    agent::Agent,
    manager::SimManager,
    state::{State, StateMiddleware},
    summarizer::Summarizer,
    time_policy::{FixedTimePolicy, TimeEnv},
    u256::{U256, *},
};
use statrs::distribution::{Dirichlet, Normal};

use crate::cozy::{
    agents::{
        altruistic_supplier::AltruisticSupplier,
        arbitrageur::{Arbitrageur, ArbitrageurPreferences},
        buyer::{Buyer, BuyerPreferences},
        set_analyzer::SetAnalyzer,
        supplier::{Supplier, SupplierPreferences},
        trigger_admin::TriggerAdmin,
    },
    constants::*,
    set_risk_model::SetRiskModel,
    summary_generators::set_summary::SetSummaryGenerator,
    types::{
        AgentSetRiskParams, AgentSetRiskSampler, ArbitrageurParams, BuyerParams, CostModelType,
        DripDecayModelType, FixedTimePolicyParams, MarketConfigParams, ProtocolDeployParams,
        ReactionTime, SetConfigParams, SimSetupParams, SupplierParams, SupplierRiskAversionSampler,
        TriggerRiskParams, TriggerSimulator, TriggerType,
    },
    utils::deserialize_cow_tuple_vec,
    world::{CozyUpdate, CozyWorld},
    EthersAddress, EthersBytes,
};

#[derive(Debug, Clone)]
pub struct ProtocolContracts {
    pub cozy_router: CozyRouter<StateMiddleware>,
    pub set_factory: SetFactory<StateMiddleware>,
    pub set_logic: Set<StateMiddleware>,
    pub ptoken_logic: PToken<StateMiddleware>,
    pub manager: Manager<StateMiddleware>,
    pub backstop: Backstop<StateMiddleware>,
    pub dynamic_level_factory: CostModelDynamicLevelFactory<StateMiddleware>,
    pub jump_rate_factory: CostModelJumpRateFactory<StateMiddleware>,
    pub drip_decay_factory: DripDecayModelConstantFactory<StateMiddleware>,
    pub _chainlink_trigger_factory: ChainlinkTriggerFactory<StateMiddleware>,
    pub _uma_trigger_factory: UMATriggerFactory<StateMiddleware>,
}

#[derive(Debug, Clone)]
pub struct SetContracts {
    pub base_token: DummyToken<StateMiddleware>,
    pub set: Set<StateMiddleware>,
    pub jump_rate_models: HashMap<u32, CostModelJumpRate<StateMiddleware>>,
    pub dynamic_level_models: HashMap<u32, CostModelDynamicLevel<StateMiddleware>>,
    pub drip_decay_models: HashMap<u32, DripDecayModelConstant<StateMiddleware>>,
    pub dummy_triggers: HashMap<u32, DummyTrigger<StateMiddleware>>,
    pub ptokens: HashMap<u32, PToken<StateMiddleware>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CozySimRunner {
    pub sim_setup_params: SimSetupParams,
    pub protocol_params: ProtocolDeployParams,
    pub time_policy_params: FixedTimePolicyParams,
    pub arbitrageur_params: ArbitrageurParams,
    pub buyer_params: BuyerParams,
    pub supplier_params: SupplierParams,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub triggers: Vec<(Cow<'static, str>, TriggerType)>,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub cost_models: Vec<(Cow<'static, str>, CostModelType)>,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub drip_decay_models: Vec<(Cow<'static, str>, DripDecayModelType)>,
    pub market_config_params: Vec<MarketConfigParams>,
    pub set_config_params: SetConfigParams,
    pub trigger_risk_params: TriggerRiskParams,
    pub agent_set_risk_params: AgentSetRiskParams,
}

impl CozySimRunner {
    pub fn run(&self, output_file: Cow<'static, str>) -> Result<(), Box<dyn std::error::Error>> {
        // Initiate world, state and time policy.
        let world = CozyWorld::new();
        let mut state = State::new(world);
        let fixed_time_policy = FixedTimePolicy::new(
            TimeEnv {
                block_number: self.time_policy_params.start_block_number,
                block_timestamp: self.time_policy_params.start_block_timestamp,
            },
            self.time_policy_params.time_per_step,
            self.time_policy_params.time_to_generate,
        );

        // Admin agent executes all set-up transactions.
        let mut rng = StdRng::seed_from_u64(self.sim_setup_params.rand_seed);
        let admin_addr = Address::random_using(&mut rng);

        // Set-up all protocol contracts and set contracts.
        let (protocol_contracts, set_contracts) = self.setup(&mut state, admin_addr)?;
        let num_markets = self.market_config_params.len();

        // Initiate manager.
        let mut manager = SimManager::new(
            state,
            Box::new(fixed_time_policy),
            Summarizer::new(output_file),
        );

        // Run altruistic supplier activation.
        let altruistic_supplier = AltruisticSupplier::new(
            ALTRUISTIC_SUPPLIER.into(),
            Address::random_using(&mut rng),
            protocol_contracts.clone(),
            set_contracts.clone(),
        );
        self.deal(
            manager.get_state_mut(),
            &set_contracts,
            admin_addr,
            altruistic_supplier.address(),
            self.supplier_params.altruistic_supplier_wealth,
        )?;
        manager.activate_agent(Box::new(altruistic_supplier))?;

        // Run trigger admin activation.
        let step_time: u64 = fixed_time_policy.time_per_step.as_u64();
        let annual_step_factor = SECONDS_IN_YEAR as f64 / step_time as f64;
        let trigger_simulator = TriggerSimulator::new(
            rng.clone(),
            self.trigger_risk_params
                .annual_probabilities
                .iter()
                .map(|x| x / annual_step_factor)
                .collect(),
            self.trigger_risk_params.pairwise_corr,
        );
        let trigger_admin = TriggerAdmin::new(
            TRIGGER_ADMIN.into(),
            Address::random_using(&mut rng),
            protocol_contracts.clone(),
            set_contracts.clone(),
            trigger_simulator.clone(),
        );
        manager.activate_agent(Box::new(trigger_admin))?;

        // Initialize agent risk sampler.
        let true_annual_probabilities =
            DVector::from(self.trigger_risk_params.annual_probabilities.clone());
        let true_correlations = trigger_simulator.mvb.corr;
        let mut agent_set_risk_sampler = AgentSetRiskSampler::new(
            rng.clone(),
            true_annual_probabilities.clone(),
            true_correlations.clone(),
            self.agent_set_risk_params
                .annual_probabilities_concentration,
            self.agent_set_risk_params.wishart_corr_df,
        );

        // Run buyer activations.
        let leverage = self.set_config_params.leverage_factor as f64 / ZOC as f64;
        let raw_market_weights = self
            .market_config_params
            .iter()
            .map(|x| x.weight)
            .collect::<Vec<_>>();
        let raw_markets_weights_sum = raw_market_weights.iter().sum::<u16>() as f64;
        let market_weights = raw_market_weights
            .iter()
            .map(|x| (*x as f64 / raw_markets_weights_sum))
            .collect::<Vec<_>>();
        let market_allocations_dirichlet = Dirichlet::new(vec![
            self.buyer_params.market_allocations_dirichlet_alpha
                / num_markets as f64;
            num_markets
        ])
        .expect("Error creating Dirichlet.");
        let balance_normal = Normal::new(
            u256_to_f64(self.buyer_params.balance_mean),
            u256_to_f64(self.buyer_params.balance_std),
        )
        .expect("Error creating normal.");

        for _ in 0..self.buyer_params.num {
            let (annual_probabilities, corr) = agent_set_risk_sampler.sample();
            let buyer_risk_model = SetRiskModel::new(
                annual_probabilities,
                corr,
                leverage,
                DVector::from(market_weights.clone()),
            );
            let buyer_reaction_time = ReactionTime::new(
                self.buyer_params.mean_reaction_time,
                self.time_policy_params.start_block_timestamp,
            );
            let buyer = Buyer::new(
                BUYER.into(),
                Address::random_using(&mut rng),
                protocol_contracts.clone(),
                set_contracts.clone(),
                BuyerPreferences::new(
                    buyer_risk_model,
                    market_allocations_dirichlet.sample(&mut rng),
                    buyer_reaction_time,
                ),
                rng.clone(),
            );
            let balance = f64_to_u256(balance_normal.sample(&mut rng).max(0.0));
            self.deal(
                manager.get_state_mut(),
                &set_contracts,
                admin_addr,
                buyer.address(),
                balance,
            )?;
            manager.activate_agent(Box::new(buyer))?;
        }

        // Run supplier activations.
        let wealth_normal = Normal::new(
            u256_to_f64(self.supplier_params.wealth_mean),
            u256_to_f64(self.supplier_params.wealth_std),
        )
        .expect("Error creating normal.");
        let mut risk_aversion_sampler = SupplierRiskAversionSampler::new(
            rng.clone(),
            self.supplier_params.risk_aversion_mean,
            self.supplier_params.risk_aversion_concentration,
        );
        for _ in 0..self.supplier_params.num {
            let (annual_probabilities, corr) = agent_set_risk_sampler.sample();
            let supplier_risk_model = SetRiskModel::new(
                annual_probabilities,
                corr,
                leverage,
                DVector::from(market_weights.clone()),
            );
            let supper_reaction_time = ReactionTime::new(
                self.supplier_params.mean_reaction_time,
                self.time_policy_params.start_block_timestamp,
            );
            let wealth = f64_to_u256(wealth_normal.sample(&mut rng).max(0.0));
            let supplier_preferences = SupplierPreferences::new(
                supplier_risk_model,
                risk_aversion_sampler.sample(),
                wealth,
                supper_reaction_time,
            );
            let supplier = Supplier::new(
                SUPPLIER.into(),
                Address::random_using(&mut rng),
                protocol_contracts.clone(),
                set_contracts.clone(),
                supplier_preferences,
                rng.clone(),
            );
            self.deal(
                manager.get_state_mut(),
                &set_contracts,
                admin_addr,
                supplier.address(),
                wealth,
            )?;
            manager.activate_agent(Box::new(supplier))?;
        }

        // Run arbitrageur activations.
        let balance_normal = Normal::new(
            u256_to_f64(self.arbitrageur_params.balance_mean),
            u256_to_f64(self.arbitrageur_params.balance_std),
        )
        .expect("Error creating normal.");

        for _ in 0..self.arbitrageur_params.num {
            let (annual_probabilities, corr) = agent_set_risk_sampler.sample();
            let arbitrageur_risk_model = SetRiskModel::new(
                annual_probabilities,
                corr,
                leverage,
                DVector::from(market_weights.clone()),
            );
            let arbitrageur_reaction_time = ReactionTime::new(
                self.arbitrageur_params.mean_reaction_time,
                self.time_policy_params.start_block_timestamp,
            );
            let arbitrageur = Arbitrageur::new(
                ARBITRAGEUR.into(),
                Address::random_using(&mut rng),
                protocol_contracts.clone(),
                set_contracts.clone(),
                ArbitrageurPreferences::new(arbitrageur_risk_model, arbitrageur_reaction_time),
                rng.clone(),
            );
            let balance = f64_to_u256(balance_normal.sample(&mut rng).max(0.0));
            self.deal(
                manager.get_state_mut(),
                &set_contracts,
                admin_addr,
                arbitrageur.address(),
                balance,
            )?;
            manager.activate_agent(Box::new(arbitrageur))?;
        }

        // Run set analyzer activations.
        let true_risk_model = SetRiskModel::new(
            true_annual_probabilities,
            true_correlations,
            leverage,
            DVector::from(market_weights),
        );
        let set_analyzer = SetAnalyzer::new(
            SET_ANALYZER.into(),
            Address::random_using(&mut rng),
            protocol_contracts.clone(),
            set_contracts.clone(),
            true_risk_model,
        );
        manager.activate_agent(Box::new(set_analyzer))?;

        // Register set summary generator.
        let set_summary_generator = SetSummaryGenerator::new(protocol_contracts, set_contracts);
        manager.register_summary_generator(set_summary_generator);

        // Run sim.
        manager.run_sim()?;

        Ok(())
    }

    pub fn setup(
        &self,
        state: &mut State<CozyUpdate, CozyWorld>,
        admin_addr: Address,
    ) -> Result<(Arc<ProtocolContracts>, Arc<SetContracts>), Box<dyn std::error::Error>> {
        let state_middleware = Arc::new(StateMiddleware());

        // Deploy wETH.
        log::info!("Deploying wETH.");
        let weth_addr =
            state.deploy_evm_contract(admin_addr, &weth9::WETH9_ABI, &weth9::WETH9_BYTECODE, ())?;
        let weth = weth9::WETH9::new(weth_addr, state_middleware.clone());

        // Deploy libraries.
        log::info!("Deploying libraries.");
        let libraries = self.deploy_libraries(state, admin_addr)?;

        // Deploy core protocol.
        log::info!("Deploying core protocol.");
        let protocol_contracts = self.deploy_protocol(
            state,
            libraries,
            admin_addr,
            weth.address().into(),
            state_middleware.clone(),
        )?;
        let protocol_contracts = Arc::new(protocol_contracts);

        // Deploy set.
        log::info!("Deploying set.");
        let set_contracts =
            self.deploy_set(state, &protocol_contracts, admin_addr, state_middleware)?;
        let set_contracts = Arc::new(set_contracts);

        Ok((protocol_contracts, set_contracts))
    }

    #[allow(clippy::type_complexity)]
    fn deploy_libraries(
        &self,
        state: &mut State<CozyUpdate, CozyWorld>,
        admin_addr: Address,
    ) -> Result<Vec<(&str, &str, EthersAddress)>, Box<dyn std::error::Error>> {
        let mut libraries: Vec<(&str, &str, EthersAddress)> = vec![];

        let configurator_lib_addr = state.deploy_evm_contract(
            admin_addr,
            &CONFIGURATORLIB_ABI,
            &CONFIGURATORLIB_BYTECODE,
            (),
        )?;
        libraries.push((
            CONFIGURATORLIB_PATH,
            CONFIGURATORLIB_NAME,
            configurator_lib_addr.into(),
        ));

        let delay_lib_addr =
            state.deploy_evm_contract(admin_addr, &DELAYLIB_ABI, &DELAYLIB_BYTECODE, ())?;
        libraries.push((DELAYLIB_PATH, DELAYLIB_NAME, delay_lib_addr.into()));

        let demand_side_lib_addr = state.deploy_evm_contract(
            admin_addr,
            &DEMANDSIDELIB_ABI,
            &DEMANDSIDELIB_BYTECODE,
            (),
        )?;
        libraries.push((
            DEMANDSIDELIB_PATH,
            DEMANDSIDELIB_NAME,
            demand_side_lib_addr.into(),
        ));

        let redemption_lib_addr = state.deploy_evm_contract(
            admin_addr,
            &REDEMPTIONLIB_ABI,
            &REDEMPTIONLIB_BYTECODE,
            (),
        )?;
        libraries.push((
            REDEMPTIONLIB_PATH,
            REDEMPTIONLIB_NAME,
            redemption_lib_addr.into(),
        ));

        let state_transitions_lib_addr = state.deploy_evm_contract(
            admin_addr,
            &STATETRANSITIONSLIB_ABI,
            &STATETRANSITIONSLIB_BYTECODE,
            (),
        )?;
        libraries.push((
            STATETRANSITIONSLIB_PATH,
            STATETRANSITIONSLIB_NAME,
            state_transitions_lib_addr.into(),
        ));

        let supply_side_lib_addr = state.deploy_evm_contract(
            admin_addr,
            &SUPPLYSIDELIB_ABI,
            &SUPPLYSIDELIB_BYTECODE,
            (),
        )?;
        libraries.push((
            SUPPLYSIDELIB_PATH,
            SUPPLYSIDELIB_NAME,
            supply_side_lib_addr.into(),
        ));

        Ok(libraries)
    }

    fn deploy_protocol(
        &self,
        state: &mut State<CozyUpdate, CozyWorld>,
        libraries: Vec<(&str, &str, EthersAddress)>,
        admin_addr: Address,
        weth_addr: Address,
        state_middleware: Arc<StateMiddleware>,
    ) -> Result<ProtocolContracts, Box<dyn std::error::Error>> {
        // Pre-compute Cozy protocol addresses.
        let mut nonce = state
            .read_account_info(admin_addr)
            .expect("Admin account exists.")
            .nonce;
        let manager_addr: Address = create_address(admin_addr.into(), nonce).into();
        nonce += 1;
        let set_logic_addr: Address = create_address(admin_addr.into(), nonce).into();
        // Next nonce is initialization of the Set logic.
        nonce += 2;
        let set_factory_addr: Address = create_address(admin_addr.into(), nonce).into();
        nonce += 1;
        let ptoken_logic_addr: Address = create_address(admin_addr.into(), nonce).into();
        // Next nonce is initialization of the PToken logic.
        nonce += 2;
        let ptoken_factory_addr: Address = create_address(admin_addr.into(), nonce).into();
        nonce += 1;
        let backstop_addr: Address = create_address(admin_addr.into(), nonce).into();
        nonce += 1;
        let cozyrouter_addr: Address = create_address(admin_addr.into(), nonce).into();

        // Deploy manager.
        let manager_args = (
            backstop_addr,
            set_factory_addr,
            admin_addr,
            admin_addr,
            self.protocol_params.delays.clone(),
            self.protocol_params.fees.clone(),
            self.protocol_params.allowed_markets_per_set,
        );
        let _ =
            state.deploy_evm_contract(admin_addr, &MANAGER_ABI, &MANAGER_BYTECODE, manager_args)?;
        let manager = Manager::new(manager_addr, state_middleware.clone());

        // Deploy set logic.
        let set_logic_args = (manager_addr, ptoken_factory_addr, backstop_addr);
        let set_bytecode =
            simulate::utils::build_linked_bytecode(SET_RAW_BYTECODE, libraries.clone())?;
        let _ = state.deploy_evm_contract(
            admin_addr,
            &SET_ABI,
            &EthersBytes::from(set_bytecode),
            set_logic_args,
        )?;
        let set_logic = set::Set::new(set_logic_addr, state_middleware.clone());

        // Initialize set logic.
        let set_logic_initialization = set_logic.initialize(
            Address::zero().into(),
            Address::zero().into(),
            weth_addr.into(),
            SetConfig {
                deposit_fee: 0,
                leverage_factor: 0,
            },
            Vec::<MarketConfig>::new(),
        );
        state.execute_evm_tx_and_decode(admin_addr, set_logic_initialization)?;

        // Deploy set factory.
        let set_factory_args = (manager_addr, set_logic_addr);
        let _ = state.deploy_evm_contract(
            admin_addr,
            &SETFACTORY_ABI,
            &SETFACTORY_BYTECODE,
            set_factory_args,
        )?;
        let set_factory = SetFactory::new(set_factory_addr, state_middleware.clone());

        // Deploy ptoken logic.
        let ptoken_logic_args = (manager_addr,);
        let _ = state.deploy_evm_contract(
            admin_addr,
            &PTOKEN_ABI,
            &PTOKEN_BYTECODE,
            ptoken_logic_args,
        )?;
        let ptoken_logic = PToken::new(ptoken_logic_addr, state_middleware.clone());

        // Initialize ptoken logic.
        let ptoken_initialization =
            ptoken_logic.initialize(Address::zero().into(), Address::zero().into(), 0_u8);
        state.execute_evm_tx_and_decode(admin_addr, ptoken_initialization)?;

        // Deploy ptoken factory.
        let ptoken_factory_args = (ptoken_logic_addr,);
        let _ = state.deploy_evm_contract(
            admin_addr,
            &PTOKENFACTORY_ABI,
            &PTOKENFACTORY_BYTECODE,
            ptoken_factory_args,
        )?;

        // Deploy backstop.
        let backstop_args = (manager_addr, weth_addr);
        let _ = state.deploy_evm_contract(
            admin_addr,
            &BACKSTOP_ABI,
            &BACKSTOP_BYTECODE,
            backstop_args,
        )?;
        let backstop = Backstop::new(backstop_addr, state_middleware.clone());

        // Deploy CozyRouter.
        let cozyrouter_args = (manager_addr, weth_addr, weth_addr, weth_addr);
        let _ = state.deploy_evm_contract(
            admin_addr,
            &COZYROUTER_ABI,
            &COZYROUTER_BYTECODE,
            cozyrouter_args,
        )?;
        let cozy_router = CozyRouter::new(cozyrouter_addr, state_middleware.clone());

        // Deploy cost model jump rate factory.
        let cost_model_jump_rate_factory_addr = state.deploy_evm_contract(
            admin_addr,
            &COSTMODELJUMPRATEFACTORY_ABI,
            &COSTMODELJUMPRATEFACTORY_BYTECODE,
            (),
        )?;
        let jump_rate_factory = CostModelJumpRateFactory::new(
            cost_model_jump_rate_factory_addr,
            state_middleware.clone(),
        );

        // Deploy cost model dynamic level factory.
        let cost_model_dynamic_level_factory_addr = state.deploy_evm_contract(
            admin_addr,
            &COSTMODELDYNAMICLEVELFACTORY_ABI,
            &COSTMODELDYNAMICLEVELFACTORY_BYTECODE,
            (),
        )?;
        let dynamic_level_factory = CostModelDynamicLevelFactory::new(
            cost_model_dynamic_level_factory_addr,
            state_middleware.clone(),
        );

        // Deploy drip decay model factory.
        let drip_decay_model_factory_addr = state.deploy_evm_contract(
            admin_addr,
            &DRIPDECAYMODELCONSTANTFACTORY_ABI,
            &DRIPDECAYMODELCONSTANTFACTORY_BYTECODE,
            (),
        )?;
        let drip_decay_factory = DripDecayModelConstantFactory::new(
            drip_decay_model_factory_addr,
            state_middleware.clone(),
        );

        // Deploy chainlink trigger factory.
        let chainlink_trigger_factory_args = (manager_addr,);
        let chainlink_trigger_factory_addr = state.deploy_evm_contract(
            admin_addr,
            &CHAINLINKTRIGGERFACTORY_ABI,
            &CHAINLINKTRIGGERFACTORY_BYTECODE,
            chainlink_trigger_factory_args,
        )?;
        let _chainlink_trigger_factory =
            ChainlinkTriggerFactory::new(chainlink_trigger_factory_addr, state_middleware.clone());

        // Deploy UMA trigger factory.
        let uma_trigger_factory_args = (manager_addr, manager_addr);
        let uma_trigger_factory_addr = state.deploy_evm_contract(
            admin_addr,
            &UMATRIGGERFACTORY_ABI,
            &UMATRIGGERFACTORY_BYTECODE,
            uma_trigger_factory_args,
        )?;
        let _uma_trigger_factory =
            UMATriggerFactory::new(uma_trigger_factory_addr, state_middleware);

        Ok(ProtocolContracts {
            cozy_router,
            set_factory,
            set_logic,
            ptoken_logic,
            manager,
            backstop,
            dynamic_level_factory,
            jump_rate_factory,
            drip_decay_factory,
            _chainlink_trigger_factory,
            _uma_trigger_factory,
        })
    }

    fn deploy_set(
        &self,
        state: &mut State<CozyUpdate, CozyWorld>,
        protocol_contracts: &ProtocolContracts,
        admin_addr: Address,
        state_middleware: Arc<StateMiddleware>,
    ) -> Result<SetContracts, Box<dyn std::error::Error>> {
        // Deploy base token.
        log::info!("Deploying base token.");
        let base_token_args = ("Cozy Base Token".to_string(), "CBT".to_string(), 6_u8);
        let base_token_addr = state.deploy_evm_contract(
            admin_addr,
            &DUMMYTOKEN_ABI,
            &DUMMYTOKEN_BYTECODE,
            base_token_args,
        )?;
        let base_token = DummyToken::new(base_token_addr, state_middleware.clone());

        // Deploy cost models.
        let mut jump_rate_models: HashMap<u32, CostModelJumpRate<StateMiddleware>> = HashMap::new();
        let mut dynamic_level_models: HashMap<u32, CostModelDynamicLevel<StateMiddleware>> =
            HashMap::new();
        let mut cost_model_addrs = vec![];
        for (i, (name, cost_model_type)) in self.cost_models.iter().enumerate() {
            log::info!("Deploying cost model: {:?}.", name);

            match cost_model_type {
                CostModelType::JumpRate(args) => {
                    let get_model_call = protocol_contracts.jump_rate_factory.get_model(
                        args.kink,
                        args.cost_factor_at_zero_utilization,
                        args.cost_factor_at_kink_utilization,
                        args.cost_factor_at_full_utilization,
                    );
                    let deployed_addr = state
                        .call_evm_tx_and_decode(admin_addr, get_model_call)
                        .expect("Error determining if cost model deployed.");
                    if deployed_addr == Address::zero().into() {
                        let model_deploy_call = protocol_contracts.jump_rate_factory.deploy_model(
                            args.kink,
                            args.cost_factor_at_zero_utilization,
                            args.cost_factor_at_kink_utilization,
                            args.cost_factor_at_full_utilization,
                        );
                        let model_addr =
                            state.execute_evm_tx_and_decode(admin_addr, model_deploy_call)?;
                        let model = CostModelJumpRate::new(model_addr, state_middleware.clone());
                        jump_rate_models.insert(i as u32, model);
                        cost_model_addrs.push(model_addr);
                    } else {
                        let model = CostModelJumpRate::new(deployed_addr, state_middleware.clone());
                        jump_rate_models.insert(i as u32, model);
                        cost_model_addrs.push(deployed_addr);
                    }
                }
                CostModelType::DynamicLevel(args) => {
                    let model_deploy_call = protocol_contracts.dynamic_level_factory.deploy_model(
                        args.u_low,
                        args.u_high,
                        args.cost_factor_at_zero_utilization,
                        args.cost_factor_at_full_utilization,
                        args.cost_factor_in_optimal_zone,
                        args.optimal_zone_rate,
                    );
                    let model_addr =
                        state.execute_evm_tx_and_decode(admin_addr, model_deploy_call)?;
                    let model = CostModelDynamicLevel::new(model_addr, state_middleware.clone());
                    dynamic_level_models.insert(i as u32, model);
                    cost_model_addrs.push(model_addr);
                }
            }
        }

        // Deploy drip decay models.
        let mut drip_decay_models: HashMap<u32, DripDecayModelConstant<StateMiddleware>> =
            HashMap::new();
        let mut drip_decay_model_addrs = vec![];
        for (i, (name, drip_decay_model_type)) in self.drip_decay_models.iter().enumerate() {
            log::info!("Deploying drip decay model: {:?}.", name);

            match drip_decay_model_type {
                DripDecayModelType::Constant(args) => {
                    let get_model_call = protocol_contracts
                        .drip_decay_factory
                        .get_model(args.rate_per_second);
                    let deployed_addr = state
                        .call_evm_tx_and_decode(admin_addr, get_model_call)
                        .expect("Error determining if cost model deployed.");
                    if deployed_addr == Address::zero().into() {
                        let model_deploy_call = protocol_contracts
                            .drip_decay_factory
                            .deploy_model(args.rate_per_second);
                        let model_addr =
                            state.execute_evm_tx_and_decode(admin_addr, model_deploy_call)?;
                        let model =
                            DripDecayModelConstant::new(model_addr, state_middleware.clone());
                        drip_decay_models.insert(i as u32, model);
                        drip_decay_model_addrs.push(model_addr);
                    } else {
                        let model =
                            DripDecayModelConstant::new(deployed_addr, state_middleware.clone());
                        drip_decay_models.insert(i as u32, model);
                        drip_decay_model_addrs.push(deployed_addr);
                    }
                }
            }
        }

        // Deploy triggers.
        let mut dummy_triggers: HashMap<u32, DummyTrigger<StateMiddleware>> = HashMap::new();
        let mut trigger_addrs = vec![];
        for (i, (name, trigger_type)) in self.triggers.iter().enumerate() {
            log::info!("Deploying trigger: {:?}.", name);

            match trigger_type {
                TriggerType::DummyTrigger => {
                    let trigger_addr = state.deploy_evm_contract(
                        admin_addr,
                        &DUMMYTRIGGER_ABI,
                        &DUMMYTRIGGER_BYTECODE,
                        (protocol_contracts.manager.address(),),
                    )?;
                    let trigger = DummyTrigger::new(trigger_addr, state_middleware.clone());
                    dummy_triggers.insert(i as u32, trigger);
                    trigger_addrs.push(trigger_addr);
                }
                TriggerType::ChainlinkTrigger => unimplemented!("Chainlink trigger."),
                TriggerType::UmaTrigger => unimplemented!("UMA trigger."),
            }
        }

        // Create set call.
        log::info!("Calling create set.");
        let mut market_configs: Vec<MarketConfig> = vec![];
        for (i, params) in self.market_config_params.clone().into_iter().enumerate() {
            market_configs.push(MarketConfig {
                trigger: trigger_addrs[i].into(),
                cost_model: cost_model_addrs[i],
                drip_decay_model: drip_decay_model_addrs[i],
                weight: params.weight,
                purchase_fee: params.purchase_fee,
                sale_fee: params.sale_fee,
            });
        }
        market_configs.sort_by(|a, b| a.trigger.cmp(&b.trigger));

        let set_create_call = protocol_contracts.manager.create_set(
            admin_addr.into(),
            admin_addr.into(),
            base_token.address(),
            SetConfig {
                deposit_fee: self.set_config_params.deposit_fee,
                leverage_factor: self.set_config_params.leverage_factor,
            },
            market_configs,
            rand::random::<[u8; 32]>(),
        );
        let set_addr = state.execute_evm_tx_and_decode(admin_addr, set_create_call)?;
        let set = Set::new(set_addr, state_middleware.clone());
        log::info!("Set deployed at {:0X}.", set_addr);

        // Register PToken contracts.
        let mut ptokens: HashMap<u32, PToken<StateMiddleware>> = HashMap::new();
        for i in 0..self.market_config_params.len() {
            let (ptoken_addr, _, _, _, _, _, _, _, _, _) = state
                .call_evm_tx_and_decode(admin_addr, set.markets(U256::from(i)))
                .expect("Error getting protection token address.");
            let ptoken = PToken::new(ptoken_addr, state_middleware.clone());
            ptokens.insert(i as u32, ptoken);
        }

        Ok(SetContracts {
            base_token,
            set,
            jump_rate_models,
            dynamic_level_models,
            drip_decay_models,
            dummy_triggers,
            ptokens,
        })
    }

    pub fn deal(
        &self,
        state: &mut State<CozyUpdate, CozyWorld>,
        set_contracts: &SetContracts,
        admin_addr: Address,
        to_addr: Address,
        amount: U256,
    ) -> Result<(), Box<dyn std::error::Error>> {
        state.execute_evm_tx_and_decode(
            admin_addr,
            set_contracts.base_token.mint(to_addr.into(), amount),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cozy::configs::build_cozy_sim_runner_from_dir;

    #[test]
    fn test_runner() -> Result<(), Box<dyn std::error::Error>> {
        let runner = build_cozy_sim_runner_from_dir("test")?;
        let _ = runner.run("output/summaries/test_output.json".into());
        Ok(())
    }
}
