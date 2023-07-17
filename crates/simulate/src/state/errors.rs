use crate::address::Address;

#[derive(thiserror::Error, Debug)]
pub enum SimStateError {
    #[error("EVM DB not set.")]
    EvmDBNotSet,
    #[error(transparent)]
    EvmStateError(#[from] anyhow::Error),
    #[error("Account not found at address: {0}.")]
    AccountNotFound(Address),
}
