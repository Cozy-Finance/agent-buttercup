use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crossbeam_channel::Sender;

use crate::{
    errors::ChannelError,
    state::update::{SimUpdate, SimUpdateResult, UpdateData},
    EvmAddress,
};

#[derive(Debug, Clone)]
pub struct AgentSimUpdate<U: UpdateData> {
    pub update: SimUpdate<U>,
    pub address: EvmAddress,
    pub tag: Option<u64>,
}
pub struct AgentChannel<U: UpdateData> {
    pub sender: Sender<AgentSimUpdate<U>>,
    pub address: EvmAddress,
}

impl<U: UpdateData> AgentChannel<U> {
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
pub struct AgentUpdateResults<'s, U: UpdateData> {
    updates: Option<&'s HashMap<u64, SimUpdateResult<U>>>,
}

impl<'s, U: UpdateData> AgentUpdateResults<'s, U> {
    pub fn new(updates: Option<&'s HashMap<u64, SimUpdateResult<U>>>) -> Self {
        AgentUpdateResults { updates }
    }

    pub fn get_update(&self, tag: &str) -> Option<&SimUpdateResult<U>> {
        let mut hasher = DefaultHasher::new();
        tag.hash(&mut hasher);
        self.updates?.get(&hasher.finish())
    }
}
