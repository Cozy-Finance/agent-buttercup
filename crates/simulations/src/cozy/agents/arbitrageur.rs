use std::{borrow::Cow, sync::Arc};

use rand::{rngs::StdRng, Rng};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannelSender, Agent},
    state::State,
    u256::{U256, *},
    utils::decode_output,
};

use crate::cozy::{
    runner::{ProtocolContracts, SetContracts},
    set_risk_model::SetRiskModel,
    world::{CozyUpdate, CozyWorld},
};

#[derive(Debug, Clone)]
pub struct ArbitrageurPreferences {
    risk_model: SetRiskModel,
}

impl ArbitrageurPreferences {
    pub fn new(risk_model: SetRiskModel) -> Self {
        Self { risk_model }
    }
}

#[derive(Debug, Clone)]
pub struct Arbitrageur {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    preferences: ArbitrageurPreferences,
    rng: StdRng,
}

impl Arbitrageur {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
        preferences: ArbitrageurPreferences,
        rng: StdRng,
    ) -> Self {
        Self {
            _name,
            address,
            protocol,
            set,
            preferences,
            rng,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for Arbitrageur {
    fn address(&self) -> Address {
        self.address
    }

    fn activation_step(&mut self, state: &mut State<CozyUpdate, CozyWorld>) {
        let router_approve_tx = self
            .set
            .base_token
            .approve(self.protocol.cozy_router.address(), U256::MAX);
        let _ = state.execute_evm_tx_and_decode(self.address, router_approve_tx);

        for (_, ptoken) in self.set.ptokens.iter() {
            let ptoken_approve_tx = ptoken.approve(self.protocol.cozy_router.address(), U256::MAX);
            let _ = state.execute_evm_tx_and_decode(self.address, ptoken_approve_tx);
        }
    }

    fn step(
        &mut self,
        state: &State<CozyUpdate, CozyWorld>,
        channel: &AgentChannelSender<CozyUpdate>,
    ) {
        // Get current balance.
        let balance = state
            .call_evm_tx_and_decode(
                self.address,
                self.set.base_token.balance_of(self.address.into()),
            )
            .expect("Error getting balance.");

        // Choose market to arb.
        let target_market = self
            .rng
            .gen_range(0..self.preferences.risk_model.market_weights.len());

        // Compute current value of protection.
        let (current_ptokens, current_protection_value) =
            self.get_protection_owned(state, target_market);
        let fair_price_percentage = self.compute_fair_cost_percentage(target_market);
        if current_protection_value > U256::zero() {
            // Sell if you can arbitrage.
            let refund_value_tx = self.protocol.cozy_router.sell(
                self.set.set.address(),
                target_market as u16,
                current_ptokens,
                self.address.into(),
                U256::zero(),
            );
            let refund_value_call = state
                .call_evm_tx(self.address, refund_value_tx.clone())
                .expect("Error getting refund value.");
            if let simulate::state::EvmTxOutput::Success(output_bytes) = refund_value_call {
                let (refund_value, refund_protection_value): (U256, U256) =
                    decode_output(&refund_value_tx.function, output_bytes)
                        .expect("Error decoding purchase cost.");
                let refund_price_percentage =
                    u256_to_f64(refund_value) / u256_to_f64(refund_protection_value);
                if refund_price_percentage > fair_price_percentage {
                    let _ = channel.execute_evm_tx(refund_value_tx);
                }
            }
        } else {
            // Buy if you can arbitrage.
            let purchase_cost_tx = self.protocol.cozy_router.purchase(
                self.set.set.address(),
                target_market as u16,
                balance,
                self.address.into(),
                U256::MAX,
            );
            let purchase_cost_call = state
                .call_evm_tx(self.address, purchase_cost_tx.clone())
                .expect("Error getting purchase cost.");
            if let simulate::state::EvmTxOutput::Success(output_bytes) = purchase_cost_call {
                let (assets_needed, _): (U256, U256) =
                    decode_output(&purchase_cost_tx.function, output_bytes)
                        .expect("Error decoding purchase cost.");
                let purchase_cost_percentage = u256_to_f64(assets_needed) / u256_to_f64(balance);
                if purchase_cost_percentage <= fair_price_percentage {
                    let _ = channel.execute_evm_tx(purchase_cost_tx);
                }
            }
        }
    }
}

impl Arbitrageur {
    fn get_protection_owned(
        &self,
        state: &State<CozyUpdate, CozyWorld>,
        market_idx: usize,
    ) -> (U256, U256) {
        let ptoken = self
            .set
            .ptokens
            .get(&(market_idx as u32))
            .expect("Error getting PToken contract.");
        let current_ptoken_balance = state
            .call_evm_tx_and_decode(self.address, ptoken.balance_of_matured(self.address.into()))
            .expect("Error reading PToken balance.");
        let current_value = state
            .call_evm_tx_and_decode(
                self.address,
                self.set
                    .set
                    .convert_to_protection(market_idx as u16, current_ptoken_balance),
            )
            .expect("Error convert PTokens to value.");
        (current_ptoken_balance, current_value)
    }

    fn compute_fair_cost_percentage(&self, market_idx: usize) -> f64 {
        let riskless_fair_price = self.preferences.risk_model.annual_probabilities[market_idx];
        let leverage = self.preferences.risk_model.leverage;
        if leverage == 1.0 {
            return riskless_fair_price;
        } else {
            let probability_factor = self
                .preferences
                .risk_model
                .probability_other_markets_trigger[market_idx];
            let correlation_factor =
                self.preferences.risk_model.correlation_with_other_markets[market_idx].max(0.0);
            // If other markets are very likely to trigger, or if they are highly correlated
            // with this market, then the fair price should be discounted.
            let adjustment_factor = (1. - leverage * probability_factor)
                / (1. + leverage * correlation_factor).min(1.0).max(0.0);
            return riskless_fair_price * adjustment_factor;
        }
    }
}
