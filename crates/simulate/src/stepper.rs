use left_right::{Absorb, ReadHandle, ReadHandleFactory, WriteHandle};
use revm::primitives::{AccountInfo};

use crate::{
    agent::agent_channel::AgentSimUpdate,
    state::{update::UpdateData, world::World, SimState},
    time_policy::TimeEnv, address::Address,
};

impl<U: UpdateData, W: World<WorldUpdateData = U>> Absorb<AgentSimUpdate<U>> for SimState<U, W> {
    fn absorb_first(&mut self, operation: &mut AgentSimUpdate<U>, _: &Self) {
        self.execute(operation);
    }

    fn sync_with(&mut self, first: &Self) {
        *self = first.clone();
    }
}
pub struct SimStepper<U: UpdateData, W: World<WorldUpdateData = U>> {
    pub read: ReadHandle<SimState<U, W>>,
    pub write: WriteHandle<SimState<U, W>, AgentSimUpdate<U>>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> SimStepper<U, W> {
    pub fn new_from_current_state(sim_state: SimState<U, W>) -> Self {
        // Clones SimState<U>.
        let (write, read) =
            left_right::new_from_empty::<SimState<U, W>, AgentSimUpdate<U>>(sim_state);
        SimStepper { read, write }
    }

    pub fn sim_state(&self) -> SimState<U, W> {
        self.read.enter().map(|guard| guard.clone()).unwrap()
    }

    pub fn sim_state_writer(&self) -> SimState<U, W> {
        self.write.enter().map(|guard| guard.clone()).unwrap()
    }

    pub fn append(&mut self, operation: AgentSimUpdate<U>) {
        self.write.append(operation);
    }

    pub fn publish(&mut self) {
        self.write.publish();
    }

    pub fn factory(&self) -> SimStepperReadHandleFactory<U, W> {
        let factory = self.read.factory();
        SimStepperReadHandleFactory { factory }
    }

    pub fn update_time_env(&mut self, time_env: TimeEnv) {
        self.sim_state_writer().update_time_env(time_env);
        self.publish();
    }

    pub fn insert_account_info(&mut self, address: Address, account_info: AccountInfo) {
        self.sim_state_writer()
            .insert_account_info(address, account_info);
        self.publish();
    }

    pub fn clear_all_results(&mut self) {
        self.sim_state_writer().clear_all_results();
        self.publish();
    }
}

pub struct SimStepperReadHandleFactory<U: UpdateData, W: World<WorldUpdateData = U>> {
    factory: ReadHandleFactory<SimState<U, W>>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> SimStepperReadHandleFactory<U, W> {
    pub fn sim_state(&self) -> SimState<U, W> {
        self.factory
            .handle()
            .enter()
            .map(|guard| guard.clone())
            .unwrap()
    }
}
