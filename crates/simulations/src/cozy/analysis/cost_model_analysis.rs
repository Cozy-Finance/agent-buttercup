use bindings::{cost_model_dynamic_level_factory, cost_model_jump_rate_factory};
use serde::Deserialize;
use simulate::u256::{deserialize_string_to_u256, U256};

use crate::cozy::{
    configs::build_cozy_sim_runner_from_dir, distributions::ProbTruncatedNorm,
    runner::CozySingleSetSummaryGenerators, types::*, utils::wad_to_float,
};

pub fn build_cost_model_analysis_config_from_file(
    file: &str,
) -> Result<config::Config, anyhow::Error> {
    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let all_configs_dir = workspace_dir.join("src/cozy/configs/cost_model_analysis");
    let config_builder =
        config::Config::builder().add_source(config::File::from(all_configs_dir.join(file)));
    Ok(config_builder.build()?)
}

#[derive(Debug, Deserialize)]
pub struct MisspecificationAnalysis {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    kink: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    min_kink_cost: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    max_kink_cost: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_full_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_zero_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    optimal_zone_rate: U256,
    agent_std: f64,
    num_runs_per_setting: i32,
    num_steps: i32,
}

pub fn run_misspecification_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let jump_rate_runner = build_cozy_sim_runner_from_dir("cost_model_analysis/jump_rate_model")?;
    let dynamic_level_runner =
        build_cozy_sim_runner_from_dir("cost_model_analysis/dynamic_level_model")?;
    let misspecification_config =
        build_cost_model_analysis_config_from_file("misspecification_analysis.yaml")?
            .try_deserialize::<MisspecificationAnalysis>()?;

    let active_agent_prob = wad_to_float(
        (misspecification_config.min_kink_cost + misspecification_config.max_kink_cost) / 2,
    );
    let step_size = (misspecification_config.max_kink_cost - misspecification_config.min_kink_cost)
        / misspecification_config.num_steps;
    let mut kink_cost = misspecification_config.min_kink_cost;

    while kink_cost <= misspecification_config.max_kink_cost {
        let mut jump_rate_runner_mut = jump_rate_runner.clone();
        let mut dynamic_level_runner_mut = dynamic_level_runner.clone();

        jump_rate_runner_mut.cost_models = vec![(
            "JumpRate".into(),
            CozyCostModelType::JumpRate(cost_model_jump_rate_factory::DeployModelCall {
                kink: misspecification_config.kink,
                cost_factor_at_full_utilization: misspecification_config
                    .cost_factor_at_full_utilization,
                cost_factor_at_zero_utilization: misspecification_config
                    .cost_factor_at_zero_utilization,
                cost_factor_at_kink_utilization: kink_cost,
            }),
        )];
        dynamic_level_runner_mut.cost_models = vec![(
            "DynamicLevel".into(),
            CozyCostModelType::DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall {
                u_low: misspecification_config.kink,
                u_high: misspecification_config.kink,
                cost_factor_at_full_utilization: misspecification_config
                    .cost_factor_at_full_utilization,
                cost_factor_at_zero_utilization: misspecification_config
                    .cost_factor_at_zero_utilization,
                cost_factor_in_optimal_zone: kink_cost,
                optimal_zone_rate: misspecification_config.optimal_zone_rate,
            }),
        )];

        jump_rate_runner_mut.active_buyers_params.trigger_prob_dist =
            CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
                active_agent_prob,
                misspecification_config.agent_std,
            ));
        dynamic_level_runner_mut
            .active_buyers_params
            .trigger_prob_dist = CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            active_agent_prob,
            misspecification_config.agent_std,
        ));

        jump_rate_runner_mut.passive_buyers_params.num_passive = 0;
        dynamic_level_runner_mut.passive_buyers_params.num_passive = 0;

        for i in 0..misspecification_config.num_runs_per_setting {
            let jump_rate_output_file_name = format!(
                "{}/output/summaries/misspecification_analysis/jump_rate_summary_{}_{}.json",
                workspace_path, kink_cost, i
            );
            let dynamic_level_output_file_name = format!(
                "{}/output/summaries/misspecification_analysis/dynamic_level_summary_{}_{}.json",
                workspace_path, kink_cost, i
            );
            jump_rate_runner_mut.clone().run(
                jump_rate_output_file_name.into(),
                vec![
                    CozySingleSetSummaryGenerators::Set,
                    CozySingleSetSummaryGenerators::CostModels,
                ],
            )?;
            dynamic_level_runner_mut.clone().run(
                dynamic_level_output_file_name.into(),
                vec![
                    CozySingleSetSummaryGenerators::Set,
                    CozySingleSetSummaryGenerators::CostModels,
                ],
            )?;
        }

        kink_cost += step_size;
    }

    Ok(())
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    run_misspecification_analysis()?;
    Ok(())
}
