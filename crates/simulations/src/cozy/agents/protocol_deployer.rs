use std::collections::HashMap;

use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use ethers::{prelude::BaseContract as EthersBaseContract, types::U256 as EthersU256};
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv, encode_function},
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

        let mut evm_txs = vec![];
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &CONFIGURATORLIB,
            (),
        )?);
        evm_txs.push(build_deploy_contract_tx(self.address(), &DELAYLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &DEMANDSIDELIB,
            (),
        )?);
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &REDEMPTIONLIB,
            (),
        )?);
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &STATETRANSITIONSLIB,
            (),
        )?);
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &SUPPLYSIDELIB,
            (),
        )?);

        for tx in evm_txs {
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
        let _ptoken_logic_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 4));
        // current_nonce + 5 is initialization of the PToken logic.
        let ptoken_factory_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 6));
        let backstop_addr = EthersAddress::from(create_address(self.address(), current_nonce + 7));

        let mut evm_txs = vec![];

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
        evm_txs.push(build_deploy_contract_tx(
            self.address(),
            &MANAGER,
            manager_args,
        )?);

        // Deploy set logic.
        let set_args = (manager_addr, ptoken_factory_addr, backstop_addr);
        evm_txs.push(build_unlinked_deploy_contract_tx(
            self.address(),
            &SET,
            &libraries,
            set_args,
        )?);

        // Initialize set logic.
        let empty_market_configs: Vec<MarketConfig> = vec![];
        let weth_addr = state
            .world
            .as_ref()
            .ok_or(CozyAgentError::MissingWorldState)?
            .contract_registry
            .get(WETH.name)
            .ok_or(CozyAgentError::UnregisteredAddress)?;
        let set_initialize_args = (
            EthersAddress::zero(),
            EthersAddress::zero(),
            EthersAddress::from(*weth_addr),
            SetConfig {
                deposit_fee: 0,
                leverage_factor: 0,
            },
            empty_market_configs,
        );
        let call_data = encode_function(
            EthersBaseContract::from((*SET).abi.clone()),
            "initialize",
            set_initialize_args,
        )
        .expect("Issue encoding func.");

        evm_txs.push(build_call_contract_txenv(
            self.address(),
            set_logic_addr.into(),
            call_data,
            None,
            None,
        ));

        Ok(())
    }
}
