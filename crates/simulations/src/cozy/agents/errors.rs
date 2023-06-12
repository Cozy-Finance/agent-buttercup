use thiserror::Error;

#[derive(Debug, Error)]
pub enum CozyAgentError {
    #[error("missing world state")]
    MissingWorldState,
    #[error("unregistered address")]
    UnregisteredAddress,
}
