#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;
pub mod cozy;

use clap::Parser;
use flexi_logger::{Duplicate, FileSpec, Logger};

/// Runs Cozy Simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specifies the highest log level to use
    #[arg(short, long)]
    log_level: String,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let log_level = args.log_level;

    Logger::try_with_str(log_level)?
        .log_to_stdout()
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    crate::cozy::run()
}
