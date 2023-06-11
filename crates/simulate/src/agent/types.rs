use revm::primitives::U256 as EvmU256;

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
