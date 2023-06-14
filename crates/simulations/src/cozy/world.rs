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
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Cow<'static, str>, CozyProtocolContract),
    AddToSets(Cow<'static, str>, CozySet),
    UpdateSetData(Cow<'static, str>, u128)
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
            CozyUpdate::UpdateSetData(name, new_apy) => {
                let set = self.sets.get_mut(name).unwrap();
                set.update_apy(*new_apy);
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
    pub apy: u128,
}

impl CozySet {
    pub fn new(address: EvmAddress) -> Self {
        CozySet {
            address,
            apy: 0 as u128,
        }
    }

    pub fn update_apy(&mut self, new_apy: u128) {
        self.apy = new_apy;
    }
}
