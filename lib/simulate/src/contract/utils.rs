use ethers::{prelude::core::types::Bytes as EthersBytes, types::H160};
use ethers_solc::{
    artifacts::{BytecodeObject, ContractBytecode},
    ConfigurableContractArtifact,
};
use eyre::Result;
use revm::primitives::Address;
use serde_json;

pub fn build_linked_bytecode(
    raw_abi: &str,
    library_links: Vec<(&str, Address)>,
) -> Result<EthersBytes> {
    let configurable_contract = serde_json::from_str::<ConfigurableContractArtifact>(&raw_abi)?;
    let mut bytecode_unlinked: ContractBytecode =
        configurable_contract.into_contract_bytecode().into();

    let (raw_abis, addresses): (Vec<_>, Vec<_>) = library_links.into_iter().unzip();
    let addresses = addresses
        .iter()
        .map(|a| H160::from(a.clone()))
        .collect::<Vec<_>>();
    let fully_qualified_paths = raw_abis
        .iter()
        .map(|raw_abi| get_fully_qualified_link(raw_abi))
        .collect::<Result<Vec<_>>>()?;
    let library_address_links = fully_qualified_paths
        .into_iter()
        .zip(addresses.into_iter())
        .into_iter();
    let mut bytecode = bytecode_unlinked
        .bytecode
        .ok_or_else(|| eyre::eyre!("Could not convert raw abi bytecode to bytecode."))?;
    bytecode.link_all_fully_qualified(library_address_links);

    match bytecode.object {
        BytecodeObject::Bytecode(b) => Ok(b.0.into()),
        BytecodeObject::Unlinked(_) => Err(eyre::eyre!("Bytecode could not be linked.")),
    }
}

pub fn get_fully_qualified_link(raw_abi: &str) -> Result<String> {
    let configurable_contract = serde_json::from_str::<ConfigurableContractArtifact>(&raw_abi)?;
    let (lib, name) = configurable_contract
        .metadata
        .ok_or_else(|| eyre::eyre!("Could not find metadata for contract in raw abi."))?
        .settings
        .compilation_target
        .pop_first()
        .ok_or_else(|| eyre::eyre!("Could not find compilation target for contract in raw abi."))?;

    Ok(lib + ":" + &name)
}
