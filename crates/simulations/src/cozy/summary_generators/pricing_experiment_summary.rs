use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::{
    address::Address,
    summarizer::SummaryGenerator,
    u256::{serialize_u256_to_u128, U256},
};

use crate::cozy::{
    types::CozyCostModelType,
    utils::wad_to_float,
    world::{CozyUpdate, CozyWorld},
    world_contracts::{CozyDynamicLevelModel, CozySetLogic},
};

#[derive(Serialize, Deserialize)]
pub struct PricingExperimentData {
    utilization: f64,
    cost_factor: Option<f64>,
    refund_factor: Option<f64>,
    cost_factor_in_optimal_zone: Option<f64>,
    #[serde(serialize_with = "serialize_u256_to_u128")]
    effective_active_protection: U256,
}

#[derive(Serialize, Deserialize)]
pub struct PricingExperimentSummary {
    #[serde(serialize_with = "serialize_u256_to_u128")]
    timestamp: U256,
    set_data: Vec<(Address, Vec<PricingExperimentData>)>,
}

pub struct PricingExperimentSummaryGenerator {
    summary_name: Cow<'static, str>,
    address: Address,
    set_logic: Arc<CozySetLogic>,
    dynamic_level_model: Option<Arc<CozyDynamicLevelModel>>,
}

impl PricingExperimentSummaryGenerator {
    pub fn new(
        set_logic: &Arc<CozySetLogic>,
        dynamic_level_model: &Option<Arc<CozyDynamicLevelModel>>,
    ) -> Box<Self> {
        Box::new(PricingExperimentSummaryGenerator {
            summary_name: Cow::Owned("Pricing Experiment Summary".to_owned()),
            address: Address::random(),
            set_logic: set_logic.clone(),
            dynamic_level_model: dynamic_level_model.clone(),
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for PricingExperimentSummaryGenerator {
    fn get_summary(
        &self,
        sim_state: &simulate::state::State<CozyUpdate, CozyWorld>,
    ) -> Result<Value, anyhow::Error> {
        let sets = sim_state.world.sets.values();

        let mut set_data = vec![];
        for set in sets {
            let mut cost_data = vec![];

            for i in 0..set.num_markets {
                let utilization = self
                    .set_logic
                    .read_utilization(self.address, sim_state, set.address, i)
                    .unwrap_or(U256::zero());

                let effective_active_protection = self
                    .set_logic
                    .read_effective_active_protection(self.address, sim_state, set.address, i)
                    .unwrap_or(U256::zero());

                let cost_model_addr = set.cost_model_lookup[&i];
                let cost_model = sim_state
                    .world
                    .cost_models
                    .get_by_addr(&cost_model_addr)
                    .expect("Cost model addr obtained from set.cost_model_lookup.");
                let (cost_factor, refund_factor, cost_factor_in_optimal_zone) = match cost_model
                    .model_type
                {
                    CozyCostModelType::DynamicLevel(_) => {
                        let dynamic_model_logic = self.dynamic_level_model.as_ref().expect(
                            "Cost model deployer should have deployed dynamic level model logic.",
                        );
                        (
                            dynamic_model_logic
                                .read_current_cost_factor(self.address, sim_state, utilization)
                                .ok(),
                            dynamic_model_logic
                                .read_current_refund_factor(self.address, sim_state, utilization)
                                .ok(),
                            dynamic_model_logic
                                .read_current_cost_factor_in_optimal_zone(
                                    self.address,
                                    sim_state,
                                    utilization,
                                )
                                .ok(),
                        )
                    }
                    _ => panic!("Pricing experiment only uses dynamic cost models."),
                };
                let float_cost_factor = cost_factor.map(wad_to_float);
                let float_refund_factor = refund_factor.map(wad_to_float);
                let float_cost_factor_in_optimal_zone =
                    cost_factor_in_optimal_zone.map(wad_to_float);

                cost_data.push(PricingExperimentData {
                    utilization: wad_to_float(utilization),
                    effective_active_protection,
                    cost_factor: float_cost_factor,
                    refund_factor: float_refund_factor,
                    cost_factor_in_optimal_zone: float_cost_factor_in_optimal_zone,
                });
            }

            set_data.push((set.address, cost_data));
        }

        let summary = PricingExperimentSummary {
            timestamp: sim_state.read_timestamp(),
            set_data,
        };

        Ok(serde_json::to_value(summary)?)
    }

    fn get_summary_name(&self) -> std::borrow::Cow<'static, str> {
        self.summary_name.clone()
    }
}
