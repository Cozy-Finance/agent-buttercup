use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, RwLock},
};

use simulate::{
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World}, address::Address,
};

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub protocol_contracts: HashMap<Cow<'static, str>, Arc<CozyProtocolContract>>,
    pub sets: HashMap<Cow<'static, str>, Arc<RwLock<CozySet>>>,
    pub cost_models: HashMap<Cow<'static, str>, Arc<CozyCostModel>>,
    pub drip_decay_models: HashMap<Cow<'static, str>, Arc<CozyDripDecayModel>>,
    pub triggers: HashMap<Cow<'static, str>, Arc<CozyTrigger>>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Cow<'static, str>, Arc<CozyProtocolContract>),
    AddToSets(Cow<'static, str>, Arc<RwLock<CozySet>>),
    AddToCostModels(Cow<'static, str>, Arc<CozyCostModel>),
    AddToDripDecayModels(Cow<'static, str>, Arc<CozyDripDecayModel>),
    AddToTriggers(Cow<'static, str>, Arc<CozyTrigger>),
    UpdateSetData(Cow<'static, str>, f64),
}

impl UpdateData for CozyUpdate {}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToProtocolContracts(name, contract) => {
                self.protocol_contracts
                    .insert(name.clone(), contract.clone());
            }
            CozyUpdate::AddToSets(name, set) => {
                self.sets.insert(name.clone(), set.clone());
            }
            CozyUpdate::AddToCostModels(name, cost_model) => {
                self.cost_models.insert(name.clone(), cost_model.clone());
            }
            CozyUpdate::AddToDripDecayModels(name, drip_decay_model) => {
                self.drip_decay_models
                    .insert(name.clone(), drip_decay_model.clone());
            }
            CozyUpdate::AddToTriggers(name, trigger) => {
                self.triggers.insert(name.clone(), trigger.clone());
            }
            CozyUpdate::UpdateSetData(name, new_apy) => {
                let mut set = self.sets.get_mut(name).unwrap().write().unwrap();
                set.apy = *new_apy;
            }
        }
        None
    }
}

impl CozyWorld {
    pub fn new() -> Self {
        log::info!("Creating Cozy World");
        CozyWorld {
            protocol_contracts: HashMap::new(),
            sets: HashMap::new(),
            cost_models: HashMap::new(),
            drip_decay_models: HashMap::new(),
            triggers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CozyProtocolContract {
    pub address: Address,
    pub contract: SimContract,
}

impl CozyProtocolContract {
    pub fn new(address: Address, contract: SimContract) -> Arc<Self> {
        Arc::new(CozyProtocolContract { address, contract })
    }
}

#[derive(Debug, Clone)]
pub struct CozySet {
    pub address: Address,
    pub trigger_lookup: HashMap<Address, u16>,
    pub apy: f64,
}

impl CozySet {
    pub fn new(address: Address, trigger_lookup: HashMap<Address, u16>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(CozySet {
            address,
            trigger_lookup,
            apy: 0.0,
        }))
    }
}

#[derive(Debug, Clone)]
pub struct CozyCostModel {
    pub address: Address,
}

impl CozyCostModel {
    pub fn new(address: Address) -> Arc<Self> {
        Arc::new(CozyCostModel { address })
    }
}

#[derive(Debug, Clone)]
pub struct CozyDripDecayModel {
    pub address: Address,
}

impl CozyDripDecayModel {
    pub fn new(address: Address) -> Arc<Self> {
        Arc::new(CozyDripDecayModel { address })
    }
}

#[derive(Debug, Clone)]
pub struct CozyTrigger {
    pub address: Address,
}

impl CozyTrigger {
    pub fn new(address: Address) -> Arc<Self> {
        Arc::new(CozyTrigger { address })
    }
}
