use std::{borrow::Cow, cmp::min, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::sim_contract::SimContract,
    state::{update::SimUpdate, SimState},
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
    target_triggers: Vec<EvmAddress>,
    protection_desired: Vec<EthersU256>,
    capital: EthersU256,
}

impl PassiveBuyer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        cozyrouter: &Arc<CozyProtocolContract>,
        token: &Arc<CozyProtocolContract>,
        target_triggers: Vec<EvmAddress>,
        protection_desired: Vec<EthersU256>,
        capital: EthersU256,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            target_triggers,
            protection_desired,
            capital,
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
        println!("Agent {:?} {:?}", self.name, self.capital);
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        self.capital = self.get_token_balance(state).unwrap();
        println!("Agent {:?} {:?}", self.name, self.capital);
        if self.capital == EthersU256::from(0) {
            return;
        }

        let sets = state.world.sets.values().collect::<Vec<_>>();
        if sets.len() == 0 {
            return;
        }

        let targets = self.get_target_sets_and_markets(sets);
        let opt_target = targets
            .iter()
            .map(|(set_address, market_id)| {
                let purchase_args = cozy_router::PurchaseCall {
                    set: (*set_address).into(),
                    market_id: market_id.clone(),
                    protection: self.protection_desired[0],
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
        channel.send(SimUpdate::Evm(evm_tx));
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.capital = self.get_token_balance(state).unwrap();
        println!("Agent {:?} {:?}", self.name, self.capital);
    }
}

impl PassiveBuyer {
    fn get_token_balance(&self, state: &SimState<CozyUpdate, CozyWorld>) -> Result<EthersU256> {
        let balance_tx = build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token
                .as_ref()
                .contract
                .encode_function("balanceOf", (EthersAddress::from(self.address)))?,
            None,
            None,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None))?;
        let balance: EthersU256 = self.token.contract.decode_output("balanceOf", result)?;
        Ok(balance)
    }

    fn get_target_sets_and_markets(&self, sets: Vec<&CozySet>) -> Vec<(EvmAddress, u16)> {
        sets.iter()
            .filter(|set| set.trigger_lookup.contains_key(&self.target_triggers[0]))
            .map(|set| {
                (
                    set.address,
                    *set.trigger_lookup.get(&self.target_triggers[0]).unwrap(),
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
            .token
            .contract
            .decode_output::<cozy_router::PurchaseReturn>("purchase", result)?;
        Ok(purchase_return.assets_needed)
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
