use std::collections::HashMap;

use crate::cozy::EvmAddress;
use simulate::state::{update::Update, world_state::WorldState};

#[derive(Debug, Clone)]
pub struct CozyWorldState {
    pub contract_registry: HashMap<String, EvmAddress>,
}

#[derive(Debug, Clone)]
pub enum CozyWorldStateUpdate {
    AddToContractRegistry(String, EvmAddress),
}

impl Update for CozyWorldStateUpdate {}

impl WorldState for CozyWorldState {
    type WorldStateUpdate = CozyWorldStateUpdate;
    fn execute(&mut self, update: &Self::WorldStateUpdate) {
        match update {
            CozyWorldStateUpdate::AddToContractRegistry(name, address) => {
                self.contract_registry
                    .insert(name.to_string(), address.clone());
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
