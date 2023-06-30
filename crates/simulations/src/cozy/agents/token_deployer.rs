use std::{borrow::Cow, collections::HashMap, sync::Arc};

use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::build_call_contract_txenv,
};

use crate::cozy::{
    bindings_wrapper::*,
    constants::BASE_TOKEN,
    types::CozyTokenDeployParams,
    utils::build_deploy_contract_tx,
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256,
};

pub struct TokenDeployer {
    name: Option<Cow<'static, str>>,
    address: Address,
    deploy_args: CozyTokenDeployParams,
    allocate_addrs: HashMap<Address, EthersU256>,
    finished_allocating: bool,
    token: Option<Arc<CozyProtocolContract>>,
}

impl TokenDeployer {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        deploy_args: CozyTokenDeployParams,
        allocate_addrs: HashMap<Address, EthersU256>,
    ) -> Self {
        Self {
            name,
            address,
            deploy_args,
            allocate_addrs,
            finished_allocating: false,
            token: None,
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
        log::info!("{:?} deploying base token.", self.name);
        self.deploy_token(state, channel)
            .expect("Error deploying token.");
    }

    fn resolve_activation_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.token = Some(
            state
                .world
                .protocol_contracts
                .get_by_name(&BASE_TOKEN)
                .unwrap()
                .clone(),
        );
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

        let dummy_token_addr: Address = Address::from(create_address(self.address.into(), 0));
        channel.send(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            CozyProtocolContract::new(BASE_TOKEN.into(), dummy_token_addr, dummy_token_contract),
        )));

        Ok(())
    }

    fn allocate_token(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        for (receiver, amount) in self.allocate_addrs.iter() {
            let receiver_address: EthersAddress = (*receiver).into();
            let call_data = self
                .token
                .as_ref()
                .unwrap()
                .contract
                .encode_function("mint", (receiver_address, *amount))?;
            let tx = build_call_contract_txenv(
                self.address,
                self.token.as_ref().unwrap().address,
                call_data,
                None,
                None,
            );
            channel.send(SimUpdate::Evm(tx));
        }

        Ok(())
    }
}
