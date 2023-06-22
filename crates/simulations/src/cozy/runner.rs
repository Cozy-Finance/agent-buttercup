use std::{borrow::Cow, collections::HashMap};

pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
use config::Config;
pub use ethers::types::U256 as EthersU256;
use rand::{rngs::StdRng, Rng, SeedableRng};
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};
use simulate::{
    manager::SimManager,
    state::SimState,
    time_policy::{FixedTimePolicy, TimePolicy},
    utils::float_to_wad,
};

use super::{
    agents::{
        cost_models_deployer::CostModelsDeployer,
        drip_decay_models_deployer::DripDecayModelsDeployer, passive_buyer::PassiveBuyer,
        passive_supplier::PassiveSupplier, protocol_deployer::ProtocolDeployer,
        set_admin::SetAdmin, token_deployer::TokenDeployer, triggers_deployer::TriggersDeployer,
        weth_deployer::WethDeployer,
    },
    constants::*,
    types::{
        CozyCostModelType, CozyDripDecayModelType, CozyFixedTimePolicyParams,
        CozyMarketConfigParams, CozyProtocolDeployParams, CozySetAdminParams, CozySetConfigParams,
        CozySimBuyersSuppliers, CozySimSetupParams, CozyTokenDeployParams, CozyTriggerType,
    },
    world::CozyWorld,
};

pub struct CozySingleSetSimRunner {
    rand_seed: u64,
    fixed_time_policy: FixedTimePolicy,
    protocol_params: CozyProtocolDeployParams,
    base_token_params: CozyTokenDeployParams,
    num_passive_buyers: u64,
    num_passive_suppliers: u64,
    passive_buyer_capital: EthersU256,
    passive_supplier_capital: EthersU256,
    triggers: Vec<(Cow<'static, str>, CozyTriggerType)>,
    cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    drip_decay_models: Vec<(Cow<'static, str>, CozyDripDecayModelType)>,
    market_config_params: Vec<CozyMarketConfigParams>,
    set_config_params: CozySetConfigParams,
}

impl CozySingleSetSimRunner {
    pub fn run(self) {
        let mut rng = StdRng::seed_from_u64(self.rand_seed);

        // Create sim manager.
        let world_state = CozyWorld::new();
        let sim_state = SimState::new(world_state);
        let mut sim_manager = SimManager::new(sim_state, Box::new(self.fixed_time_policy));

        // Create and activate agents.
        // Weth deployer.
        let weth_deployer = Box::new(WethDeployer::new(
            Some(WETH_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
        ));
        let _ = sim_manager.activate_agent(weth_deployer);

        // Protocol deployer.
        let protocol_deployer = Box::new(ProtocolDeployer::new(
            Some(PROTOCOL_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
            self.protocol_params,
        ));
        let _ = sim_manager.activate_agent(protocol_deployer);

        // Pre-generate <Address, Capital> map for passive buyers and suppliers.
        let mut buyers_map = HashMap::new();
        for i in 0..self.num_passive_buyers {
            buyers_map.insert(
                EvmAddress::random_using(&mut rng),
                self.passive_buyer_capital,
            );
        }
        let mut suppliers_map = HashMap::new();
        for i in 0..self.num_passive_suppliers {
            suppliers_map.insert(
                EvmAddress::random_using(&mut rng),
                self.passive_supplier_capital,
            );
        }

        // Base token deployer.
        let mut allocate_addrs = buyers_map.clone();
        allocate_addrs.extend(suppliers_map.clone());
        let base_token_deployer = Box::new(TokenDeployer::new(
            Some(BASE_TOKEN_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
            self.base_token_params,
            allocate_addrs,
        ));
        let _ = sim_manager.activate_agent(base_token_deployer);

        // Store protocol contracts.
        let world_protocol_contracts = sim_manager.get_state().world.protocol_contracts;

        // Cost models deployer.
        let cost_models_deployer = Box::new(CostModelsDeployer::new(
            Some(COST_MODELS_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
            self.cost_models.iter().cloned().collect(),
            world_protocol_contracts
                .get(COSTMODELJUMPRATEFACTORY.name)
                .unwrap(),
            world_protocol_contracts
                .get(COSTMODELDYNAMICLEVELFACTORY.name)
                .unwrap(),
        ));
        let _ = sim_manager.activate_agent(cost_models_deployer);

        // Drip decay models deployer.
        let drip_decay_models_deployer = Box::new(DripDecayModelsDeployer::new(
            Some(DRIP_DECAY_MODELS_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
            self.drip_decay_models.iter().cloned().collect(),
            world_protocol_contracts
                .get(DRIPDECAYMODELCONSTANTFACTORY.name)
                .unwrap(),
        ));
        let _ = sim_manager.activate_agent(drip_decay_models_deployer);

        // Triggers deployer.
        let triggers_deployer = Box::new(TriggersDeployer::new(
            Some(TRIGGERS_DEPLOYER.into()),
            EvmAddress::random_using(&mut rng),
            self.triggers.iter().cloned().collect(),
            world_protocol_contracts
                .get(UMATRIGGERFACTORY.name)
                .unwrap(),
            world_protocol_contracts
                .get(CHAINLINKTRIGGERFACTORY.name)
                .unwrap(),
            world_protocol_contracts.get(MANAGER.name).unwrap(),
        ));
        let _ = sim_manager.activate_agent(triggers_deployer);

        // Store cost model, drip decay model and trigger contracts.
        let world_cost_models = sim_manager.get_state().world.cost_models;
        let world_drip_decay_models = sim_manager.get_state().world.cost_models;
        let world_triggers = sim_manager.get_state().world.triggers;

        // Set admin.
        let mut market_configs = vec![];
        for (i, market_config_param) in self.market_config_params.into_iter().enumerate() {
            let cost_model_addr = world_cost_models
                .get(&self.cost_models[i].0)
                .unwrap()
                .address;
            let drip_decay_model_addr = world_drip_decay_models
                .get(&self.drip_decay_models[i].0)
                .unwrap()
                .address;
            let trigger_addr = world_triggers.get(&self.triggers[i].0).unwrap().address;
            market_configs.push(MarketConfig {
                trigger: trigger_addr.into(),
                cost_model: cost_model_addr.into(),
                drip_decay_model: drip_decay_model_addr.into(),
                weight: market_config_param.weight,
                purchase_fee: market_config_param.purchase_fee,
                sale_fee: market_config_param.sale_fee,
            })
        }
        let salt: Option<[u8; 32]> = Some(rng.gen());
        let base_asset_addr = world_protocol_contracts.get(BASE_TOKEN).unwrap().address;
        let set_params = CozySetAdminParams {
            asset: base_asset_addr.into(),
            set_config: self.set_config_params.into(),
            market_configs,
            salt,
        };

        let set_admin = Box::new(SetAdmin::new(
            Some(SET_ADMIN.into()),
            EvmAddress::random_using(&mut rng),
            set_params,
            world_protocol_contracts.get(SET.name).unwrap(),
            world_protocol_contracts.get(MANAGER.name).unwrap(),
        ));
        let _ = sim_manager.activate_agent(set_admin);

        // Passive buyers.
        for (i, (addr, _)) in buyers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_BUYER, i + 1);
            let passive_buyer = Box::new(PassiveBuyer::new(
                Some(name.into()),
                addr,
                world_protocol_contracts.get(COZYROUTER.name).unwrap(),
                world_protocol_contracts.get(BASE_TOKEN).unwrap(),
                world_protocol_contracts.get(SET.name).unwrap(),
                vec![],
                vec![],
            ));
            let _ = sim_manager.activate_agent(passive_buyer);
        }

        // Passive suppliers.
        for (i, (addr, _)) in suppliers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_SUPPLIER, i + 1);
            let passive_supplier = Box::new(PassiveSupplier::new(
                Some(name.into()),
                addr,
                world_protocol_contracts.get(COZYROUTER.name).unwrap(),
                world_protocol_contracts.get(BASE_TOKEN).unwrap(),
            ));
            let _ = sim_manager.activate_agent(passive_supplier);
        }

        // Run sim.
        sim_manager.run_sim();
    }
}

impl Default for CozySingleSetSimRunner {
    fn default() -> Self {
        let sim_setup_params = get_default_config("sim_setup")
            .try_deserialize::<CozySimSetupParams>()
            .unwrap();

        let protocol_params = get_default_config("protocol_deploy_params")
            .try_deserialize::<CozyProtocolDeployParams>()
            .unwrap();

        let time_policy_params = get_default_config("fixed_time_policy")
            .try_deserialize::<CozyFixedTimePolicyParams>()
            .unwrap();
        let fixed_time_policy = FixedTimePolicy::new(
            time_policy_params.start_block_number.into(),
            time_policy_params.start_block_timestamp.into(),
            time_policy_params.time_per_block,
            time_policy_params.blocks_per_step,
            time_policy_params.blocks_to_generate,
            time_policy_params.time_to_generate,
        )
        .unwrap();

        let base_token_params = get_default_config("base_token")
            .try_deserialize::<CozyTokenDeployParams>()
            .unwrap();

        let buyers_and_suppliers = get_default_config("buyers_and_suppliers")
            .try_deserialize::<CozySimBuyersSuppliers>()
            .unwrap();

        let set_config_params = get_default_config("set_config_params")
            .try_deserialize::<CozySetConfigParams>()
            .unwrap();

        CozySingleSetSimRunner {
            rand_seed: sim_setup_params.rand_seed,
            fixed_time_policy,
            protocol_params,
            triggers: vec![],
            cost_models: vec![],
            drip_decay_models: vec![],
            base_token_params,
            num_passive_buyers: buyers_and_suppliers.num_passive_buyers,
            num_passive_suppliers: buyers_and_suppliers.num_passive_suppliers,
            passive_buyer_capital: buyers_and_suppliers.passive_buyer_capital,
            passive_supplier_capital: buyers_and_suppliers.passive_supplier_capital,
            market_config_params: vec![],
            set_config_params,
        }
    }
}

fn get_default_config(name: &str) -> Config {
    Config::builder()
        .add_source(config::File::new(
            &format!("configs/{}.yaml", name),
            config::FileFormat::Yaml,
        ))
        .build()
        .unwrap()
}
