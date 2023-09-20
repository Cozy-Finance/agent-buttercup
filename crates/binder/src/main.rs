use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use clap::Parser;
use ethers_contract::{Abigen, ContractFilter, ExcludeContracts, MultiAbigen, SelectContracts};
use multi_metadata::{MultiMetadataAbigen, RawAbiData};

mod multi_metadata;
mod utils;

#[derive(Parser)]
struct Binder {
    /// Path to where the raw abis are stored.
    #[clap(long)]
    pub abis: PathBuf,

    /// Path to the bindings library.
    #[clap(long)]
    pub bindings: PathBuf,

    /// Create bindings only for contracts whose names match the specified filter(s)
    #[clap(long, required = false)]
    pub select: Vec<regex::Regex>,

    /// Create bindings only for contracts whose names do not match the specified filter(s)
    #[clap(long, conflicts_with = "select", required = false)]
    pub skip: Vec<regex::Regex>,

    /// The name of the Rust module to generate.
    #[clap(long, value_name = "NAME")]
    mod_name: String,

    /// Overwrite existing generated bindings.
    ///
    /// By default, the command will check that the bindings are correct, and then exit. If
    /// --overwrite is passed, it will instead delete and overwrite the bindings.
    #[clap(long, default_value_t = false)]
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
    fn get_multi(&self) -> Result<(MultiAbigen, MultiMetadataAbigen), anyhow::Error> {
        // Counter to increment contract names in case of collisions.
        let mut contract_name_counter: HashMap<String, u64> = HashMap::new();
        // Build a vec of raw abigen data.
        let mut multi_raw_abigen_data = utils::json_files(&self.abis)
            .map(|path| RawAbiData::from_path(path, Some(&mut contract_name_counter)))
            .collect::<Result<Vec<_>, anyhow::Error>>()?;

        // Apply filter to `multi_raw_abigen_data`.
        multi_raw_abigen_data.retain(|data| self.get_filter().is_match(&data.name));

        // Build `MultiAbigen` from abigens.
        let abigens = multi_raw_abigen_data
            .iter()
            .map(|data| {
                Abigen::new(data.name.clone(), data.abi_source.clone())
                    .map(|abigen| abigen.format(true))
                    .map_err(|e| anyhow::anyhow!(e))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        let multi_abigen = MultiAbigen::from_abigens(abigens);
        anyhow::ensure!(!multi_abigen.is_empty(), "No contract artifacts found.");

        // Build `MultiMetadataAbigen`.
        let multi_metadata_abigen = MultiMetadataAbigen::new(multi_raw_abigen_data);
        anyhow::ensure!(
            !multi_metadata_abigen.is_empty(),
            "No contract artifacts found."
        );

        Ok((multi_abigen, multi_metadata_abigen))
    }

    /// Check that the existing bindings match the expected abigen output
    fn check_existing_bindings(&self) -> Result<(), anyhow::Error> {
        let (multi_abigen, _) = self.get_multi()?;
        let contract_bindings = multi_abigen.build().map_err(|e| anyhow::anyhow!(e))?;

        println!(
            "Checking bindings for {} contracts.",
            contract_bindings.len()
        );
        contract_bindings
            .ensure_consistent_module(self.module_root(), false)
            .map_err(|e| anyhow::anyhow!(e))?;
        println!("OK.");

        Ok(())
    }

    fn generate_bindings(&self, write_exports: bool) -> Result<(), anyhow::Error> {
        let (multi_abigen, multi_metadata_abigen) = self.get_multi()?;

        // Write contract bindings to crate.
        let contract_bindings = multi_abigen.build().map_err(|e| anyhow::anyhow!(e))?;
        println!(
            "Generating contract bindings for {} contracts.",
            contract_bindings.len()
        );

        contract_bindings
            .write_to_module(self.module_root(), false)
            .map_err(|e| anyhow::anyhow!(e))?;

        let metadata_binding = multi_metadata_abigen.build()?;
        println!(
            "Generating metadata bindings for {} contracts.",
            multi_metadata_abigen.len()
        );
        MultiMetadataAbigen::write_to_module(metadata_binding, self.module_root())?;

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

fn main() -> Result<(), anyhow::Error> {
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
