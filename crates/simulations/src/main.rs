#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::{borrow::Cow, error::Error};

use chrono;
use clap::Parser;
use flexi_logger::{Duplicate, Logger};

pub mod cozy;

/// Runs Cozy Simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specifies the highest log level to use
    #[arg(short, long)]
    log_level: String,
    #[arg(short, long)]
    output_file: Option<String>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let log_level = args.log_level;
    let output_file_name = format!(
        "output/summaries/{}_summary.txt",
        chrono::Utc::now().to_rfc3339()
    );

    Logger::try_with_str(log_level)?
        .log_to_stdout()
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    let settings = crate::cozy::configs::build_cozy_sim_settings_from_dir("cost_models_analysis")?;
    let runner = crate::cozy::runner::CozySingleSetSimRunner::new(settings);
    runner.run(output_file_name.into());

    Ok(())
}
