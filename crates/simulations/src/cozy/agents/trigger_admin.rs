use std::{borrow::Cow, collections::HashSet, sync::Arc};

#[allow(unused_imports)]
use rand_distr::Distribution;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannelSender, Agent},
    state::State,
    u256::U256,
};

use crate::cozy::{
    runner::{ProtocolContracts, SetContracts},
    types::TriggerSimulator,
    world::{CozyUpdate, CozyWorld},
};

pub struct TriggerAdmin {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    trigger_simulator: TriggerSimulator,
    triggered_markets: HashSet<usize>,
}

impl TriggerAdmin {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
        trigger_simulator: TriggerSimulator,
    ) -> Self {
        Self {
            _name,
            address,
            protocol,
            set,
            trigger_simulator,
            triggered_markets: HashSet::new(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for TriggerAdmin {
    fn address(&self) -> Address {
        self.address
    }

    fn activation_step(&mut self, state: &mut State<CozyUpdate, CozyWorld>) {
        let router_approve_tx = self
            .set
            .base_token
            .approve(self.protocol.cozy_router.address(), U256::MAX);
        let _ = state.execute_evm_tx_and_decode(self.address, router_approve_tx);
    }

    fn step(
        &mut self,
        _state: &State<CozyUpdate, CozyWorld>,
        _channel: &AgentChannelSender<CozyUpdate>,
    ) {
        // Sample triggers.
        let trigger_outcomes = self.trigger_simulator.sample();
        let triggered_idxs: Vec<usize> = trigger_outcomes
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == 1.0)
            .map(|(i, _)| i)
            .collect();

        // Trigger markets if needed.
        if !triggered_idxs.is_empty() {
            for idx in triggered_idxs {
                if self.triggered_markets.get(&idx).is_some() {
                    continue;
                }
                self.triggered_markets.insert(idx);
                let _trigger_tx = self
                    .set
                    .dummy_triggers
                    .get(&(idx as u32))
                    .expect("Trigger index out of bounds.")
                    .trigger();
                // _channel.execute_evm_tx(_trigger_tx);
            }
        }
    }
}
