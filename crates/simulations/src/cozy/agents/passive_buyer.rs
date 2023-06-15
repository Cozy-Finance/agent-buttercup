use std::{borrow::Cow, cmp::min, collections::HashMap, sync::Arc};

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
    utils::{build_call_contract_txenv, unpack_execution},
};

use crate::cozy::{
    agents::errors::CozyAgentError,
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmAddress,
};

pub struct PassiveBuyer {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    cozyrouter: Arc<CozyProtocolContract>,
    token: Arc<CozyProtocolContract>,
    set_logic: Arc<CozyProtocolContract>,
    target_triggers: Vec<EvmAddress>,
    protection_desired: Vec<EthersU256>,
    capital: Option<EthersU256>,
    ptokens_owned: Option<Vec<EthersU256>>,
    protection_owned: Option<Vec<EthersU256>>,
}

impl PassiveBuyer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        cozyrouter: &Arc<CozyProtocolContract>,
        token: &Arc<CozyProtocolContract>,
        set_logic: &Arc<CozyProtocolContract>,
        target_triggers: Vec<EvmAddress>,
        protection_desired: Vec<EthersU256>,
        capital: EthersU256,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            set_logic: set_logic.clone(),
            target_triggers,
            protection_desired,
            capital: None,
            ptokens_owned: None,
            protection_owned: None,
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
    }

    fn resolve_activation_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.capital = Some(self.get_token_balance(state).unwrap());
        self.ptokens_owned = Some(vec![EthersU256::from(0); self.target_triggers.len()]);
        self.protection_owned = Some(vec![EthersU256::from(0); self.target_triggers.len()]);
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if self.capital.unwrap() == EthersU256::from(0) {
            return;
        }

        let sets = state.world.sets.values().collect::<Vec<_>>();
        if sets.len() == 0 {
            return;
        }

        for (i, trigger) in self.target_triggers.iter().enumerate() {
            if self.protection_desired[i] <= self.protection_owned.as_ref().unwrap()[i] {
                continue;
            }
            let protection_delta =
                self.protection_desired[i] - self.protection_owned.as_ref().unwrap()[i];
            let targets = self.get_target_sets_and_markets_ids(
                state,
                sets.clone(),
                trigger,
                protection_delta,
            );
            if targets.len() == 0 {
                continue;
            }
            let opt_target = targets
                .iter()
                .map(|(set_address, market_id)| {
                    let purchase_args = cozy_router::PurchaseCall {
                        set: (*set_address).into(),
                        market_id: market_id.clone(),
                        protection: protection_delta,
                        receiver: self.address.into(),
                        max_cost: EthersU256::MAX,
                    };
                    self.get_purchase_cost(state, purchase_args).unwrap()
                })
                .collect::<Vec<_>>()
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(index, _)| index);

            let (opt_set_address, opt_market_id) = targets[opt_target.unwrap()];
            let opt_purchase_args = cozy_router::PurchaseCall {
                set: opt_set_address.into(),
                market_id: opt_market_id,
                protection: self.protection_desired[0],
                receiver: self.address.into(),
                max_cost: EthersU256::MAX,
            };

            let evm_tx = self.build_purchase_tx(opt_purchase_args).unwrap();
            channel.send_with_tag(SimUpdate::Evm(evm_tx), format!("{}", i).into());
        }
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.capital = Some(self.get_token_balance(state).unwrap());
        let purchase_results = match state.update_results.get(&self.address) {
            Some(pr) => pr,
            None => {
                return;
            }
        };

        for (id, result) in purchase_results {
            let trigger_index: usize = id.parse().unwrap();
            if let SimUpdateResult::Evm(execution_result) = result {
                let parsed_execution_result = self.get_ptokens_received(execution_result);
                if let Ok(ptokens_received) = parsed_execution_result {
                    self.ptokens_owned.as_mut().unwrap()[trigger_index] += ptokens_received;
                }
            }
        }
    }
}

impl PassiveBuyer {
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

    fn get_token_balance(&self, state: &SimState<CozyUpdate, CozyWorld>) -> Result<EthersU256> {
        let balance_tx = build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("balanceOf", EthersAddress::from(self.address))?,
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
        sets: Vec<&CozySet>,
        trigger: &EvmAddress,
        protection_delta: EthersU256,
    ) -> Vec<(EvmAddress, u16)> {
        sets.iter()
            .filter(|set| set.trigger_lookup.contains_key(trigger))
            .map(|set| (set.address, *set.trigger_lookup.get(trigger).unwrap()))
            .filter(|(set_addr, market_id)| {
                self.get_remaining_protection(state, *set_addr, *market_id)
                    .unwrap()
                    >= protection_delta
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
        set_address: EvmAddress,
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
        set_address: EvmAddress,
        market_id: u16,
    ) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            set_address.into(),
            self.set_logic
                .as_ref()
                .contract
                .encode_function("remainingProtection", (market_id))?,
            None,
            None,
        ))
    }

    fn build_max_approve_router_tx(&self) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token.as_ref().contract.encode_function(
                "approve",
                (
                    EthersAddress::from(self.cozyrouter.as_ref().address),
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
