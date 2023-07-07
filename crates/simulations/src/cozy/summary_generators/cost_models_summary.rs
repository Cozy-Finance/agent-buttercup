use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::{address::Address, summarizer::SummaryGenerator, utils::unpack_execution};

use crate::cozy::{
    types::CozyCostModelType,
    utils::wad_to_float,
    world::{CozyUpdate, CozyWorld},
    world_contracts::{CozyDynamicLevelModel, CozyJumpRateModel, CozySetLogic},
    EthersU256,
};

#[derive(Serialize, Deserialize)]
pub struct CostModelsSummary {
    timestamp: EthersU256,
    set_utilizations: Vec<Vec<Option<f64>>>,
    set_cost_factors: Vec<Vec<Option<f64>>>,
}

pub struct CostModelsSummaryGenerator {
    summary_name: Cow<'static, str>,
    // Place Holder address for calling read txs.
    address: Address,
    set_logic: Arc<CozySetLogic>,
    jump_rate_model: Option<Arc<CozyJumpRateModel>>,
    dynamic_level_model: Option<Arc<CozyDynamicLevelModel>>,
}

/// Generates general summary data about the deployed sets in CozyWorld.
impl CostModelsSummaryGenerator {
    pub fn new(
        set_logic: &Arc<CozySetLogic>,
        jump_rate_model: &Option<Arc<CozyJumpRateModel>>,
        dynamic_level_model: &Option<Arc<CozyDynamicLevelModel>>,
    ) -> Box<Self> {
        Box::new(CostModelsSummaryGenerator {
            summary_name: Cow::Owned("Cost Model Summary".to_owned()),
            address: Address::random(),
            set_logic: set_logic.clone(),
            jump_rate_model: jump_rate_model.clone(),
            dynamic_level_model: dynamic_level_model.clone(),
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for CostModelsSummaryGenerator {
    fn get_summary(
        &self,
        sim_state: &simulate::state::SimState<CozyUpdate, CozyWorld>,
    ) -> eyre::Result<Value> {
        let mut set_utilizations = vec![];
        let mut set_cost_factors = vec![];

        let sets = sim_state.world.sets.values();

        for set in sets {
            let mut utilizations = vec![];
            let mut cost_factors = vec![];
            for i in 0..set.num_markets {
                let utilization = self
                    .set_logic
                    .read_utilization(self.address, sim_state, set.address, i as u16)
                    .unwrap_or(EthersU256::from(0));
                utilizations.push(Some(wad_to_float(utilization)));

                let cost_model_addr = set.cost_model_lookup[&i];
                let cost_model = sim_state
                    .world
                    .cost_models
                    .get_by_addr(&cost_model_addr)
                    .unwrap();
                let cost_factor = match cost_model.model_type {
                    CozyCostModelType::JumpRate(_) => self
                        .jump_rate_model
                        .as_ref()
                        .expect("Cost model deployer should have deployed jump rate model logic.")
                        .read_current_cost_factor(self.address, sim_state, utilization)
                        .ok(),
                    CozyCostModelType::DynamicLevel(_) => self
                        .dynamic_level_model
                        .as_ref()
                        .expect(
                            "Cost model deployer should have deployed dynamic level model logic.",
                        )
                        .read_current_cost_factor(self.address, sim_state, utilization)
                        .ok(),
                };
                let float_cost_factor = match cost_factor {
                    Some(cost_factor) => Some(wad_to_float(cost_factor)),
                    None => None,
                };
                cost_factors.push(float_cost_factor);
            }

            set_utilizations.push(utilizations);
            set_cost_factors.push(cost_factors);
        }

        let summary = CostModelsSummary {
            timestamp: EthersU256::from(sim_state.read_timestamp()),
            set_utilizations,
            set_cost_factors,
        };

        let return_val = serde_json::to_value(summary)?;
        Ok(return_val)
    }

    fn get_summary_name(&self) -> std::borrow::Cow<'static, str> {
        self.summary_name.clone()
    }
}
