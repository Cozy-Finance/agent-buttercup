pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory,
};

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

#[derive(Debug, Clone)]
pub struct CozyTokenDeployParams {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}
