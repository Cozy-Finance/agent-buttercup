use thiserror::Error;

use crate::agent::types::AgentId;

#[derive(Error, Debug)]
pub enum ChannelError<T> {
    #[error("error sending message on channel")]
    SendError(T),
}

#[derive(Error, Debug)]
pub enum SimManagerError {
    #[error("address collision for agent id: `{0:?}`")]
    AddressCollision(AgentId),
}
