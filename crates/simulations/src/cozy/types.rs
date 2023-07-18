use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
use rand::Rng;
use serde::Deserialize;
use simulate::{
    address::Address,
    u256::{deserialize_string_to_u256, U256},
};

use super::distributions::LinkedProbTruncatedNorm;
use crate::cozy::distributions::{
    Exponential, ProbTruncatedNorm, TriggerProbModel, U256UniformRange,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "cost_model_jump_rate_factory::DeployModelCall")]
pub struct CozyJumpRateDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub kink: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_zero_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_kink_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_full_utilization: U256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "cost_model_dynamic_level_factory::DeployModelCall")]
pub struct CozyDynamicLevelDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub u_low: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub u_high: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_zero_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_full_utilization: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_in_optimal_zone: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub optimal_zone_rate: U256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "drip_decay_model_constant_factory::DeployModelCall")]
pub struct CozyDripDecayConstantDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub rate_per_second: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub enum CozyCostModelType {
    #[serde(with = "CozyJumpRateDeployModelCall")]
    JumpRate(cost_model_jump_rate_factory::DeployModelCall),
    #[serde(with = "CozyDynamicLevelDeployModelCall")]
    DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall),
}

#[derive(Debug, Clone, Deserialize)]
pub enum CozyDripDecayModelType {
    #[serde(with = "CozyDripDecayConstantDeployModelCall")]
    Constant(drip_decay_model_constant_factory::DeployModelCall),
}

#[derive(Debug, Clone, Deserialize)]
pub enum CozyTriggerType {
    DummyTrigger(TriggerProbModel),
    UmaTrigger,
    ChainlinkTrigger,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyTokenDeployParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "Delays")]
pub struct CozyDelays {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub config_update_delay: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub config_update_grace_period: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub min_deposit_duration: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub redemption_delay: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub purchase_delay: U256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "Fees")]
pub struct CozyFees {
    pub deposit_fee_reserves: u16,
    pub deposit_fee_backstop: u16,
    pub purchase_fee_reserves: u16,
    pub purchase_fee_backstop: u16,
    pub sale_fee_reserves: u16,
    pub sale_fee_backstop: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyProtocolDeployParams {
    #[serde(with = "CozyDelays")]
    pub delays: Delays,
    #[serde(with = "CozyFees")]
    pub fees: Fees,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub allowed_markets_per_set: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyFixedTimePolicyParams {
    pub start_block_number: u64,
    pub start_block_timestamp: u64,
    pub time_per_block: u64,
    pub blocks_per_step: u64,
    pub blocks_to_generate: Option<u64>,
    pub time_to_generate: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySimSetupParams {
    pub rand_seed: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyPassiveBuyersParams {
    pub num_passive: u64,
    pub capital_dist: U256UniformRange,
    pub protection_desired_dist: U256UniformRange,
    pub time_dist: Exponential,
}

#[derive(Debug, Clone, Deserialize)]
pub enum CozyAgentTriggerProbModel {
    Static(ProbTruncatedNorm),
    Dynamic(LinkedProbTruncatedNorm),
}

impl CozyAgentTriggerProbModel {
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R, oracle_prob: f64) -> f64 {
        match self {
            CozyAgentTriggerProbModel::Static(dist) => dist.sample(rng),
            CozyAgentTriggerProbModel::Dynamic(dist) => dist.sample(oracle_prob, rng),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyActiveBuyersParams {
    pub num_active: u64,
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
    pub trigger_prob_dist: CozyAgentTriggerProbModel,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySuppliersParams {
    pub num_passive: u64,
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyMarketConfigParams {
    pub weight: u16,
    pub purchase_fee: u16,
    pub sale_fee: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySetConfigParams {
    pub leverage_factor: u32,
    pub deposit_fee: u16,
}

impl From<CozySetConfigParams> for SetConfig {
    fn from(val: CozySetConfigParams) -> Self {
        SetConfig {
            leverage_factor: val.leverage_factor,
            deposit_fee: val.deposit_fee,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CozySetAdminParams {
    pub asset: Address,
    pub set_config: SetConfig,
    pub market_configs: Vec<MarketConfig>,
    pub salt: Option<[u8; 32]>,
}
