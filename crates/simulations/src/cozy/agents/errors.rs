use simulate::address::Address;

#[derive(Debug, thiserror::Error)]
pub enum CozyAgentError {
    #[error("Unregistered address: {0}.")]
    UnregisteredAddress(Address),
}

pub type CozyAgentResult<T> = Result<T, anyhow::Error>;
