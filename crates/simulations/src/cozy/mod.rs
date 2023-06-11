use std::error::Error;

use agents::protocol_deployer::{ProtocolDeployer, ProtocolDeployerParams};
use bindings::cozy_protocol::shared_types::{Delays, Fees};
pub use bindings::{
    cost_model_dynamic_level_factory::DeployModelCall as DeployCostModelDynamicLevelParams,
    cost_model_jump_rate_factory::DeployModelCall as DeployCostModelJumpRateParams,
    drip_decay_model_constant_factory::DeployModelCall as DeployDripDecayModelConstantParams,
};
use ethers::types::U256 as EthersU256;
pub use ethers::types::{Bytes as EthersBytes, H160 as EthersAddress};
use eyre::Result;
use revm::primitives::U256 as EvmU256;
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};
use simulate::{manager::SimManager, state::SimState, time_policy::FixedTimePolicy};
use world_state::CozyWorldState;

pub mod agents;
pub mod bindings_wrapper;
pub mod sim_types;
pub mod utils;
pub mod world_state;

pub fn run() -> Result<(), Box<dyn Error>> {
    let world_state = CozyWorldState::new();
    let sim_state = SimState::new(Some(Box::new(world_state)));
    let time_policy = Box::new(FixedTimePolicy::new(
        EvmU256::from(0),
        EvmU256::from(1),
        12_u64,
        10_u64,
        Some(500_000_u64),
        None,
    )?);
    let mut sim_manager = SimManager::new(sim_state, time_policy, 99_u64);

    /*
        // Create and activate agents.
        // Weth deployer.
        let weth_deployer = Box::new(WethDeployer::new("Weth deployer".to_owned()));
        sim_manager.activate_agent(weth_deployer);
    */
    // Protocol deployer.
    let deploy_params = ProtocolDeployerParams {
        delays: Delays {
            config_update_delay: EthersU256::from(172800),
            config_update_grace_period: EthersU256::from(259200),
            min_deposit_duration: EthersU256::from(86400),
            redemption_delay: EthersU256::from(43200),
            purchase_delay: EthersU256::from(57600),
        },
        fees: Fees {
            deposit_fee_reserves: 0_u16,
            deposit_fee_backstop: 0_u16,
            purchase_fee_reserves: 0_u16,
            purchase_fee_backstop: 0_u16,
            sale_fee_reserves: 0_u16,
            sale_fee_backstop: 0_u16,
        },
        allowed_markets_per_set: EthersU256::from(10),
    };
    let protocol_deployer = Box::new(ProtocolDeployer::new(deploy_params));
    sim_manager.activate_agent(protocol_deployer);

    /*
        // Set admin.
        let weth_addr = sim_manager
            .data
            .contract_registry
            .get(WETH.name)
            .unwrap()
            .address
            .clone();
        let salt: Option<[u8; 32]> = Some(rand::random());
        let set_params = SetAdminParams {
            asset: weth_addr,
            set_config: SetConfig {
                leverage_factor: 10000_u32,
                deposit_fee: 0_u16,
            },
            triggers: vec![CozySimTrigger::DummyTrigger],
            cost_models: vec![CozySimCostModel::JumpRate(DeployCostModelJumpRateParams {
                kink: float_to_wad(0.8),
                cost_factor_at_full_utilization: float_to_wad(0.95),
                cost_factor_at_kink_utilization: float_to_wad(0.8),
                cost_factor_at_zero_utilization: float_to_wad(0.01),
            })],
            drip_decay_models: vec![CozySimDripDecayModel::Constant(
                DeployDripDecayModelConstantParams {
                    rate_per_second: float_to_wad(0.8),
                },
            )],
            market_params_configs: vec![MarketParamsConfig {
                weight: 10000_u16,
                purchase_fee: 0_u16,
                sale_fee: 0_u16,
            }],
            salt,
        };
        let set_admin = Box::new(SetAdmin::new("Set admin".to_owned(), set_params));
        sim_manager.activate_agent(set_admin);
    */
    sim_manager.run_sim();
    Ok(())
}
