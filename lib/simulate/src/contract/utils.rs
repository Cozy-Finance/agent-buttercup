use ethers::{types::Bytes as EthersBytes, types::H160 as Address};
use ethers_solc::artifacts::BytecodeObject;
use eyre::Result;

pub fn build_linked_bytecode(
    unlinked_bytecode_str: &str,
    links: Vec<(&str, &str, Address)>,
) -> Result<EthersBytes> {
    let mut bytecode = BytecodeObject::Unlinked(unlinked_bytecode_str.to_string());
    bytecode.link_all(links);
    match bytecode.resolve() {
        Some(b) => {
            return Ok(EthersBytes::from(b.0.to_owned()));
        }
        None => {
            return Err("");
        }
    }
}
