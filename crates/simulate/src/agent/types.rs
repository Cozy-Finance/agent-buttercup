use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use revm::primitives::{Address as EvmAddress, U256 as EvmU256};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AgentId {
    pub address: EvmAddress,
    pub name: Option<Cow<'static, str>>,
}

impl Hash for AgentId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct AgentTxGas {
    pub gas_limit: u64,
    pub gas_price: EvmU256,
    pub gas_priority_fee: Option<EvmU256>,
}

impl Default for AgentTxGas {
    fn default() -> Self {
        Self {
            gas_limit: u64::MAX,
            gas_price: EvmU256::ZERO,
            gas_priority_fee: None,
        }
    }
}
