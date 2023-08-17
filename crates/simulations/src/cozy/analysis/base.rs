use crate::cozy::{
    configs::build_cozy_sim_runner_from_dir, runner::CozySingleSetSummaryGenerators,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let runner = build_cozy_sim_runner_from_dir("base")?;

    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let output_file_name = format!(
        "{}/output/summaries/{}_summary.json",
        workspace_path,
        chrono::Utc::now().to_rfc3339()
    );
    runner.run(
        output_file_name.into(),
        vec![
            CozySingleSetSummaryGenerators::Set,
            CozySingleSetSummaryGenerators::CostModels,
        ],
    )?;

    Ok(())
}
