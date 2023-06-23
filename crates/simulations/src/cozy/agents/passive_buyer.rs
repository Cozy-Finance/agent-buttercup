use std::{
    borrow::Cow,
    cmp::min,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::{ExecutionResult, TxEnv};
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::sim_contract::SimContract,
    state::{
        update::{SimUpdate, SimUpdateResult},
        SimState,
    },
    utils::{build_call_contract_txenv, unpack_execution}, address::Address,
};

use crate::cozy::{
    agents::errors::CozyAgentError,
    constants::PASSIVE_BUYER_PURCHASE,
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
<<<<<<< HEAD
    EthersAddress, EthersU256
=======
    EthersAddress, EthersU256, EvmAddress, EvmU256,
>>>>>>> d70411b (Fix passive buyer)
};

pub struct PassiveBuyer {
    name: Option<Cow<'static, str>>,
    address: Address,
    cozyrouter: Arc<CozyProtocolContract>,
    token: Arc<CozyProtocolContract>,
    set_logic: Arc<CozyProtocolContract>,
<<<<<<< HEAD
    target_triggers: Vec<Address>,
    protection_desired: Vec<EthersU256>,
=======
    target_trigger: EvmAddress,
    target_set: Option<(EvmAddress, u16)>,
    protection_owned: EthersU256,
    ptokens_owned: HashMap<(EvmAddress, u16), EvmU256>,
>>>>>>> d70411b (Fix passive buyer)
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
<<<<<<< HEAD
        target_triggers: Vec<Address>,
        protection_desired: Vec<EthersU256>,
=======
        target_trigger: EvmAddress,
        waiting_time: f64,
>>>>>>> d70411b (Fix passive buyer)
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            target_trigger,
            target_set: None,
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

    fn resolve_activation_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {}

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.is_time_to_act(state.read_timestamp()) || self.capital <= EthersU256::from(0) {
            return;
        }

        let sets = state.world.sets.values().collect::<Vec<_>>();
        let targets =
            self.get_target_sets_and_markets_ids(state, sets.clone(), &self.target_trigger);
        if targets.len() == 0 {
            return;
        }

        let opt_target = targets
            .iter()
            .map(|(set_address, market_id)| {
                let purchase_args = cozy_router::PurchaseCall {
                    set: (*set_address).into(),
                    market_id: market_id.clone(),
                    protection: self.capital,
                    receiver: self.address.into(),
                    max_cost: EthersU256::MAX,
                };
                self.get_purchase_cost(state, purchase_args).unwrap()
            })
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index);

        let (opt_set_address, opt_market_id) = targets[opt_target.unwrap()];
        let opt_purchase_args = cozy_router::PurchaseCall {
            set: opt_set_address.into(),
            market_id: opt_market_id,
            protection: self.capital,
            receiver: self.address.into(),
            max_cost: EthersU256::MAX,
        };
        let evm_tx = self.build_purchase_tx(opt_purchase_args).unwrap();
        channel.send_with_tag(SimUpdate::Evm(evm_tx), PASSIVE_BUYER_PURCHASE.into());
        self.target_set = Some((opt_set_address, opt_market_id));
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        if !self.is_time_to_act(state.read_timestamp()) {
            return;
        }
        self.capital = self.get_token_balance(state).unwrap();
        self.last_action_time = state.read_timestamp();

        if let Some(update_results) = state.update_results.get(&self.address) {
            for (tag, result) in update_results {
                match result {
                    SimUpdateResult::Evm(purchase_result) if tag == PASSIVE_BUYER_PURCHASE => {
                        let ptokens = self.get_ptokens_received(purchase_result);
                        if let Ok(ptokens) = ptokens {
                            self.ptokens_owned
                                .insert(self.target_set.unwrap(), ptokens.into());
                            self.protection_owned = self
                                .get_protection_balance(
                                    state,
                                    self.target_set.unwrap().0,
                                    self.target_set.unwrap().1,
                                    ptokens.into(),
                                )
                                .unwrap();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

impl PassiveBuyer {
    fn is_time_to_act(&self, curr_timestamp: EvmU256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
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
        set_addr: EvmAddress,
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
        sets: Vec<&Arc<RwLock<CozySet>>>,
<<<<<<< HEAD
        trigger: &Address,
        protection_delta: EthersU256,
    ) -> Vec<(Address, u16)> {
=======
        trigger: &EvmAddress,
    ) -> Vec<(EvmAddress, u16)> {
>>>>>>> d70411b (Fix passive buyer)
        sets.iter()
            .filter(|set| set.read().unwrap().trigger_lookup.contains_key(trigger))
            .map(|set| {
                (
                    set.read().unwrap().address,
                    *set.read().unwrap().trigger_lookup.get(trigger).unwrap(),
                )
            })
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

    fn build_remaining_protection_tx(
        &self,
        set_address: Address,
        market_id: u16,
    ) -> Result<TxEnv> {
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
            self.token.as_ref().contract.encode_function(
                "approve",
                (
                    cozyrouter_address,
                    EthersU256::MAX,
                ),
            )?,
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
