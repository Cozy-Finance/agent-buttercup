use std::{borrow::Cow, sync::Arc};

use bindings::cozy_protocol::shared_types::{MarketConfig, SetConfig};
pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory, manager,
};
use eyre::Result;
use revm::primitives::{create_address, TxEnv};
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
    bindings_wrapper::*,
    types::{CozyMarketParams, CozySimCostModel, CozySimDripDecayModel, CozySimTrigger},
    utils::build_deploy_contract_tx,
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
    EthersAddress, EvmAddress, EvmBytes,
};

#[derive(Debug, Clone)]
pub struct SetAdminParams {
    pub asset: EthersAddress,
    pub set_config: SetConfig,
    pub triggers: Vec<CozySimTrigger>,
    pub cost_models: Vec<CozySimCostModel>,
    pub drip_decay_models: Vec<CozySimDripDecayModel>,
    pub market_params_configs: Vec<CozyMarketParams>,
    pub salt: Option<[u8; 32]>,
}

pub struct SetAdmin {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    set_admin_params: SetAdminParams,
    manager: Option<Arc<CozyProtocolContract>>,
    set_address: Option<EvmAddress>,
    set_registered: bool,
    set_logic: Option<Arc<CozyProtocolContract>>,
}

impl SetAdmin {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        set_admin_params: SetAdminParams,
    ) -> Self {
        Self {
            name,
            address,
            set_admin_params,
            manager: None,
            set_address: None,
            set_registered: false,
            set_logic: None,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for SetAdmin {
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
        let mut nonce = 0 as u64;

        self.manager = Some(
            state
                .world
                .protocol_contracts
                .get("Manager")
                .ok_or(CozyAgentError::UnregisteredAddress)
                .unwrap()
                .clone(),
        );

        self.set_logic = Some(
            state
                .world
                .protocol_contracts
                .get("Set logic")
                .ok_or(CozyAgentError::UnregisteredAddress)
                .unwrap()
                .clone(),
        );

        let jump_rate = state
            .world
            .protocol_contracts
            .get("CostModelJumpRateFactory")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();

        let dynamic_level = state
            .world
            .protocol_contracts
            .get("CostModelDynamicLevelFactory")
            .ok_or(CozyAgentError::UnregisteredAddress)
            .unwrap();

        let drip_decay = state
            .world
            .protocol_contracts
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
                CozySimTrigger::DummyTrigger => {
                    self.build_deploy_dummy_trigger_tx(state, &mut nonce)
                }
            })
            .collect::<Result<Vec<_>>>()
            .expect("Error building trigger deploy txs");

        // Deploy all cost models.
        let cost_models =
            self.set_admin_params
                .cost_models
                .clone()
                .into_iter()
                .map(|model| match model {
                    CozySimCostModel::JumpRate(args) => self
                        .build_deploy_cost_model_jump_rate_tx(state, jump_rate, args, &mut nonce),
                    CozySimCostModel::DynamicLevel(args) => self
                        .build_deploy_cost_model_dynamic_level_tx(
                            state,
                            dynamic_level,
                            args,
                            &mut nonce,
                        ),
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
                    self.build_deploy_drip_decay_model_tx(state, drip_decay, args, &mut nonce)
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
            owner: self.address.into(),
            pauser: self.address.into(),
            asset: self.set_admin_params.asset,
            set_config: self.set_admin_params.set_config.clone(),
            market_configs,
            salt: self
                .set_admin_params
                .salt
                .unwrap_or(rand::random::<[u8; 32]>()),
        };

        for (_, tx) in triggers {
            channel.send(SimUpdate::Evm(tx));
        }
        for (_, tx) in cost_models {
            channel.send(SimUpdate::Evm(tx));
        }
        for (_, tx) in drip_decay_models {
            channel.send(SimUpdate::Evm(tx));
        }

        let set_evm_tx = self
            .build_create_set_tx(state, create_set_args, &mut nonce)
            .expect("Error building create set tx.");

        channel.send_with_tag(SimUpdate::Evm(set_evm_tx), "Set deployment".into());
    }

    fn resolve_activation_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        let results = state
            .update_results
            .get(&self.address)
            .expect("No set deployment results.")
            .get("Set deployment")
            .expect("No set deployment results.");
        if let SimUpdateResult::Evm(evm_result) = results {
            let evm_result = unpack_execution(evm_result.clone()).expect("Set deployment failed.");
            let set_address: EthersAddress = self
                .manager
                .as_ref()
                .unwrap()
                .contract
                .decode_output("createSet", evm_result)
                .unwrap();
            self.set_address = Some(set_address.into());
        }
    }

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        if !self.set_registered {
            channel.send(SimUpdate::World(CozyUpdate::AddToSets(
                format!("{:?}'s Set", self.name).into(),
                CozySet::new(self.set_address.unwrap()),
            )));
            self.set_registered = true;
        } else {
        }
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl SetAdmin {
    fn build_deploy_cost_model_jump_rate_tx(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        factory: &CozyProtocolContract,
        args: cost_model_jump_rate_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory.contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            (*factory.address).into(),
            call_data,
            None,
            None,
        );
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = factory.contract.decode_output("deployModel", tx_result)?;
        *nonce += 1;

        Ok((addr.into(), tx))
    }

    fn build_deploy_cost_model_dynamic_level_tx(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        factory: &CozyProtocolContract,
        args: cost_model_dynamic_level_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory.contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            (*factory.address).into(),
            call_data,
            None,
            None,
        );
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = factory.contract.decode_output("deployModel", tx_result)?;
        *nonce += 1;

        Ok((addr.into(), tx))
    }

    fn build_deploy_drip_decay_model_tx(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        factory: &CozyProtocolContract,
        args: drip_decay_model_constant_factory::DeployModelCall,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let call_data = factory.contract.encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            (*factory.address).into(),
            call_data,
            None,
            None,
        );
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating drip decay model deployment.");
        let addr: EthersAddress = factory.contract.decode_output("deployModel", tx_result)?;
        *nonce += 1;

        Ok((addr.into(), tx))
    }

    fn build_deploy_dummy_trigger_tx(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        nonce: &mut u64,
    ) -> Result<(EvmAddress, TxEnv)> {
        let args = (EthersAddress::from(self.manager.as_ref().unwrap().address),);
        let (tx, _) = build_deploy_contract_tx(self.address, &DUMMYTRIGGER, args)?;
        let addr = create_address(self.address, *nonce);
        *nonce += 1;

        Ok((addr, tx))
    }

    fn build_create_set_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: manager::CreateSetCall,
        nonce: &mut u64,
    ) -> Result<TxEnv> {
        let call_data = self
            .manager
            .as_ref()
            .unwrap()
            .contract
            .encode_function("createSet", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            self.manager.as_ref().unwrap().address.into(),
            call_data,
            None,
            None,
        );

        *nonce += 1;
        Ok(tx)
    }

    fn compute_current_apy(state: &SimState<CozyUpdate, CozyWorld>) -> f64 {
        0.0
    }
}
