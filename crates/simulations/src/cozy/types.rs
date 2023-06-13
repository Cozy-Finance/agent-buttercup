pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory,
};

#[derive(Debug, Clone)]
pub enum CozySimCostModel {
    JumpRate(cost_model_jump_rate_factory::DeployModelCall),
    DynamicLevel(cost_model_dynamic_level_factory::DeployModelCall),
}

#[derive(Debug, Clone)]
pub enum CozySimDripDecayModel {
    Constant(drip_decay_model_constant_factory::DeployModelCall),
}

#[derive(Debug, Clone)]
pub enum CozySimTrigger {
    DummyTrigger,
}

#[derive(Debug, Clone)]
pub struct CozyMarketParamsConfig {
    pub weight: u16,
    pub purchase_fee: u16,
    pub sale_fee: u16,
}
