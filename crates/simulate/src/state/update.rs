use std::fmt::Debug;

use crossbeam_channel::Sender;
use ethers::{abi::Function, types::transaction::eip2718::TypedTransaction};
use revm::primitives::TxEnv;

use crate::{address::Address, state::EvmTxOutput};

pub trait Update: Send + Sync + Debug {}

#[derive(Debug, Clone)]
pub enum EvmStateUpdate {
    Execute(
        Address,
        TypedTransaction,
        Function,
        Sender<EvmStateUpdateOutput>,
    ),
    ExecuteRaw(Address, TxEnv, Sender<EvmStateUpdateOutput>),
}

#[derive(Debug, Clone)]
pub enum EvmStateUpdateOutput {
    Execute(EvmTxOutput, Function),
    ExecuteRaw(EvmTxOutput),
}

#[derive(Debug, Clone)]
pub struct WorldStateUpdate<U: Update> {
    pub update: U,
    pub result_sender: Sender<U>,
}
