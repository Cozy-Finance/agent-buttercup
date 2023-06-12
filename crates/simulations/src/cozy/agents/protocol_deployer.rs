use bindings::cozy_protocol::shared_types::{Delays, Fees};
use ethers::types::U256 as EthersU256;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    bindings_wrapper::*, utils::build_deploy_contract_tx, world_state::CozyUpdate, EthersAddress,
    EvmAddress,
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

impl Agent<CozyUpdate> for ProtocolDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(&mut self, state: &SimState<CozyUpdate>, channel: AgentChannel<CozyUpdate>) {
        // Deploy external libraries.
        self.deploy_libraries(state, &channel);
        // Deploy core protocol.
        //self.deploy_core_protocol(state, &sender);
        // Deploy periphery.
        // self.deploy_periphery(state);
    }

    fn step(&mut self, state: &SimState<CozyUpdate>, channel: AgentChannel<CozyUpdate>) {
        // Deploy external libraries.
        self.deploy_core_protocol(state, &channel);
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate>) {}
}

impl ProtocolDeployer {
    fn deploy_libraries(
        &mut self,
        _state: &SimState<CozyUpdate>,
        channel: &AgentChannel<CozyUpdate>,
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
            channel.send_with_tag(SimUpdate::Evm(tx), format!("test {}", num).into());
            num += 1;
        }

        channel.send_with_tag(
            SimUpdate::World(CozyUpdate::AddToContractRegistry(
                "x".to_string(),
                EvmAddress::from_low_u64_be(3),
            )),
            "test".into(),
        );

        Ok(())
    }

    fn deploy_core_protocol(
        &mut self,
        state: &SimState<CozyUpdate>,
        _channel: &AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        // Pre-compute Cozy protocol addresses
        let y = vec![3, 4, 5];
        let x = format!("{}", y.len());
        let z = &x;

        _channel.send_with_tag(
            SimUpdate::World(CozyUpdate::AddToContractRegistry(
                "x".to_string(),
                EvmAddress::from_low_u64_be(3),
            )),
            x.into(),
        );

        let current_nonce = state.get_account_info(self.address()).unwrap().nonce;
        let _manager_addr = EthersAddress::from(create_address(self.address(), current_nonce));
        let _set_logic_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 1));
        // current_nonce + 2 is initialization of the Set logic.
        let _set_factory_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 3));
        let _ptoken_logic_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 4));
        // current_nonce + 5 is initialization of the PToken logic.
        let _ptoken_factory_addr =
            EthersAddress::from(create_address(self.address(), current_nonce + 6));
        let _backstop_addr = EthersAddress::from(create_address(self.address(), current_nonce + 7));

        Ok(())
    }
}
