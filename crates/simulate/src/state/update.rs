use std::fmt::Debug;

use revm::primitives::{ExecutionResult, TxEnv};

pub trait Update: Send + Sync + Clone + Debug {}

#[derive(Debug, Clone)]
pub enum SimUpdate<U: Update> {
    Evm(TxEnv),
    World(Box<U>),
    Bundle(TxEnv, Box<U>),
    MultiBundle(Vec<TxEnv>, Vec<Box<U>>),
}

#[derive(Debug, Clone)]
pub enum SimUpdateResult {
    Evm(ExecutionResult),
    Bundle(bool),
    MultiBundle(bool),
}
