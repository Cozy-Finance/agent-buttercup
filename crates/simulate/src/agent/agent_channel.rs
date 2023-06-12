use std::borrow::Cow;

use crossbeam_channel::Sender;

use crate::{
    errors::ChannelError,
    state::update::{SimUpdate, UpdateData},
    EvmAddress,
};

#[derive(Debug, Clone)]
pub struct AgentSimUpdate<U: UpdateData> {
    pub update: SimUpdate<U>,
    pub address: EvmAddress,
    pub tag: Option<Cow<'static, str>>,
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
