use clap::Parser;
use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use ethers_contract::{Abigen, ContractFilter, ExcludeContracts, MultiAbigen, SelectContracts};
use eyre::{Context as _, Result};
use regex;

use multi_raw_abis::{MultiRawAbigen, RawAbiData};

mod multi_raw_abis;
mod utils;

#[derive(Parser)]
struct Binder {
    /// Path to where the raw abis are stored.
    #[arg(long)]
    pub abis: PathBuf,

    /// Path to the bindings library.
    #[arg(long)]
    pub bindings: PathBuf,

    /// Create bindings only for contracts whose names match the specified filter(s)
    #[arg(long, required = false)]
    pub select: Vec<regex::Regex>,

    /// Create bindings only for contracts whose names do not match the specified filter(s)
    #[arg(long, conflicts_with = "select", required = false)]
    pub skip: Vec<regex::Regex>,

    /// The name of the Rust module to generate.
    #[arg(long, value_name = "NAME")]
    mod_name: String,

    /// Overwrite existing generated bindings.
    ///
    /// By default, the command will check that the bindings are correct, and then exit. If
    /// --overwrite is passed, it will instead delete and overwrite the bindings.
    #[arg(long, default_value_t = false)]
    overwrite: bool,
}

impl Binder {
    fn module_root(&self) -> PathBuf {
        self.bindings.join("src").join(self.mod_name.clone())
    }

    fn bindings_exist(&self) -> bool {
        self.module_root().is_dir()
    }

    /// Returns the filter to use for `MultiAbigen`.
    fn get_filter(&self) -> ContractFilter {
        if !self.select.is_empty() {
            return SelectContracts::default()
                .extend_regex(self.select.clone())
                .into();
        }
        if !self.skip.is_empty() {
            return ExcludeContracts::default()
                .extend_regex(self.skip.clone())
                .into();
        }
        // This excludes all test/script and forge-std contracts.
        ExcludeContracts::default()
            .extend_pattern([
                ".*Test.*",
                ".*test.*",
                ".*Script",
                ".*script",
                "\\.t\\.",
                "\\.s\\.",
                "^.*Benchmark.*$",
                "^.*benchmark.*$",
                "^.*Mock.*$",
                "^.*mock.*$",
                "console[2]?",
                "CommonBase",
                "Components",
                "[Ss]td(Chains|Math|Error|Json|Utils|Cheats|Assertions|Storage(Safe)?)",
                "[Vv]m.*",
            ])
            .extend_names(["IMulticall3"])
            .into()
    }

    /// Instantiate the `MultiAbigen` and `MultiRawAbis`.
    fn get_multi(&self) -> Result<(MultiAbigen, MultiRawAbigen)> {
        // Counter to increment contract names in case of collisions.
        let mut contract_name_counter: HashMap<String, u64> = HashMap::new();
        // Build a vec of raw abigen data.
        let mut multi_raw_abigen_data = utils::json_files(&self.abis)
            .map(|path| RawAbiData::from_path(path, Some(&mut contract_name_counter)))
            .collect::<Result<Vec<_>>>()?;

        // Apply filter to `multi_raw_abigen_data`.
        multi_raw_abigen_data.retain(|data| self.get_filter().is_match(data.name.to_string()));

        // Build `MultiAbigen` from abigens.
        let abigens = multi_raw_abigen_data
            .iter()
            .map(|data| {
                Abigen::new(data.name.clone(), data.abi_source.clone())
                    .map(|abigen| abigen.format(true))
            })
            .collect::<Result<Vec<_>>>()?;
        let multi_abigen = MultiAbigen::from_abigens(abigens);
        eyre::ensure!(!multi_abigen.is_empty(), "No contract artifacts found.");

        // Build MultiRawAbis.
        let multi_raw_abis = MultiRawAbigen::new(multi_raw_abigen_data);
        eyre::ensure!(!multi_raw_abis.is_empty(), "No contract artifacts found.");

        Ok((multi_abigen, multi_raw_abis))
    }

    /// Check that the existing bindings match the expected abigen output
    fn check_existing_bindings(&self) -> Result<()> {
        let (multi_abi_gen, _) = self.get_multi()?;
        let contract_bindings = multi_abi_gen.build()?;

        println!(
            "Checking bindings for {} contracts.",
            contract_bindings.len()
        );
        contract_bindings.ensure_consistent_module(self.module_root(), false)?;
        println!("OK.");

        Ok(())
    }

    fn generate_bindings(&self, write_exports: bool) -> Result<()> {
        let (multi_abi_gen, multi_raw_abis) = self.get_multi()?;

        // Write contract bindings to crate.
        let contract_bindings = multi_abi_gen.build()?;
        println!(
            "Generating contract bindings for {} contracts.",
            contract_bindings.len()
        );

        contract_bindings.write_to_module(&self.module_root(), false)?;

        let raw_abis_binding = multi_raw_abis.build()?;
        println!(
            "Generating raw abi bindings for {} contracts.",
            multi_raw_abis.len()
        );
        MultiRawAbigen::write_to_module(raw_abis_binding, &self.module_root())?;

        if write_exports {
            // Include project module in bindings module.
            let mut file = fs::OpenOptions::new()
                .append(true)
                .open(self.bindings.join("src").join("lib.rs"))?;
            writeln!(file, r#"pub mod {};"#, self.mod_name)?;
            writeln!(file, r#"pub use {}::*;"#, self.mod_name)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let binder = Binder::parse();

    let bindings_exist = binder.bindings_exist();

    if !binder.overwrite && bindings_exist {
        println!("Bindings found. Checking for consistency.");
        return binder.check_existing_bindings();
    }

    if binder.overwrite && bindings_exist {
        fs::remove_dir_all(binder.module_root())?
    }

    binder.generate_bindings(!bindings_exist)?;
    println!(
        "Bindings have been output to {}",
        binder.module_root().to_str().unwrap()
    );

    Ok(())
}
