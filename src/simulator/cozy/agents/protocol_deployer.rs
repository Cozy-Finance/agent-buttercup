use std::collections::HashMap;

use bindings::{
    backstop_1::*,
    configurator_lib::*,
    cozy_protocol::metadata::*,
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    cozy_router::*,
    delay_lib::*,
    demand_side_lib::*,
    manager::*,
    p_token::*,
    p_token_factory::*,
    redemption_lib::*,
    set::*,
    set_factory::*,
    state_transitions_lib::*,
    supply_side_lib::*,
};
use ethers::abi::{Contract as EthersContract, Token, Tokenize};
use ethers::types::{Address as EthersAddress, Bytes as EthersBytes, U256 as EthersU256};
use eyre::Result;
use revm::primitives::{create_address, Address as EVMAddress, U256 as EVMU256};
use simulate::{
    agent::Agent,
    contract::{
        sim_contract::{IsDeployed, SimulationContract},
        utils,
    },
    environment::sim_environment::SimulationEnvironment,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolDeployerErrors {
    #[error("tried accessing a non-existent library addr")]
    NonExistentLibraryAddr,
    #[error("failed to link set bytecode")]
    SetBytecodeFailure,
}

#[derive(Debug, Clone)]
pub struct ProtocolDeployerParams {
    owner: EthersAddress,
    pauser: EthersAddress,
    delays: Delays,
    fees: Fees,
    allowed_markets_per_set: EthersU256,
}

pub struct ProtocolDeployer {
    name: String,
    deploy_params: ProtocolDeployerParams,
}

impl ProtocolDeployer {
    fn new(name: String, deploy_params: ProtocolDeployerParams) -> Self {
        Self {
            name,
            deploy_params,
        }
    }
}

impl Agent for ProtocolDeployer {
    fn name(&self) -> Option<String> {
        Option::Some(self.name)
    }

    fn activation_step(&self, simulation_environment: &mut SimulationEnvironment) {
        // Deploy external libraries.
        let mut library_addresses: HashMap<&str, EthersAddress> = HashMap::new();
        self.deploy_libraries(simulation_environment, &mut library_addresses);
        // Deploy core protocol.
        self.deploy_core_protocol(simulation_environment, &mut library_addresses);
    }

    fn step(&self, simulation_environment: &mut SimulationEnvironment) {}
}

impl ProtocolDeployer {
    fn deploy_contract_with_args<T: Tokenize>(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        abi: EthersContract,
        bytecode: EthersBytes,
        args: T,
        name: &str,
        add_to_registry: bool,
    ) -> Result<SimulationContract<IsDeployed>> {
        let contract = SimulationContract::new(abi, bytecode);
        let contract = self.deploy_contract(
            simulation_environment,
            &contract,
            contract.encode_constructor(args)?,
        )?;
        println!("{} deployed at address: {}.", name, contract.address);

        if add_to_registry {
            simulation_environment
                .data
                .address_registry
                .insert(name.to_string(), contract.address);
        }

        Ok(contract)
    }

    fn deploy_libraries(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        library_addresses: &mut HashMap<&str, Address>,
    ) -> Result<()> {
        let configurator_lib = self.deploy_contract_with_args(
            simulation_environment,
            CONFIGURATORLIB_ABI.clone(),
            CONFIGURATORLIB_BYTECODE.clone(),
            (),
            CONFIGURATORLIB_NAME,
            false,
        )?;
        library_addresses.insert(CONFIGURATORLIB_NAME, configurator_lib.address);

        let delay_lib_addr = self.deploy_contract_with_args(
            simulation_environment,
            DELAYLIB_ABI.clone(),
            DELAYLIB_BYTECODE.clone(),
            (),
            DELAYLIB_NAME,
            false,
        )?;
        library_addresses.insert(DELAYLIB_NAME, delay_lib_addr.address);

        let demand_side_lib = self.deploy_contract_with_args(
            simulation_environment,
            DEMANDSIDELIB_ABI.clone(),
            DEMANDSIDELIB_BYTECODE.clone(),
            (),
            DEMANDSIDELIB_NAME,
            false,
        )?;
        library_addresses.insert(DEMANDSIDELIB_NAME, demand_side_lib.address);

        let redemption_lib = self.deploy_contract_with_args(
            simulation_environment,
            REDEMPTIONLIB_ABI.clone(),
            REDEMPTIONLIB_BYTECODE.clone(),
            (),
            REDEMPTIONLIB_NAME,
            false,
        )?;
        library_addresses.insert(REDEMPTIONLIB_NAME, redemption_lib.address);

        let state_transitions_lib = self.deploy_contract_with_args(
            simulation_environment,
            STATETRANSITIONSLIB_ABI.clone(),
            STATETRANSITIONSLIB_BYTECODE.clone(),
            (),
            STATETRANSITIONSLIB_NAME,
            false,
        )?;
        library_addresses.insert(STATETRANSITIONSLIB_NAME, state_transitions_lib.address);

        let supply_side_lib = self.deploy_contract_with_args(
            simulation_environment,
            SUPPLYSIDELIB_ABI.clone(),
            SUPPLYSIDELIB_BYTECODE.clone(),
            (),
            SUPPLYSIDELIB_NAME,
            false,
        )?;
        library_addresses.insert(SUPPLYSIDELIB_NAME, supply_side_lib.address);

        Ok(())
    }

    fn deploy_core_protocol(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        library_addresses: &mut HashMap<&str, Address>,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let current_nonce = self.get_nonce(simulation_environment);
        let manager_addr = Address::from(create_address(self.address(), current_nonce));
        let set_logic_addr = Address::from(create_address(self.address(), current_nonce + 1));
        // current_nonce + 2 is initialization of the Set logic.
        let set_factory_addr = Address::from(create_address(self.address(), current_nonce + 3));
        let p_token_logic_addr = Address::from(create_address(self.address(), current_nonce + 4));
        // current_nonce + 5 is initialization of the PToken logic.
        let p_token_factory_addr = Address::from(create_address(self.address(), current_nonce + 6));
        let backstop_addr = Address::from(create_address(self.address(), current_nonce + 7));

        self.deploy_contract_with_args(
            simulation_environment,
            MANAGER_ABI.clone(),
            MANAGER_BYTECODE.clone(),
            (
                backstop_addr,
                set_factory_addr,
                self.deploy_params.owner,
                self.deploy_params.pauser,
                self.deploy_params.delays,
                self.deploy_params.fees,
                self.deploy_params.allowed_markets_per_set,
            ),
            MANAGER_NAME,
            true,
        )?;

        let set_bytecode = utils::build_linked_bytecode(
            SET_RAW_BYTECODE,
            vec![
                (
                    CONFIGURATORLIB_PATH,
                    CONFIGURATORLIB_NAME,
                    *library_addresses
                        .get(CONFIGURATORLIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
                (
                    DEMANDSIDELIB_PATH,
                    DEMANDSIDELIB_NAME,
                    *library_addresses
                        .get(DEMANDSIDELIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
                (
                    DELAYLIB_PATH,
                    DELAYLIB_NAME,
                    *library_addresses
                        .get(DELAYLIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
                (
                    REDEMPTIONLIB_PATH,
                    REDEMPTIONLIB_NAME,
                    *library_addresses
                        .get(REDEMPTIONLIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
                (
                    STATETRANSITIONSLIB_PATH,
                    STATETRANSITIONSLIB_NAME,
                    *library_addresses
                        .get(STATETRANSITIONSLIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
                (
                    SUPPLYSIDELIB_PATH,
                    SUPPLYSIDELIB_NAME,
                    *library_addresses
                        .get(SUPPLYSIDELIB_NAME)
                        .ok_or(ProtocolDeployerErrors::NonExistentLibraryAddr)?,
                ),
            ],
        )
        .map_err(|_| ProtocolDeployerErrors::SetBytecodeFailure)?;

        let set_logic = self.deploy_contract_with_args(
            simulation_environment,
            SET_ABI.clone(),
            set_bytecode,
            (
                manager_addr,
                p_token_logic_addr,
                self.deploy_params.owner,
                self.deploy_params.pauser,
                self.deploy_params.delays,
                self.deploy_params.fees,
                self.deploy_params.allowed_markets_per_set,
            ),
            MANAGER_NAME,
            true,
        )?;

        let empty_market_configs: Vec<MarketConfig> = vec![];
        self.call_contract(
            &mut manager.environment,
            &set_logic_addr,
            set_logic.encode_function(
                "initialize",
                (
                    H160::ZERO,
                    H160::ZERO,
                    simulation_environment.contract_registry.get("Weth"),
                    SetConfig {
                        deposit_fee: 0,
                        leverage_factor: 0,
                    },
                    empty_market_configs,
                ),
            )?,
        )?;
        println!("Set logic initialized.");

        self.deploy_contract_with_args(
            simulation_environment,
            SETFACTORY_ABI.clone(),
            SETFACTORY_BYTECODE.clone(),
            (manager_addr, set_logic_addr),
            SETFACTORY_NAME,
            true,
        )?;

        let p_token_logic = self.deploy_contract_with_args(
            simulation_environment,
            PTOKEN_ABI.clone(),
            PTOKEN_BYTECODE.clone(),
            (manager_addr,),
            PTOKEN_NAME,
            true,
        )?;

        self.call_contract(
            &mut simulation_environment,
            &p_token_logic,
            p_token_logic.encode_function("initialize", (B160::ZERO, B160::ZERO, 0_u8))?,
        );
        println!("Ptoken logic initialized.");

        self.deploy_contract_with_args(
            simulation_environment,
            PTOKENFACTORY_ABI.clone(),
            PTOKENFACTORY_BYTECODE.clone(),
            (p_token_logic_address,),
            PTOKENFACTORY_NAME,
            true,
        )?;

        self.deploy_contract_with_args(
            simulation_environment,
            BACKSTOP_1_ABI.clone(),
            BACKSTOP_1_BYTECODE.clone(),
            (
                manager_address,
                simulation_environment.contract_registry.get("Weth"),
            ),
            BACKSTOP_1_NAME,
            true,
        )?;

        self.deploy_contract_with_args(
            simulation_environment,
            COZYROUTER_ABI.clone(),
            COZYROUTER_BYTECODE.clone(),
            (
                manager_address,
                simulation_environment.contract_registry.get("Weth"),
                simulation_environment.contract_registry.get("Weth"),
                simulation_environment.contract_registry.get("Weth"),
            ),
            COZYROUTER_NAME,
            true,
        )?;
    }
}
