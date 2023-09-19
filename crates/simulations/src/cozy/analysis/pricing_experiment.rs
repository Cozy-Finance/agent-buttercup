use std::borrow::Cow;

use bindings::{cost_model_dynamic_level_factory, drip_decay_model_constant_factory};
use serde::Deserialize;
use simulate::u256::{deserialize_string_to_u256, U256};

use crate::cozy::{
    configs::build_cozy_sim_runner_from_dir,
    distributions::ProbTruncatedNorm,
    runner::CozySingleSetSummaryGenerators,
    types::*,
    utils::{deserialize_cow_tuple_vec, wad},
};

pub fn build_pricing_experiment_analysis_config_from_file(
    file: &str,
) -> Result<config::Config, anyhow::Error> {
    let workspace_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let all_configs_dir = workspace_dir.join("src/cozy/configs/pricing_experiment");
    let config_builder =
        config::Config::builder().add_source(config::File::from(all_configs_dir.join(file)));
    Ok(config_builder.build()?)
}

#[derive(Debug, Deserialize)]
pub struct KinkLocationAnalysis {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_zero_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_full_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    min_u_opt: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    max_u_opt: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    width: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    rate_cost_factor_delta: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    rate_time_delta_in_seconds: U256,
    agent_mean: f64,
    num_steps: i32,
}

#[derive(Debug, Deserialize)]
pub struct WidthAnalysis {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_zero_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    cost_factor_at_full_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    u_opt: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    min_width: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    max_width: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    rate_cost_factor_delta: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    rate_time_delta_in_seconds: U256,
    agent_mean: f64,
    num_steps: i32,
}

#[derive(Debug, Deserialize)]
pub struct ScenarioAnalysis {
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    agent_low_demand_mean: f64,
    agent_high_demand_mean: f64,
    agent_std: f64,
}

#[derive(Debug, Deserialize)]
pub struct DecayAnalysis {
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    agent_mean: f64,
    agent_std: f64,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    min_rate_per_second: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    max_rate_per_second: U256,
    num_steps: i32,
}

#[derive(Debug, Deserialize)]
pub struct GrowthRateAnalysis {
    #[serde(deserialize_with = "deserialize_cow_tuple_vec")]
    cost_models: Vec<(Cow<'static, str>, CozyCostModelType)>,
    agent_mean: f64,
    agent_std: f64,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    min_rate_cost_factor_delta: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    max_rate_cost_factor_delta: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    rate_time_delta_in_seconds: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    u_opt: U256,
    num_steps: i32,
}

pub fn run_kink_location_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let dynamic_level_runner = build_cozy_sim_runner_from_dir("pricing_experiment/base")?;
    let kink_location_config =
        build_pricing_experiment_analysis_config_from_file("kink_location_analysis.yaml")?
            .try_deserialize::<KinkLocationAnalysis>()?;

    let step_size = (kink_location_config.max_u_opt - kink_location_config.min_u_opt)
        / kink_location_config.num_steps;
    let mut u_opt = kink_location_config.min_u_opt;
    let optimal_zone_rate = (kink_location_config.rate_cost_factor_delta * wad())
        / (kink_location_config.width * kink_location_config.rate_time_delta_in_seconds);

    while u_opt <= kink_location_config.max_u_opt {
        let mut dynamic_level_runner_mut = dynamic_level_runner.clone();

        dynamic_level_runner_mut.cost_models = vec![(
            "DynamicLevel".into(),
            CozyCostModelType::DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall {
                u_low: u_opt - kink_location_config.width,
                u_high: u_opt + kink_location_config.width,
                cost_factor_at_full_utilization: kink_location_config
                    .cost_factor_at_full_utilization,
                cost_factor_at_zero_utilization: kink_location_config
                    .cost_factor_at_zero_utilization,
                cost_factor_in_optimal_zone: kink_location_config.cost_factor_at_zero_utilization,
                optimal_zone_rate,
            }),
        )];

        dynamic_level_runner_mut
            .active_buyers_params
            .trigger_prob_dist = CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            kink_location_config.agent_mean,
            0.0,
        ));

        let dynamic_level_output_file_name = format!(
            "{}/output/summaries/kink_location_analysis/u_opt_{}.json",
            workspace_path, u_opt,
        );
        dynamic_level_runner_mut.clone().run(
            dynamic_level_output_file_name.into(),
            vec![
                CozySingleSetSummaryGenerators::Set,
                CozySingleSetSummaryGenerators::PricingExperiment,
            ],
        )?;

        u_opt += step_size;
    }

    Ok(())
}

pub fn run_width_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let dynamic_level_runner = build_cozy_sim_runner_from_dir("pricing_experiment/base")?;
    let width_config = build_pricing_experiment_analysis_config_from_file("width_analysis.yaml")?
        .try_deserialize::<WidthAnalysis>()?;

    let step_size = (width_config.max_width - width_config.min_width) / width_config.num_steps;
    let mut width = width_config.min_width;

    while width <= width_config.max_width {
        let mut dynamic_level_runner_mut = dynamic_level_runner.clone();
        let u_low = (width_config.u_opt - width).max(U256::zero());
        let u_high = (width_config.u_opt + width).min(wad());
        let optimal_zone_rate = (width_config.rate_cost_factor_delta * wad())
            / (width * width_config.rate_time_delta_in_seconds);

        dynamic_level_runner_mut.cost_models = vec![(
            "DynamicLevel".into(),
            CozyCostModelType::DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall {
                u_low,
                u_high,
                cost_factor_at_full_utilization: width_config.cost_factor_at_full_utilization,
                cost_factor_at_zero_utilization: width_config.cost_factor_at_zero_utilization,
                cost_factor_in_optimal_zone: width_config.cost_factor_at_zero_utilization,
                optimal_zone_rate,
            }),
        )];

        dynamic_level_runner_mut
            .active_buyers_params
            .trigger_prob_dist =
            CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(width_config.agent_mean, 0.0));

        let dynamic_level_output_file_name = format!(
            "{}/output/summaries/width_analysis/width_{}.json",
            workspace_path, width,
        );
        dynamic_level_runner_mut.clone().run(
            dynamic_level_output_file_name.into(),
            vec![
                CozySingleSetSummaryGenerators::Set,
                CozySingleSetSummaryGenerators::PricingExperiment,
            ],
        )?;

        width += step_size;
    }

    Ok(())
}

pub fn run_scenarios() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let dynamic_level_runner = build_cozy_sim_runner_from_dir("pricing_experiment/base")?;
    let scenarios_config =
        build_pricing_experiment_analysis_config_from_file("demand_scenarios_analysis.yaml")?
            .try_deserialize::<ScenarioAnalysis>()?;

    let mut low_demand_runner = dynamic_level_runner.clone();
    let mut high_demand_runner = dynamic_level_runner.clone();
    low_demand_runner.cost_models = scenarios_config.cost_models.clone();
    high_demand_runner.cost_models = scenarios_config.cost_models;

    low_demand_runner.active_buyers_params.trigger_prob_dist =
        CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            scenarios_config.agent_low_demand_mean,
            scenarios_config.agent_std,
        ));
    high_demand_runner.active_buyers_params.trigger_prob_dist =
        CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            scenarios_config.agent_high_demand_mean,
            scenarios_config.agent_std,
        ));

    low_demand_runner.run(
        format!(
            "{}/output/summaries/scenarios_analysis/low_demand.json",
            workspace_path
        )
        .into(),
        vec![
            CozySingleSetSummaryGenerators::Set,
            CozySingleSetSummaryGenerators::PricingExperiment,
        ],
    )?;
    high_demand_runner.run(
        format!(
            "{}/output/summaries/scenarios_analysis/high_demand.json",
            workspace_path
        )
        .into(),
        vec![
            CozySingleSetSummaryGenerators::Set,
            CozySingleSetSummaryGenerators::PricingExperiment,
        ],
    )?;

    Ok(())
}

pub fn run_decay_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let dynamic_level_runner = build_cozy_sim_runner_from_dir("pricing_experiment/base")?;
    let decay_config = build_pricing_experiment_analysis_config_from_file("decay_analysis.yaml")?
        .try_deserialize::<DecayAnalysis>()?;

    let step_size = (decay_config.max_rate_per_second - decay_config.min_rate_per_second)
        / decay_config.num_steps;
    let mut decay_param = decay_config.min_rate_per_second;

    while decay_param <= decay_config.max_rate_per_second {
        let mut dynamic_level_runner_mut = dynamic_level_runner.clone();
        dynamic_level_runner_mut.cost_models = decay_config.cost_models.clone();
        dynamic_level_runner_mut.drip_decay_models = vec![(
            "DripDecayModel".into(),
            CozyDripDecayModelType::Constant(drip_decay_model_constant_factory::DeployModelCall {
                rate_per_second: decay_param,
            }),
        )];
        dynamic_level_runner_mut
            .active_buyers_params
            .trigger_prob_dist = CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            decay_config.agent_mean,
            decay_config.agent_std,
        ));

        let dynamic_level_output_file_name = format!(
            "{}/output/summaries/decay_analysis/decay_{}.json",
            workspace_path, decay_param,
        );
        dynamic_level_runner_mut.clone().run(
            dynamic_level_output_file_name.into(),
            vec![
                CozySingleSetSummaryGenerators::Set,
                CozySingleSetSummaryGenerators::PricingExperiment,
            ],
        )?;

        decay_param += step_size;
    }

    Ok(())
}

pub fn run_growth_rate_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .to_str()
        .expect("CARGO_MANIFEST_DIR str.");
    let dynamic_level_runner = build_cozy_sim_runner_from_dir("pricing_experiment/base")?;
    let growth_rate_config =
        build_pricing_experiment_analysis_config_from_file("growth_rate_analysis.yaml")?
            .try_deserialize::<GrowthRateAnalysis>()?;

    let step_size = (growth_rate_config.max_rate_cost_factor_delta
        - growth_rate_config.min_rate_cost_factor_delta)
        / growth_rate_config.num_steps;
    let mut cost_factor_delta = growth_rate_config.min_rate_cost_factor_delta;
    let (base_cost_model_name, base_cost_model) = match &growth_rate_config.cost_models[0] {
        (name, CozyCostModelType::DynamicLevel(cost_model)) => (name, cost_model),
        _ => panic!("Expected DynamicLevel cost model"),
    };

    while cost_factor_delta <= growth_rate_config.max_rate_cost_factor_delta {
        let mut dynamic_level_runner_mut = dynamic_level_runner.clone();
        let optimal_zone_rate = (cost_factor_delta * wad())
            / (growth_rate_config.u_opt * growth_rate_config.rate_time_delta_in_seconds);

        dynamic_level_runner_mut.cost_models = vec![(
            base_cost_model_name.clone(),
            CozyCostModelType::DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall {
                u_low: base_cost_model.u_low,
                u_high: base_cost_model.u_high,
                cost_factor_at_full_utilization: base_cost_model.cost_factor_at_full_utilization,
                cost_factor_at_zero_utilization: base_cost_model.cost_factor_at_zero_utilization,
                cost_factor_in_optimal_zone: base_cost_model.cost_factor_at_zero_utilization,
                optimal_zone_rate,
            }),
        )];

        dynamic_level_runner_mut
            .active_buyers_params
            .trigger_prob_dist = CozyAgentTriggerProbModel::Static(ProbTruncatedNorm::new(
            growth_rate_config.agent_mean,
            growth_rate_config.agent_std,
        ));

        let dynamic_level_output_file_name = format!(
            "{}/output/summaries/growth_rate_analysis/rate_{}.json",
            workspace_path, optimal_zone_rate,
        );
        dynamic_level_runner_mut.clone().run(
            dynamic_level_output_file_name.into(),
            vec![
                CozySingleSetSummaryGenerators::Set,
                CozySingleSetSummaryGenerators::PricingExperiment,
            ],
        )?;

        cost_factor_delta += step_size;
    }

    Ok(())
}
