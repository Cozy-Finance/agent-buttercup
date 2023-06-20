use std::{borrow::Cow, collections::HashMap};

use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::build_call_contract_txenv,
};

use crate::cozy::{
    bindings_wrapper::*,
    types::CozyTokenDeployParams,
    utils::build_deploy_contract_tx,
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256, EvmAddress,
};

pub struct TokenDeployer {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
    deploy_args: CozyTokenDeployParams,
    allocate_addresses: HashMap<EvmAddress, EthersU256>,
    finished_allocating: bool,
}

impl TokenDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: EvmAddress,
        deploy_args: CozyTokenDeployParams,
        allocate_addresses: HashMap<EvmAddress, EthersU256>,
    ) -> Self {
        Self {
            name,
            address,
            deploy_args,
            allocate_addresses,
            finished_allocating: false,
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

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.finished_allocating {
            self.allocate_token(state, channel)
                .expect("Error allocating tokens");
            self.finished_allocating = true;
        }
    }
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
            DUMMYTOKEN.name.into(),
            CozyProtocolContract::new(dummy_token_addr, dummy_token_contract),
        )));

        Ok(())
    }

    fn allocate_token(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        let token = state.world.protocol_contracts.get(DUMMYTOKEN.name).unwrap();

        for (receiver, amount) in self.allocate_addresses.iter() {
            let call_data = token
                .contract
                .encode_function("mint", (EthersAddress::from(*receiver), *amount))?;
            let tx = build_call_contract_txenv(self.address, token.address, call_data, None, None);
            channel.send(SimUpdate::Evm(tx));
        }

        Ok(())
    }
}
