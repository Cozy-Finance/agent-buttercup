use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use simulate::address::Address;
use simulate::summarizer::SummaryGenerator;
use simulate::utils::unpack_execution;

use crate::cozy::bindings_wrapper::SET;
use crate::cozy::world::{CozyUpdate, CozyWorld};
use crate::cozy::EthersU256;

#[derive(Serialize, Deserialize)]
pub struct ProtectionSummary {
    timestamp: EthersU256,
    protection_bought: Vec<(Address, EthersU256)>,
}

pub struct SetSummaryGenerator {
    summary_name: Cow<'static, str>,

    // Place Holder address for calling read txs.
    address: Address,
}

/// Generates general summary data about the deployed sets in CozyWorld.
impl SetSummaryGenerator {
    pub fn new() -> Box<Self> {
        Box::new(SetSummaryGenerator {
            summary_name: Cow::Owned("Protection Summary".to_owned()),
            address: Address::random(),
        })
    }
}

impl SummaryGenerator<CozyUpdate, CozyWorld> for SetSummaryGenerator {
    fn get_summary(
        &self,
        sim_state: &simulate::state::SimState<CozyUpdate, CozyWorld>,
    ) -> eyre::Result<Value> {
        let set_addrs = sim_state
            .world
            .sets
            .items
            .iter()
            .map(|set| set.address)
            .collect::<Vec<_>>();

        let set_logic = sim_state
            .world
            .set_logic
            .as_ref()
            .expect("Set Logic should be set when summarizer runs");

        let addr_protection_bought = set_addrs
            .iter()
            .map(|set_addr| {
                (
                    set_addr.clone(),
                    set_logic
                        .read_total_protection_available(self.address, sim_state, set_addrs[0])
                        .unwrap_or(EthersU256::from(0)),
                )
            })
            .collect::<Vec<_>>();

        let summary = ProtectionSummary {
            timestamp: EthersU256::from(sim_state.read_timestamp()),
            protection_bought: addr_protection_bought,
        };

        let return_val = serde_json::to_value(summary)?;
        Ok(return_val)
    }

    fn get_summary_name(&self) -> std::borrow::Cow<'static, str> {
        self.summary_name.clone()
    }
}
