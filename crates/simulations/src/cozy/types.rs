use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
use serde::{Deserialize, Deserializer};
use simulate::address::Address;

use crate::cozy::{
    distributions::{Exponential, TimeUnit, TriggerProbModel, U256UniformRange},
    EthersU256,
};

pub fn deserialize_string_to_u256<'de, D>(deserializer: D) -> Result<EthersU256, D::Error>
where
    D: Deserializer<'de>,
{
    let string_value: String = serde::Deserialize::deserialize(deserializer)?;
    let u256_value: EthersU256 =
        EthersU256::from_dec_str(string_value.as_str()).map_err(serde::de::Error::custom)?;
    Ok(u256_value)
}

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

impl Default for CozyTokenDeployParams {
    fn default() -> Self {
        CozyTokenDeployParams {
            name: "Cozy Base Token".into(),
            symbol: "CBT".into(),
            decimals: 6,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(remote = "Delays")]
pub struct CozyDelays {
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
    pub allowed_markets_per_set: EthersU256,
}

impl Default for CozyProtocolDeployParams {
    fn default() -> Self {
        CozyProtocolDeployParams {
            delays: Delays {
                config_update_delay: 172_800.into(),
                config_update_grace_period: 259_200.into(),
                min_deposit_duration: 86_400.into(),
                redemption_delay: 43_200.into(),
                purchase_delay: 57_600.into(),
            },
            fees: Fees {
                deposit_fee_reserves: 0,
                deposit_fee_backstop: 0,
                purchase_fee_reserves: 0,
                purchase_fee_backstop: 0,
                sale_fee_reserves: 0,
                sale_fee_backstop: 0,
            },
            allowed_markets_per_set: 50.into(),
        }
    }
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

impl Default for CozyFixedTimePolicyParams {
    fn default() -> Self {
        CozyFixedTimePolicyParams {
            start_block_number: 1,
            start_block_timestamp: 1,
            time_per_block: 60,
            blocks_per_step: 10,
            blocks_to_generate: Some(500_000),
            time_to_generate: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySimSetupParams {
    pub rand_seed: u64,
}

impl Default for CozySimSetupParams {
    fn default() -> Self {
        CozySimSetupParams { rand_seed: 1 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyPassiveBuyersParams {
    pub num_passive: u64,
    pub capital_dist: U256UniformRange,
    pub protection_desired_dist: U256UniformRange,
    pub time_dist: Exponential,
}

impl Default for CozyPassiveBuyersParams {
    fn default() -> Self {
        CozyPassiveBuyersParams {
            num_passive: 0,
            capital_dist: U256UniformRange {
                min: (1_000_000 as i64).into(),
                max: (2_000_000 as i64).into(),
            },
            protection_desired_dist: U256UniformRange {
                min: (1_000_000 as i64).into(),
                max: (2_000_000 as i64).into(),
            },
            time_dist: Exponential {
                rate: 10.0,
                time_unit: TimeUnit::Day,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyActiveBuyersParams {
    pub num_active: u64,
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
}

impl Default for CozyActiveBuyersParams {
    fn default() -> Self {
        CozyActiveBuyersParams {
            num_active: 1,
            capital_dist: U256UniformRange {
                min: (1_000_000 as i64).into(),
                max: (2_000_000 as i64).into(),
            },
            time_dist: Exponential {
                rate: 1.0,
                time_unit: TimeUnit::Day,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySuppliersParams {
    pub num_passive: u64,
    pub capital_dist: U256UniformRange,
    pub time_dist: Exponential,
}

impl Default for CozySuppliersParams {
    fn default() -> Self {
        CozySuppliersParams {
            num_passive: 1,
            capital_dist: U256UniformRange {
                min: (1_000_000_000 as i64).into(),
                max: (2_000_000_000 as i64).into(),
            },
            time_dist: Exponential {
                rate: 1.0,
                time_unit: TimeUnit::Day,
            },
        }
    }
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

impl Default for CozySetConfigParams {
    fn default() -> Self {
        CozySetConfigParams {
            leverage_factor: 10_000,
            deposit_fee: 0,
        }
    }
}

impl Into<SetConfig> for CozySetConfigParams {
    fn into(self) -> SetConfig {
        SetConfig {
            leverage_factor: self.leverage_factor,
            deposit_fee: self.deposit_fee,
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
