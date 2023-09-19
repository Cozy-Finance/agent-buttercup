use std::{borrow::Cow, sync::Arc};

use nalgebra::DVector;
use rand::{rngs::StdRng, Rng};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannelSender, Agent},
    state::State,
    u256::{U256, *},
    utils::decode_output,
};

use crate::cozy::{
    decay_normalizer::normalize_constant_decay_price,
    runner::{ProtocolContracts, SetContracts},
    set_risk_model::SetRiskModel,
    types::ReactionTime,
    world::{CozyUpdate, CozyWorld},
};

#[derive(Debug, Clone)]
pub struct BuyerPreferences {
    risk_model: SetRiskModel,
    market_allocations: DVector<f64>,
    reaction_time: ReactionTime,
}

impl BuyerPreferences {
    pub fn new(
        risk_model: SetRiskModel,
        market_allocations: DVector<f64>,
        reaction_time: ReactionTime,
    ) -> Self {
        Self {
            risk_model,
            market_allocations,
            reaction_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buyer {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    preferences: BuyerPreferences,
    rng: StdRng,
}

impl Buyer {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
        preferences: BuyerPreferences,
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

impl Agent<CozyUpdate, CozyWorld> for Buyer {
    fn address(&self) -> Address {
        self.address
    }

    fn activation_step(&mut self, state: &mut State<CozyUpdate, CozyWorld>) {
        let router_approve_tx = self
            .set
            .base_token
            .approve(self.protocol.cozy_router.address(), U256::MAX);
        let _ = state.execute_evm_tx_and_decode(self.address, router_approve_tx);
    }

    fn step(
        &mut self,
        state: &State<CozyUpdate, CozyWorld>,
        channel: &AgentChannelSender<CozyUpdate>,
    ) {
        if !self
            .preferences
            .reaction_time
            .time_to_react(state.timestamp(), &mut self.rng)
        {
            return;
        }

        // Get current balance.
        let balance = state
            .call_evm_tx_and_decode(
                self.address,
                self.set.base_token.balance_of(self.address.into()),
            )
            .expect("Error getting balance.");

        // Choose market and buy protection.
        if balance > U256::zero() {
            // Select random market and compute protection amount.
            let target_market = self
                .rng
                .gen_range(0..self.preferences.market_allocations.len());
            let purchase_amt = self.compute_protection_amount(target_market, balance);

            // Compute purchase cost and buy if under-priced.
            let purchase_cost_tx = self.protocol.cozy_router.purchase(
                self.set.set.address(),
                target_market as u16,
                purchase_amt,
                self.address.into(),
                U256::MAX,
            );
            let purchase_cost_call = state
                .call_evm_tx(self.address, purchase_cost_tx.clone())
                .expect("Error getting purchase cost.");
            if let simulate::state::EvmTxOutput::Success(output_bytes) = purchase_cost_call {
                let (assets_needed, protection_amt): (U256, U256) =
                    decode_output(&purchase_cost_tx.function, output_bytes)
                        .expect("Error decoding purchase cost.");
                let purchase_cost_percentage =
                    u256_to_f64(assets_needed) / u256_to_f64(purchase_amt);
                let purchase_cost_percentage = normalize_constant_decay_price(
                    purchase_cost_percentage,
                    state
                        .call_evm_tx_and_decode(
                            self.address,
                            self.set.drip_decay_models[&(target_market as u32)].rate_per_second(),
                        )
                        .expect("Error getting drip decay rate."),
                );

                let fair_price_percentage = self.compute_fair_cost_percentage(target_market);
                if purchase_cost_percentage <= fair_price_percentage {
                    log::info!(
                        "Buyering {} is buying {} protection.",
                        self.address,
                        protection_amt
                    );
                    channel.execute_evm_tx(purchase_cost_tx)
                }
            }
        }
    }
}

impl Buyer {
    fn compute_protection_amount(&self, market_idx: usize, balance: U256) -> U256 {
        let protection_amount =
            u256_to_f64(balance) * self.preferences.market_allocations[market_idx];
        f64_to_u256(protection_amount)
    }

    fn compute_fair_cost_percentage(&self, market_idx: usize) -> f64 {
        let riskless_fair_price = self.preferences.risk_model.annual_probabilities[market_idx];
        let leverage = self.preferences.risk_model.leverage;
        if leverage == 1.0 {
            riskless_fair_price
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
            riskless_fair_price * adjustment_factor
        }
    }
}
