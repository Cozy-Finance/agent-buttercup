use std::{borrow::Cow, sync::Arc};

use bindings::cozy_protocol::cozy_router;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    u256::{f64_to_u256, U256},
};

use crate::cozy::{
    world::{CozyUpdate, CozyWorld},
    world_contracts::{CozyBaseToken, CozyRouter},
};

pub struct PassiveSupplier {
    name: Cow<'static, str>,
    address: Address,
    cozyrouter: Arc<CozyRouter>,
    token: Arc<CozyBaseToken>,
    capital: U256,
    waiting_time: U256,
    last_action_time: U256,
}

impl PassiveSupplier {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        cozyrouter: &Arc<CozyRouter>,
        token: &Arc<CozyBaseToken>,
        waiting_time: f64,
    ) -> Self {
        Self {
            name,
            address,
            cozyrouter: cozyrouter.clone(),
            token: token.clone(),
            capital: U256::zero(),
            waiting_time: f64_to_u256(waiting_time),
            last_action_time: U256::zero(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for PassiveSupplier {
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
        channel.send(SimUpdate::Evm(
            self.token
                .build_max_approve_router_tx(self.address, self.cozyrouter.address)
                .expect("PassiveSupplier failed to build approve router tx."),
        ));
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("PassiveSupplier failed to read token balance.");
        self.last_action_time = state.read_timestamp();
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        if !self.is_time_to_act(state.read_timestamp()) || self.capital <= U256::zero() {
            return;
        }

        let mut sets = state.world.sets.values().clone();
        if !sets.is_empty() {
            sets.sort_by(|a, b| a.apy.partial_cmp(&b.apy).expect("f64 comparison."));
            let deposit_tx = self
                .cozyrouter
                .build_deposit_tx(
                    self.address,
                    cozy_router::DepositCall {
                        set: sets[0].address.into(),
                        assets: self.capital,
                        receiver: self.address.into(),
                        min_shares_received: U256::zero(),
                    },
                )
                .expect("PassiveSupplier failed to build deposit tx.");
            channel.send(SimUpdate::Evm(deposit_tx));
        }
    }

    fn resolve_step(&mut self, state: &SimState<CozyUpdate, CozyWorld>) {
        if !self.is_time_to_act(state.read_timestamp()) {
            return;
        }
        self.capital = self
            .token
            .read_token_balance(self.address, state)
            .expect("PassiveSupplier failed to read token balance.");
        self.last_action_time = state.read_timestamp();
    }
}

impl PassiveSupplier {
    fn is_time_to_act(&self, curr_timestamp: U256) -> bool {
        (curr_timestamp - self.last_action_time) >= self.waiting_time
    }
}
