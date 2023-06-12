use std::collections::HashMap;

use simulate::state::{update::UpdateData, world::World};

use crate::cozy::EvmAddress;

#[derive(Debug, Clone)]
pub struct CozyWorldState {
    pub contract_registry: HashMap<String, EvmAddress>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToContractRegistry(String, EvmAddress),
}

impl UpdateData for CozyUpdate {}

impl World for CozyWorldState {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToContractRegistry(name, address) => {
                self.contract_registry.insert(name.to_string(), *address);
                None
            }
        }
    }
}

impl CozyWorldState {
    pub fn new() -> Self {
        CozyWorldState {
            contract_registry: HashMap::new(),
        }
    }
}
