use eyre::Result;
use simulate::{
    agent::{agent_channel::AgentChannel, Agent},
    state::{update::SimUpdate, SimState},
};

use crate::cozy::{
    bindings_wrapper::*,
    utils::build_deploy_contract_tx,
    world_state::{CozyUpdate, CozyWorld},
    EvmAddress,
};

pub struct WethDeployer {
    address: Option<EvmAddress>,
}

impl WethDeployer {
    pub fn new() -> Self {
        Self { address: None }
    }
}

impl Agent<CozyUpdate, CozyWorld> for WethDeployer {
    fn address(&self) -> EvmAddress {
        self.address.unwrap()
    }

    fn register_address(&mut self, address: &EvmAddress) {
        self.address = Some(*address);
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        self.deploy_weth(state, channel)
            .expect("Error deploying weth.");
    }

    fn step(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        _channel: AgentChannel<CozyUpdate>,
    ) {
    }

    fn resolve_step(&mut self, _state: &SimState<CozyUpdate, CozyWorld>) {}
}

impl WethDeployer {
    fn deploy_weth(
        &mut self,
        _state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) -> Result<()> {
        let evm_tx = build_deploy_contract_tx(self.address(), &WETH, ())?;
        channel.send(SimUpdate::Evm(evm_tx));
        Ok(())
    }
}
