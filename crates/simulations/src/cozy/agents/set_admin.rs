pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory, manager,
};
use bindings::{
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    manager::{CreateSetCall, CreateSetReturn},
};
use ethers::abi::AbiDecode;
use eyre::Result;
use revm::primitives::{create_address, TxEnv};
use simulate::{
    agent::{
        agent_channel::{AgentChannel, AgentSimUpdate},
        Agent,
    },
    contract::{sim_contract::SimContract, utils},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv, build_deploy_contract_txenv, unpack_execution},
};
use thiserror::Error;
use std::sync::Arc;

use crate::cozy::{
    agents::errors::CozyAgentError,
    bindings_wrapper::*,
    types::{CozyMarketParamsConfig, CozySimCostModel, CozySimDripDecayModel, CozySimTrigger},
    utils::build_deploy_contract_tx,
    world::{CozyUpdate, CozyWorld},
    EthersAddress, EthersBytes, EvmAddress,
};

#[derive(Debug, Clone)]
pub struct SetAdminParams {
    pub asset: EthersAddress,
    pub set_config: SetConfig,
    pub triggers: Vec<CozySimTrigger>,
    pub cost_models: Vec<CozySimCostModel>,
    pub drip_decay_models: Vec<CozySimDripDecayModel>,
    pub market_params_configs: Vec<CozyMarketParamsConfig>,
    pub salt: Option<[u8; 32]>,
}

pub struct SetAdmin {
    set_admin_params: SetAdminParams,
    address: Option<EvmAddress>,
}

impl SetAdmin {
    pub fn new(set_admin_params: SetAdminParams) -> Self {
        Self {
            set_admin_params,
            address: None,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for SetAdmin {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        let mut nonce = 0 as u64;

        let (jump_rate_addr, jump_rate_contract) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)
            .unwrap()
            .contract_registry
            .get("CostModelJumpRateFactory")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();

        let (dynamic_level_addr, dynamic_level_contract) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)
            .unwrap()
            .contract_registry
            .get("CostModelDynamicLevelFactory")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();

        let (drip_decay_addr, drip_decay_contract) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)
            .unwrap()
            .contract_registry
            .get("DripDecayFactory")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();

        // Deploy all triggers.
        let triggers = self
            .set_admin_params
            .triggers
            .clone()
            .into_iter()
            .map(|trigger| match trigger {
                CozySimTrigger::DummyTrigger => self.build_deploy_dummy_trigger_tx(state, &mut nonce),
            })
            .collect::<Result<Vec<_>>>()
            .expect("Error building trigger deploy txs");

        // Deploy all cost models.
        let cost_models = self
            .set_admin_params
            .cost_models
            .clone()
            .into_iter()
            .map(|model| match model {
                CozySimCostModel::JumpRate(args) => {
                    self.build_deploy_cost_model_jump_rate_tx(jump_rate_addr, jump_rate_contract, args, &mut nonce)
                }
                CozySimCostModel::DynamicLevel(args) => {
                    self.build_deploy_cost_model_dynamic_level_tx(dynamic_level_addr, dynamic_level_contract, args, &mut nonce)
                }
            })
            .collect::<Result<Vec<_>>>()
            .expect("Error building cost model deploy txs");

        // Deploy all drip decay models.
        let drip_decay_models = self
            .set_admin_params
            .drip_decay_models
            .clone()
            .into_iter()
            .map(|model| match model {
                CozySimDripDecayModel::Constant(args) => {
                    self.build_deploy_drip_decay_model_tx(drip_decay_addr, drip_decay_contract, args, &mut nonce)
                }
            })
            .collect::<Result<Vec<_>>>()
            .expect("Error building drip decay model deploy txs");

        // Build market configs.
        let market_configs: Vec<MarketConfig> = self
            .set_admin_params
            .market_params_configs
            .clone()
            .into_iter()
            .enumerate()
            .map(|(i, params)| MarketConfig {
                trigger: EthersAddress::from(triggers[i].0),
                cost_model: EthersAddress::from(cost_models[i].0),
                drip_decay_model: EthersAddress::from(drip_decay_models[i].0),
                weight: params.weight,
                purchase_fee: params.purchase_fee,
                sale_fee: params.sale_fee,
            })
            .collect();

        let create_set_args = manager::CreateSetCall {
            owner: self.address().into(),
            pauser: self.address().into(),
            asset: self.set_admin_params.asset,
            set_config: self.set_admin_params.set_config.clone(),
            market_configs,
            salt: self
                .set_admin_params
                .salt
                .unwrap_or(rand::random::<[u8; 32]>()),
        };

        let (_, create_set_tx) = self
            .build_create_set_tx(state, create_set_args, &mut nonce)
            .expect("Error building create set tx.");

        for (_, tx) in triggers {
            channel.send(SimUpdate::Evm(tx));
        }
        for (_, tx) in cost_models {
            channel.send(SimUpdate::Evm(tx));
        }
        for (_, tx) in drip_decay_models {
            channel.send(SimUpdate::Evm(tx));
        }
        channel.send(SimUpdate::Evm(create_set_tx));
    }

    fn resolve_activation_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {
        println!("{:?}", _state.get_read_db().accounts);
    }

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl SetAdmin {
    fn build_deploy_cost_model_jump_rate_tx(
        &mut self,
        factory_addr: &EvmAddress,
        factory_contract: &Arc<SimContract>,
        args: cost_model_jump_rate_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory_contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address(),
            (*factory_addr).into(),
            call_data,
            None,
            None,
        );
        let addr = create_address(self.address(), *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }

    fn build_deploy_cost_model_dynamic_level_tx(
        &mut self,
        factory_addr: &EvmAddress,
        factory_contract: &Arc<SimContract>,
        args: cost_model_dynamic_level_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory_contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address(),
            (*factory_addr).into(),
            call_data,
            None,
            None,
        );
        let addr = create_address(self.address(), *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }

    fn build_deploy_drip_decay_model_tx(
        &mut self,
        factory_addr: &EvmAddress,
        factory_contract: &Arc<SimContract>,
        args: drip_decay_model_constant_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory_contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address(),
            (*factory_addr).into(),
            call_data,
            None,
            None,
        );
        let addr = create_address(self.address(), *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }

    fn build_deploy_dummy_trigger_tx(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let (manager_addr, _) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)?
            .contract_registry
            .get("Manager")
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        let args = (EthersAddress::from(*manager_addr),);
        let (tx, _) = build_deploy_contract_tx(self.address(), &DUMMYTRIGGER, args)?;
        let addr = create_address(self.address(), *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }

    fn build_create_set_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: manager::CreateSetCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let (manager_addr, manager_contract) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)?
            .contract_registry
            .get("Manager")
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        let call_data = manager_contract.encode_function("createSet", args)?;
        let tx = build_call_contract_txenv(
            self.address(),
            (*manager_addr).into(),
            call_data,
            None,
            None,
        );
        let addr = create_address(self.address(), *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }
}
