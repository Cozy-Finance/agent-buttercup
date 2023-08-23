use std::{borrow::Cow, collections::HashMap};

use bindings::cozy_protocol::shared_types::MarketConfig;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::Deserialize;
use simulate::{
    address::Address, manager::SimManager, state::SimState, summarizer::Summarizer,
    time_policy::FixedTimePolicy,
};

use crate::cozy::{
    agents::{
        active_buyer::ActiveBuyer, cost_models_deployer::CostModelsDeployer,
        drip_decay_models_deployer::DripDecayModelsDeployer, passive_buyer::PassiveBuyer,
        passive_supplier::PassiveSupplier, protocol_deployer::ProtocolDeployer,
        set_admin::SetAdmin, token_deployer::TokenDeployer, triggers_deployer::TriggersDeployer,
        weth_deployer::WethDeployer,
    },
    constants::*,
    distributions::CozyDistribution,
    summary_generators::{
        cost_models_summary::CostModelsSummaryGenerator,
        pricing_experiment_summary::PricingExperimentSummaryGenerator,
        set_summary::SetSummaryGenerator,
    },
    types::{
        CozyActiveBuyersParams, CozyCostModelType, CozyDripDecayModelType,
        CozyFixedTimePolicyParams, CozyMarketConfigParams, CozyPassiveBuyersParams,
        CozyProtocolDeployParams, CozySetAdminParams, CozySetConfigParams, CozySimSetupParams,
        CozySuppliersParams, CozyTokenDeployParams, CozyTriggerType,
    },
    utils::deserialize_cow_tuple_vec,
    world::{CozyUpdate, CozyWorld},
};

#[derive(Debug, Clone)]
pub enum CozySingleSetSummaryGenerators {
    Set,
    CostModels,
    PricingExperiment,
}

impl CozySingleSetSummaryGenerators {
    pub fn register_generator(&self, sim_manager: &mut SimManager<CozyUpdate, CozyWorld>) {
        match self {
            CozySingleSetSummaryGenerators::Set => {
                sim_manager
                    .summarizer
                    .register_summary_generator(SetSummaryGenerator::new(
                        sim_manager
                            .get_state()
                            .world
                            .set_logic
                            .as_ref()
                            .expect("Set logic added to world."),
                    ));
            }
            CozySingleSetSummaryGenerators::CostModels => {
                sim_manager
                    .summarizer
                    .register_summary_generator(CostModelsSummaryGenerator::new(
                        sim_manager
                            .get_state()
                            .world
                            .set_logic
                            .as_ref()
                            .expect("Set logic added to world."),
                        &sim_manager.get_state().world.jump_rate_model,
                        &sim_manager.get_state().world.dynamic_level_model,
                    ));
            }
            CozySingleSetSummaryGenerators::PricingExperiment => {
                sim_manager.summarizer.register_summary_generator(
                    PricingExperimentSummaryGenerator::new(
                        sim_manager
                            .get_state()
                            .world
                            .set_logic
                            .as_ref()
                            .expect("Set logic added to world."),
                        &sim_manager.get_state().world.dynamic_level_model,
                    ),
                );
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CozySingleSetSimRunner {
    pub sim_setup_params: CozySimSetupParams,
    pub protocol_params: CozyProtocolDeployParams,
    pub time_policy_params: CozyFixedTimePolicyParams,
    pub base_token_params: CozyTokenDeployParams,
    pub passive_buyers_params: CozyPassiveBuyersParams,
    pub active_buyers_params: CozyActiveBuyersParams,
    pub suppliers_params: CozySuppliersParams,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub triggers: Vec<(Cow<'static, str>, CozyTriggerType)>,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    pub drip_decay_models: Vec<(Cow<'static, str>, CozyDripDecayModelType)>,
    pub market_config_params: Vec<CozyMarketConfigParams>,
    pub set_config_params: CozySetConfigParams,
}

impl CozySingleSetSimRunner {
    pub fn run(
        self,
        output_file: Cow<'static, str>,
        summary_generators: Vec<CozySingleSetSummaryGenerators>,
    ) -> Result<(), anyhow::Error> {
        let mut rng = StdRng::seed_from_u64(self.sim_setup_params.rand_seed);

        // Create sim manager.
        let world_state = CozyWorld::new();
        let sim_state = SimState::new(world_state);
        let fixed_time_policy = FixedTimePolicy::new(
            self.time_policy_params.start_block_number,
            self.time_policy_params.start_block_timestamp,
            self.time_policy_params.time_per_block,
            self.time_policy_params.blocks_per_step,
            self.time_policy_params.blocks_to_generate,
            self.time_policy_params.time_to_generate,
        )
        .expect("FixedTimePolicy params error.");
        let mut sim_manager = SimManager::new(
            sim_state,
            Box::new(fixed_time_policy),
            Summarizer::new(output_file),
        );

        // Create and activate agents.
        // Weth deployer.
        let weth_deployer = Box::new(WethDeployer::new(
            WETH_DEPLOYER.into(),
            Address::random_using(&mut rng),
        ));
        sim_manager.activate_agent(weth_deployer)?;

        // Protocol deployer.
        let protocol_deployer = Box::new(ProtocolDeployer::new(
            PROTOCOL_DEPLOYER.into(),
            Address::random_using(&mut rng),
            self.protocol_params,
            sim_manager
                .get_state()
                .world
                .weth
                .as_ref()
                .expect("Weth added to world."),
        ));
        sim_manager.activate_agent(protocol_deployer)?;

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
            BASE_TOKEN_DEPLOYER.into(),
            Address::random_using(&mut rng),
            self.base_token_params,
            allocate_addrs,
        ));
        sim_manager.activate_agent(base_token_deployer)?;

        // Cost models deployer.
        let cost_models_deployer = Box::new(CostModelsDeployer::new(
            COST_MODELS_DEPLOYER.into(),
            Address::random_using(&mut rng),
            self.cost_models.iter().cloned().collect(),
            sim_manager
                .get_state()
                .world
                .jump_rate_factory
                .as_ref()
                .expect("Jump rate factory added to world."),
            sim_manager
                .get_state()
                .world
                .dynamic_level_factory
                .as_ref()
                .expect("Dynamic level factory added to world."),
        ));
        sim_manager.activate_agent(cost_models_deployer)?;

        // Drip decay models deployer.
        let drip_decay_models_deployer = Box::new(DripDecayModelsDeployer::new(
            DRIP_DECAY_MODELS_DEPLOYER.into(),
            Address::random_using(&mut rng),
            self.drip_decay_models.iter().cloned().collect(),
            sim_manager
                .get_state()
                .world
                .drip_decay_constant_factory
                .as_ref()
                .expect("Drip decay constant factory added to world."),
        ));
        sim_manager.activate_agent(drip_decay_models_deployer)?;

        // Triggers deployer.
        let triggers_deployer = Box::new(TriggersDeployer::new(
            TRIGGERS_DEPLOYER.into(),
            Address::random_using(&mut rng),
            self.triggers.iter().cloned().collect(),
            sim_manager
                .get_state()
                .world
                .uma_trigger_factory
                .as_ref()
                .expect("Uma trigger factory added to world."),
            sim_manager
                .get_state()
                .world
                .chainlink_trigger_factory
                .as_ref()
                .expect("Chainlink trigger factory added to world."),
            sim_manager
                .get_state()
                .world
                .manager
                .as_ref()
                .expect("Manager added to world."),
            rng.clone(),
        ));
        sim_manager.activate_agent(triggers_deployer)?;

        // Store cost model, drip decay model and trigger contracts.
        let world_cost_models = sim_manager.get_state().world.cost_models.clone();
        let world_drip_decay_models = sim_manager.get_state().world.drip_decay_models.clone();
        let world_triggers = sim_manager.get_state().world.triggers.clone();

        // Set admin.
        let mut market_configs = vec![];
        for (i, market_config_param) in self.market_config_params.into_iter().enumerate() {
            let cost_model_addr = world_cost_models
                .get_addr(&self.cost_models[i].0)
                .expect("Cost models added to world.");
            let drip_decay_model_addr = world_drip_decay_models
                .get_addr(&self.drip_decay_models[i].0)
                .expect("Drip decay models added to world.");
            let trigger_addr = world_triggers
                .get_addr(&self.triggers[i].0)
                .expect("Triggers added to world.");
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
        let base_asset_addr = sim_manager
            .get_state()
            .world
            .base_token
            .as_ref()
            .expect("Base token added to world.")
            .address;
        let set_params = CozySetAdminParams {
            asset: base_asset_addr,
            set_config: self.set_config_params.into(),
            market_configs,
            salt,
        };

        let set_admin = Box::new(SetAdmin::new(
            SET_ADMIN.into(),
            Address::random_using(&mut rng),
            set_params,
            sim_manager
                .get_state()
                .world
                .set_logic
                .as_ref()
                .expect("Set logic added to world."),
            sim_manager
                .get_state()
                .world
                .manager
                .as_ref()
                .expect("Manager added to world."),
        ));
        sim_manager.activate_agent(set_admin)?;

        // Passive buyers.
        let world_triggers_vec = world_triggers
            .values()
            .iter()
            .map(|wt| wt.address)
            .collect::<Vec<_>>();
        for (i, (addr, _)) in passive_buyers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_BUYER, i + 1);
            let passive_buyer = Box::new(PassiveBuyer::new(
                name.into(),
                addr,
                sim_manager
                    .get_state()
                    .world
                    .cozy_router
                    .as_ref()
                    .expect("Rouer added to world."),
                sim_manager
                    .get_state()
                    .world
                    .base_token
                    .as_ref()
                    .expect("Base token added to world."),
                sim_manager
                    .get_state()
                    .world
                    .set_logic
                    .as_ref()
                    .expect("Set logic added to world."),
                world_triggers_vec[rng.gen_range(0..world_triggers_vec.len())],
                self.passive_buyers_params
                    .protection_desired_dist
                    .sample(&mut rng),
                self.passive_buyers_params
                    .time_dist
                    .sample_in_secs(&mut rng),
            ));
            sim_manager.activate_agent(passive_buyer)?;
        }

        // Active buyers.
        for (i, (addr, _)) in active_buyers_map.into_iter().enumerate() {
            let name = format!("{}{}", ACTIVE_BUYER, i + 1);
            let passive_buyer = Box::new(ActiveBuyer::new(
                name.into(),
                addr,
                sim_manager
                    .get_state()
                    .world
                    .cozy_router
                    .as_ref()
                    .expect("Router added to world."),
                sim_manager
                    .get_state()
                    .world
                    .base_token
                    .as_ref()
                    .expect("Base Token added to world"),
                sim_manager
                    .get_state()
                    .world
                    .set_logic
                    .as_ref()
                    .expect("Set logic added to world."),
                sim_manager
                    .get_state()
                    .world
                    .ptoken_logic
                    .as_ref()
                    .expect("Ptoken logic added to world."),
                world_triggers_vec[rng.gen_range(0..world_triggers_vec.len())],
                self.active_buyers_params.time_dist.sample_in_secs(&mut rng),
                self.active_buyers_params.trigger_prob_dist.clone(),
                rng.clone(),
            ));
            sim_manager.activate_agent(passive_buyer)?;
        }

        // Passive suppliers.
        for (i, (addr, _)) in suppliers_map.into_iter().enumerate() {
            let name = format!("{}{}", PASSIVE_SUPPLIER, i + 1);
            let passive_supplier = Box::new(PassiveSupplier::new(
                name.into(),
                addr,
                sim_manager
                    .get_state()
                    .world
                    .cozy_router
                    .as_ref()
                    .expect("Router added to world."),
                sim_manager
                    .get_state()
                    .world
                    .base_token
                    .as_ref()
                    .expect("Base Token added to world"),
                self.suppliers_params.time_dist.sample_in_secs(&mut rng),
            ));
            sim_manager.activate_agent(passive_supplier)?;
        }

        // Register summarizer generators.
        for generator in summary_generators {
            generator.register_generator(&mut sim_manager);
        }

        // Run sim.
        sim_manager.run_sim()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cozy::configs::build_cozy_sim_runner_from_dir;

    #[test]
    fn test_runner() -> Result<(), Box<dyn std::error::Error>> {
        let runner = build_cozy_sim_runner_from_dir("test")?;
        runner.run(
            Cow::Borrowed("output/summaries/test_output.json"),
            vec![
                CozySingleSetSummaryGenerators::Set,
                CozySingleSetSummaryGenerators::CostModels,
            ],
        )?;
        Ok(())
    }
}
