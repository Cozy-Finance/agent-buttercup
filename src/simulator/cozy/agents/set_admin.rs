use bindings::{
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    manager::{CreateSetCall, CreateSetReturn},
    cost_model_dynamic_level_factory::DeployModelReturn as DeployCostModelJumpRateReturn,
    cost_model_jump_rate_factory::DeployModelReturn as DeployCostModelDynamicLevelReturn,
    drip_decay_model_constant_factory::DeployModelReturn as DeployDripDecayModelConstantReturn
};
use ethers::abi::AbiDecode;
use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::Agent,
    contract::{
        sim_contract::{IsDeployed, SimContract},
        utils,
    },
    environment::sim_env::SimEnv,
    utils::unpack_execution
};
use thiserror::Error;

use crate::simulator::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress}, sim_types::CozySimTrigger,
};
use crate::simulator::cozy::sim_types::*;
use crate::simulator::cozy::sim_types::{
    CozySimCostModel, 
    CozySimDripDecayModel, 
    MarketParamsConfig,
};

#[derive(Debug, Error)]
pub enum SetAdminError {
    #[error("unregistered address")]
    UnregisteredAddress,
}

#[derive(Debug, Clone)]
pub struct SetAdminParams {
    pub asset: EthersAddress,
    pub set_config: SetConfig,
    pub triggers: Vec<CozySimTrigger>,
    pub cost_models: Vec<CozySimCostModel>,
    pub drip_decay_models: Vec<CozySimDripDecayModel>,
    pub market_params_configs: Vec<MarketParamsConfig>,
    pub salt: Option<[u8; 32]>
}

pub struct SetAdmin {
    name: String,
    set_admin_params: SetAdminParams,
    address: Option<EvmAddress>,
}

impl SetAdmin {
    pub fn new(name: String, set_admin_params: SetAdminParams) -> Self {
        Self {
            name,
            set_admin_params,
            address: None,
        }
    }
}

impl Agent for SetAdmin {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn name(&self) -> Option<String> {
        Option::Some(self.name.clone())
    }

    fn activation_step(&mut self, sim_env: &mut SimEnv) {
        // Deploy all triggers.
        let trigger_addrs = self.set_admin_params
            .triggers
            .iter()
            .map(|trigger| match trigger {
                CozySimTrigger::DummyTrigger => self.deploy_dummy_trigger(sim_env)
            })
            .collect::<Result<Vec<_>>>()
            .unwrap();

        // Deploy all cost models.
        let cost_model_addrs = self.set_admin_params
            .cost_models
            .iter()
            .map(|model| match model {
                CozySimCostModel::JumpRate(args) => self.deploy_cost_model_jump_rate(sim_env, *args),
                CozySimCostModel::DynamicLevel(args) => self.deploy_cost_model_dynamic_level(sim_env, *args)
            })
            .collect::<Result<Vec<_>>>()
            .unwrap();

        // Deploy all drip decay models.
        let drip_decay_model_addrs = self.set_admin_params
            .drip_decay_models
            .iter()
            .map(|model| match model {
                CozySimDripDecayModel::Constant(args) => self.deploy_drip_decay_model_constant(sim_env, *args),
            })
            .collect::<Result<Vec<_>>>()
            .unwrap();

        let market_configs: Vec<MarketConfig> = self.set_admin_params
            .market_params_configs
            .iter()
            .enumerate()
            .map(|(i, params)| MarketConfig {
                trigger: trigger_addrs[i],
                cost_model: cost_model_addrs[i],
                drip_decay_model: drip_decay_model_addrs[i],
                weight: params.weight,
                purchase_fee: params.purchase_fee,
                sale_fee: params.sale_fee
            })
            .collect();

        self.create_set(sim_env, CreateSetCall {
            owner: self.address().into(),
            pauser: self.address().into(),
            asset: self.set_admin_params.asset,
            set_config: self.set_admin_params.set_config.clone(),
            market_configs,
            salt: self.set_admin_params.salt.unwrap_or(rand::random::<[u8; 32]>())
        });
    }

    fn step(&mut self, sim_env: &mut SimEnv) {}
}

impl SetAdmin {
    fn deploy_cost_model_jump_rate(&self, sim_env: &mut SimEnv, args: DeployCostModelJumpRateParams) -> Result<EthersAddress> {
        let factory = sim_env
            .data
            .contract_registry
            .get(COSTMODELJUMPRATEFACTORY.name)
            .ok_or(SetAdminError::UnregisteredAddress)?;
        let result = unpack_execution(self.call_contract_by_address(
            sim_env,
            factory.address.into(),
            factory.encode_function("deployModel", args)?,
            None,
            None
        ))?;
        let result = DeployCostModelJumpRateReturn::decode(result)?;

        Ok(result.model)
    }

    fn deploy_cost_model_dynamic_level(
        &self,
        sim_env: &mut SimEnv,
        args: DeployCostModelDynamicLevelParams,
    ) -> Result<EthersAddress> {
        let factory = sim_env
            .data
            .contract_registry
            .get(COSTMODELDYNAMICLEVELFACTORY.name)
            .ok_or(SetAdminError::UnregisteredAddress)?;
        let result = unpack_execution(self.call_contract_by_address(
            sim_env,
            factory.address.into(),
            factory.encode_function("deployModel", args)?,
            None,
            None
        ))?;
        let result = DeployCostModelDynamicLevelReturn::decode(result)?;

        Ok(result.model)
    }

    fn deploy_drip_decay_model_constant(&self, sim_env: &mut SimEnv, args: DeployDripDecayModelConstantParams) -> Result<EthersAddress> {
        let factory = sim_env
            .data
            .contract_registry
            .get(DRIPDECAYMODELCONSTANTFACTORY.name)
            .ok_or(SetAdminError::UnregisteredAddress)?;
        let result = unpack_execution(self.call_contract_by_address(
            sim_env,
            factory.address.into(),
            factory.encode_function("deployModel", args)?,
            None,
            None
        ))?;
        let result = DeployDripDecayModelConstantReturn::decode(result)?;

        Ok(result.model)
    }

    fn deploy_dummy_trigger(&self, sim_env: &mut SimEnv) -> Result<EthersAddress> {
        let factory = sim_env
            .data
            .contract_registry
            .get(DUMMYTRIGGER.name)
            .ok_or(SetAdminError::UnregisteredAddress)?
            .clone();
        let result = self.call_contract(
            sim_env,
            factory,
            factory.encode_function("deployModel", ())?,
        );
        let result = unpack_execution(result)?;
        let result = factory.decode_output(
            "deployModel",
            result
        )?;

        Ok(result)
    }

    fn create_set(&self, sim_env: &mut SimEnv, args: CreateSetCall) -> Result<EthersAddress> {
        let manager = sim_env
            .data
            .contract_registry
            .get(MANAGER.name)
            .ok_or(SetAdminError::UnregisteredAddress)?;
        let result = unpack_execution(
            self.call_contract(
            sim_env,
            manager,
            manager.encode_function("createSet", args)?
        ))?;
        let result = CreateSetReturn::decode(result)?;
        Ok(result.set)
    }
}
