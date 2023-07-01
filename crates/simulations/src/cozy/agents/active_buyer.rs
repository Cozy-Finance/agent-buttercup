use std::{borrow::Cow, cmp::min, collections::HashMap, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::{ExecutionResult, TxEnv};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
    utils::{build_call_tx, unpack_execution},
};

use crate::cozy::{
    constants::*,
    distributions::ProbTruncatedNorm,
    utils::float_to_wad,
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyRouter, CozySetLogic},
    EthersAddress, EthersU256, EvmU256,
};

pub struct ActiveBuyer {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    set_logic: Arc<CozySetLogic>,
    target_trigger: Address,
    protection_owned: EthersU256,
    ptokens_owned: HashMap<(Address, u16), EthersU256>,
    capital: EthersU256,
    waiting_time: EvmU256,
    last_action_time: EvmU256,
    rng: rand::rngs::StdRng,
}

pub struct ArbData {
    tx: TxEnv,
    amt: EthersU256,
    set_address: Address,
    market_id: u16,
}

impl ActiveBuyer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        set_logic: &Arc<CozySetLogic>,
        target_trigger: Address,
        waiting_time: f64,
        rng: rand::rngs::StdRng,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            target_trigger,
            protection_owned: EthersU256::from(0),
            ptokens_owned: HashMap::new(),
            capital: EthersU256::from(0),
            waiting_time: EvmU256::from(waiting_time),
            last_action_time: EvmU256::from(0),
            rng,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for ActiveBuyer {
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

        let targets = match self.get_target_sets_and_markets_ids(
            state,
            state.world.sets.values(),
            &self.target_trigger,
        ) {
            targets if targets.is_empty() => return,
            targets => targets,
        };

        let oracle_trigger_prob = state
            .world
            .triggers
            .get_by_addr(&self.target_trigger)
            .unwrap()
            .current_prob;
        let my_trigger_prob =
            ProbTruncatedNorm::new(oracle_trigger_prob + 0.01, 0.000001).sample(&mut self.rng);
        println!("{:?}", my_trigger_prob);

        let chosen_purchase = targets
            .iter()
            .map(|(set_address, market_id)| {
                self.get_arb_purchase_tx(state, *set_address, *market_id, my_trigger_prob)
            })
            .filter_map(Result::ok)
            .flatten()
            .max_by(|a, b| a.amt.cmp(&b.amt));
        if let Some(chosen_purchase) = chosen_purchase {
            channel.send_with_tag(
                SimUpdate::Evm(chosen_purchase.tx),
                format!(
                    "{} {:X} {}",
                    ACTIVE_BUYER_PURCHASE, chosen_purchase.set_address, chosen_purchase.market_id
                )
                .into(),
            );
        } else {
            println!("Sale loop");
            let chosen_sale = targets
                .iter()
                .map(|(set_address, market_id)| {
                    self.get_arb_sell_tx(state, *set_address, *market_id, my_trigger_prob)
                })
                .filter_map(Result::ok)
                .flatten()
                .max_by(|a, b| a.amt.cmp(&b.amt));
            if let Some(chosen_sale) = chosen_sale {
                println!("SALEEEEE");
                channel.send_with_tag(
                    SimUpdate::Evm(chosen_sale.tx),
                    format!(
                        "{} {:X} {}",
                        ACTIVE_BUYER_SALE, chosen_sale.set_address, chosen_sale.market_id
                    )
                    .into(),
                );
            }
        }
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
                                    self.ptokens_owned.insert(target, ptokens);
                                }
                                Some(curr_ptokens) => {
                                    *curr_ptokens += Into::<EthersU256>::into(ptokens);
                                    println!("curr_ptokens: {:?}", curr_ptokens);
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
                .get_protection_balance(self.address, state, *set_addr, *set_market_id, *ptokens)
                .unwrap();
        }
        self.protection_owned = protection_owned;
    }
}

impl ActiveBuyer {
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

    fn get_arb_purchase_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
        my_prob: f64,
    ) -> Result<Option<ArbData>> {
        let mut purchase_amt = self.set_logic.read_remaining_protection(
            self.address,
            state,
            set_address,
            market_id,
        )?;
        let mut max_cost = EthersU256::MAX;
        loop {
            if purchase_amt == EthersU256::zero() || max_cost == EthersU256::zero() {
                return Ok(None);
            }
            max_cost = min(
                (purchase_amt * float_to_wad(my_prob)) / EthersU256::from(1e18 as u128),
                self.capital,
            );
            let purchase_args = cozy_router::PurchaseCall {
                set: set_address.into(),
                market_id,
                protection: purchase_amt,
                receiver: self.address.into(),
                max_cost,
            };
            let purchase_tx = Some(
                self.cozyrouter
                    .build_purchase_tx(self.address, purchase_args)?,
            );
            let purchase_result = match unpack_execution(
                state.simulate_evm_tx_ref(purchase_tx.as_ref().unwrap(), None),
            ) {
                Ok(bytes) => bytes,
                _ => {
                    purchase_amt /= 2;
                    continue;
                }
            };
            let assets_needed = self
                .cozyrouter
                .contract
                .decode_output::<cozy_router::PurchaseReturn>("purchase", purchase_result)?
                .assets_needed;
            println!("assets_needed: {:?}", assets_needed);
            return Ok(Some(ArbData {
                tx: purchase_tx.unwrap(),
                amt: purchase_amt,
                set_address,
                market_id,
            }));
        }
    }

    fn get_arb_sell_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
        my_prob: f64,
    ) -> Result<Option<ArbData>> {
        let min_refund = (self.capital * float_to_wad(my_prob)) / EthersU256::from(1e18 as u128);

        let mut sell_amt = match self.ptokens_owned.get(&(set_address, market_id)) {
            None => return Ok(None),
            Some(ptokens_owned) => *ptokens_owned,
        };
        loop {
            if sell_amt == EthersU256::zero() {
                return Ok(None);
            }
            let sell_args = cozy_router::SellCall {
                set: set_address.into(),
                market_id,
                ptokens: sell_amt,
                receiver: self.address.into(),
                min_refund,
            };
            let sell_tx = Some(self.cozyrouter.build_sell_tx(self.address, sell_args)?);
            match unpack_execution(state.simulate_evm_tx_ref(sell_tx.as_ref().unwrap(), None)) {
                Ok(bytes) => bytes,
                _ => {
                    sell_amt /= 2;
                    continue;
                }
            };
            return Ok(Some(ArbData {
                tx: sell_tx.unwrap(),
                amt: sell_amt,
                set_address,
                market_id,
            }));
        }
    }
}
