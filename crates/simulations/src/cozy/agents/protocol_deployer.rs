use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use crossbeam_channel::{unbounded, Receiver, Sender};
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
    state::{update::SimUpdate, SimState},
};
use thiserror::Error;

use crate::cozy::agents::errors::CozyAgentError;
use crate::cozy::bindings_utils;
use crate::cozy::{
    bindings_wrapper::*,
    world_state::CozyWorldStateUpdate,
    {EthersAddress, EthersBytes, EvmAddress},
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

impl Agent<CozyWorldStateUpdate> for ProtocolDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        sender: &Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) {
        // Deploy external libraries.
        self.deploy_libraries(state, sender);
        // Deploy core protocol.
        // self.deploy_core_protocol(state);
        // Deploy periphery.
        // self.deploy_periphery(state);
    }

    fn step(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        sender: &Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) {
    }
}

impl ProtocolDeployer {
    fn deploy_libraries(
        &mut self,
        _state: &SimState<CozyWorldStateUpdate>,
        sender: &Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) -> Result<()> {
        let mut evm_txs = vec![];

        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &CONFIGURATORLIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &DELAYLIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &DEMANDSIDELIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &CONFIGURATORLIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &REDEMPTIONLIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &STATETRANSITIONSLIB,
            (),
        )?);
        evm_txs.push(bindings_utils::build_deploy_contract_tx(
            self,
            &SUPPLYSIDELIB,
            (),
        )?);

        for tx in evm_txs {
            sender.send(SimUpdate::Evm(tx))?;
        }

        Ok(())
    }
}
