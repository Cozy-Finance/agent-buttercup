use std::borrow::Cow;

use eyre::Result;
use revm::primitives::create_address;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    bindings_wrapper::*,
    utils::build_deploy_contract_tx,
    world::{CozyProtocolContract, CozyUpdate, CozyWorld},
    EvmAddress,
};

pub struct WethDeployer {
    name: Option<Cow<'static, str>>,
    address: EvmAddress,
}

impl WethDeployer {
    pub fn new(name: Option<Cow<'static, str>>, address: EvmAddress) -> Self {
        Self { name, address }
    }
}

impl Agent<CozyUpdate, CozyWorld> for WethDeployer {
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
        self.deploy_weth(state, channel)
            .expect("Error deploying weth.");
    }
}

impl WethDeployer {
    fn deploy_weth(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        let (evm_tx, weth_contract) = build_deploy_contract_tx(self.address, &WETH, ())?;
        channel.send(SimUpdate::Evm(evm_tx));

        let weth_addr = create_address(self.address, 0);
        channel.send(SimUpdate::World(CozyUpdate::AddToProtocolContracts(
            WETH.name.into(),
            CozyProtocolContract::new(weth_addr, weth_contract),
        )));

        Ok(())
    }
}
