use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
    sync::Arc,
};

use bindings::cozy_protocol::cozy_router;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
    u256::{f64_to_u256, U256},
    utils::is_execution_success,
};

use crate::cozy::{
    constants::*,
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyRouter, CozySetLogic},
};

pub struct PassiveBuyer {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    set_logic: Arc<CozySetLogic>,
    target_trigger: Address,
    protection_desired: U256,
    protection_owned: U256,
    ptokens_owned: HashMap<(Address, u16), U256>,
    capital: U256,
    waiting_time: U256,
    last_action_time: U256,
}

#[derive(Debug, Clone)]
pub struct PassiveBuyerTxData {
    tx_type: Cow<'static, str>,
    amt: U256,
    set_address: Address,
    market_id: u16,
}

impl Display for PassiveBuyerTxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:X} {} {}",
            self.tx_type, self.set_address, self.market_id, self.amt
        )?;
        Ok(())
    }
}

impl FromStr for PassiveBuyerTxData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!(
                "PassiveBuyerTxData string must split into four tokens: {}.",
                s
            ));
        }

        let amt = U256::from_dec_str(parts.pop().expect("Checked parts.len() == 4."))?;
        let market_id: u16 = parts.pop().expect("Checked parts.len() == 4.").parse()?;
        let set_address: Address = parts.pop().expect("Checked parts.len() == 4.").parse()?;
        let tx_type: String = parts.pop().expect("Checked parts.len() == 4.").into();

        Ok(PassiveBuyerTxData {
            tx_type: tx_type.into(),
            amt,
            set_address,
            market_id,
        })
    }
}

impl PassiveBuyer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        set_logic: &Arc<CozySetLogic>,
        target_trigger: Address,
        protection_desired: U256,
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
            protection_owned: U256::zero(),
            ptokens_owned: HashMap::new(),
            capital: U256::zero(),
            waiting_time: f64_to_u256(waiting_time),
            last_action_time: U256::zero(),
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
                .expect("PassiveBuyer failed to build approve tx."),
        ));
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("PassiveBuyer failed to read token balance.");
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.is_time_to_act(state.read_timestamp()) || self.capital <= U256::zero() {
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
                    max_cost: U256::MAX,
                };
                self.cozyrouter
                    .read_purchase_assets_needed(self.address, state, purchase_args)
                    .expect("PassiveBuyer failed to read purchase assets needed.")
            })
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .expect("Checked that targets is not empty.");

        let (set_addr, set_market_id) = targets[target_set_idx];
        let purchase_args = cozy_router::PurchaseCall {
            set: set_addr.into(),
            market_id: set_market_id,
            protection: protection_delta,
            receiver: self.address.into(),
            max_cost: U256::MAX,
        };
        let evm_tx = self
            .cozyrouter
            .build_purchase_tx(self.address, purchase_args)
            .expect("PassiveBuyer failed to build purchase tx.");

        channel.send_with_tag(
            SimUpdate::Evm(evm_tx),
            format!(
                "{}",
                PassiveBuyerTxData {
                    tx_type: PASSIVE_BUYER_PURCHASE.into(),
                    amt: protection_delta,
                    set_address: set_addr,
                    market_id: set_market_id,
                }
            )
            .into(),
        );
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        if !self.is_time_to_act(state.read_timestamp()) {
            return;
        }
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("PassiveBuyer failed to read token balance.");
        self.last_action_time = state.read_timestamp();

        if let Some(update_results) = state.update_results.get(&self.address) {
            for (tag, result) in update_results {
                match result {
                    SimUpdateResult::Evm(result) if tag.parse::<PassiveBuyerTxData>().is_ok() => {
                        let tx_data: PassiveBuyerTxData =
                            tag.parse().expect("PassiveBuyer failed to parse tag.");
                        if tx_data.tx_type == ACTIVE_BUYER_PURCHASE && is_execution_success(result)
                        {
                            let ptokens_received = self
                                .cozyrouter
                                .decode_ptokens_received(result)
                                .expect("PassiveBuyer failed to decode ptokens received.");
                            match self
                                .ptokens_owned
                                .get_mut(&(tx_data.set_address, tx_data.market_id))
                            {
                                None => {
                                    self.ptokens_owned.insert(
                                        (tx_data.set_address, tx_data.market_id),
                                        ptokens_received,
                                    );
                                }
                                Some(curr_ptokens) => {
                                    *curr_ptokens += Into::<U256>::into(ptokens_received);
                                }
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        self.protection_owned = U256::zero();
        for ((set_addr, set_market_id), ptokens) in self.ptokens_owned.iter() {
            self.protection_owned += self
                .set_logic
                .read_protection_balance(self.address, state, *set_addr, *set_market_id, *ptokens)
                .expect("PassiveBuyer failed to read protection balance.");
        }
    }
}

impl PassiveBuyer {
    fn is_time_to_act(&self, curr_timestamp: U256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
    }

    fn get_target_sets_and_markets_ids(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        sets: &[CozySet],
        trigger: &Address,
    ) -> Vec<(Address, u16)> {
        sets.iter()
            .filter(|set| set.trigger_lookup.contains_key(trigger))
            .map(|set| {
                (
                    set.address,
                    *set.trigger_lookup.get(trigger).expect("Checked in filter."),
                )
            })
            .collect::<Vec<_>>()
    }
}
