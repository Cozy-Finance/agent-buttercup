use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
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
    sim_env_data::SimEnvData,
};
use thiserror::Error;

use crate::cozy::deploy_utils;
use crate::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress},
};
use crate::cozy::agents::errors::CozyAgentError;

#[derive(Debug, Clone)]
pub struct ProtocolDeployerParams {
    pub delays: Delays,
    pub fees: Fees,
    pub allowed_markets_per_set: EthersU256,
}

pub struct ProtocolDeployer {
    name: String,
    deploy_params: ProtocolDeployerParams,
    address: Option<EvmAddress>,
}

impl ProtocolDeployer {
    pub fn new(name: String, deploy_params: ProtocolDeployerParams) -> Self {
        Self {
            name,
            deploy_params,
            address: None,
        }
    }
}

impl Agent for ProtocolDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn name(&self) -> Option<String> {
        Option::Some(self.name.clone())
    }

    fn activation_step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) {
        // Deploy external libraries.
        self.deploy_libraries(sim_env, sim_data);
        // Deploy core protocol.
        self.deploy_core_protocol(sim_env, sim_data);
        // Deploy periphery.
        self.deploy_periphery(sim_env, sim_data);
    }

    fn step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) {}
}

impl ProtocolDeployer {
    fn deploy_libraries(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) -> Result<()> {
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &CONFIGURATORLIB,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(self, sim_env, sim_data, &DELAYLIB, ())?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &DEMANDSIDELIB,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &REDEMPTIONLIB,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &STATETRANSITIONSLIB,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &SUPPLYSIDELIB,
            (),
        )?;
        Ok(())
    }

    fn deploy_core_protocol(
        &mut self,
        mut sim_env: &mut SimEnv,
        sim_data: &mut SimEnvData,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let current_nonce = sim_env.get_account_info(self.address()).nonce;
        let manager_addr = EthersAddress::from(create_address(self.address(), current_nonce));
        let set_logic_addr = EthersAddress::from(create_address(self.address(), current_nonce + 1));
        // current_nonce + 2 is initialization of the Set logic.
        let set_factory_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 3));
        let ptoken_logic_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 4));
        // current_nonce + 5 is initialization of the PToken logic.
        let ptoken_factory_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 6));
        let backstop_addr = EthersAddress::from(create_address(self.address(), current_nonce + 7));

        // Deploy manager.
        let manager_args = (
            backstop_addr,
            set_factory_addr,
            EthersAddress::from(self.address().clone()),
            EthersAddress::from(self.address()),
            self.deploy_params.delays.clone(),
            self.deploy_params.fees.clone(),
            self.deploy_params.allowed_markets_per_set,
        );
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &MANAGER,
            manager_args,
        )?;

        // Deploy set logic.
        deploy_utils::deploy_unlinked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &SET,
            vec![
                &CONFIGURATORLIB,
                &REDEMPTIONLIB,
                &SUPPLYSIDELIB,
                &DEMANDSIDELIB,
                &DELAYLIB,
                &STATETRANSITIONSLIB,
            ],
            (manager_addr, ptoken_factory_addr, backstop_addr),
        )?;
        let set_logic = sim_data
            .contract_registry
            .get(SET.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
    
        // Initialize set logic.
        let empty_market_configs: Vec<MarketConfig> = vec![];
        let weth_addr = sim_data
            .contract_registry
            .get(WETH.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?
            .address;
        self.call_contract(
            &mut sim_env,
            &set_logic,
            set_logic.encode_function(
                "initialize",
                (
                    EthersAddress::zero(),
                    EthersAddress::zero(),
                    weth_addr,
                    SetConfig {
                        deposit_fee: 0,
                        leverage_factor: 0,
                    },
                    empty_market_configs,
                ),
            )?,
        );
        println!("Set logic initialized.");

        // Deploy set factory.
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &SETFACTORY,
            (manager_addr, set_logic_addr),
        )?;

        // Deploy ptoken logic.
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &PTOKEN,
            (manager_addr,),
        )?;
        let ptoken_logic = sim_data
            .contract_registry
            .get(PTOKEN.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;

        // Initialize ptoken logic.
        self.call_contract(
            &mut sim_env,
            &ptoken_logic,
            ptoken_logic.encode_function(
                "initialize",
                (EthersAddress::zero(), EthersAddress::zero(), 0_u8),
            )?,
        );
        println!("Ptoken logic initialized.");

        // Deploy ptoken factory.
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &PTOKENFACTORY,
            (ptoken_logic_addr,),
        )?;

        // Deploy backstop.
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &BACKSTOP,
            (manager_addr, weth_addr),
        )?;

        // Deploy cozy router.
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &COZYROUTER,
            (manager_addr, weth_addr, weth_addr, weth_addr),
        )?;

        Ok(())
    }

    fn deploy_periphery(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) -> Result<()> {
        let manager_addr = sim_data
            .contract_registry
            .get(PTOKEN.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?
            .address;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &COSTMODELJUMPRATEFACTORY,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &COSTMODELDYNAMICLEVELFACTORY,
            (),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &DRIPDECAYMODELCONSTANTFACTORY,
            (),
        )?;
        // todo!("Add optimistic oracle address");
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &UMATRIGGERFACTORY,
            (manager_addr, manager_addr),
        )?;
        deploy_utils::deploy_linked_contract_with_args(
            self,
            sim_env,
            sim_data,
            &CHAINLINKTRIGGERFACTORY,
            (manager_addr),
        )?;
        Ok(())
    }
}
