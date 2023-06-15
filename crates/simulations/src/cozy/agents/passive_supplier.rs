use std::{borrow::Cow, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv, unpack_execution},
};

use crate::cozy::{
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmAddress,
};

pub struct PassiveSupplier {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    cozyrouter: Arc<CozyProtocolContract>,
    token: Arc<CozyProtocolContract>,
    capital: EthersU256,
}

impl PassiveSupplier {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        cozyrouter: &Arc<CozyProtocolContract>,
        token: &Arc<CozyProtocolContract>,
        capital: EthersU256,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            capital,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for PassiveSupplier {
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

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if self.capital > EthersU256::from(0) {
            let mut sets = state.world.sets.values().collect::<Vec<_>>();
            if sets.len() > 0 {
                sets.sort_by(|a, b| a.apy.cmp(&b.apy));
                let deposit_tx = self
                    .build_deposit_tx(cozy_router::DepositCall {
                        set: sets[0].address.into(),
                        assets: self.capital,
                        receiver: self.address.into(),
                        min_shares_received: EthersU256::from(0),
                    })
                    .unwrap();
                channel.send(SimUpdate::Evm(deposit_tx));
            }
        }
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.capital = self.get_token_balance(state).unwrap();
        println!("{:?}", self.capital);
    }
}

impl PassiveSupplier {
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

    fn build_transfer_token_to_router_tx(&self, amount: EthersU256) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.token.as_ref().address,
            self.token.as_ref().contract.encode_function(
                "transfer",
                (
                    EthersAddress::from(self.cozyrouter.as_ref().address),
                    amount,
                ),
            )?,
            None,
            None,
        ))
    }

    fn build_deposit_tx(&self, args: cozy_router::DepositCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("deposit", args)?,
            None,
            None,
        ))
    }

    fn build_deposit_without_transfer_tx(
        &self,
        args: cozy_router::DepositWithoutTransferCall,
    ) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("depositWithoutTransfer", args)?,
            None,
            None,
        ))
    }

    fn build_withdraw_tx(&self, args: cozy_router::WithdrawCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("withdraw", args)?,
            None,
            None,
        ))
    }

    fn build_redeem_tx(&self, args: cozy_router::RedeemCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("redeem", args)?,
            None,
            None,
        ))
    }

    fn build_complete_withdraw_tx(&self, args: cozy_router::CompleteWithdrawCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("completeRedeem", args)?,
            None,
            None,
        ))
    }

    fn build_complete_redeem_tx(&self, args: cozy_router::CompleteRedeemCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address,
            self.cozyrouter.as_ref().address,
            self.cozyrouter
                .as_ref()
                .contract
                .encode_function("completeRedeem", args)?,
            None,
            None,
        ))
    }
}
