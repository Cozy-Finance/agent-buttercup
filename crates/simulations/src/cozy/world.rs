use std::{borrow::Cow, collections::HashMap, sync::Arc};

use ethers_contract::multicall_contract;
use simulate::{
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World},
};

use crate::cozy::EvmAddress;

#[derive(Debug, Clone)]
pub struct CozyContractData {
    pub address: EvmAddress,
    pub contract: Arc<SimContract>,
}

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub protocol_contracts: HashMap<Cow<'static, str>, CozyContractData>,
    pub sets: HashMap<Cow<'static, str>, EvmAddress>,
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Cow<'static, str>, EvmAddress, Arc<SimContract>),
    AddToSets(Cow<'static, str>, EvmAddress),
}

impl UpdateData for CozyUpdate {}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToProtocolContracts(name, address, contract) => {
                self.protocol_contracts.insert(
                    name.clone(),
                    CozyContractData {
                        address: *address,
                        contract: contract.clone(),
                    },
                );
                None
            }
            CozyUpdate::AddToSets(name, address) => {
                self.sets.insert(name.clone(), *address);
                None
            }
        }
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
