use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::{cost_model_dynamic_level_factory, cost_model_jump_rate_factory};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    types::CozyCostModelType,
    world::{CozyCostModel, CozyUpdate, CozyWorld},
    world_contracts::{CozyDynamicLevelFactory, CozyJumpRateFactory},
};

pub struct CostModelsDeployer {
    name: Cow<'static, str>,
    address: Address,
    cost_models: HashMap<Cow<'static, str>, CozyCostModelType>,
    jump_rate_factory: Arc<CozyJumpRateFactory>,
    dynamic_level_factory: Arc<CozyDynamicLevelFactory>,
}

impl CostModelsDeployer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cost_models: HashMap<Cow<'static, str>, CozyCostModelType>,
        jump_rate_factory: &Arc<CozyJumpRateFactory>,
        dynamic_level_factory: &Arc<CozyDynamicLevelFactory>,
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
                        .jump_rate_factory
                        .build_deploy_cost_model_jump_rate_tx(self.address, state, args.clone())
                        .unwrap();
                    let world_update =
                        CozyUpdate::AddToCostModels(CozyCostModel::new((*name).clone(), addr));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
                CozyCostModelType::DynamicLevel(args) => {
                    let (addr, evm_tx) = self
                        .dynamic_level_factory
                        .build_deploy_cost_model_dynamic_level_tx(self.address, state, args.clone())
                        .unwrap();
                    let world_update =
                        CozyUpdate::AddToCostModels(CozyCostModel::new((*name).clone(), addr));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
            }
        }
    }
}
