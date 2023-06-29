use std::{
    borrow::Cow,
    cmp::min,
    collections::HashMap,
    f32::consts::E,
    sync::{Arc, RwLock},
};

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
    utils::{build_call_contract_txenv, unpack_execution},
};

use crate::cozy::{
    constants::*,
    distributions::ProbTruncatedNorm,
    utils::{float_to_wad, wad_to_float},
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmU256,
};

pub struct ActiveBuyer {
    name: Option<Cow<'static, str>>,
    address: Address,
    cozyrouter: Arc<CozyProtocolContract>,
    token: Arc<CozyProtocolContract>,
    set_logic: Arc<CozyProtocolContract>,
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
        name: Option<Cow<'static, str>>,
        address: Address,
        cozyrouter: &Arc<CozyProtocolContract>,
        token: &Arc<CozyProtocolContract>,
        set_logic: &Arc<CozyProtocolContract>,
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
        channel.send(SimUpdate::Evm(self.build_max_approve_router_tx().unwrap()));
        self.capital = self.get_token_balance(state).unwrap();
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
        self.capital = self.get_token_balance(state).unwrap();
        self.last_action_time = state.read_timestamp();

        if let Some(update_results) = state.update_results.get(&self.address) {
            for (tag, result) in update_results {
                let target = self.parse_purchase_tag(tag);
                match result {
                    SimUpdateResult::Evm(purchase_result) => {
                        let ptokens = self.get_ptokens_received(purchase_result);
                        if let Ok(ptokens) = ptokens {
                            match self.ptokens_owned.get_mut(&target) {
                                None => {
                                    self.ptokens_owned.insert(target, ptokens.into());
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
                .get_protection_balance(state, *set_addr, *set_market_id, (*ptokens).into())
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

    fn get_ptokens_received(&self, execution_result: &ExecutionResult) -> Result<EthersU256> {
        let purchase_output = self
            .cozyrouter
            .contract
            .decode_output::<cozy_router::PayoutReturn>(
                "purchase",
                unpack_execution(execution_result.clone())?,
            )?;
        Ok(purchase_output.ptokens)
    }

    fn get_protection_balance(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_addr: Address,
        market_id: u16,
        ptokens: EthersU256,
    ) -> Result<EthersU256> {
        let balance_tx = build_call_contract_txenv(
            self.address,
            set_addr,
            self.set_logic
                .as_ref()
                .contract
                .encode_function("convertToProtection", (market_id, ptokens))?,
            None,
            None,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None))?;
        let balance: EthersU256 = self
            .set_logic
            .contract
            .decode_output("convertToProtection", result)?;
        Ok(balance)
    }

    fn get_token_balance(&self, state: &SimState<CozyUpdate, CozyWorld>) -> Result<EthersU256> {
        let ethers_address: EthersAddress = self.address.into();
        let balance_tx = build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("balanceOf", ethers_address)?,
            None,
            None,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None))?;
        let balance: EthersU256 = self.token.contract.decode_output("balanceOf", result)?;
        Ok(balance)
    }

    fn get_target_sets_and_markets_ids(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
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
        let mut purchase_amt = self.get_remaining_protection(state, set_address, market_id)?;
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
            let purchase_tx = Some(self.build_purchase_tx(purchase_args)?);
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
            let sell_tx = Some(self.build_sell_tx(sell_args)?);
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

    fn get_purchase_cost(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cozy_router::PurchaseCall,
    ) -> Result<EthersU256> {
        let purchase_tx = self.build_purchase_tx(args)?;
        let result = match unpack_execution(state.simulate_evm_tx_ref(&purchase_tx, None)) {
            Ok(bytes) => bytes,
            _ => return Ok(EthersU256::MAX),
        };
        let purchase_return = self
            .cozyrouter
            .contract
            .decode_output::<cozy_router::PurchaseReturn>("purchase", result)?;
        Ok(purchase_return.assets_needed)
    }

    fn get_remaining_protection(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
    ) -> Result<EthersU256> {
        let remaining_protection_tx = self.build_remaining_protection_tx(set_address, market_id)?;
        let result = unpack_execution(state.simulate_evm_tx_ref(&remaining_protection_tx, None))?;
        let remaining_protection_return: EthersU256 = self
            .set_logic
            .contract
            .decode_output("remainingProtection", result)?;
        Ok(remaining_protection_return)
    }

    fn build_remaining_protection_tx(&self, set_address: Address, market_id: u16) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            set_address.into(),
            self.set_logic
                .as_ref()
                .contract
                .encode_function("remainingProtection", market_id)?,
            None,
            None,
        ))
    }

    fn build_max_approve_router_tx(&self) -> Result<TxEnv> {
        let cozyrouter_address: EthersAddress = self.cozyrouter.as_ref().address.into();
        Ok(build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("approve", (cozyrouter_address, EthersU256::MAX))?,
            None,
            None,
        ))
    }

    fn build_purchase_tx(&self, args: cozy_router::PurchaseCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("purchase", args)?,
            None,
            None,
        ))
    }

    fn build_purchase_without_transfer_tx(
        &self,
        args: cozy_router::PurchaseWithoutTransferCall,
    ) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("purchaseWithoutTransfer", args)?,
            None,
            None,
        ))
    }

    fn build_cancel_tx(&self, args: cozy_router::CancelCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("cancel", args)?,
            None,
            None,
        ))
    }

    fn build_sell_tx(&self, args: cozy_router::SellCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("sell", args)?,
            None,
            None,
        ))
    }

    fn build_claim_tx(&self, args: cozy_router::ClaimCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("claim", args)?,
            None,
            None,
        ))
    }

    fn build_payout_tx(&self, args: cozy_router::PayoutCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("payout", args)?,
            None,
            None,
        ))
    }
}
