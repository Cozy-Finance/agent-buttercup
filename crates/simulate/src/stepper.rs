use left_right::{Absorb, ReadHandle, ReadHandleFactory, WriteHandle};
use revm::primitives::{AccountInfo, Address};

use crate::{
    agent::agent_channel::{AgentSimUpdate},
    state::{
        update::{Update},
        SimState,
    },
    time_policy::TimeEnv,
};

impl<U: Update> Absorb<AgentSimUpdate<U>> for SimState<U> {
    fn absorb_first(&mut self, operation: &mut AgentSimUpdate<U>, _: &Self) {
        self.execute(operation);
    }

    fn sync_with(&mut self, first: &Self) {
        *self = first.clone();
    }
}

pub struct SimStepper<U: Update> {
    pub read: ReadHandle<SimState<U>>,
    pub write: WriteHandle<SimState<U>, AgentSimUpdate<U>>,
}

impl<U: Update> SimStepper<U> {
    pub fn new_from_default() -> Self {
        // Initializes SimState<U> to its default.
        let (write, read) = left_right::new::<SimState<U>, AgentSimUpdate<U>>();
        SimStepper { read, write }
    }

    pub fn new_from_current_state(sim_state: SimState<U>) -> Self {
        // Clones SimState<U>.
        let (write, read) = left_right::new_from_empty::<SimState<U>, AgentSimUpdate<U>>(sim_state);
        SimStepper { read, write }
    }

    pub fn sim_state(&self) -> SimState<U> {
        self.read.enter().map(|guard| guard.clone()).unwrap()
    }

    fn sim_state_writer(&self) -> SimState<U> {
        self.write.enter().map(|guard| guard.clone()).unwrap()
    }

    pub fn append(&mut self, operation: AgentSimUpdate<U>) {
        self.write.append(operation);
    }

    pub fn publish(&mut self) {
        self.write.publish();
    }

    pub fn factory(&self) -> SimStepperReadHandleFactory<U> {
        let factory = self.read.factory();
        SimStepperReadHandleFactory { factory }
    }

    pub fn update_time_env(&mut self, time_env: TimeEnv) {
        self.sim_state_writer().update_time_env(time_env);
        self.write.publish();
    }

    pub fn insert_account_info(&mut self, address: Address, account_info: AccountInfo) {
        self.sim_state_writer()
            .insert_account_info(address, account_info);
        self.write.publish();
    }

    pub fn clear_all_results(&mut self) {
        self.sim_state_writer().clear_all_results();
        self.write.publish();
    }
}

pub struct SimStepperReadHandleFactory<U: Update> {
    factory: ReadHandleFactory<SimState<U>>,
}

impl<U: Update> SimStepperReadHandleFactory<U> {
    pub fn sim_state(&self) -> SimState<U> {
        self.factory
            .handle()
            .enter()
            .map(|guard| guard.clone())
            .unwrap()
    }
}
