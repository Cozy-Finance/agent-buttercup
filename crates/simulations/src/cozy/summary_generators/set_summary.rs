use std::{borrow::Cow, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::{
    address::Address,
    state::State,
    summarizer::SummaryGenerator,
    u256::{serialize_u256_to_u128, U256},
};

use crate::cozy::{
    runner::{ProtocolContracts, SetContracts},
    utils::wad_to_float,
    world::{CozyUpdate, CozyWorld},
};

#[derive(Serialize, Deserialize)]
pub struct SetSummary {
    #[serde(serialize_with = "serialize_u256_to_u128")]
    timestamp: U256,
    apy: f64,
    market_apys: Vec<f64>,
    portfolio_weights: Vec<f64>,
    utilizations: Vec<f64>,
    effective_active_protections: Vec<f64>,
}

pub struct SetSummaryGenerator {
    summary_name: Cow<'static, str>,
    _protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    address: Address,
}

/// Generates general summary data about a deployed set.
impl SetSummaryGenerator {
    pub fn new(_protocol: Arc<ProtocolContracts>, set: Arc<SetContracts>) -> Box<Self> {
        Box::new(SetSummaryGenerator {
            summary_name: Cow::Owned("Set Summary".to_owned()),
            address: Address::random(),
            _protocol,
            set,
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for SetSummaryGenerator {
    fn get_summary(&self, state: &State<CozyUpdate, CozyWorld>) -> Result<Value, anyhow::Error> {
        let set_analytics = state.world.set_analytics.clone();

        let mut eaps = vec![];
        for i in 0..set_analytics.market_apys.len() {
            let eap = state
                .call_evm_tx_and_decode(
                    self.address,
                    self.set.set.effective_active_protection(i as u16),
                )
                .expect("Error getting eap.");
            eaps.push(wad_to_float(eap));
        }

        let summary = SetSummary {
            apy: set_analytics.set_apy,
            market_apys: set_analytics.market_apys.as_slice().to_vec(),
            portfolio_weights: set_analytics.portfolio_weights.as_slice().to_vec(),
            utilizations: set_analytics.utilizations.as_slice().to_vec(),
            timestamp: state.timestamp(),
            effective_active_protections: eaps,
        };

        Ok(serde_json::to_value(summary)?)
    }

    fn get_summary_name(&self) -> std::borrow::Cow<'static, str> {
        self.summary_name.clone()
    }
}
