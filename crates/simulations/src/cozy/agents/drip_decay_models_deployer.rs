use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::drip_decay_model_constant_factory;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    types::CozyDripDecayModelType,
    world::{CozyDripDecayModel, CozyUpdate, CozyWorld},
    world_contracts::CozyDripDecayConstantFactory,
};

pub struct DripDecayModelsDeployer {
    name: Cow<'static, str>,
    address: Address,
    drip_decay_models: HashMap<Cow<'static, str>, CozyDripDecayModelType>,
    drip_decay_constant_factory: Arc<CozyDripDecayConstantFactory>,
}

impl DripDecayModelsDeployer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        drip_decay_models: HashMap<Cow<'static, str>, CozyDripDecayModelType>,
        drip_decay_constant_factory: &Arc<CozyDripDecayConstantFactory>,
    ) -> Self {
        Self {
            name,
            address,
            drip_decay_models,
            drip_decay_constant_factory: drip_decay_constant_factory.clone(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for DripDecayModelsDeployer {
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
        for (name, drip_decay_model_type) in &self.drip_decay_models {
            log::info!("{:?} deploying {}.", self.name, name);

            match drip_decay_model_type {
                CozyDripDecayModelType::Constant(args) => {
                    let (addr, evm_tx) = self
                        .drip_decay_constant_factory
                        .build_deploy_drip_decay_model_constant_tx(
                            self.address,
                            state,
                            args.clone(),
                        )
                        .expect("DripDecayModelsDeployer failed to build constant model tx.");
                    let world_update = CozyUpdate::AddToDripDecayModels(CozyDripDecayModel::new(
                        (*name).clone(),
                        addr,
                    ));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
            }
        }
    }
}
