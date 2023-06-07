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
    sim_env_data::SimEnvData
};
use thiserror::Error;

use crate::simulator::cozy::{
    bindings_wrapper::*,
    {EthersAddress, EthersBytes, EvmAddress},
};

#[derive(Error, Debug)]
pub enum ProtocolDeployerError {
    #[error("tried accessing a non-existent library addr")]
    NonExistentLibraryAddr,
    #[error("failed to link set bytecode")]
    LinkingBytecodeFailure,
    #[error("missing linked bytecode")]
    MissingLinkedBytecode,
    #[error("missing unlinked bytecode")]
    MissingUnlinkedBytecode,
    #[error("uninitialized address")]
    UninitializedAddr,
}

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
        self.deploy_libraries(sim_env);
        // Deploy core protocol.
        self.deploy_core_protocol(sim_env, sim_data);
        // Deploy periphery.
        self.deploy_periphery(sim_env);
    }

    fn step(&mut self, sim_env: &mut SimEnv, sim_data: &mut SimEnvData) {}
}

impl ProtocolDeployer {
    fn create_and_deploy_sim_contract<T: Tokenize>(
        &self,
        sim_env: &mut SimEnv,
        abi: EthersContract,
        bytecode: EthersBytes,
        name: &str,
        args: T,
    ) -> Result<()> {
        let contract = SimContract::new(abi, bytecode);
        let contract =
            self.deploy_contract(sim_env, &contract, contract.encode_constructor(args)?)?;
        println!("{} deployed at address: {}.", name, contract.address);

        sim_env
            .data
            .contract_registry
            .insert(name.to_string(), contract);

        Ok(())
    }

    fn deploy_linked_contract_with_args<T: Tokenize>(
        &self,
        sim_env: &mut SimEnv,
        contract_bindings: &BindingsWrapper,
        args: T,
    ) -> Result<()> {
        let abi = (*contract_bindings).abi.clone();
        let bytecode = (*contract_bindings)
            .bytecode
            .ok_or(ProtocolDeployerError::MissingLinkedBytecode)?
            .clone();
        let name = (*contract_bindings).name;

        self.create_and_deploy_sim_contract(sim_env, abi, bytecode, name, args)
    }

    fn deploy_unlinked_contract_with_args<T: Tokenize>(
        &self,
        sim_env: &mut SimEnv,
        contract_bindings: &BindingsWrapper,
        libraries: Vec<&BindingsWrapper>,
        args: T,
    ) -> Result<()> {
        let mut links: Vec<(&str, &str, EthersAddress)> = vec![];
        for lib_binding in libraries.iter() {
            links.push((
                lib_binding.path,
                lib_binding.name,
                (*sim_env
                    .data
                    .contract_registry
                    .get(lib_binding.name)
                    .ok_or(ProtocolDeployerError::NonExistentLibraryAddr)?)
                .address,
            ));
        }

        let bytecode = utils::build_linked_bytecode(
            (*contract_bindings)
                .unlinked_bytecode
                .ok_or(ProtocolDeployerError::MissingUnlinkedBytecode)?,
            links,
        )?;

        let abi = (*contract_bindings).abi.clone();
        let name = (*contract_bindings).name;

        self.create_and_deploy_sim_contract(sim_env, abi, bytecode.into(), name, args)
    }

    fn deploy_libraries(&self, sim_env: &mut SimEnv) -> Result<()> {
        self.deploy_linked_contract_with_args(sim_env, &CONFIGURATORLIB, ())?;
        self.deploy_linked_contract_with_args(sim_env, &DELAYLIB, ())?;
        self.deploy_linked_contract_with_args(sim_env, &DEMANDSIDELIB, ())?;
        self.deploy_linked_contract_with_args(sim_env, &REDEMPTIONLIB, ())?;
        self.deploy_linked_contract_with_args(sim_env, &STATETRANSITIONSLIB, ())?;
        self.deploy_linked_contract_with_args(sim_env, &SUPPLYSIDELIB, ())?;
        Ok(())
    }

    fn deploy_core_protocol(&self, mut sim_env: &mut SimEnv, sim_data: &mut SimEnvData) -> Result<()> {
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
        self.deploy_linked_contract_with_args(
            sim_env,
            &MANAGER,
            (
                backstop_addr,
                set_factory_addr,
                EthersAddress::from(self.address()),
                EthersAddress::from(self.address()),
                self.deploy_params.delays.clone(),
                self.deploy_params.fees.clone(),
                self.deploy_params.allowed_markets_per_set,
            ),
        )?;

        // Deploy set logic.
        self.deploy_unlinked_contract_with_args(
            sim_env,
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
        )
        .map_err(|_| ProtocolDeployerError::LinkingBytecodeFailure)?;
        let set_logic = sim_data.contract_registry.get(SET.name).unwrap();

        // Initialize set logic.
        let empty_market_configs: Vec<MarketConfig> = vec![];
        let weth_addr = sim_data
            .contract_registry
            .get("Weth")
            .ok_or(ProtocolDeployerError::UninitializedAddr)?
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

        // Deploy ptoken logic.
        self.deploy_linked_contract_with_args(sim_env, &PTOKEN, (manager_addr, set_logic_addr))?;

        self.deploy_linked_contract_with_args(sim_env, &PTOKEN, (manager_addr,))?;
        let mut ptoken_logic = sim_data.contract_registry.get(PTOKEN.name).unwrap();

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
        self.deploy_linked_contract_with_args(sim_env, &PTOKENFACTORY, (ptoken_logic_addr,))?;

        // Deploy backstop.
        self.deploy_linked_contract_with_args(sim_env, &BACKSTOP, (manager_addr, weth_addr))?;

        // Deploy cozy router.
        self.deploy_linked_contract_with_args(
            sim_env,
            &COZYROUTER,
            (weth_addr, weth_addr, weth_addr),
        )?;

        Ok(())
    }

    fn deploy_periphery(&self, sim_env: &mut SimEnv) -> Result<()> {
        self.deploy_linked_contract_with_args(sim_env, &COSTMODELJUMPRATEFACTORY, ())?;
        self.deploy_linked_contract_with_args(sim_env, &COSTMODELDYNAMICLEVELFACTORY, ())?;
        self.deploy_linked_contract_with_args(sim_env, &DRIPDECAYMODELCONSTANTFACTORY, ())?;
        self.deploy_linked_contract_with_args(sim_env, &UMATRIGGERFACTORY, ())?;
        self.deploy_linked_contract_with_args(sim_env, &CHAINLINKTRIGGERFACTORY, ())?;
        Ok(())
    }
}
