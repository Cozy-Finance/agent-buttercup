use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
};
use auto_impl::auto_impl;

use simulate::{
    address::Address,
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World},
};

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddToProtocolContracts(Arc<CozyProtocolContract>),
    AddToSets(CozySet),
    AddToCostModels(Arc<CozyCostModel>),
    AddToDripDecayModels(Arc<CozyDripDecayModel>),
    AddToTriggers(CozyTrigger),
    UpdateSetData(Cow<'static, str>, f64),
    UpdateTriggerData(Cow<'static, str>, f64),
    Triggered(Cow<'static, str>),
}

impl UpdateData for CozyUpdate {}

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub protocol_contracts: CozyMap<Arc<CozyProtocolContract>>,
    pub sets: CozyMap<CozySet>,
    pub cost_models: CozyMap<Arc<CozyCostModel>>,
    pub drip_decay_models: CozyMap<Arc<CozyDripDecayModel>>,
    pub triggers: CozyMap<CozyTrigger>,
}

impl CozyWorld {
    pub fn new() -> Self {
        log::info!("Creating Cozy World");
        CozyWorld {
            protocol_contracts: CozyMap::new(),
            sets: CozyMap::new(),
            cost_models: CozyMap::new(),
            drip_decay_models: CozyMap::new(),
            triggers: CozyMap::new(),
        }
    }
}

impl Default for CozyWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddToProtocolContracts(contract) => {
                let _ = self.protocol_contracts.insert(contract.clone());
            }
            CozyUpdate::AddToSets(set) => {
                let _ = self.sets.insert(set.clone());
            }
            CozyUpdate::AddToCostModels(cost_model) => {
                let _ = self.cost_models.insert(cost_model.clone());
            }
            CozyUpdate::AddToDripDecayModels(drip_decay_model) => {
                let _ = self.drip_decay_models.insert(drip_decay_model.clone());
            }
            CozyUpdate::AddToTriggers(trigger) => {
                let _ = self.triggers.insert(trigger.clone());
            }
            CozyUpdate::UpdateSetData(name, new_apy) => {
                let mut set = self.sets.get_mut_by_name(name).unwrap();
                set.apy = *new_apy;
            }
            CozyUpdate::UpdateTriggerData(name, new_prob) => {
                let mut trigger = self.triggers.get_mut_by_name(name).unwrap();
                trigger.current_prob = *new_prob;
            }
            CozyUpdate::Triggered(name) => {
                let mut trigger = self.triggers.get_mut_by_name(name).unwrap();
                trigger.triggered = true;
            }
        }
        None
    }
}

#[auto_impl(&, Arc)]
pub trait CozyMapId {
    fn name(&self) -> Cow<'static, str>;
    fn address(&self) -> Address;
}

macro_rules! impl_cozy_map_id {
    ($struct_name:ident) => {
        impl CozyMapId for $struct_name {
            fn name(&self) -> Cow<'static, str> {
                self.name.clone()
            }

            fn address(&self) -> Address {
                self.address
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct CozyProtocolContract {
    pub name: Cow<'static, str>,
    pub address: Address,
    pub contract: SimContract,
}

impl CozyProtocolContract {
    pub fn new(name: Cow<'static, str>, address: Address, contract: SimContract) -> Arc<Self> {
        Arc::new(CozyProtocolContract {
            name,
            address,
            contract,
        })
    }
}
impl_cozy_map_id!(CozyProtocolContract);

#[derive(Debug, Clone)]
pub struct CozySet {
    pub name: Cow<'static, str>,
    pub address: Address,
    pub trigger_lookup: HashMap<Address, u16>,
    pub apy: f64,
}

impl CozySet {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        trigger_lookup: HashMap<Address, u16>,
    ) -> Self {
        CozySet {
            name,
            address,
            trigger_lookup,
            apy: 0.0,
        }
    }
}

impl_cozy_map_id!(CozySet);

#[derive(Debug, Clone)]
pub struct CozyCostModel {
    pub name: Cow<'static, str>,
    pub address: Address,
}

impl CozyCostModel {
    pub fn new(name: Cow<'static, str>, address: Address) -> Arc<Self> {
        Arc::new(CozyCostModel { name, address })
    }
}

impl_cozy_map_id!(CozyCostModel);

#[derive(Debug, Clone)]
pub struct CozyDripDecayModel {
    pub name: Cow<'static, str>,
    pub address: Address,
}

impl CozyDripDecayModel {
    pub fn new(name: Cow<'static, str>, address: Address) -> Arc<Self> {
        Arc::new(CozyDripDecayModel { name, address })
    }
}

impl_cozy_map_id!(CozyDripDecayModel);

#[derive(Debug, Clone)]
pub struct CozyTrigger {
    pub name: Cow<'static, str>,
    pub address: Address,
    pub current_prob: f64,
    pub triggered: bool,
}

impl CozyTrigger {
    pub fn new(name: Cow<'static, str>, address: Address, current_prob: f64) -> Self {
        CozyTrigger {
            name,
            address,
            current_prob,
            triggered: false,
        }
    }
}

impl_cozy_map_id!(CozyTrigger);

#[derive(Debug, Clone)]
pub struct CozyMap<T: CozyMapId> {
    name_to_idx_map: HashMap<Cow<'static, str>, usize>,
    addr_to_idx_map: HashMap<Address, usize>,
    pub items: Vec<T>,
}

impl<T: CozyMapId> CozyMap<T> {
    pub fn new() -> Self {
        CozyMap {
            name_to_idx_map: HashMap::new(),
            addr_to_idx_map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn get_by_addr(&self, addr: &Address) -> Option<&T> {
        let idx = self.addr_to_idx_map.get(addr)?;
        self.items.get(*idx)
    }

    pub fn get_mut_by_addr(&mut self, addr: &Address) -> Option<&mut T> {
        let idx = self.addr_to_idx_map.get(addr)?;
        self.items.get_mut(*idx)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        let idx = self.name_to_idx_map.get(name)?;
        self.items.get(*idx)
    }

    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut T> {
        let idx = self.name_to_idx_map.get(name)?;
        self.items.get_mut(*idx)
    }

    pub fn get_addr(&self, name: &str) -> Option<Address> {
        let idx = self.name_to_idx_map.get(name)?;
        let addr = self.items[*idx].address();
        Some(addr)
    }

    pub fn get_name(&self, addr: &Address) -> Option<Cow<'static, str>> {
        let idx = self.addr_to_idx_map.get(addr)?;
        let name = self.items[*idx].name();
        Some(name)
    }

    pub fn insert(&mut self, val: T) -> Result<(), String> {
        let name = val.name();
        let addr = val.address();

        if self.name_to_idx_map.contains_key(&name) {
            return Err(format!("Name {} already exists in map", name));
        }
        if self.addr_to_idx_map.contains_key(&addr) {
            return Err(format!("Address {:?} already exists in map", addr));
        }

        let idx = self.items.len();
        self.name_to_idx_map.insert(name, idx);
        self.addr_to_idx_map.insert(addr, idx);
        self.items.push(val);
        Ok(())
    }

    pub fn values(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T: CozyMapId> Default for CozyMap<T> {
    fn default() -> Self {
        Self::new()
    }
}