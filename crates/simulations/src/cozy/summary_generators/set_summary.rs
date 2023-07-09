use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::{address::Address, summarizer::SummaryGenerator};

use crate::cozy::{
    utils::serialize_EthersU256_to_u128,
    world::{CozyUpdate, CozyWorld},
    world_contracts::CozySetLogic,
    EthersU256,
};

#[derive(Serialize, Deserialize)]
pub struct SetData {
    #[serde(serialize_with = "serialize_EthersU256_to_u128")]
    protection_remaining: EthersU256,
    apy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SetSummary {
    #[serde(serialize_with = "serialize_EthersU256_to_u128")]
    timestamp: EthersU256,
    set_data: Vec<(Address, SetData)>,
}

pub struct SetSummaryGenerator {
    summary_name: Cow<'static, str>,
    address: Address,
    set_logic: Arc<CozySetLogic>,
}

/// Generates general summary data about the deployed sets in CozyWorld.
impl SetSummaryGenerator {
    pub fn new(set_logic: &Arc<CozySetLogic>) -> Box<Self> {
        Box::new(SetSummaryGenerator {
            set_logic: set_logic.clone(),
            summary_name: Cow::Owned("Set Summary".to_owned()),
            address: Address::random(),
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for SetSummaryGenerator {
    fn get_summary(
        &self,
        sim_state: &simulate::state::SimState<CozyUpdate, CozyWorld>,
    ) -> eyre::Result<Value> {
        let mut set_data = vec![];

        let sets = sim_state.world.sets.values();

        for set in sets {
            let protection_remaining = self
                .set_logic
                .read_total_protection_available(self.address, sim_state, set.address)
                .unwrap_or(EthersU256::from(0));

            set_data.push((
                set.address,
                SetData {
                    protection_remaining,
                    apy: set.apy,
                },
            ))
        }

        let summary = SetSummary {
            timestamp: EthersU256::from(sim_state.read_timestamp()),
            set_data,
        };

        Ok(serde_json::to_value(summary)?)
    }

    fn get_summary_name(&self) -> std::borrow::Cow<'static, str> {
        self.summary_name.clone()
    }
}
