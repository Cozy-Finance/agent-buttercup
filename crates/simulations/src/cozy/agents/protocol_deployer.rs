use std::collections::HashMap;
use std::sync::Arc;

use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::{prelude::BaseContract as EthersBaseContract, types::U256 as EthersU256};
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv},
};

use super::errors::CozyAgentError;
use crate::cozy::{
    bindings_wrapper::*,
    utils::{build_deploy_contract_tx, build_unlinked_deploy_contract_tx},
    world_state::{CozyUpdate, CozyWorld},
    EthersAddress, EvmAddress,
};

#[derive(Debug, Clone)]
pub struct ProtocolDeployerParams {
    pub delays: Delays,
    pub fees: Fees,
    pub allowed_markets_per_set: EthersU256,
}

pub struct ProtocolDeployer {
    deploy_params: ProtocolDeployerParams,
    address: Option<EvmAddress>,
}

impl ProtocolDeployer {
    pub fn new(deploy_params: ProtocolDeployerParams) -> Self {
        Self {
            deploy_params,
            address: None,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for ProtocolDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        // Deploy external libraries.
        let libraries = self
            .deploy_libraries(state, &channel)
            .expect("Error deploying libraries.");
        // Deploy core protocol.
        self.deploy_core_protocol(state, &channel, &libraries.clone());
        // Deploy periphery.
        // self.deploy_periphery(state);
    }

    fn resolve_activation_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl ProtocolDeployer {
    fn deploy_libraries(
        &self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: &AgentChannel<CozyUpdate>,
    ) -> Result<HashMap<EthersAddress, &BindingsWrapper>> {
        let mut libraries: HashMap<EthersAddress, &BindingsWrapper> = HashMap::new();

        let current_nonce = 0;
        let configurator_addr = EthersAddress::from(create_address(self.address(), current_nonce));
        let delay_lib_addr = EthersAddress::from(create_address(self.address(), current_nonce + 1));
        let demandside_lib_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 2));
        let redemption_lib_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 3));
        let state_transitions_lib_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 4));
        let supply_side_lib_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 5));

        libraries.insert(configurator_addr, &CONFIGURATORLIB);
        libraries.insert(delay_lib_addr, &DELAYLIB);
        libraries.insert(demandside_lib_addr, &DEMANDSIDELIB);
        libraries.insert(redemption_lib_addr, &REDEMPTIONLIB);
        libraries.insert(state_transitions_lib_addr, &STATETRANSITIONSLIB);
        libraries.insert(supply_side_lib_addr, &SUPPLYSIDELIB);

        let (configurator_lib_tx, _) =
            build_deploy_contract_tx(self.address(), &CONFIGURATORLIB, ())?;
        let (delay_lib_tx, _) = build_deploy_contract_tx(self.address(), &DELAYLIB, ())?;
        let (demand_side_lib_tx, _) = build_deploy_contract_tx(self.address(), &DEMANDSIDELIB, ())?;
        let (redemption_lib_tx, _) = build_deploy_contract_tx(self.address(), &REDEMPTIONLIB, ())?;
        let (state_transitions_lib_tx, _) =
            build_deploy_contract_tx(self.address(), &STATETRANSITIONSLIB, ())?;
        let (supply_side_lib_tx, _) = build_deploy_contract_tx(self.address(), &SUPPLYSIDELIB, ())?;

        for tx in vec![
            configurator_lib_tx,
            delay_lib_tx,
            demand_side_lib_tx,
            redemption_lib_tx,
            state_transitions_lib_tx,
            supply_side_lib_tx,
        ] {
            channel.send(SimUpdate::Evm(tx));
        }

        Ok(libraries)
    }

    fn deploy_core_protocol(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        _channel: &AgentChannel<CozyUpdate>,
        libraries: &HashMap<EthersAddress, &BindingsWrapper>,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let current_nonce = libraries.len() as u64;
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
        let cozyrouter_addr = EthersAddress::from(create_address(self.address(), current_nonce + 8));

        let mut evm_updates: Vec<SimUpdate<CozyUpdate>> = vec![];
        let mut world_updates: Vec<SimUpdate<CozyUpdate>> = vec![];

        // Deploy manager.
        let manager_args = (
            backstop_addr,
            set_factory_addr,
            EthersAddress::from(self.address()),
            EthersAddress::from(self.address()),
            self.deploy_params.delays.clone(),
            self.deploy_params.fees.clone(),
            self.deploy_params.allowed_markets_per_set,
        );
        let (manager_tx, manager_contract) = build_deploy_contract_tx(
            self.address(),
            &MANAGER,
            manager_args,
        )?;
        evm_updates.push(SimUpdate::Evm(manager_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Manager".into(),
            manager_addr.into(),
            manager_contract,
        )));

        // Deploy set logic.
        let set_logic_args = (manager_addr, ptoken_factory_addr, backstop_addr);
        let (set_logic_tx, set_logic_contract) = build_unlinked_deploy_contract_tx(
            self.address(),
            &SET,
            &libraries,
            set_logic_args,
        )?;
        evm_updates.push(SimUpdate::Evm(set_logic_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Set logic".into(),
            set_logic_addr.into(),
            set_logic_contract
        )));

        // Initialize set logic.
        let empty_market_configs: Vec<MarketConfig> = vec![];
        let (weth_addr, _) = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)?
            .contract_registry
            .get("Weth")
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        let weth_addr = EthersAddress::from(*weth_addr);
        let set_initialize_args = (
            EthersAddress::zero(),
            EthersAddress::zero(),
            weth_addr,
            SetConfig {
                deposit_fee: 0,
                leverage_factor: 0,
            },
            empty_market_configs,
        );
        let set_initialize_call_data = set_logic_contract.encode_function(
            "initialize",
            set_initialize_args,
        ).expect("Issue encoding func.");
        let set_initialize_tx = build_call_contract_txenv(
            self.address(), 
            set_logic_addr.into(), 
            set_initialize_call_data, 
            None, 
            None
        );
        evm_updates.push(SimUpdate::Evm(set_initialize_tx));

        // Deploy set factory.
        let set_factory_args = (manager_addr, set_logic_addr);
        let (set_factory_tx, set_factory_contract) = build_deploy_contract_tx(
            self.address(),
            &SETFACTORY,
            set_factory_args,
        )?;
        evm_updates.push(SimUpdate::Evm(set_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Set factory".into(),
            set_factory_addr.into(),
            set_factory_contract
        )));

        // Deploy ptoken logic.
        let ptoken_logic_args = (manager_addr,);
        let (ptoken_logic_tx, ptoken_logic_contract) = build_deploy_contract_tx(
            self.address(),
            &PTOKEN,
            ptoken_logic_args,
        )?;
        evm_updates.push(SimUpdate::Evm(ptoken_logic_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Ptoken logic".into(),
            ptoken_logic_addr.into(),
            ptoken_logic_contract
        )));

        // Initialize ptoken logic.
        let ptoken_initialize_args = (EthersAddress::zero(), EthersAddress::zero(), 0_u8);
        let ptoken_initialize_call_data = ptoken_logic_contract.encode_function(
            "initialize",
            ptoken_initialize_args
        ).expect("Issue encoding func.");
        let ptoken_initialize_tx = build_call_contract_txenv(
            self.address(), 
            ptoken_logic_addr.into(), 
            ptoken_initialize_call_data, 
            None, 
            None
        );
        evm_updates.push(SimUpdate::Evm(ptoken_initialize_tx));

        // Deploy ptoken factory.
        let ptoken_factory_args = (ptoken_logic_addr,);
        let (ptoken_factory_tx, ptoken_factory_contract) = build_deploy_contract_tx(
            self.address(),
            &PTOKENFACTORY,
            ptoken_factory_args,
        )?;
        evm_updates.push(SimUpdate::Evm(ptoken_factory_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Ptoken factory".into(),
            ptoken_factory_addr.into(),
            ptoken_factory_contract
        )));

        // Deploy backstop.
        let backstop_args = (manager_addr, weth_addr);
        let (backstop_tx, backstop_contract) = build_deploy_contract_tx(
            self.address(),
            &BACKSTOP,
            backstop_args,
        )?;
        evm_updates.push(SimUpdate::Evm(backstop_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "Backstop".into(),
            backstop_addr.into(),
            backstop_contract
        )));

        // Deploy CozyRouter.
        let cozyrouter_args = (manager_addr, weth_addr, weth_addr, weth_addr);
        let (cozyrouter_tx, cozyrouter_contract) = build_deploy_contract_tx(
            self.address(),
            &COZYROUTER,
            cozyrouter_args,
        )?;
        evm_updates.push(SimUpdate::Evm(cozyrouter_tx));
        world_updates.push(SimUpdate::World(CozyUpdate::AddToContractRegistry(
            "CozyRouter".into(),
            cozyrouter_addr.into(),
            cozyrouter_contract
        )));

        Ok(())
    }
}
