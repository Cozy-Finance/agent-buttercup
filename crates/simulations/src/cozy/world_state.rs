use std::{borrow::Cow, collections::HashMap};
use std::sync::Arc;

use ethers_contract::multicall_contract;
use simulate::{
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World},
};

use crate::cozy::EvmAddress;

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub contract_registry: HashMap<Cow<'static, str>, (EvmAddress, Arc<SimContract>)>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToContractRegistry(Cow<'static, str>, EvmAddress, Arc<SimContract>),
}

impl UpdateData for CozyUpdate {}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToContractRegistry(name, address, contract) => {
                self.contract_registry
                    .insert(name.clone(), (*address, contract.clone()));
                None
            }
        }
    }
}

impl CozyWorld {
    pub fn new() -> Self {
        CozyWorld {
            contract_registry: HashMap::new(),
        }
    }
}
