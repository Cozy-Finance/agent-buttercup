use std::error::Error;

use clap::Parser;
use flexi_logger::{Duplicate, Logger};

pub mod cozy;

pub enum Sim {
    Base,
    CostModelAnalysis,
    KinkCostAnalysis,
    GrowthRateAnalysis,
    WidthAnalysis,
    ScenarioAnalysis,
}

impl Sim {
    pub fn map_from_str(name: &str) -> Self {
        match name {
            "base" => Sim::Base,
            "cost_model_analysis" => Sim::CostModelAnalysis,
            "kink_cost_analysis" => Sim::KinkCostAnalysis,
            "growth_rate_analysis" => Sim::GrowthRateAnalysis,
            "width_analysis" => Sim::WidthAnalysis,
            "scenario_analysis" => Sim::ScenarioAnalysis,
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
        Sim::KinkCostAnalysis => cozy::analysis::pricing_experiment::run_kink_location_analysis()?,
        Sim::GrowthRateAnalysis => cozy::analysis::pricing_experiment::run_growth_rate_analysis()?,
        Sim::WidthAnalysis => cozy::analysis::pricing_experiment::run_width_analysis()?,
        Sim::ScenarioAnalysis => cozy::analysis::pricing_experiment::run_scenarios()?,
    }

    Ok(())
}
