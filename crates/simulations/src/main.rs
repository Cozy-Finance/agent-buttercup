use std::error::Error;

use clap::Parser;
use flexi_logger::{Duplicate, Logger};

pub mod cozy;

pub enum Sim {
    Base,
    CostModelAnalysis,
}

impl Sim {
    pub fn map_from_str(name: &str) -> Self {
        match name {
            "base" => Sim::Base,
            "cost_model_analysis" => Sim::CostModelAnalysis,
            _ => panic!("Unknown simulation: {}", name),
        }
    }
}

/// Runs Cozy Simulation
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Specifies the highest log level to use
    #[arg(short, long)]
    log_level: String,
    /// Specifies the sim type to run
    #[arg(short, long)]
    sim: Option<String>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let log_level = args.log_level;
    Logger::try_with_str(log_level)?
        .log_to_stdout()
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    let sim = args.sim.as_deref().map_or(Sim::Base, Sim::map_from_str);

    match sim {
        Sim::Base => cozy::analysis::base::run()?,
        Sim::CostModelAnalysis => cozy::analysis::cost_model_analysis::run()?,
    }

    Ok(())
}
