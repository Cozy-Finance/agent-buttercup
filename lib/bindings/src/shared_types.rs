///`BackstopApproval(address,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct BackstopApproval {
    pub set: ::ethers::core::types::Address,
    pub status: bool,
}
///`Delays(uint256,uint256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Delays {
    pub config_update_delay: ::ethers::core::types::U256,
    pub config_update_grace_period: ::ethers::core::types::U256,
    pub min_deposit_duration: ::ethers::core::types::U256,
    pub redemption_delay: ::ethers::core::types::U256,
    pub purchase_delay: ::ethers::core::types::U256,
}
///`DepositFeesAssets(uint128,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct DepositFeesAssets {
    pub reserve_fee_assets: u128,
    pub backstop_fee_assets: u128,
    pub set_owner_fee_assets: u128,
}
///`Fees(uint16,uint16,uint16,uint16,uint16,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Fees {
    pub deposit_fee_reserves: u16,
    pub deposit_fee_backstop: u16,
    pub purchase_fee_reserves: u16,
    pub purchase_fee_backstop: u16,
    pub sale_fee_reserves: u16,
    pub sale_fee_backstop: u16,
}
///`MarketConfig(address,address,address,uint16,uint16,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MarketConfig {
    pub trigger: ::ethers::core::types::Address,
    pub cost_model: ::ethers::core::types::Address,
    pub drip_decay_model: ::ethers::core::types::Address,
    pub weight: u16,
    pub purchase_fee: u16,
    pub sale_fee: u16,
}
///`MarketConfigStorage(address,address,uint16,uint16,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MarketConfigStorage {
    pub cost_model: ::ethers::core::types::Address,
    pub drip_decay_model: ::ethers::core::types::Address,
    pub weight: u16,
    pub purchase_fee: u16,
    pub sale_fee: u16,
}
///`MintData(uint216,uint40)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct MintData {
    pub amount: ::ethers::core::types::U256,
    pub time: u64,
}
///`Request(address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Request {
    pub proposer: ::ethers::core::types::Address,
    pub disputer: ::ethers::core::types::Address,
    pub currency: ::ethers::core::types::Address,
    pub settled: bool,
    pub request_settings: RequestSettings,
    pub proposed_price: ::ethers::core::types::I256,
    pub resolved_price: ::ethers::core::types::I256,
    pub expiration_time: ::ethers::core::types::U256,
    pub reward: ::ethers::core::types::U256,
    pub final_fee: ::ethers::core::types::U256,
}
///`RequestSettings(bool,bool,bool,bool,bool,uint256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RequestSettings {
    pub event_based: bool,
    pub refund_on_dispute: bool,
    pub callback_on_price_proposed: bool,
    pub callback_on_price_disputed: bool,
    pub callback_on_price_settled: bool,
    pub bond: ::ethers::core::types::U256,
    pub custom_liveness: ::ethers::core::types::U256,
}
///`Order(uint128,uint128,bool,uint64,bool)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Order {
    pub input: u128,
    pub output: u128,
    pub use_max: bool,
    pub pool_id: u64,
    pub sell_asset: bool,
}
///`PortfolioCurve(uint128,uint16,uint16,uint16,uint16,uint32)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PortfolioCurve {
    pub strike_price: u128,
    pub fee: u16,
    pub duration: u16,
    pub volatility: u16,
    pub priority_fee: u16,
    pub created_at: u32,
}
///`PortfolioPair(address,uint8,address,uint8)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PortfolioPair {
    pub token_asset: ::ethers::core::types::Address,
    pub decimals_asset: u8,
    pub token_quote: ::ethers::core::types::Address,
    pub decimals_quote: u8,
}
///`ProtocolFees(uint16,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct ProtocolFees {
    pub reserve_fee: u16,
    pub backstop_fee: u16,
}
///`PurchaseFeesAssets(uint128,uint128,uint128,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct PurchaseFeesAssets {
    pub total_cost: u128,
    pub cost: u128,
    pub reserve_fee_assets: u128,
    pub backstop_fee_assets: u128,
    pub set_owner_fee_assets: u128,
}
///`RedemptionPreview(uint40,uint216,uint128,address,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct RedemptionPreview {
    pub delay_remaining: u64,
    pub shares: ::ethers::core::types::U256,
    pub assets: u128,
    pub owner: ::ethers::core::types::Address,
    pub receiver: ::ethers::core::types::Address,
}
///`SaleFeesAssets(uint128,uint128,uint128)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SaleFeesAssets {
    pub reserve_fee_assets: u128,
    pub backstop_fee_assets: u128,
    pub supplier_fee_assets: u128,
}
///`SetConfig(uint32,uint16)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct SetConfig {
    pub leverage_factor: u32,
    pub deposit_fee: u16,
}
///`TriggerMetadata(string,string,string,string)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct TriggerMetadata {
    pub name: ::std::string::String,
    pub category: ::std::string::String,
    pub description: ::std::string::String,
    pub logo_uri: ::std::string::String,
}
