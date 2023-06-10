use crate::{EthersAddress, EvmBytes};
use ethers_solc::artifacts::BytecodeObject;
use eyre::{Result, *};

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, EthersAddress)>,
) -> Result<EvmBytes> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links);
    match bytecode.resolve() {
        Some(b) => {
            return Ok(b.0.clone());
        }
        None => {
            return Err(eyre!("Could not link bytecode."));
        }
    }
}
