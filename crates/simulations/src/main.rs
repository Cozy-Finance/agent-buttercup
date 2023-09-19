use std::error::Error;

use clap::{ArgEnum, Parser};
use flexi_logger::{Duplicate, Logger};

pub mod cozy;

#[derive(Debug, ArgEnum, Clone)]
#[clap(rename_all = "kebab_case")]
pub enum CozySimRunner {
    Base,
}

/// Runs Cozy Simulation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CozySimSettings {
    /// Specifies the highest log level to use
    #[clap(short, long)]
    log_level: String,
    /// Specifies the sim type to run
    #[clap(short, long, arg_enum)]
    sim: Option<CozySimRunner>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let settings = CozySimSettings::parse();

    let log_level = settings.log_level;
    Logger::try_with_str(log_level)?
        .log_to_stdout()
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    match settings.sim {
        Some(CozySimRunner::Base) => cozy::analysis::base::run()?,
        None => cozy::analysis::base::run()?,
    }

    Ok(())
}
