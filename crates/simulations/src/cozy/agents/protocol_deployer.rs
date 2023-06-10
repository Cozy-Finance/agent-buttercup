use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::agent::Agent,
    contract::{
        sim_contract::{IsDeployed, SimContract},
        utils,
    },
    state::{update::SimUpdate, SimState},
};
use thiserror::Error;

use crate::cozy::agents::errors::CozyAgentError;
use crate::cozy::utils::build_deploy_contract_tx;
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
        sender: Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) {
        // Deploy external libraries.
        self.deploy_libraries(state, &sender);
        // Deploy core protocol.
        self.deploy_core_protocol(state, &sender);
        // Deploy periphery.
        // self.deploy_periphery(state);
    }

    fn step(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        sender: Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) {
        // Deploy external libraries.
        self.deploy_libraries(state, &sender);
    }
}

impl ProtocolDeployer {
    fn deploy_libraries(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        sender: &Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) -> Result<()> {
        let mut evm_txs = vec![];
        //println!("{:}", current_nonce);
        evm_txs.push(build_deploy_contract_tx(self, &CONFIGURATORLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &DELAYLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &DEMANDSIDELIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &CONFIGURATORLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &REDEMPTIONLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &STATETRANSITIONSLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &SUPPLYSIDELIB, ())?);

        for tx in evm_txs {
            sender.send(SimUpdate::Evm(tx))?;
        }

        Ok(())
    }

    fn deploy_core_protocol(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        sender: &Sender<SimUpdate<CozyWorldStateUpdate>>,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let current_nonce = 3;
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

        Ok(())
    }
}
