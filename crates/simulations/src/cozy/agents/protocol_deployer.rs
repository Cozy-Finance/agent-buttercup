use bindings::cozy_protocol::shared_types::{Delays, Fees, MarketConfig, SetConfig};
use crossbeam_channel::{unbounded, Receiver, Sender};
use ethers::abi::{Contract as EthersContract, Tokenize};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::agent_channel::AgentChannel,
    agent::Agent,
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
        channel: AgentChannel<CozyWorldStateUpdate>,
    ) {
        // Deploy external libraries.
        self.deploy_libraries(state, &channel);
        // Deploy core protocol.
        //self.deploy_core_protocol(state, &sender);
        // Deploy periphery.
        // self.deploy_periphery(state);
    }

    fn step(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        channel: AgentChannel<CozyWorldStateUpdate>,
    ) {
        // Deploy external libraries.
        self.deploy_core_protocol(state, &channel);
    }

    fn resolve_step(&mut self, state: &SimState<CozyWorldStateUpdate>) {
        println!("{:?}", state.get_results(&self.address()).get_update("test 20"));
    }

}

impl ProtocolDeployer {
    fn deploy_libraries(
        &mut self,
        _state: &SimState<CozyWorldStateUpdate>,
        channel: &AgentChannel<CozyWorldStateUpdate>,
    ) -> Result<()> {
        let mut evm_txs = vec![];
        evm_txs.push(build_deploy_contract_tx(self, &CONFIGURATORLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &DELAYLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &DEMANDSIDELIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &CONFIGURATORLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &REDEMPTIONLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &STATETRANSITIONSLIB, ())?);
        evm_txs.push(build_deploy_contract_tx(self, &SUPPLYSIDELIB, ())?);

        let mut num = 3;
        for tx in evm_txs {
            channel.send_with_tag(SimUpdate::Evm(tx), &format!("test {}", num));
            num += 1;
        }

        channel.send_with_tag(
            SimUpdate::World(Box::new(CozyWorldStateUpdate::AddToContractRegistry(
                "x".to_string(),
                EvmAddress::from_low_u64_be(3),
            ))),
            "test",
        );

        Ok(())
    }

    fn deploy_core_protocol(
        &mut self,
        state: &SimState<CozyWorldStateUpdate>,
        channel: &AgentChannel<CozyWorldStateUpdate>,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let current_nonce = state.get_account_info(self.address()).unwrap().nonce;
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
