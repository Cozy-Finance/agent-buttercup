use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::{address::Address, summarizer::SummaryGenerator};

use crate::cozy::{
    utils::serialize_ethers_u256_to_u128,
    world::{CozyUpdate, CozyWorld},
    world_contracts::CozySetLogic,
    EthersU256,
};

#[derive(Serialize, Deserialize)]
pub struct SetData {
    apy: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SetSummary {
    #[serde(serialize_with = "serialize_ethers_u256_to_u128")]
    timestamp: EthersU256,
    set_data: Vec<(Address, SetData)>,
}

pub struct SetSummaryGenerator {
    summary_name: Cow<'static, str>,
    _address: Address,
    _set_logic: Arc<CozySetLogic>,
}

/// Generates general summary data about the deployed sets in CozyWorld.
impl SetSummaryGenerator {
    pub fn new(set_logic: &Arc<CozySetLogic>) -> Box<Self> {
        Box::new(SetSummaryGenerator {
            _set_logic: set_logic.clone(),
            summary_name: Cow::Owned("Set Summary".to_owned()),
            _address: Address::random(),
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for SetSummaryGenerator {
    fn get_summary(
        &self,
        sim_state: &simulate::state::SimState<CozyUpdate, CozyWorld>,
    ) -> Result<Value, anyhow::Error> {
        let mut set_data = vec![];

        let sets = sim_state.world.sets.values();

        for set in sets {
            set_data.push((set.address, SetData { apy: set.apy }))
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