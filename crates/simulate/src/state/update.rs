use std::fmt::Debug;

use revm::primitives::TxEnv;

pub trait Update: Send + Sync + Clone + Debug {}

#[derive(Debug)]
pub enum SimUpdate<U: Update> {
    Evm(TxEnv),
    World(Box<U>),
    Bundle(TxEnv, Box<U>),
    MultiBundle(Vec<TxEnv>, Vec<Box<U>>),
}
