use std::fs;

use eyre::Result;

use crate::cozy::runner::CozySingleSetSimRunner;

pub fn build_config_from_dir(dir: &str) -> Result<config::Config> {
    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let all_configs_dir = workspace_dir.join("src/cozy/configs");
    let configs_dir = all_configs_dir.join(dir);

    let files = match fs::read_dir(configs_dir.clone()) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect::<Vec<_>>(),
        Err(_) => {
            return Err(eyre::eyre!("Failed to read directory: {:?}", configs_dir));
        }
    };

    let mut config_builder =
        config::Config::builder().add_source(config::File::from(all_configs_dir.join("base.yaml")));
    for file in files {
        config_builder = config_builder.add_source(config::File::from(configs_dir.join(file)));
    }

    Ok(config_builder.build()?)
}

pub fn build_cozy_sim_runner_from_dir(dir: &str) -> Result<CozySingleSetSimRunner> {
    let config = build_config_from_dir(dir)?;
    Ok(config.try_deserialize::<CozySingleSetSimRunner>()?)
}
