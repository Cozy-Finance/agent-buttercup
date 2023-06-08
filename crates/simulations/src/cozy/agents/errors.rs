use thiserror::Error;

#[derive(Debug, Error)]
pub enum CozyAgentError {
    #[error("unregistered address")]
    UnregisteredAddress,
}
