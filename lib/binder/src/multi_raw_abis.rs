use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use eyre::{Context as _, Result};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

use crate::utils;

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

        let abi_source = fs::read_to_string(&path)?;

        Ok(RawAbiData {
            name,
            abi_source,
            path,
        })
    }
}

#[derive(Debug)]
pub struct MultiRawAbigen {
    names_and_abi_paths: Vec<(String, PathBuf)>,
}

impl std::ops::Deref for MultiRawAbigen {
    type Target = Vec<(String, PathBuf)>;

    fn deref(&self) -> &Self::Target {
        &self.names_and_abi_paths
    }
}

impl std::ops::DerefMut for MultiRawAbigen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.names_and_abi_paths
    }
}

impl From<Vec<(String, PathBuf)>> for MultiRawAbigen {
    fn from(names_and_abi_paths: Vec<(String, PathBuf)>) -> Self {
        Self {
            names_and_abi_paths,
        }
    }
}

impl std::iter::FromIterator<(String, PathBuf)> for MultiRawAbigen {
    fn from_iter<I: IntoIterator<Item = (String, PathBuf)>>(iter: I) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl MultiRawAbigen {
    pub fn new(raw_abis_data: Vec<RawAbiData>) -> Self {
        MultiRawAbigen {
            names_and_abi_paths: raw_abis_data
                .iter()
                .map(|data| (data.name.clone(), data.path.clone()))
                .collect(),
        }
    }

    pub fn build(&self) -> Result<TokenStream> {
        let include_strs_code = self
            .names_and_abi_paths
            .iter()
            .map(
                |(name, path)| match utils::get_canonical_path_string(path) {
                    Ok(path_string) => {
                        let var_name = format_ident!("{}_RAW_ABI", name.to_uppercase());
                        let full_path = Literal::string(&path_string);
                        return Ok(quote! {
                            pub static #var_name: &str =  include_str!(#full_path);
                        });
                    }
                    Err(e) => return Err(e),
                },
            )
            .collect::<Vec<Result<_>>>();
        let ok_include_strs_code = include_strs_code
            .iter()
            .filter_map(|code| code.as_ref().ok())
            .collect::<Vec<_>>();

        let mod_name = format_ident!("raw_abis");
        Ok(quote! {
            pub use #mod_name::*;

            pub mod #mod_name {
                #(#ok_include_strs_code)*
            }
        })
    }

    pub fn write_to_module(binding: TokenStream, mod_path: impl AsRef<Path>) -> Result<()> {
        let mod_path = mod_path.as_ref();
        let file = mod_path.join("raw_abis.rs");
        let syntax_tree = syn::parse2::<syn::File>(binding.clone())?;
        let pretty_string = prettyplease::unparse(&syntax_tree);
        fs::write(file, &pretty_string)?;

        // Include raw_abis module in project module.
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(mod_path.join("mod.rs"))?;
        writeln!(file, r#"pub mod {};"#, "raw_abis")?;

        Ok(())
    }
}
