use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory, drip_decay_model_constant,
    shared_types::{MarketConfig, SetConfig},
    manager::CreateSetCall
};

use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::Agent,
    contract::{
        sim_contract::{IsDeployed, SimContract},
        utils,
    },
    environment::sim_env::SimEnv,
};
use thiserror::Error;

use crate::simulator::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress}, sim_types::CozySimTrigger,
};
use crate::simulator::cozy::sim_types::{
    CozySimCostModel, 
    CozySimDripDecayModel, 
    MarketParamsConfig,
};
use crate::simulator::errors::CozyAgentErrors;

use std::error::Error;

use bindings::cozy_router;
use crossbeam_channel::Receiver;
use ethers::{
    prelude::U256,
    types::{Sign, I256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, Address, B160};
use simulate::{
    agent::{
        cozy_passive_buyer::CozyPassiveBuyer, simple_arbitrageur::SimpleArbitrageur, Agent,
        AgentType, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::unpack_execution,
};


pub struct PassiveBuyer {
    name: String,
    deploy_params: ProtocolDeployerParams,
    address: Option<EvmAddress>,
}

impl PassiveBuyer {
    pub fn new(name: String, deploy_params: ProtocolDeployerParams) -> Self {
        Self {
            name,
            deploy_params,
            address: None,
        }
    }
}

impl Agent for PassiveBuyer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn name(&self) -> Option<String> {
        Option::Some(self.name.clone())
    }

    fn activation_step(&mut self, sim_env: &mut SimEnv) {}

    fn step(&mut self, sim_env: &mut SimEnv) {}
}

impl PassiveBuyer {
    fn purchase_protection(&self, args: PurchaseCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("purchase", args)?,
        );
    }

    fn purchase_protection_without_transfer(&self, args: PurchaseWithoutTransferCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("purchaseWithoutTransfer", args)?,
        );
    }

    fn cancel_protection(&self, args: CancelCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("cancel", args)?,
        );
    }

    fn sell_protection(&self, args: SellCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("sell", args)?,
        );
    }

    fn claim_ptokens(&self, args: ClaimCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("claim", args)?,
        );
    }

    fn payout_protection(&self, args: PayoutCall) {
        let router = sim_env
            .data
            .contract_registry
            .get(COZYROUTER.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        self.call_contract(
            sim_env,
            router,
            router.encode_function("payout", args)?,
        );
    }
}