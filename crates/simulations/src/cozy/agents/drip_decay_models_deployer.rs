use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::drip_decay_model_constant_factory;
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv, unpack_execution},
};

use crate::cozy::{
    types::{CozyCostModelType, CozyDripDecayModelType},
    world::{CozyCostModel, CozyDripDecayModel, CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress,
};

pub struct DripDecayModelsDeployer {
    name: Option<Cow<'static, str>>,
    address: Address,
    drip_decay_models: HashMap<Cow<'static, str>, CozyDripDecayModelType>,
    drip_decay_constant_factory: Arc<CozyProtocolContract>,
}

impl DripDecayModelsDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        drip_decay_models: HashMap<Cow<'static, str>, CozyDripDecayModelType>,
        drip_decay_constant_factory: &Arc<CozyProtocolContract>,
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
                        .build_deploy_drip_decay_model_constant_tx(state, args.clone())
                        .unwrap();
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

impl DripDecayModelsDeployer {
    fn build_deploy_drip_decay_model_constant_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: drip_decay_model_constant_factory::DeployModelCall,
    ) -> Result<(Address, TxEnv)> {
        let call_data = self
            .drip_decay_constant_factory
            .contract
            .encode_function("deployModel", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            self.drip_decay_constant_factory.address,
            call_data,
            None,
            None,
        );
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating drip decay model deployment.");
        let addr: EthersAddress = self
            .drip_decay_constant_factory
            .contract
            .decode_output("deployModel", tx_result)?;

        Ok((Address::from(addr), tx))
    }
}
