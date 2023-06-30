use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::{cost_model_dynamic_level_factory, cost_model_jump_rate_factory};
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_tx, unpack_execution},
};

use crate::cozy::{
    types::CozyCostModelType,
    world::{CozyCostModel, CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress,
};

pub struct CostModelsDeployer {
    name: Option<Cow<'static, str>>,
    address: Address,
    cost_models: HashMap<Cow<'static, str>, CozyCostModelType>,
    jump_rate_factory: Arc<CozyProtocolContract>,
    dynamic_level_factory: Arc<CozyProtocolContract>,
}

impl CostModelsDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        cost_models: HashMap<Cow<'static, str>, CozyCostModelType>,
        jump_rate_factory: &Arc<CozyProtocolContract>,
        dynamic_level_factory: &Arc<CozyProtocolContract>,
    ) -> Self {
        Self {
            name,
            address,
            cost_models,
            jump_rate_factory: jump_rate_factory.clone(),
            dynamic_level_factory: dynamic_level_factory.clone(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for CostModelsDeployer {
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
        for (name, cost_model_type) in &self.cost_models {
            log::info!("{:?} deploying {}.", self.name, name);
            match cost_model_type {
                CozyCostModelType::JumpRate(args) => {
                    let (addr, evm_tx) = self
                        .build_deploy_cost_model_jump_rate_tx(state, args.clone())
                        .unwrap();
                    let world_update =
                        CozyUpdate::AddToCostModels(CozyCostModel::new((*name).clone(), addr));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
                CozyCostModelType::DynamicLevel(args) => {
                    let (addr, evm_tx) = self
                        .build_deploy_cost_model_dynamic_level_tx(state, args.clone())
                        .unwrap();
                    let world_update =
                        CozyUpdate::AddToCostModels(CozyCostModel::new((*name).clone(), addr));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
            }
        }
    }
}

impl CostModelsDeployer {
    fn build_deploy_cost_model_jump_rate_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cost_model_jump_rate_factory::DeployModelCall,
    ) -> Result<(Address, TxEnv)> {
        let call_data = self
            .jump_rate_factory
            .contract
            .encode_function("deployModel", args)?;
        let tx = build_call_tx(self.address, self.jump_rate_factory.address, call_data);
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = self
            .jump_rate_factory
            .contract
            .decode_output("deployModel", tx_result)?;

        Ok((Address::from(addr), tx))
    }

    fn build_deploy_cost_model_dynamic_level_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cost_model_dynamic_level_factory::DeployModelCall,
    ) -> Result<(Address, TxEnv)> {
        let call_data = self
            .dynamic_level_factory
            .contract
            .encode_function("deployModel", args)?;
        let tx = build_call_tx(self.address, self.dynamic_level_factory.address, call_data);
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = self
            .dynamic_level_factory
            .contract
            .decode_output("deployModel", tx_result)?;

        Ok((Address::from(addr), tx))
    }
}
