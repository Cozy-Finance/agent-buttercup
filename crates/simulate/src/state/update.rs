use std::fmt::Debug;

use revm::primitives::{ExecutionResult, TxEnv};

pub trait UpdateData: Send + Sync + Debug + Clone {}

#[derive(Debug, Clone)]
pub enum SimUpdate<U: UpdateData> {
    Evm(TxEnv),
    World(U),
    Bundle(TxEnv, U),
    MultiBundle(Vec<TxEnv>, Vec<U>),
}

#[derive(Debug, Clone)]
pub enum SimUpdateResult<U: UpdateData> {
    Evm(ExecutionResult),
    World(Option<U>),
    Bundle(bool, ExecutionResult, Option<U>),
    MultiBundle(bool, Vec<ExecutionResult>, Vec<Option<U>>),
}
