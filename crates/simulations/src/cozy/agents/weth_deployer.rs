use std::borrow::Cow;

use revm::primitives::create_address;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    contract::utils::build_deploy_tx_and_contract,
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    agents::errors::CozyAgentResult,
    bindings_wrapper::*,
    world::{CozyUpdate, CozyWorld},
    world_contracts::Weth,
};

pub struct WethDeployer {
    name: Cow<'static, str>,
    address: Address,
}

impl WethDeployer {
    pub fn new(name: Cow<'static, str>, address: Address) -> Self {
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
        log::info!("{:?} deploying wETH.", self.name);
        self.deploy_weth(state, channel)
            .expect("Weth deployer failed to deploy wETH.");
    }
}

impl WethDeployer {
    fn deploy_weth(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> CozyAgentResult<()> {
        let (evm_tx, weth_contract) = build_deploy_tx_and_contract(
            self.address,
            WETH.abi,
            WETH.bytecode.expect("Linked bytecode."),
            (),
        )?;
        channel.send(SimUpdate::Evm(evm_tx));

        let weth_addr = create_address(self.address.into(), 0);
        channel.send(SimUpdate::World(CozyUpdate::AddWeth(Weth::new(
            WETH.name.into(),
            weth_addr.into(),
            weth_contract,
        ))));

        Ok(())
    }
}
