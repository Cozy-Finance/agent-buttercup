use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

use revm::primitives::U256 as EvmU256;

use crate::address::Address;

#[derive(Debug, Clone)]
pub struct AgentId {
    pub address: Address,
    pub name: Option<Cow<'static, str>>,
}

impl Hash for AgentId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.address.hash(state);
    }
}

impl PartialEq for AgentId {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
impl Eq for AgentId {}

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
