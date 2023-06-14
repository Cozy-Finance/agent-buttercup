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
    pub sets: HashMap<Cow<'static, str>, Arc<CozySet>>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Cow<'static, str>, CozyProtocolContract),
    AddToSets(Cow<'static, str>, CozySet),
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
                self.sets.insert(name.clone(), Arc::new(set.clone()));
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
    pub current_apy: f64,
}

impl CozySet {
    pub fn new(address: EvmAddress) -> Self {
        CozySet {
            address,
            current_apy: 0.0,
        }
    }

    pub fn compute_current_apy() -> f64 {
        0.0
    }
}
