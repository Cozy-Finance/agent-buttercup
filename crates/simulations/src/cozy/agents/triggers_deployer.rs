use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::drip_decay_model_constant_factory;
use eyre::Result;
use rand_distr::{Bernoulli, Distribution};
use revm::primitives::{create_address, TxEnv};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::utils::build_deploy_tx_and_contract,
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    bindings_wrapper::*,
    distributions::TriggerProbModel,
    types::CozyTriggerType,
    world::{CozyTrigger, CozyUpdate, CozyWorld},
    world_contracts::{CozyChainlinkTriggerFactory, CozyManager, CozyUmaTriggerFactory},
    EthersAddress,
};

pub struct TriggersDeployer {
    name: Cow<'static, str>,
    address: Address,
    triggers: HashMap<Cow<'static, str>, CozyTriggerType>,
    triggers_models: HashMap<Cow<'static, str>, Option<TriggerProbModel>>,
    _uma_trigger_factory: Arc<CozyUmaTriggerFactory>,
    _chainlink_trigger_factory: Arc<CozyChainlinkTriggerFactory>,
    manager: Arc<CozyManager>,
    rng: rand::rngs::StdRng,
}

impl TriggersDeployer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        triggers: HashMap<Cow<'static, str>, CozyTriggerType>,
        _uma_trigger_factory: &Arc<CozyUmaTriggerFactory>,
        _chainlink_trigger_factory: &Arc<CozyChainlinkTriggerFactory>,
        manager: &Arc<CozyManager>,
        rng: rand::rngs::StdRng,
    ) -> Self {
        Self {
            name,
            address,
            triggers,
            triggers_models: HashMap::new(),
            _uma_trigger_factory: _uma_trigger_factory.clone(),
            _chainlink_trigger_factory: _chainlink_trigger_factory.clone(),
            manager: manager.clone(),
            rng,
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
            log::info!("{:?} deploying {}.", self.name, name);
            match trigger_type {
                CozyTriggerType::DummyTrigger(trigger_prob_model) => {
                    self.triggers_models
                        .insert(name.clone(), Some(trigger_prob_model.clone()));

                    let (addr, evm_tx) = self
                        .build_deploy_dummy_trigger_tx(state, &mut nonce)
                        .unwrap();
                    let world_update = CozyUpdate::AddToTriggers(CozyTrigger::new(
                        name.clone(),
                        addr,
                        trigger_prob_model.current_prob,
                    ));
                    channel.send(SimUpdate::Bundle(evm_tx, world_update));
                }
                CozyTriggerType::ChainlinkTrigger => {}
                CozyTriggerType::UmaTrigger => {}
            }
        }
    }

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        for (name, trigger_prob_model) in self.triggers_models.iter_mut() {
            match trigger_prob_model {
                Some(model) => {
                    let prob = model.step(&mut self.rng);
                    let triggered = Bernoulli::new(prob).unwrap().sample(&mut self.rng);
                    let prob_world_update =
                        CozyUpdate::UpdateTriggerData(name.clone(), model.step(&mut self.rng));
                    channel.send(SimUpdate::World(prob_world_update));
                    if triggered {
                        let triggered_world_update = CozyUpdate::Triggered(name.clone());
                        channel.send(SimUpdate::World(triggered_world_update));
                    }
                }
                None => {}
            }
        }
    }
}

impl TriggersDeployer {
    fn build_deploy_dummy_trigger_tx(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        nonce: &mut u64,
    ) -> Result<(Address, TxEnv)> {
        let args: EthersAddress = self.manager.address.into();
        let (tx, _) = build_deploy_tx_and_contract(
            self.address,
            DUMMYTRIGGER.abi,
            DUMMYTRIGGER.bytecode.unwrap(),
            args,
        )?;
        let addr = create_address(self.address.into(), *nonce);
        *nonce += 1;

        Ok((Address::from(addr), tx))
    }
}
