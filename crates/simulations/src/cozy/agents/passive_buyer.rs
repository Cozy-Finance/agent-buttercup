use std::{borrow::Cow, cmp::min, collections::HashMap, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
};

use crate::cozy::{
    constants::*,
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyRouter, CozySetLogic},
    EthersU256, EvmU256,
};

pub struct PassiveBuyer {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    set_logic: Arc<CozySetLogic>,
    target_trigger: Address,
    protection_desired: EthersU256,
    protection_owned: EthersU256,
    ptokens_owned: HashMap<(Address, u16), EvmU256>,
    capital: EthersU256,
    waiting_time: EvmU256,
    last_action_time: EvmU256,
}

impl PassiveBuyer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        set_logic: &Arc<CozySetLogic>,
        target_trigger: Address,
        protection_desired: EthersU256,
        waiting_time: f64,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            target_trigger,
            protection_desired,
            protection_owned: EthersU256::from(0),
            ptokens_owned: HashMap::new(),
            capital: EthersU256::from(0),
            waiting_time: EvmU256::from(waiting_time),
            last_action_time: EvmU256::from(0),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for PassiveBuyer {
    fn id(&self) -> AgentId {
        AgentId {
            name: self.name.clone(),
            address: self.address,
        }
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        channel.send(SimUpdate::Evm(
            self.token
                .build_max_approve_router_tx(self.address, self.cozyrouter.address)
                .unwrap(),
        ));
        self.capital = self.token.read_token_balance(self.address, state).unwrap();
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.is_time_to_act(state.read_timestamp()) || self.capital <= EthersU256::from(0) {
            return;
        }

        let protection_delta = if self.protection_desired > self.protection_owned {
            self.protection_desired - self.protection_owned
        } else {
            0.into()
        };

        let sets = state.world.sets.values();
        let targets = self.get_target_sets_and_markets_ids(state, sets, &self.target_trigger);
        if targets.is_empty() {
            return;
        }

        let target_set_idx = targets
            .iter()
            .map(|(set_address, market_id)| {
                let purchase_args = cozy_router::PurchaseCall {
                    set: (*set_address).into(),
                    market_id: *market_id,
                    protection: protection_delta,
                    receiver: self.address.into(),
                    max_cost: EthersU256::MAX,
                };
                self.cozyrouter
                    .read_purchase_assets_needed(self.address, state, purchase_args)
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        let (set_addr, set_market_id) = targets[target_set_idx];
        let protection_purchase_amt = min(
            protection_delta,
            self.set_logic
                .read_remaining_protection(self.address, state, set_addr, set_market_id)
                .unwrap(),
        );
        let purchase_args = cozy_router::PurchaseCall {
            set: set_addr.into(),
            market_id: set_market_id,
            protection: protection_purchase_amt,
            receiver: self.address.into(),
            max_cost: EthersU256::MAX,
        };
        let evm_tx = self
            .cozyrouter
            .build_purchase_tx(self.address, purchase_args)
            .unwrap();
        channel.send_with_tag(
            SimUpdate::Evm(evm_tx),
            format!(
                "{} {:X} {}",
                PASSIVE_BUYER_PURCHASE, set_addr, set_market_id
            )
            .into(),
        );
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        if !self.is_time_to_act(state.read_timestamp()) {
            return;
        }
        self.capital = self.token.read_token_balance(self.address, state).unwrap();
        self.last_action_time = state.read_timestamp();

        if let Some(update_results) = state.update_results.get(&self.address) {
            for (tag, result) in update_results {
                let target = self.parse_purchase_tag(tag);
                match result {
                    SimUpdateResult::Evm(purchase_result) => {
                        let ptokens = self.cozyrouter.decode_ptokens_received(purchase_result);
                        if let Ok(ptokens) = ptokens {
                            match self.ptokens_owned.get_mut(&target) {
                                None => {
                                    self.ptokens_owned.insert(target, ptokens.into());
                                }
                                Some(curr_ptokens) => {
                                    *curr_ptokens += Into::<EvmU256>::into(ptokens)
                                }
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut protection_owned = EthersU256::from(0);
        for ((set_addr, set_market_id), ptokens) in self.ptokens_owned.iter() {
            protection_owned += self
                .set_logic
                .get_protection_balance(
                    self.address,
                    state,
                    *set_addr,
                    *set_market_id,
                    (*ptokens).into(),
                )
                .unwrap();
        }
        self.protection_owned = protection_owned;
    }
}

impl PassiveBuyer {
    fn is_time_to_act(&self, curr_timestamp: EvmU256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
    }

    fn parse_purchase_tag(&self, tag: &str) -> (Address, u16) {
        let mut split = tag.split_whitespace();
        let _ = split.next();
        let set_addr: Address = split.next().unwrap().parse().unwrap();
        let market_id: u16 = split.next().unwrap().parse().unwrap();
        (set_addr, market_id)
    }

    fn get_target_sets_and_markets_ids(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        sets: &Vec<CozySet>,
        trigger: &Address,
    ) -> Vec<(Address, u16)> {
        sets.iter()
            .filter(|set| set.trigger_lookup.contains_key(trigger))
            .map(|set| (set.address, *set.trigger_lookup.get(trigger).unwrap()))
            .collect::<Vec<_>>()
    }
}
