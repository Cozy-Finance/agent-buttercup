use crate::state::update::SimUpdateResult;
use crate::state::update::{SimUpdate, Update};
use crate::EvmAddress;
use crossbeam_channel::Sender;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct AgentSimUpdate<U: Update> {
    pub update: SimUpdate<U>,
    pub address: EvmAddress,
    pub tag: Option<u64>,
}

pub struct AgentChannel<U: Update> {
    pub sender: Sender<AgentSimUpdate<U>>,
    pub address: EvmAddress,
}

impl<U: Update> AgentChannel<U> {
    pub fn send(&self, update: SimUpdate<U>) {
        self.sender.send(AgentSimUpdate {
            update,
            address: self.address.clone(),
            tag: None,
        });
    }

    pub fn send_with_tag(&self, update: SimUpdate<U>, tag: &str) {
        let mut hasher = DefaultHasher::new();
        tag.hash(&mut hasher);
        self.sender.send(AgentSimUpdate {
            update,
            address: self.address.clone(),
            tag: Some(hasher.finish()),
        });
    }
}

#[derive(Debug)]
pub struct AgentUpdateResults<'s> {
    updates: Option<&'s HashMap<u64, SimUpdateResult>>,
}

impl <'s> AgentUpdateResults<'s> {
    pub fn new(updates: Option<&'s HashMap<u64, SimUpdateResult>>) -> Self {
        AgentUpdateResults { updates }
    }

    pub fn get_update(&self, tag: &str) -> Option<&SimUpdateResult> {
        let mut hasher = DefaultHasher::new();
        tag.hash(&mut hasher);
        self.updates?.get(&hasher.finish())
    }
}
