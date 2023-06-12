use std::sync::Arc;

use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::{agent_channel::AgentChannel, Agent},
    contract::sim_contract::SimContract,
    state::{update::SimUpdate, SimState},
    utils::build_call_contract_txenv,
};

use crate::cozy::{
    agents::errors::CozyAgentError,
    world::{CozyUpdate, CozyWorld},
    EthersAddress, EvmAddress,
};

pub struct PassiveBuyer {
    address: Option<EvmAddress>,
    cozyrouter_address: Option<EvmAddress>,
    cozyrouter_contract: Option<Arc<SimContract>>,
}

impl PassiveBuyer {
    pub fn new() -> Self {
        Self {
            address: None,
            cozyrouter_address: None,
            cozyrouter_contract: None,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for PassiveBuyer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
        let (cozyrouter_addr, cozyrouter_contract) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)
            .unwrap()
            .contract_registry
            .get("CozyRouter")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();
        self.cozyrouter_address = Some(*cozyrouter_addr);
        self.cozyrouter_contract = Some(cozyrouter_contract.clone());
    }

    fn resolve_activation_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl PassiveBuyer {
    fn build_purchase_tx(&self, args: cozy_router::PurchaseCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("purchase", args)?,
            None,
            None,
        ))
    }

    fn purchase_protection_without_transfer(
        &self,
        args: cozy_router::PurchaseWithoutTransferCall,
    ) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("purchaseWithoutTransfer", args)?,
            None,
            None,
        ))
    }

    fn cancel_protection(&self, args: cozy_router::CancelCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("cancel", args)?,
            None,
            None,
        ))
    }

    fn sell_protection(&self, args: cozy_router::SellCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("sell", args)?,
            None,
            None,
        ))
    }

    fn claim_ptokens(&self, args: cozy_router::ClaimCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("claim", args)?,
            None,
            None,
        ))
    }

    fn payout_protection(&self, args: cozy_router::PayoutCall) -> Result<TxEnv> {
        Ok(build_call_contract_txenv(
            self.address(),
            self.cozyrouter_address
                .ok_or(CozyAgentError::UnregisteredAddress)?,
            self.cozyrouter_contract
                .as_ref()
                .ok_or(CozyAgentError::UnregisteredAddress)?
                .encode_function("payout", args)?,
            None,
            None,
        ))
    }
}
