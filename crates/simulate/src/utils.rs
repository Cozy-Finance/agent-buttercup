use ethers::{
    abi::{Detokenize, Function},
    prelude::decode_function_data,
};
use ethers_solc::artifacts::BytecodeObject;

use crate::{u256::U256, EthersAddress, EvmBytes};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct GasSettings {
    pub gas_limit: u64,
    pub gas_price: U256,
    pub gas_priority_fee: Option<U256>,
}

impl Default for GasSettings {
    fn default() -> Self {
        Self {
            gas_limit: u64::MAX,
            gas_price: U256::zero(),
            gas_priority_fee: None,
        }
    }
}

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, EthersAddress)>,
) -> Result<EvmBytes, anyhow::Error> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links);
    match bytecode.resolve() {
        Some(b) => Ok(b.0.clone()),
        None => Err(anyhow::format_err!("Error linking bytecode")),
    }
}

pub fn decode_output<D: Detokenize>(
    function: &Function,
    output_bytes: EvmBytes,
) -> Result<D, ethers::abi::AbiError> {
    decode_function_data::<D, &[u8]>(function, &output_bytes, false)
}
