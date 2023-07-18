use std::{borrow::Cow, collections::HashMap, sync::Arc};

use revm::primitives::create_address;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::utils::build_deploy_tx_and_contract,
    state::{update::SimUpdate, SimState},
    u256::U256,
    utils::build_call_tx,
};

use crate::cozy::{
    agents::errors::CozyAgentResult,
    bindings_wrapper::*,
    constants::BASE_TOKEN,
    types::CozyTokenDeployParams,
    world::{CozyUpdate, CozyWorld},
    world_contracts::CozyBaseToken,
    EthersAddress,
};

pub struct TokenDeployer {
    name: Cow<'static, str>,
    address: Address,
    deploy_args: CozyTokenDeployParams,
    allocate_addrs: HashMap<Address, U256>,
    finished_allocating: bool,
    token: Option<Arc<CozyBaseToken>>,
}

impl TokenDeployer {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        deploy_args: CozyTokenDeployParams,
        allocate_addrs: HashMap<Address, U256>,
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
            .expect("TokenDeployer failed to deploy token.");
    }

    fn resolve_activation_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        self.token = state.world.base_token.clone();
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.finished_allocating {
            self.allocate_token(state, channel)
                .expect("TokenDeployer failed to allocate tokens.");
            self.finished_allocating = true;
        }
    }
}

impl TokenDeployer {
    fn deploy_token(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> CozyAgentResult<()> {
        let (evm_tx, dummy_token_contract) = build_deploy_tx_and_contract(
            self.address,
            DUMMYTOKEN.abi,
            DUMMYTOKEN.bytecode.expect("Linked bytecode."),
            (
                self.deploy_args.name.to_string(),
                self.deploy_args.symbol.to_string(),
                self.deploy_args.decimals,
            ),
        )?;
        channel.send(SimUpdate::Evm(evm_tx));

        let dummy_token_addr: Address = Address::from(create_address(self.address.into(), 0));
        channel.send(SimUpdate::World(CozyUpdate::AddCozyBaseToken(
            CozyBaseToken::new(BASE_TOKEN.into(), dummy_token_addr, dummy_token_contract),
        )));

        Ok(())
    }

    fn allocate_token(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> CozyAgentResult<()> {
        for (receiver, amount) in self.allocate_addrs.iter() {
            let receiver_address: EthersAddress = (*receiver).into();
            let call_data = self
                .token
                .as_ref()
                .expect("Token set.")
                .contract
                .encode_function("mint", (receiver_address, *amount))?;
            let tx = build_call_tx(
                self.address,
                self.token.as_ref().expect("Token set").address,
                call_data,
            );
            channel.send(SimUpdate::Evm(tx));
        }

        Ok(())
    }
}
