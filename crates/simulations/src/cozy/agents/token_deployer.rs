use std::borrow::Cow;

use ethers::abi::Tokenize;
use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    bindings_wrapper::*,
    types::CozyTokenDeployParams,
    utils::build_deploy_contract_tx,
    world::{CozyUpdate, CozyWorld},
    EthersAddress, EvmAddress,
};

pub struct TokenDeployer {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    deploy_args: CozyTokenDeployParams,
}

impl TokenDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        deploy_args: CozyTokenDeployParams,
    ) -> Self {
        Self {
            name,
            address,
            deploy_args,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for TokenDeployer {
    fn id(&self) -> AgentId {
        AgentId {
            name: self.name.clone(),
            address: self.address,
        }
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        self.deploy_token(state, channel)
            .expect("Error deploying token.");
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

impl TokenDeployer {
    fn deploy_token(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        let (evm_tx, dummy_token_contract) = build_deploy_contract_tx(
            self.address,
            &DUMMYTOKEN,
            (
                self.deploy_args.name.to_string(),
                self.deploy_args.symbol.to_string(),
                self.deploy_args.decimals,
            ),
        )?;
        channel.send(SimUpdate::Evm(evm_tx));

        let dummy_token_addr = create_address(self.address, 0);
        channel.send(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            "DummyToken".into(),
            dummy_token_addr,
            dummy_token_contract,
        )));

        Ok(())
    }
}
