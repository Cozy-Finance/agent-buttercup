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
