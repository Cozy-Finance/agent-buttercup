use ethers_solc::artifacts::BytecodeObject;
use eyre::{Result, *};

use crate::{EthersAddress, EvmBytes};

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, EthersAddress)>,
) -> Result<EvmBytes> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links);
    match bytecode.resolve() {
        Some(b) => {
            Ok(b.0.clone())
        }
        None => {
            Err(eyre!("Could not link bytecode."))
        }
    }
}
