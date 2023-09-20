use std::{borrow::Cow, sync::Arc};

use rand::rngs::StdRng;
use simulate::{
    address::Address,
    agent::{
        agent_channel::{AgentChannelReceiver, AgentChannelSender},
        Agent,
    },
    state::State,
    u256::{U256, *},
};

use crate::cozy::{
    runner::{ProtocolContracts, SetContracts},
    set_risk_model::SetRiskModel,
    types::ReactionTime,
    world::{CozyUpdate, CozyWorld},
};

#[derive(Debug, Clone)]
pub struct RedemptionTracker {
    pub queued: bool,
    pub queue_timestamp: Option<U256>,
    pub redemption_id: Option<u64>,
    pub redemption_delay: Option<u32>,
}

impl RedemptionTracker {
    pub fn new(queued: bool) -> Self {
        Self {
            queued,
            queue_timestamp: None,
            redemption_id: None,
            redemption_delay: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SupplierPreferences {
    risk_model: SetRiskModel,
    risk_aversion: f64,
    total_wealth: U256,
    reaction_time: ReactionTime,
}

impl SupplierPreferences {
    pub fn new(
        risk_model: SetRiskModel,
        risk_aversion: f64,
        total_wealth: U256,
        reaction_time: ReactionTime,
    ) -> Self {
        Self {
            risk_model,
            risk_aversion,
            total_wealth,
            reaction_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Supplier {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    preferences: SupplierPreferences,
    redemption_tracker: RedemptionTracker,
    rng: StdRng,
}

impl Supplier {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
        preferences: SupplierPreferences,
        rng: StdRng,
    ) -> Self {
        Self {
            _name,
            address,
            protocol,
            set,
            preferences,
            redemption_tracker: RedemptionTracker::new(false),
            rng,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for Supplier {
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
        if self.redemption_tracker.queued {
            let realized_delay = state.timestamp()
                - self
                    .redemption_tracker
                    .queue_timestamp
                    .expect("Error getting queue timestamp.");
            if realized_delay.as_u32()
                > self
                    .redemption_tracker
                    .redemption_delay
                    .expect("Error getting redemption delay.")
            {
                let id = self
                    .redemption_tracker
                    .redemption_id
                    .expect("Error getting redemption id.");
                let complete_redeem_tx = self.set.set.complete_redeem(id);
                channel.execute_evm_tx(complete_redeem_tx);
                self.redemption_tracker = RedemptionTracker::new(false);
                log::info!(
                    "Supplier {} is completing redemption id {}.",
                    self.address,
                    id
                );
                return;
            }
        }

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
        let current_set_shares = state
            .call_evm_tx_and_decode(
                self.address,
                self.set.set.balance_of_matured(self.address.into()),
            )
            .expect("Error getting shares.");
        let current_value = state
            .call_evm_tx_and_decode(
                self.address,
                self.set.set.convert_to_assets(current_set_shares),
            )
            .expect("Error converting shares to assets.");
        self.preferences.total_wealth = balance + current_value;

        // Compute optimal allocation.
        let optimal_allocation = self.compute_optimal_allocation(state);
        let optimal_allocation_value =
            f64_to_u256(optimal_allocation) * self.preferences.total_wealth;

        // Supply or withdraw to target optimal allocation.
        match optimal_allocation_value.cmp(&current_value) {
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => {
                let supply_amount = optimal_allocation_value - current_value;
                let router_supply_tx = self.protocol.cozy_router.deposit(
                    self.set.set.address(),
                    supply_amount,
                    self.address.into(),
                    U256::zero(),
                );
                log::info!(
                    "Supplier {} is deposting {} assets.",
                    self.address,
                    supply_amount
                );
                channel.execute_evm_tx(router_supply_tx);
            }
            std::cmp::Ordering::Less => {
                let withdraw_amount = current_value - optimal_allocation_value;
                let redeem_shares = state
                    .call_evm_tx_and_decode(
                        self.address,
                        self.set.set.convert_to_shares(withdraw_amount),
                    )
                    .expect("Error converting assets to shares.");
                let redeem_tx =
                    self.set
                        .set
                        .redeem(redeem_shares, self.address.into(), self.address.into());
                log::info!(
                    "Supplier {} is withdrawing {} assets.",
                    self.address,
                    withdraw_amount
                );
                channel.execute_evm_tx(redeem_tx);
                self.redemption_tracker = RedemptionTracker::new(true);
            }
        }
    }

    fn resolve_step(
        &mut self,
        state: &State<CozyUpdate, CozyWorld>,
        channel: &AgentChannelReceiver<CozyUpdate>,
    ) {
        if !self.redemption_tracker.queued {
            return;
        }
        if let Ok(redemption_output) = channel.receive_evm_tx_output() {
            match redemption_output {
                Some(successful_redemption_output) => {
                    let (id, _): (u64, U256) = successful_redemption_output;
                    self.redemption_tracker.redemption_id = Some(id);
                    self.redemption_tracker.queue_timestamp = Some(state.timestamp());
                    self.redemption_tracker.redemption_delay = Some(
                        state
                            .call_evm_tx_and_decode(
                                self.address,
                                self.protocol.manager.redemption_delay(),
                            )
                            .expect("Error getting redemption delay."),
                    )
                }
                None => {
                    self.redemption_tracker.queued = false;
                }
            }
        }
    }
}

impl Supplier {
    fn compute_optimal_allocation(&self, state: &State<CozyUpdate, CozyWorld>) -> f64 {
        let portfolio_weights = &state.world.set_analytics.portfolio_weights;
        let annual_market_apys = &state.world.set_analytics.portfolio_weights;
        if !portfolio_weights.is_empty() {
            let set_variance = self.preferences.risk_model.variance(portfolio_weights);
            let set_risk_premium = self
                .preferences
                .risk_model
                .set_risk_premium(annual_market_apys, portfolio_weights);
            (set_risk_premium / (set_variance * self.preferences.risk_aversion))
                .max(0.0)
                .min(1.0)
        } else {
            0.0
        }
    }
}
