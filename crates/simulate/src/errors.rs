use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChannelError<T> {
    #[error("error sending message on channel")]
    SendError(T),
}
