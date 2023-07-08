use std::{
    borrow::Cow,
    cmp::min,
    collections::HashMap,
    fmt::{Display, Formatter},
    str::FromStr,
    sync::Arc,
};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
    utils::{is_execution_success, unpack_execution},
};

use crate::cozy::{
    constants::*,
    types::CozyActiveBuyerTriggerProbDist,
    utils::{float_to_wad, wad},
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyPtokenLogic, CozyRouter, CozySetLogic},
    EthersU256, EvmU256,
};

pub struct ActiveBuyer {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    ptoken_logic: Arc<CozyPtokenLogic>,
    set_logic: Arc<CozySetLogic>,
    target_trigger: Address,
    protection_owned: EthersU256,
    ptokens_owned: HashMap<(Address, u16), EthersU256>,
    capital: EthersU256,
    waiting_time: EvmU256,
    last_action_time: EvmU256,
    trigger_prob_dist: CozyActiveBuyerTriggerProbDist,
    rng: rand::rngs::StdRng,
}

#[derive(Debug, Clone)]
pub struct ActiveBuyerTxData {
    tx_type: Cow<'static, str>,
    amt: EthersU256,
    set_address: Address,
    market_id: u16,
}

impl Display for ActiveBuyerTxData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:X} {} {}",
            self.tx_type, self.set_address, self.market_id, self.amt
        )?;
        Ok(())
    }
}

impl FromStr for ActiveBuyerTxData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace().collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err("Invalid input format".to_string());
        }

        let amt = EthersU256::from_dec_str(parts.pop().unwrap()).unwrap();
        let market_id: u16 = parts.pop().unwrap().parse().unwrap();
        let set_address: Address = parts.pop().unwrap().parse().unwrap();
        let tx_type: String = parts.pop().unwrap().into();

        Ok(ActiveBuyerTxData {
            tx_type: tx_type.into(),
            amt,
            set_address,
            market_id,
        })
    }
}

impl ActiveBuyer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        set_logic: &Arc<CozySetLogic>,
        ptoken_logic: &Arc<CozyPtokenLogic>,
        target_trigger: Address,
        waiting_time: f64,
        trigger_prob_dist: CozyActiveBuyerTriggerProbDist,
        rng: rand::rngs::StdRng,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            ptoken_logic: ptoken_logic.clone(),
            target_trigger,
            protection_owned: EthersU256::from(0),
            ptokens_owned: HashMap::new(),
            capital: EthersU256::from(0),
            waiting_time: EvmU256::from(waiting_time),
            last_action_time: EvmU256::from(0),
            trigger_prob_dist,
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
        let my_trigger_prob = self
            .trigger_prob_dist
            .sample(&mut self.rng, oracle_trigger_prob);

        // Check if you want to make a purchase.
        let chosen_purchase = targets
            .iter()
            .map(|(set_address, market_id)| {
                self.get_arb_purchase_tx(state, *set_address, *market_id, my_trigger_prob)
            })
            .filter_map(Result::ok)
            .flatten()
            .max_by(|(_, a_data), (_, b_data)| a_data.amt.cmp(&b_data.amt));

        if let Some((chosen_purchase_tx, chosen_purchase_data)) = chosen_purchase {
            channel.send_with_tag(
                SimUpdate::Evm(chosen_purchase_tx),
                format!("{}", chosen_purchase_data).into(),
            );

            // Approve CozyRouter to spend corresponding pTokens, in case you want to sell.
            let ptoken_addr = self
                .set_logic
                .read_ptoken_addr(
                    self.address,
                    state,
                    chosen_purchase_data.set_address,
                    chosen_purchase_data.market_id,
                )
                .unwrap();
            let ptoken_approve_tx = self
                .ptoken_logic
                .build_max_approve_router_tx(self.address, ptoken_addr, self.cozyrouter.address)
                .unwrap();

            channel.send(SimUpdate::Evm(ptoken_approve_tx));
        } else {
            // Check if you want to make a sale.
            let chosen_sale = targets
                .iter()
                .map(|(set_address, market_id)| {
                    self.get_arb_sell_tx(state, *set_address, *market_id, my_trigger_prob)
                })
                .filter_map(Result::ok)
                .flatten()
                .max_by(|(_, a_data), (_, b_data)| a_data.amt.cmp(&b_data.amt));

            if let Some((chosen_sale_tx, chosen_sale_data)) = chosen_sale {
                channel.send_with_tag(
                    SimUpdate::Evm(chosen_sale_tx),
                    format!("{}", chosen_sale_data).into(),
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
                match result {
                    SimUpdateResult::Evm(result) if tag.parse::<ActiveBuyerTxData>().is_ok() => {
                        let tx_data: ActiveBuyerTxData = tag.parse().unwrap();
                        if tx_data.tx_type == ACTIVE_BUYER_PURCHASE && is_execution_success(result)
                        {
                            let ptokens_received =
                                self.cozyrouter.decode_ptokens_received(result).unwrap();
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
                                    *curr_ptokens += Into::<EthersU256>::into(ptokens_received);
                                }
                            };
                        } else if tx_data.tx_type == ACTIVE_BUYER_SALE
                            && is_execution_success(result)
                        {
                            match self
                                .ptokens_owned
                                .get_mut(&(tx_data.set_address, tx_data.market_id))
                            {
                                None => {}
                                Some(curr_ptokens) => {
                                    *curr_ptokens -= tx_data.amt;
                                }
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        self.protection_owned = EthersU256::from(0);
        for ((set_addr, set_market_id), ptokens) in self.ptokens_owned.iter() {
            self.protection_owned += self
                .set_logic
                .read_protection_balance(self.address, state, *set_addr, *set_market_id, *ptokens)
                .unwrap();
        }
    }
}

impl ActiveBuyer {
    fn is_time_to_act(&self, curr_timestamp: EvmU256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
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
    ) -> Result<Option<(TxEnv, ActiveBuyerTxData)>> {
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
            max_cost = min((purchase_amt * float_to_wad(my_prob)) / wad(), self.capital);
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
            match unpack_execution(state.simulate_evm_tx_ref(purchase_tx.as_ref().unwrap(), None)) {
                Ok(_) => {
                    return Ok(Some((
                        purchase_tx.unwrap(),
                        ActiveBuyerTxData {
                            tx_type: ACTIVE_BUYER_PURCHASE.into(),
                            amt: purchase_amt,
                            set_address,
                            market_id,
                        },
                    )));
                }
                _ => {
                    purchase_amt /= 2;
                    continue;
                }
            };
        }
    }

    fn get_arb_sell_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
        my_prob: f64,
    ) -> Result<Option<(TxEnv, ActiveBuyerTxData)>> {
        let min_refund = (self.capital * float_to_wad(my_prob)) / wad();

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
            let sell_tx = Some(
                self.cozyrouter
                    .build_sell_tx(self.address, sell_args.clone())?,
            );
            match unpack_execution(state.simulate_evm_tx_ref(sell_tx.as_ref().unwrap(), None)) {
                Ok(_) => {
                    return Ok(Some((
                        sell_tx.unwrap(),
                        ActiveBuyerTxData {
                            tx_type: ACTIVE_BUYER_SALE.into(),
                            amt: sell_amt,
                            set_address,
                            market_id,
                        },
                    )));
                }
                _ => {
                    sell_amt /= 2;
                    continue;
                }
            };
        }
    }
}
