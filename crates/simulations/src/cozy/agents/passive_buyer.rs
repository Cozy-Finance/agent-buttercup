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
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmU256,
};

pub struct PassiveBuyer {
    name: Option<Cow<'static, str>>,
    address: Address,
    cozyrouter: Arc<CozyProtocolContract>,
    token: Arc<CozyProtocolContract>,
    set_logic: Arc<CozyProtocolContract>,
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
        name: Option<Cow<'static, str>>,
        address: Address,
        cozyrouter: &Arc<CozyProtocolContract>,
        token: &Arc<CozyProtocolContract>,
        set_logic: &Arc<CozyProtocolContract>,
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
        channel.send(SimUpdate::Evm(self.build_max_approve_router_tx().unwrap()));
        self.capital = self.get_token_balance(state).unwrap();
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
                self.get_purchase_cost(state, purchase_args).unwrap()
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
            self.get_remaining_protection(state, set_addr, set_market_id)
                .unwrap(),
        );
        let purchase_args = cozy_router::PurchaseCall {
            set: set_addr.into(),
            market_id: set_market_id,
            protection: protection_purchase_amt,
            receiver: self.address.into(),
            max_cost: EthersU256::MAX,
        };
        let evm_tx = self.build_purchase_tx(purchase_args).unwrap();
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
                .get_protection_balance(state, *set_addr, *set_market_id, (*ptokens).into())
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
        let balance_tx = build_call_tx(
            self.address,
            set_addr,
            self.set_logic
                .as_ref()
                .contract
                .encode_function("convertToProtection", (market_id, ptokens))?,
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
        let balance_tx = build_call_tx(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("balanceOf", ethers_address)?,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None))?;
        let balance: EthersU256 = self.token.contract.decode_output("balanceOf", result)?;
        Ok(balance)
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
        Ok(build_call_tx(
            self.address,
            set_address,
            self.set_logic
                .as_ref()
                .contract
                .encode_function("remainingProtection", market_id)?,
        ))
    }

    fn build_max_approve_router_tx(&self) -> Result<TxEnv> {
        let cozyrouter_address: EthersAddress = self.cozyrouter.as_ref().address.into();
        Ok(build_call_tx(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("approve", (cozyrouter_address, EthersU256::MAX))?,
        ))
    }

    fn build_purchase_tx(&self, args: cozy_router::PurchaseCall) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("purchase", args)?,
        ))
    }

    fn build_purchase_without_transfer_tx(
        &self,
        args: cozy_router::PurchaseWithoutTransferCall,
    ) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("purchaseWithoutTransfer", args)?,
        ))
    }

    fn build_cancel_tx(&self, args: cozy_router::CancelCall) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("cancel", args)?,
        ))
    }

    fn build_sell_tx(&self, args: cozy_router::SellCall) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("sell", args)?,
        ))
    }

    fn build_claim_tx(&self, args: cozy_router::ClaimCall) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("claim", args)?,
        ))
    }

    fn build_payout_tx(&self, args: cozy_router::PayoutCall) -> Result<TxEnv> {
        Ok(build_call_tx(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("payout", args)?,
        ))
    }
}
