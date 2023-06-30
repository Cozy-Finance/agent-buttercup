use std::{borrow::Cow, collections::HashMap};

use bindings::cozy_protocol::shared_types::MarketConfig;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use simulate::{
    address::Address, manager::SimManager, state::SimState, time_policy::FixedTimePolicy,
};

use crate::cozy::{
    agents::{
        active_buyer::ActiveBuyer, cost_models_deployer::CostModelsDeployer,
        drip_decay_models_deployer::DripDecayModelsDeployer, passive_buyer::PassiveBuyer,
        passive_supplier::PassiveSupplier, protocol_deployer::ProtocolDeployer,
        set_admin::SetAdmin, token_deployer::TokenDeployer, triggers_deployer::TriggersDeployer,
        weth_deployer::WethDeployer,
    },
    bindings_wrapper::*,
    constants::*,
    distributions::CozyDistribution,
    types::{
        CozyActiveBuyersParams, CozyCostModelType, CozyDripDecayModelType,
        CozyFixedTimePolicyParams, CozyMarketConfigParams, CozyPassiveBuyersParams,
        CozyProtocolDeployParams, CozySetAdminParams, CozySetConfigParams, CozySimSetupParams,
        CozySuppliersParams, CozyTokenDeployParams, CozyTriggerType,
    },
    world::CozyWorld,
};

#[derive(Deserialize, Debug, Clone)]
pub struct CozySingleSetSimRunnerSettings {
    pub sim_setup_params: CozySimSetupParams,
    pub protocol_params: CozyProtocolDeployParams,
    pub time_policy_params: CozyFixedTimePolicyParams,
    pub base_token_params: CozyTokenDeployParams,
    pub passive_buyers_params: CozyPassiveBuyersParams,
    pub active_buyers_params: CozyActiveBuyersParams,
    pub suppliers_params: CozySuppliersParams,
    pub triggers: Vec<(String, CozyTriggerType)>,
    pub cost_models: Vec<(String, CozyCostModelType)>,
    pub drip_decay_models: Vec<(String, CozyDripDecayModelType)>,
    pub market_config_params: Vec<CozyMarketConfigParams>,
    pub set_config_params: CozySetConfigParams,
}

pub struct CozySingleSetSimRunner {
    rand_seed: u64,
    fixed_time_policy: FixedTimePolicy,
    protocol_params: CozyProtocolDeployParams,
    base_token_params: CozyTokenDeployParams,
    passive_buyers_params: CozyPassiveBuyersParams,
    active_buyers_params: CozyActiveBuyersParams,
    suppliers_params: CozySuppliersParams,
    triggers: Vec<(Cow<'static, str>, CozyTriggerType)>,
    cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    drip_decay_models: Vec<(Cow<'static, str>, CozyDripDecayModelType)>,
    market_config_params: Vec<CozyMarketConfigParams>,
    set_config_params: CozySetConfigParams,
}

impl CozySingleSetSimRunner {
    pub fn new(settings: CozySingleSetSimRunnerSettings) -> Self {
        let fixed_time_policy = FixedTimePolicy::new(
            settings.time_policy_params.start_block_number,
            settings.time_policy_params.start_block_timestamp,
            settings.time_policy_params.time_per_block,
            settings.time_policy_params.blocks_per_step,
            settings.time_policy_params.blocks_to_generate,
            settings.time_policy_params.time_to_generate,
        )
        .unwrap();

        let triggers = settings
            .triggers
            .into_iter()
            .map(|(name, val)| (name.into(), val))
            .collect::<Vec<_>>();
        let cost_models = settings
            .cost_models
            .into_iter()
            .map(|(name, val)| (name.into(), val))
            .collect::<Vec<_>>();
        let drip_decay_models = settings
            .drip_decay_models
            .into_iter()
            .map(|(name, val)| (name.into(), val))
            .collect::<Vec<_>>();

        CozySingleSetSimRunner {
            rand_seed: settings.sim_setup_params.rand_seed,
            fixed_time_policy,
            protocol_params: settings.protocol_params,
            base_token_params: settings.base_token_params,
            passive_buyers_params: settings.passive_buyers_params,
            active_buyers_params: settings.active_buyers_params,
            suppliers_params: settings.suppliers_params,
            triggers,
            cost_models,
            drip_decay_models,
            market_config_params: settings.market_config_params,
            set_config_params: settings.set_config_params,
        }
    }

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
            Address::random_using(&mut rng),
        ));
        let _ = sim_manager.activate_agent(weth_deployer);

        let world_protocol_contracts = sim_manager.get_state().world.protocol_contracts;
        let weth = world_protocol_contracts.get_by_name(&WETH.name).unwrap();

        // Protocol deployer.
        let protocol_deployer = Box::new(ProtocolDeployer::new(
            Some(PROTOCOL_DEPLOYER.into()),
            Address::random_using(&mut rng),
            self.protocol_params,
            weth,
        ));
        let _ = sim_manager.activate_agent(protocol_deployer);

        // Pre-generate <Address, Capital> map for passive buyers and suppliers.
        let mut passive_buyers_map = HashMap::new();
        for _i in 0..self.passive_buyers_params.num_passive {
            passive_buyers_map.insert(
                Address::random_using(&mut rng),
                self.passive_buyers_params.capital_dist.sample(&mut rng),
            );
        }
        let mut active_buyers_map = HashMap::new();
        for _i in 0..self.active_buyers_params.num_active {
            active_buyers_map.insert(
                Address::random_using(&mut rng),
                self.active_buyers_params.capital_dist.sample(&mut rng),
            );
        }
        let mut suppliers_map = HashMap::new();
        for _i in 0..self.suppliers_params.num_passive {
            suppliers_map.insert(
                Address::random_using(&mut rng),
                self.suppliers_params.capital_dist.sample(&mut rng),
            );
        }

        // Base token deployer.
        let mut allocate_addrs = passive_buyers_map.clone();
        allocate_addrs.extend(active_buyers_map.clone());
        allocate_addrs.extend(suppliers_map.clone());
        let base_token_deployer = Box::new(TokenDeployer::new(
            Some(BASE_TOKEN_DEPLOYER.into()),
            Address::random_using(&mut rng),
            self.base_token_params,
            allocate_addrs,
        ));
        let _ = sim_manager.activate_agent(base_token_deployer);

        // Store protocol contracts.
        let world_protocol_contracts = sim_manager.get_state().world.protocol_contracts;

        // Cost models deployer.
        let cost_models_deployer = Box::new(CostModelsDeployer::new(
            Some(COST_MODELS_DEPLOYER.into()),
            Address::random_using(&mut rng),
            self.cost_models.iter().cloned().collect(),
            world_protocol_contracts
                .get_by_name(&COSTMODELJUMPRATEFACTORY.name)
                .unwrap(),
            world_protocol_contracts
                .get_by_name(&COSTMODELDYNAMICLEVELFACTORY.name)
                .unwrap(),
        ));
        let _ = sim_manager.activate_agent(cost_models_deployer);

        // Drip decay models deployer.
        let drip_decay_models_deployer = Box::new(DripDecayModelsDeployer::new(
            Some(DRIP_DECAY_MODELS_DEPLOYER.into()),
            Address::random_using(&mut rng),
            self.drip_decay_models.iter().cloned().collect(),
            world_protocol_contracts
                .get_by_name(&DRIPDECAYMODELCONSTANTFACTORY.name)
                .unwrap(),
        ));
        let _ = sim_manager.activate_agent(drip_decay_models_deployer);

        // Triggers deployer.
        let triggers_deployer = Box::new(TriggersDeployer::new(
            Some(TRIGGERS_DEPLOYER.into()),
            Address::random_using(&mut rng),
            self.triggers.iter().cloned().collect(),
            world_protocol_contracts
                .get_by_name(&UMATRIGGERFACTORY.name)
                .unwrap(),
            world_protocol_contracts
                .get_by_name(&CHAINLINKTRIGGERFACTORY.name)
                .unwrap(),
            world_protocol_contracts.get_by_name(&MANAGER.name).unwrap(),
            rng.clone(),
        ));
        let _ = sim_manager.activate_agent(triggers_deployer);

        // Store cost model, drip decay model and trigger contracts.
        let world_cost_models = sim_manager.get_state().world.cost_models;
        let world_drip_decay_models = sim_manager.get_state().world.drip_decay_models;
        let world_triggers = sim_manager.get_state().world.triggers;

        // Set admin.
        let mut market_configs = vec![];
        for (i, market_config_param) in self.market_config_params.into_iter().enumerate() {
            let cost_model_addr = world_cost_models.get_addr(&self.cost_models[i].0).unwrap();
            let drip_decay_model_addr = world_drip_decay_models
                .get_addr(&self.drip_decay_models[i].0)
                .unwrap();
            let trigger_addr = world_triggers.get_addr(&self.triggers[i].0).unwrap();
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
        let base_asset_addr = world_protocol_contracts
            .get_by_name(&BASE_TOKEN)
            .unwrap()
            .address;
        let set_params = CozySetAdminParams {
            asset: base_asset_addr,
            set_config: self.set_config_params.into(),
            market_configs,
            salt,
        };

        let set_admin = Box::new(SetAdmin::new(
            Some(SET_ADMIN.into()),
            Address::random_using(&mut rng),
            set_params,
            world_protocol_contracts.get_by_name(&SET.name).unwrap(),
            world_protocol_contracts.get_by_name(&MANAGER.name).unwrap(),
        ));
        let _ = sim_manager.activate_agent(set_admin);

        // Passive buyers.
        let world_triggers_vec = world_triggers
            .values()
            .iter()
            .map(|wt| wt.address)
            .collect::<Vec<_>>();
        for (i, (addr, _)) in passive_buyers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_BUYER, i + 1);
            let passive_buyer = Box::new(PassiveBuyer::new(
                Some(name.into()),
                addr,
                world_protocol_contracts
                    .get_by_name(&COZYROUTER.name)
                    .unwrap(),
                world_protocol_contracts.get_by_name(&BASE_TOKEN).unwrap(),
                world_protocol_contracts.get_by_name(&SET.name).unwrap(),
                world_triggers_vec[rng.gen_range(0..world_triggers_vec.len())],
                self.passive_buyers_params
                    .protection_desired_dist
                    .sample(&mut rng),
                self.passive_buyers_params
                    .time_dist
                    .sample_in_secs(&mut rng),
            ));
            let _ = sim_manager.activate_agent(passive_buyer);
        }

        // Active buyers.
        for (i, (addr, _)) in active_buyers_map.into_iter().enumerate() {
            let name = format!("{}{}", ACTIVE_BUYER, i + 1);
            let passive_buyer = Box::new(ActiveBuyer::new(
                Some(name.into()),
                addr,
                world_protocol_contracts
                    .get_by_name(&COZYROUTER.name)
                    .unwrap(),
                world_protocol_contracts.get_by_name(&BASE_TOKEN).unwrap(),
                world_protocol_contracts.get_by_name(&SET.name).unwrap(),
                world_triggers_vec[rng.gen_range(0..world_triggers_vec.len())],
                self.active_buyers_params.time_dist.sample_in_secs(&mut rng),
                rng.clone(),
            ));
            let _ = sim_manager.activate_agent(passive_buyer);
        }

        // Passive suppliers.
        for (i, (addr, _)) in suppliers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_SUPPLIER, i + 1);
            let passive_supplier = Box::new(PassiveSupplier::new(
                Some(name.into()),
                addr,
                world_protocol_contracts
                    .get_by_name(&COZYROUTER.name)
                    .unwrap(),
                world_protocol_contracts.get_by_name(&BASE_TOKEN).unwrap(),
                self.suppliers_params.time_dist.sample_in_secs(&mut rng),
            ));
            let _ = sim_manager.activate_agent(passive_supplier);
        }

        // Run sim.
        sim_manager.run_sim();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cozy::configs::build_cozy_sim_settings_from_dir;

    #[test]
    fn test_runner() -> Result<(), Box<dyn std::error::Error>> {
        let settings = build_cozy_sim_settings_from_dir("test")?;
        let runner = CozySingleSetSimRunner::new(settings);
        runner.run();
        Ok(())
    }
}
