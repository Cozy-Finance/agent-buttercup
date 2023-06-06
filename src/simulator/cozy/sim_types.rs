pub use bindings::{
    cost_model_dynamic_level_factory::DeployModelCall as DeployCostModelDynamicLevelParams,
    cost_model_jump_rate_factory::DeployModelCall as DeployCostModelJumpRateParams,
    drip_decay_model_constant_factory::DeployModelCall as DeployDripDecayModelConstantParams
};

#[derive(Debug, Clone)]
pub enum CozySimCostModel {
    JumpRate(DeployCostModelJumpRateParams),
    DynamicLevel(DeployCostModelDynamicLevelParams)
}

#[derive(Debug, Clone)]
pub enum CozySimDripDecayModel {
    Constant(DeployDripDecayModelConstantParams)
}

#[derive(Debug, Clone)]
pub enum CozySimTrigger {
    DummyTrigger,
}

#[derive(Debug, Clone)]
pub struct MarketParamsConfig {
    pub weight: u16,
    pub purchase_fee: u16,
    pub sale_fee: u16,
}