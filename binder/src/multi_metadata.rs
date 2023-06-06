use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use ethers_solc::{
    artifacts::{BytecodeObject, ContractBytecode},
    ConfigurableContractArtifact,
};
use eyre::{Context as _, Result};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

#[derive(Debug)]
pub struct RawAbiData {
    pub name: String,
    pub abi_source: String,
    pub path: PathBuf,
}

impl RawAbiData {
    pub fn from_path(
        path: PathBuf,
        name_counter: Option<&mut HashMap<String, u64>>,
    ) -> Result<Self> {
        let file_name = path
            .file_name()
            .ok_or_else(|| eyre::eyre!("Invalid path"))?;
        let mut name = file_name
            .to_str()
            .ok_or_else(|| eyre::eyre!("File name contains invalid UTF-8"))?
            .split('.') // ignore everything after the first `.`
            .next()
            .unwrap(); // file_name is not empty as asserted by .file_name() already
        let abi_source = fs::read_to_string(&path)?;

        let name = match name_counter {
            Some(nc) => match nc.get_mut(name) {
                Some(count) => {
                    *count += 1;
                    format!("{}_{}", name, *count)
                }
                None => {
                    nc.insert(name.to_string(), 0);
                    name.to_string()
                }
            },
            None => name.to_string(),
        };

        Ok(RawAbiData {
            name,
            abi_source,
            path,
        })
    }
}

#[derive(Debug)]
pub struct MultiMetadataAbigen {
    names_and_abi_paths: Vec<(String, PathBuf)>,
}

impl std::ops::Deref for MultiMetadataAbigen {
    type Target = Vec<(String, PathBuf)>;

    fn deref(&self) -> &Self::Target {
        &self.names_and_abi_paths
    }
}

impl std::ops::DerefMut for MultiMetadataAbigen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.names_and_abi_paths
    }
}

impl From<Vec<(String, PathBuf)>> for MultiMetadataAbigen {
    fn from(names_and_abi_paths: Vec<(String, PathBuf)>) -> Self {
        Self {
            names_and_abi_paths,
        }
    }
}

impl std::iter::FromIterator<(String, PathBuf)> for MultiMetadataAbigen {
    fn from_iter<I: IntoIterator<Item = (String, PathBuf)>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl MultiMetadataAbigen {
    pub fn new(raw_abis_data: Vec<RawAbiData>) -> Self {
        MultiMetadataAbigen {
            names_and_abi_paths: raw_abis_data
                .iter()
                .map(|data| (data.name.clone(), data.path.clone()))
                .collect(),
        }
    }

    pub fn build(&self) -> Result<TokenStream> {
        let mut ok_include_strs_code: Vec<TokenStream> = vec![];

        for (name, path) in self.names_and_abi_paths.clone() {
            let raw_abi_source = fs::read_to_string(path)?;
            let configurable_contract =
                serde_json::from_str::<ConfigurableContractArtifact>(&raw_abi_source)?;

            if let Some(mut metadata) = configurable_contract.metadata.clone() {
                let (abi_contract_path, abi_contract_name) = metadata
                    .settings
                    .compilation_target
                    .pop_first()
                    .ok_or_else(|| eyre::eyre!("Could not get name and lib from abi metadata."))?;
                let var_abi_contract_name = format_ident!("{}_NAME", name.to_uppercase());
                let var_abi_contract_path = format_ident!("{}_PATH", name.to_uppercase());
                let abi_contract_name = Literal::string(&abi_contract_name);
                let abi_contract_path = Literal::string(&abi_contract_path);

                let bytecode: ContractBytecode =
                    configurable_contract.into_contract_bytecode().into();
                let mut bytecode = bytecode.bytecode.ok_or_else(|| {
                    eyre::eyre!("Could not convert raw abi bytecode to bytecode.")
                })?;

                match bytecode.object {
                    BytecodeObject::Bytecode(_) => ok_include_strs_code.push(quote! {
                        pub static #var_abi_contract_name: &str = #abi_contract_name;
                        pub static #var_abi_contract_path: &str = #abi_contract_path;
                    }),
                    BytecodeObject::Unlinked(s) => {
                        let var_bytecode = format_ident!("{}_RAW_BYTECODE", name.to_uppercase());
                        let bytecode_str = Literal::string(&s);
                        ok_include_strs_code.push(quote! {
                            pub static #var_abi_contract_name: &str = #abi_contract_name;
                            pub static #var_abi_contract_path: &str = #abi_contract_path;
                            pub static #var_bytecode: &str = #bytecode_str;
                        });
                    }
                };
            }
        }

        let mod_name = format_ident!("metadata");
        Ok(quote! {
            pub use #mod_name::*;

            pub mod #mod_name {
                #(#ok_include_strs_code)*
            }
        })
    }

    pub fn write_to_module(binding: TokenStream, mod_path: impl AsRef<Path>) -> Result<()> {
        let mod_path = mod_path.as_ref();
        let file = mod_path.join("metadata.rs");
        let syntax_tree = syn::parse2::<syn::File>(binding.clone())?;
        let pretty_string = prettyplease::unparse(&syntax_tree);
        fs::write(file, &pretty_string)?;

        // Include metadata module in project module.
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(mod_path.join("mod.rs"))?;
        writeln!(file, r#"pub mod {};"#, "metadata")?;

        Ok(())
    }
}
