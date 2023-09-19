use std::{borrow::Cow, sync::Arc};

use simulate::{address::Address, agent::Agent, state::State, u256::U256};

use crate::cozy::{
    runner::{ProtocolContracts, SetContracts},
    world::{CozyUpdate, CozyWorld},
};

pub struct AltruisticSupplier {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
}

impl AltruisticSupplier {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
    ) -> Self {
        Self {
            _name,
            address,
            protocol,
            set,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for AltruisticSupplier {
    fn address(&self) -> Address {
        self.address
    }

    fn activation_step(&mut self, state: &mut State<CozyUpdate, CozyWorld>) {
        let balance = state
            .call_evm_tx_and_decode(
                self.address,
                self.set.base_token.balance_of(self.address.into()),
            )
            .expect("Error getting balance.");
        let router_approve_tx = self
            .set
            .base_token
            .approve(self.protocol.cozy_router.address(), U256::MAX);
        let deposit_tx = self.protocol.cozy_router.deposit(
            self.set.set.address(),
            balance,
            self.address.into(),
            U256::zero(),
        );
        let _ = state.execute_evm_tx_and_decode(self.address, router_approve_tx);
        let _ = state.execute_evm_tx_and_decode(self.address, deposit_tx);
        log::info!(
            "Altruistic supplier {} is supplying {} assets.",
            self.address,
            balance
        );
    }
}
