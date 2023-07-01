use std::borrow::Cow;

use crossbeam_channel::Sender;

use super::types::AgentId;
use crate::{
    address::Address,
    errors::ChannelError,
    state::update::{SimUpdate, UpdateData},
};

#[derive(Debug, Clone)]
pub struct AgentSimUpdate<U: UpdateData> {
    pub update: SimUpdate<U>,
    pub address: Address,
    pub tag: Option<Cow<'static, str>>,
}
pub struct AgentChannel<U: UpdateData> {
    pub sender: Sender<AgentSimUpdate<U>>,
    pub address: Address,
    pub tag: Option<Cow<'static, str>>,
}

impl<U: UpdateData> AgentChannel<U> {
    pub fn new(sender: &Sender<AgentSimUpdate<U>>, agent_id: &AgentId) -> Self {
        AgentChannel {
            address: agent_id.address,
            sender: sender.clone(),
            tag: agent_id.clone().name.into(),
        }
    }

    /// Sends an update to the stepper. If the tag of the AgentChannel is not none,
    /// the update will be defaultly tagged with the AgentChannel tag.
    pub fn send(&self, update: SimUpdate<U>) {
        self.sender
            .send(AgentSimUpdate {
                update,
                address: self.address,
                tag: self.tag.clone().or(None),
            })
            .map_err(ChannelError::SendError)
            .unwrap();
    }

    pub fn send_with_tag(&self, update: SimUpdate<U>, tag: Cow<'static, str>) {
        self.sender
            .send(AgentSimUpdate {
                update,
                address: self.address,
                tag: Some(tag),
            })
            .map_err(ChannelError::SendError)
            .unwrap();
    }
}
