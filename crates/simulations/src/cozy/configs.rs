use std::fs;

use eyre::Result;

use crate::cozy::runner::CozySingleSetSimRunnerSettings;

pub fn build_config_from_dir(dir: &str) -> Result<config::Config> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory.");
    let all_configs_dir = base_path.join("src/cozy/configs");

    let configs_dir = base_path.join("src/cozy/configs").join(dir);
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
        config_builder =
            config_builder.add_source(config::File::from(all_configs_dir.join(dir).join(file)));
    }

    Ok(config_builder.build()?)
}

pub fn build_cozy_sim_settings_from_dir(dir: &str) -> Result<CozySingleSetSimRunnerSettings> {
    let config = build_config_from_dir(dir)?;
    Ok(config.try_deserialize::<CozySingleSetSimRunnerSettings>()?)
}
