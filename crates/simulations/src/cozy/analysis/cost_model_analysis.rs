use crate::cozy::configs::build_cozy_sim_runner_from_dir;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let jump_rate_runner = build_cozy_sim_runner_from_dir("cost_model_analysis/jump_rate_model")?;
    let dynamic_level_runner =
        build_cozy_sim_runner_from_dir("cost_model_analysis/dynamic_level_model")?;

    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .unwrap();
    let jump_rate_output_file_name =
        format!("{}/output/summaries/jump_rate_summary.json", workspace_path,);
    let dynamic_level_output_file_name = format!(
        "{}/output/summaries/dynamic_level_summary.json",
        workspace_path,
    );

    jump_rate_runner.run(jump_rate_output_file_name.into());
    dynamic_level_runner.run(dynamic_level_output_file_name.into());

    Ok(())
}
