use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
<<<<<<< HEAD
use rand::Rng;
=======
use simulate::{address::Address, u256::U256};
>>>>>>> 1f1e355 (init refac to custom u256 type)
use serde::Deserialize;
use simulate::address::Address;

use super::distributions::LinkedProbTruncatedNorm;
use crate::cozy::{
<<<<<<< HEAD
    distributions::{Exponential, ProbTruncatedNorm, TriggerProbModel, U256UniformRange},
    utils::deserialize_string_to_u256,
    EthersU256,
=======
    distributions::{Exponential, TimeUnit, UniformRange}
>>>>>>> 1f1e355 (init refac to custom u256 type)
};

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "cost_model_jump_rate_factory::DeployModelCall")]
pub struct CozyJumpRateDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub kink: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_zero_utilization: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_kink_utilization: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_full_utilization: EthersU256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "cost_model_dynamic_level_factory::DeployModelCall")]
pub struct CozyDynamicLevelDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub u_low: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub u_high: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_zero_utilization: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_at_full_utilization: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub cost_factor_in_optimal_zone: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub optimal_zone_rate: EthersU256,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "drip_decay_model_constant_factory::DeployModelCall")]
pub struct CozyDripDecayConstantDeployModelCall {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub rate_per_second: EthersU256,
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
<<<<<<< HEAD
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub config_update_delay: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub config_update_grace_period: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub min_deposit_duration: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub redemption_delay: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub purchase_delay: EthersU256,
=======
    pub config_update_delay: U256,
    pub config_update_grace_period: U256,
    pub min_deposit_duration: U256,
    pub redemption_delay: U256,
    pub purchase_delay: U256,
>>>>>>> 1f1e355 (init refac to custom u256 type)
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
<<<<<<< HEAD
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub allowed_markets_per_set: EthersU256,
=======
    pub allowed_markets_per_set: U256,
>>>>>>> 1f1e355 (init refac to custom u256 type)
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyFixedTimePolicyParams {
<<<<<<< HEAD
    pub start_block_number: u64,
    pub start_block_timestamp: u64,
=======
    pub start_block_number: U256,
    pub start_block_timestamp: U256,
>>>>>>> 1f1e355 (init refac to custom u256 type)
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
<<<<<<< HEAD
pub struct CozyActiveBuyersParams {
    pub num_active: u64,
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
    pub trigger_prob_dist: CozyAgentTriggerProbModel,
=======
pub struct CozyBuyersParams {
    pub num_passive: u64,
    pub capital_dist: UniformRange<U256>,
    pub protection_desired_dist: UniformRange<U256>,
    pub time_dist: Exponential,
}

impl Default for CozyBuyersParams {
    fn default() -> Self {
        CozyBuyersParams {
            num_passive: 100,
            capital_dist: UniformRange::<U256> {
                min: (1_000_000 as u64).into(),
                max: (2_000_000 as u64).into(),
            },
            protection_desired_dist: UniformRange::<U256> {
                min: (1_000_000 as u64).into(),
                max: (2_000_000 as u64).into(),
            },
            time_dist: Exponential {
                rate: 1.0,
                time_unit: TimeUnit::Day,
            },
        }
    }
>>>>>>> 1f1e355 (init refac to custom u256 type)
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySuppliersParams {
    pub num_passive: u64,
<<<<<<< HEAD
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
}

=======
    pub capital_dist: UniformRange<U256>,
    pub time_dist: Exponential,
}

impl Default for CozySuppliersParams {
    fn default() -> Self {
        CozySuppliersParams {
            num_passive: 5,
            capital_dist: UniformRange::<U256> {
                min: (1_000_000 as u64).into(),
                max: (2_000_000 as u64).into(),
            },
            time_dist: Exponential {
                rate: 1.0,
                time_unit: TimeUnit::Day,
            },
        }
    }
}

>>>>>>> 1f1e355 (init refac to custom u256 type)
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
