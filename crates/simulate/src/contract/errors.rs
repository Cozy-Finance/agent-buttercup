use crate::EthersAddress;

#[derive(thiserror::Error, Debug)]
pub enum SimContractError {
    #[error(transparent)]
    ConstructorEncodeError(#[from] ethers::abi::Error),
    #[error(transparent)]
    AbiError(#[from] ethers::abi::AbiError),
    #[error("ABI missing specified error name: {0}.")]
    MissingErrorName(String),
    #[error("Error linking bytecode str: {0}. Links: {1:?}.")]
    BytecodeLinkingError(String, Vec<(String, String, EthersAddress)>),
}
