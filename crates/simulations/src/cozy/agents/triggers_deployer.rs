use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::drip_decay_model_constant_factory;
use eyre::Result;
use revm::primitives::{create_address, TxEnv};
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState}, address::Address,
};

use crate::cozy::{
    bindings_wrapper::*,
    types::CozyTriggerType,
    utils::build_deploy_contract_tx,
    world::{CozyDripDecayModel, CozyProtocolContract, CozyTrigger, CozyUpdate, CozyWorld},
    EthersAddress
};

pub struct TriggersDeployer {
    name: Option<Cow<'static, str>>,
    address: Address,
    triggers: HashMap<Cow<'static, str>, CozyTriggerType>,
    uma_trigger_factory: Arc<CozyProtocolContract>,
    chainlink_trigger_factory: Arc<CozyProtocolContract>,
    manager: Arc<CozyProtocolContract>,
}

impl TriggersDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        triggers: HashMap<Cow<'static, str>, CozyTriggerType>,
        uma_trigger_factory: &Arc<CozyProtocolContract>,
        chainlink_trigger_factory: &Arc<CozyProtocolContract>,
        manager: &Arc<CozyProtocolContract>,
    ) -> Self {
        Self {
            name,
            address,
            triggers,
            uma_trigger_factory: uma_trigger_factory.clone(),
            chainlink_trigger_factory: chainlink_trigger_factory.clone(),
            manager: manager.clone(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for TriggersDeployer {
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
        let mut nonce = 0;

        for (name, trigger_type) in &self.triggers {
            match trigger_type {
                CozyTriggerType::DummyTrigger => {
                    let (addr, evm_tx) = self
                        .build_deploy_dummy_trigger_tx(state, &mut nonce)
                        .unwrap();
                    let world_update =
                        CozyUpdate::AddToTriggers((*name).clone(), CozyTrigger::new(addr));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
                CozyTriggerType::ChainlinkTrigger => {}
                CozyTriggerType::UmaTrigger => {}
            }
        }
    }
}

impl TriggersDeployer {
    fn build_deploy_dummy_trigger_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        nonce: &mut u64,
    ) -> Result<(Address, TxEnv)> {
        let args: (EthersAddress,) = (self.manager.address.into(),);
        let (tx, _) = build_deploy_contract_tx(self.address, &DUMMYTRIGGER, args)?;
        let addr = create_address(self.address.into(), *nonce);
        *nonce += 1;

        Ok((Address::from(addr), tx))
    }
}
