use std::{borrow::Cow, collections::HashMap};

use bindings::{
    cost_model_jump_rate_factory, cozy_protocol::shared_types::MarketConfig,
    drip_decay_model_constant_factory,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use simulate::{
    address::Address, manager::SimManager, state::SimState, time_policy::FixedTimePolicy,
};

use crate::cozy::{
    agents::{
        active_buyer::ActiveBuyer, cost_models_deployer::CostModelsDeployer,
        drip_decay_models_deployer::DripDecayModelsDeployer, passive_buyer::PassiveBuyer,
        passive_supplier::PassiveSupplier, protocol_deployer::ProtocolDeployer,
        set_admin::SetAdmin, token_deployer::TokenDeployer, triggers_deployer::TriggersDeployer,
        weth_deployer::WethDeployer,
    },
    bindings_wrapper::*,
    constants::*,
    distributions::{CozyDistribution, TriggerProbModel},
    runner,
    runner::CozySingleSetSimRunner,
    types::{
        CozyActiveBuyersParams, CozyCostModelType, CozyDripDecayModelType,
        CozyFixedTimePolicyParams, CozyMarketConfigParams, CozyPassiveBuyersParams,
        CozyProtocolDeployParams, CozySetAdminParams, CozySetConfigParams, CozySimSetupParams,
        CozySuppliersParams, CozyTokenDeployParams, CozyTriggerType,
    },
    utils::float_to_wad,
    world::CozyWorld,
};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
