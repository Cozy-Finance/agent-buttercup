use thiserror::Error;

use crate::EvmAddress;

#[derive(Error, Debug)]
pub enum ChannelError<T> {
    #[error("error sending message on channel")]
    SendError(T),
}

#[derive(Error, Debug)]
pub enum ManagerError {
    #[error("collision in agent address: `{0}`")]
    AddressCollision(EvmAddress),
}
