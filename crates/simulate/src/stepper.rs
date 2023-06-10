use left_right::{Absorb, ReadHandle, ReadHandleFactory, WriteHandle};

use crate::state::{
    update::{SimUpdate, Update},
    SimState,
};

impl<U: Update> Absorb<SimUpdate<U>> for SimState<U> {
    fn absorb_first(&mut self, operation: &mut SimUpdate<U>, _: &Self) {
        self.execute(operation);
    }

    fn sync_with(&mut self, first: &Self) {
        *self = first.clone();
    }
}

pub struct SimStepper<U: Update> {
    pub read: ReadHandle<SimState<U>>,
    pub write: WriteHandle<SimState<U>, SimUpdate<U>>,
}

impl<U: Update> SimStepper<U> {
    pub fn new() -> Self {
        let (write, read) = left_right::new::<SimState<U>, SimUpdate<U>>();
        SimStepper { read, write }
    }

    pub fn sim_state(&self) -> SimState<U> {
        self.read
            .enter()
            .map(|guard| guard.clone())
            .unwrap()
    }

    pub fn append(&mut self, operation: SimUpdate<U>) {
        self.write.append(operation);
    }

    pub fn publish(&mut self) {
        self.write.publish();
    }

    pub fn factory(&self) -> SimStepperReadHandleFactory<U> {
        let factory = self.read.factory();
        SimStepperReadHandleFactory { factory }
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
