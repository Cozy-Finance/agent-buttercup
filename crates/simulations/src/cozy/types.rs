pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig},
    drip_decay_model_constant_factory,
};
use ethers::types::{Address as EthersAddress, U256 as EthersU256};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum CozyCostModelType {
    JumpRate(cost_model_jump_rate_factory::DeployModelCall),
    DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall),
}

#[derive(Debug, Clone)]
pub enum CozyDripDecayModelType {
    Constant(drip_decay_model_constant_factory::DeployModelCall),
}

#[derive(Debug, Clone)]
pub enum CozyTriggerType {
    DummyTrigger,
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
    pub config_update_delay: EthersU256,
    pub config_update_grace_period: EthersU256,
    pub min_deposit_duration: EthersU256,
    pub redemption_delay: EthersU256,
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
    pub allowed_markets_per_set: EthersU256,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CozyFixedTimePolicyParams {
    pub start_block_number: EthersU256,
    pub start_block_timestamp: EthersU256,
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
pub struct CozySimBuyersSuppliers {
    pub num_passive_buyers: u64,
    pub num_passive_suppliers: u64,
    pub passive_buyer_capital: EthersU256,
    pub passive_supplier_capital: EthersU256,
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
    pub asset: EthersAddress,
    pub set_config: SetConfig,
    pub market_configs: Vec<MarketConfig>,
    pub salt: Option<[u8; 32]>,
}
