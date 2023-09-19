use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
use nalgebra::{DMatrix, DVector};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Dirichlet, Distribution};
use serde::Deserialize;
use simulate::{
    address::Address,
    u256::{deserialize_string_to_u256, U256},
};
use statrs::distribution::Beta;

use super::{
    distributions::LinkedProbTruncatedNorm,
    statistics::{
        mvbernoulli::MultivariateBernoulli, mvbeta::MultivariateBeta, wishart::WishartCorrelation,
    },
};
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
pub struct FixedTimePolicyParams {
    pub start_block_number: u64,
    pub start_block_timestamp: u64,
    pub time_per_block: u64,
    pub blocks_per_step: u64,
    pub time_to_generate: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozySimSetupParams {
    pub rand_seed: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArbitrageurParams {
    pub num: u64,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub balance_mean: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub balance_std: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BuyerParams {
    pub num: u64,
    pub market_allocations_dirichlet_alpha: f64,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub balance_mean: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub balance_std: U256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupplierParams {
    pub num: u64,
    pub risk_aversion_mean: f64,
    pub risk_aversion_concentration: f64,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub wealth_mean: U256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub wealth_std: U256,
    pub altruistic_supplier_wealth: U256,
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

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRiskParams {
    pub annual_probabilities: Vec<f64>,
    pub pairwise_corr: f64,
}

#[derive(Debug, Clone)]
pub struct TriggerSimulator {
    rng: rand::rngs::StdRng,
    pub mvb: MultivariateBernoulli,
}

impl TriggerSimulator {
    pub fn new(rng: StdRng, probabilities: Vec<f64>, correlation: f64) -> Self {
        let n = probabilities.len();
        let corr_matrix =
            nalgebra::DMatrix::from_fn(n, n, |i, j| if i == j { 1.0 } else { correlation })
                .as_slice()
                .to_vec();
        let mvb = MultivariateBernoulli::new(probabilities, corr_matrix)
            .expect("Error creating multivariate Bernoulli.");
        Self { rng, mvb }
    }

    pub fn sample(&mut self) -> DVector<f64> {
        self.mvb.sample(&mut self.rng)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentSetRiskParams {
    pub annual_probabilities_concentration: f64,
    pub wishart_corr_df: f64,
}

#[derive(Debug, Clone)]
pub struct AgentSetRiskSampler {
    rng: rand::rngs::StdRng,
    mv_beta: MultivariateBeta,
    wishart_corr: WishartCorrelation,
}

impl AgentSetRiskSampler {
    pub fn new(
        rng: StdRng,
        annual_probabilities: DVector<f64>,
        corr_matrix: DMatrix<f64>,
        annual_probabilities_concentration: f64,
        wishart_corr_df: f64,
    ) -> Self {
        let mv_beta = MultivariateBeta::new(
            annual_probabilities.as_slice().to_vec(),
            annual_probabilities_concentration,
        )
        .expect("Error creating multivariate Beta.");
        let wishart_corr = WishartCorrelation::new(wishart_corr_df, corr_matrix)
            .expect("Error creating Wishart correlation.");
        Self {
            rng,
            mv_beta,
            wishart_corr,
        }
    }

    pub fn sample(&mut self) -> (DVector<f64>, DMatrix<f64>) {
        (
            self.mv_beta.sample(&mut self.rng),
            self.wishart_corr.sample(&mut self.rng),
        )
    }
}

#[derive(Debug, Clone)]
pub struct SupplierRiskAversionSampler {
    rng: rand::rngs::StdRng,
    beta: Beta,
}

impl SupplierRiskAversionSampler {
    pub fn new(rng: StdRng, mean: f64, concentration: f64) -> Self {
        let beta = Beta::new(mean * concentration, (1. - mean) * concentration).unwrap();
        Self { rng, beta }
    }

    pub fn sample(&mut self) -> f64 {
        self.beta.sample(&mut self.rng)
    }
}
