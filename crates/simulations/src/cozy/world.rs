use std::{borrow::Cow, collections::HashMap, sync::Arc};

use simulate::{
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World},
};

use super::bindings_wrapper::{SET, SETFACTORY};
use crate::cozy::EvmAddress;

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub protocol_contracts: HashMap<Cow<'static, str>, Arc<CozyProtocolContract>>,
    pub sets: HashMap<Cow<'static, str>, CozySet>,
    pub cost_models: HashMap<Cow<'static, str>, CozyCostModel>,
    pub drip_decay_models: HashMap<Cow<'static, str>, CozyDripDecayModel>,
    pub triggers: HashMap<Cow<'static, str>, CozyTrigger>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Cow<'static, str>, CozyProtocolContract),
    AddToSets(Cow<'static, str>, CozySet),
    AddToCostModels(Cow<'static, str>, CozyCostModel),
    AddToDripDecayModels(Cow<'static, str>, CozyDripDecayModel),
    AddToTriggers(Cow<'static, str>, CozyTrigger),
    UpdateSetData(Cow<'static, str>, u128),
}

impl UpdateData for CozyUpdate {}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToProtocolContracts(name, contract) => {
                self.protocol_contracts
                    .insert(name.clone(), Arc::new(contract.clone()));
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
                let set = self.sets.get_mut(name).unwrap();
                set.apy = *new_apy;
            }
        }
        None
    }
}

impl CozyWorld {
    pub fn new() -> Self {
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
    pub address: EvmAddress,
    pub contract: SimContract,
}

impl CozyProtocolContract {
    pub fn new(address: EvmAddress, contract: SimContract) -> Self {
        CozyProtocolContract { address, contract }
    }
}

#[derive(Debug, Clone)]
pub struct CozySet {
    pub address: EvmAddress,
    pub trigger_lookup: HashMap<EvmAddress, u16>,
    pub apy: u128,
}

impl CozySet {
    pub fn new(address: EvmAddress, trigger_lookup: HashMap<EvmAddress, u16>) -> Self {
        CozySet {
            address,
            trigger_lookup,
            apy: 0 as u128,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CozyCostModel {
    pub address: EvmAddress,
}

impl CozyCostModel {
    pub fn new(address: EvmAddress) -> Self {
        CozyCostModel { address }
    }
}

#[derive(Debug, Clone)]
pub struct CozyDripDecayModel {
    pub address: EvmAddress,
}

impl CozyDripDecayModel {
    pub fn new(address: EvmAddress) -> Self {
        CozyDripDecayModel { address }
    }
}

#[derive(Debug, Clone)]
pub struct CozyTrigger {
    pub address: EvmAddress,
}

impl CozyTrigger {
    pub fn new(address: EvmAddress) -> Self {
        CozyTrigger { address }
    }
}
