use std::{borrow::Cow, collections::HashMap, sync::Arc};

pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    cozy_protocol::shared_types::{MarketConfig, SetConfig},
    drip_decay_model_constant_factory, manager,
    set::{AccountingReturn, MarketsReturn},
};
use eyre::Result;
use revm::primitives::{bitvec::macros::internal::funty::Fundamental, TxEnv};
use simulate::{
    address::Address,
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_tx, unpack_execution},
};

pub use crate::cozy::constants;
use crate::cozy::{
    constants::SECONDS_IN_YEAR,
    types::CozySetAdminParams,
    world::{CozySet, CozyUpdate, CozyWorld},
    world_contracts::{CozyManager, CozySetLogic},
    EthersU256,
};

pub struct SetAdmin {
    name: Cow<'static, str>,
    address: Address,
    set_admin_params: CozySetAdminParams,
    set_address: Option<Address>,
    set_name: Option<String>,
    set_logic: Arc<CozySetLogic>,
    manager: Arc<CozyManager>,
}

impl SetAdmin {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        set_admin_params: CozySetAdminParams,
        set_logic: &Arc<CozySetLogic>,
        manager: &Arc<CozyManager>,
    ) -> Self {
        Self {
            name,
            address,
            set_admin_params,
            set_address: None,
            set_name: None,
            set_logic: set_logic.clone(),
            manager: manager.clone(),
        }
    }
}

impl Agent<CozyUpdate, CozyWorld> for SetAdmin {
    fn id(&self) -> AgentId {
        AgentId {
            name: self.name.clone(),
            address: self.address,
        }
    }

    fn activation_step(
        &mut self,
        state: &SimState<CozyUpdate, CozyWorld>,
        channel: AgentChannel<CozyUpdate>,
    ) {
        self.set_name = Some(format!("{:?}'s Set", self.name));
        log::info!("{:?} deploying {:?}.", self.name, self.set_name);

        let create_set_args = manager::CreateSetCall {
            owner: self.address.into(),
            pauser: self.address.into(),
            asset: self.set_admin_params.asset.into(),
            set_config: self.set_admin_params.set_config.clone(),
            market_configs: self.set_admin_params.market_configs.clone(),
            salt: self
                .set_admin_params
                .salt
                .unwrap_or(rand::random::<[u8; 32]>()),
        };

        let (set_addr, evm_tx) = self
            .manager
            .build_create_set_tx(self.address, state, create_set_args)
            .unwrap();
        self.set_address = Some(set_addr);

        let trigger_lookup = self
            .set_admin_params
            .market_configs
            .iter()
            .enumerate()
            .map(|(i, config)| (config.trigger.into(), i as u16))
            .collect::<HashMap<_, _>>();

        let world_update = CozyUpdate::AddToSets(CozySet::new(
            self.set_name.clone().unwrap().into(),
            self.set_address.unwrap(),
            trigger_lookup,
        ));

        channel.send(SimUpdate::Bundle(evm_tx, world_update));
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        let apy = self.compute_current_apy(state).unwrap();

        log::info!("{:?} apy: {}", self.set_name, apy);
        channel.send(SimUpdate::World(CozyUpdate::UpdateSetData(
            self.set_name.clone().unwrap().into(),
            self.compute_current_apy(state).unwrap(),
        )));
    }
}

impl SetAdmin {
    fn compute_market_return(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        market_num: usize,
    ) -> Result<EthersU256> {
        let market_data = self
            .set_logic
            .read_market_data(self.address, state, self.set_address.unwrap(), market_num)
            .unwrap();

        let total_fees = market_data.purchases_fee_pool + market_data.sales_fee_pool;
        let drip_rate = market_data.last_drip_rate;

        Ok(drip_rate * EthersU256::from(total_fees))
    }

    fn compute_current_apy(&self, state: &SimState<CozyUpdate, CozyWorld>) -> Result<f64> {
        let num_markets = self.set_admin_params.market_configs.len();
        // Get total unscaled market returns.
        let mut total_market_return = EthersU256::from(0);
        for i in 0..num_markets {
            total_market_return += self.compute_market_return(state, i)?;
        }
        let total_assets = self
            .set_logic
            .read_total_assets(self.address, state, self.set_address.unwrap())
            .unwrap();

        if total_assets > 0 {
            let apy = total_market_return / total_assets;
            Ok((apy.as_u128() * SECONDS_IN_YEAR.as_u128()) as f64 / 1e18)
        } else {
            Ok(0.0)
        }
    }
}
