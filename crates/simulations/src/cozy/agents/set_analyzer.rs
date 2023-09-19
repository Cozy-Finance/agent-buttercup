use std::{borrow::Cow, sync::Arc};

use nalgebra::DVector;
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannelSender, Agent},
    state::State,
    u256::U256,
};

use crate::cozy::{
    constants::SECONDS_IN_YEAR,
    runner::{ProtocolContracts, SetContracts},
    set_risk_model::SetRiskModel,
    utils::{wad, wad_to_float},
    world::{CozyUpdate, CozyWorld, SetAnalytics},
};

#[derive(Debug, Clone)]
pub struct SetAnalyzer {
    _name: Cow<'static, str>,
    address: Address,
    protocol: Arc<ProtocolContracts>,
    set: Arc<SetContracts>,
    true_risk_model: SetRiskModel,
}

impl SetAnalyzer {
    pub fn new(
        _name: Cow<'static, str>,
        address: Address,
        protocol: Arc<ProtocolContracts>,
        set: Arc<SetContracts>,
        true_risk_model: SetRiskModel,
    ) -> Self {
        Self {
            _name,
            address,
            protocol,
            set,
            true_risk_model,
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for SetAnalyzer {
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
        let utilizations = self
            .get_utilizations(state)
            .into_iter()
            .map(|x| wad_to_float(x))
            .collect::<Vec<f64>>();
        let (market_apys, set_apy) = self.get_apys(state, self.get_per_second_fee_drips(state));
        let portfolio_weights = utilizations
            .iter()
            .enumerate()
            .map(|(i, u)| {
                u * self.true_risk_model.leverage * self.true_risk_model.market_weights[i]
            })
            .collect::<Vec<f64>>();
        let world_update = CozyUpdate::UpdateSetAnalytics(SetAnalytics::new(
            set_apy,
            DVector::from(market_apys),
            DVector::from(portfolio_weights),
            DVector::from(utilizations),
        ));
        let _ = channel.execute_world_update(world_update);
    }
}

impl SetAnalyzer {
    fn get_utilizations(&self, state: &State<CozyUpdate, CozyWorld>) -> Vec<U256> {
        let mut utilizations = vec![];
        for i in 0..self.true_risk_model.market_weights.len() {
            let utilization = state
                .call_evm_tx_and_decode(self.address, self.set.set.utilization(i as u16))
                .expect("Error getting utilization.");
            utilizations.push(utilization);
        }
        utilizations
    }

    fn get_per_second_fee_drips(&self, state: &State<CozyUpdate, CozyWorld>) -> Vec<U256> {
        let mut returns = vec![];
        for i in 0..self.true_risk_model.market_weights.len() {
            let (_, _, _, _, _, _, last_drip_rate, purchases_fee_pool, sales_fee_pool, _) = state
                .call_evm_tx_and_decode(self.address, self.set.set.markets(U256::from(i)))
                .expect("Error getting market return.");
            let total_fees = purchases_fee_pool + sales_fee_pool;
            returns.push(last_drip_rate * U256::from(total_fees))
        }
        returns
    }

    fn get_apys(
        &self,
        state: &State<CozyUpdate, CozyWorld>,
        per_second_fee_drips: Vec<U256>,
    ) -> (Vec<f64>, f64) {
        let (asset_balance, _, _, _, _, _, _) = state
            .call_evm_tx_and_decode(self.address, self.set.set.accounting())
            .expect("Error getting accounting.");

        if asset_balance > 0 {
            let mut market_apys = vec![];
            let mut total_per_second_fee_drip = U256::zero();
            for (i, per_second_fee_drip) in per_second_fee_drips.iter().enumerate() {
                total_per_second_fee_drip += *per_second_fee_drip;
                let per_second_yield = (per_second_fee_drip / asset_balance).as_u128();
                let unscaled_market_apy = ((per_second_yield * SECONDS_IN_YEAR as u128) as f64)
                    / (wad().as_u128() as f64);
                market_apys.push(unscaled_market_apy / self.true_risk_model.market_weights[i]);
            }
            let set_per_second_yield = (total_per_second_fee_drip / asset_balance).as_u128();
            let set_apy = ((set_per_second_yield * SECONDS_IN_YEAR as u128) as f64)
                / (wad().as_u128() as f64);
            return (market_apys, set_apy);
        }
        return (vec![0.0; per_second_fee_drips.len()], 0.0);
    }
}
