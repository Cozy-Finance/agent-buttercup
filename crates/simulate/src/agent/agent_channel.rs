use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crossbeam_channel::Sender;

use crate::{
    errors::ChannelError,
    state::update::{SimUpdate, SimUpdateResult, Update},
    EvmAddress,
};

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
        self.sender
            .send(AgentSimUpdate {
                update,
                address: self.address,
                tag: None,
            })
            .map_err(ChannelError::SendError)
            .unwrap();
    }

    pub fn send_with_tag(&self, update: SimUpdate<U>, tag: &str) {
        let mut hasher = DefaultHasher::new();
        tag.hash(&mut hasher);
        self.sender
            .send(AgentSimUpdate {
                update,
                address: self.address,
                tag: Some(hasher.finish()),
            })
            .map_err(ChannelError::SendError)
            .unwrap();
    }
}
#[derive(Debug)]
pub struct AgentUpdateResults<'s> {
    updates: Option<&'s HashMap<u64, SimUpdateResult>>,
}

impl<'s> AgentUpdateResults<'s> {
    pub fn new(updates: Option<&'s HashMap<u64, SimUpdateResult>>) -> Self {
        AgentUpdateResults { updates }
    }

    pub fn get_update(&self, tag: &str) -> Option<&SimUpdateResult> {
        let mut hasher = DefaultHasher::new();
        tag.hash(&mut hasher);
        self.updates?.get(&hasher.finish())
    }
}
