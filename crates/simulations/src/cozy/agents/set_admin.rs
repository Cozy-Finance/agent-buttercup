use std::{borrow::Cow, collections::HashMap, sync::Arc};

use bindings::cozy_protocol::shared_types::{MarketConfig, SetConfig};
pub use bindings::{
    cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
    drip_decay_model_constant_factory, manager,
    set::{AccountingReturn, MarketsReturn},
};
use eyre::Result;
use revm::primitives::TxEnv;
use simulate::{
    agent::{agent_channel::AgentChannel, types::AgentId, Agent},
    state::{update::SimUpdate, SimState},
    utils::{build_call_contract_txenv, unpack_execution}, address::Address,
};

pub use crate::cozy::constants;
use crate::cozy::{
    constants::SECONDS_IN_YEAR,
    world::{CozyProtocolContract, CozySet, CozyUpdate, CozyWorld},
    EthersAddress, EthersU256
};

#[derive(Debug, Clone)]
pub struct SetAdminParams {
    pub asset: Address,
    pub set_config: SetConfig,
    pub market_configs: Vec<MarketConfig>,
    pub salt: Option<[u8; 32]>,
}

pub struct SetAdmin {
    name: Option<Cow<'static, str>>,
    address: Address,
    set_admin_params: SetAdminParams,
    manager: Arc<CozyProtocolContract>,
    set_address: Option<Address>,
    set_name: Option<String>,
    set_logic: Arc<CozyProtocolContract>,
}

impl SetAdmin {
    pub fn new(
        name: Option<Cow<'static, str>>,
        address: Address,
        set_admin_params: SetAdminParams,
        set_logic: &Arc<CozyProtocolContract>,
        manager: &Arc<CozyProtocolContract>,
    ) -> Self {
        Self {
            name,
            address,
            set_admin_params,
            set_logic: set_logic.clone(),
            manager: manager.clone(),
            set_address: None,
            set_name: None,
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

        let (set_addr, evm_tx) = self.build_create_set_tx(state, create_set_args).unwrap();
        self.set_address = Some(set_addr);
        self.set_name = Some(format!("{:?}'s Set", self.name));

        let trigger_lookup = self
            .set_admin_params
            .market_configs
            .iter()
            .enumerate()
            .map(|(i, config)| (config.trigger.into(), i as u16))
            .collect::<HashMap<_, _>>();

        let world_update = CozyUpdate::AddToSets(
            self.set_name.clone().unwrap().into(),
            CozySet::new(self.set_address.unwrap().into(), trigger_lookup),
        );

        channel.send(SimUpdate::Bundle(evm_tx, world_update));
    }

    fn step(&mut self, state: &SimState<CozyUpdate, CozyWorld>, channel: AgentChannel<CozyUpdate>) {
        channel.send(SimUpdate::World(CozyUpdate::UpdateSetData(
            self.set_name.clone().unwrap().into(),
            self.compute_current_apy(state).unwrap(),
        )));
    }
}

impl SetAdmin {
    fn build_create_set_tx(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: manager::CreateSetCall,
    ) -> Result<(Address, TxEnv)> {
        let call_data = self.manager.contract.encode_function("createSet", args)?;
        let tx = build_call_contract_txenv(
            self.address,
            self.manager.address.into(),
            call_data,
            None,
            None,
        );
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None))
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = self
            .manager
            .contract
            .decode_output("createSet", tx_result)?;

        Ok((Address::from(addr), tx))
    }

    fn compute_market_return(
        &self,
        state: &SimState<CozyUpdate, CozyWorld>,
        market_num: usize,
    ) -> Result<EthersU256> {
        let call_data = self
            .set_logic
            .contract
            .encode_function("markets", (EthersU256::from(market_num),))?;
        let query = build_call_contract_txenv(
            self.address,
            self.set_address.unwrap(),
            call_data,
            None,
            None,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None))?;
        let market_data = self
            .set_logic
            .contract
            .decode_output::<MarketsReturn>("markets", result)?;

        let total_fees = market_data.purchases_fee_pool + market_data.sales_fee_pool;
        let drip_rate = market_data.last_drip_rate;

        Ok(drip_rate * EthersU256::from(total_fees))
    }

    fn compute_current_apy(&self, state: &SimState<CozyUpdate, CozyWorld>) -> Result<u128> {
        let num_markets = self.set_admin_params.market_configs.len();
        // Get total unscaled market returns.
        let mut total_market_return = EthersU256::from(0);
        for i in 0..num_markets {
            total_market_return += self.compute_market_return(state, i)?;
        }

        // Get total assets.
        let call_data = self.set_logic.contract.encode_function("accounting", ())?;
        let query = build_call_contract_txenv(
            self.address,
            self.set_address.unwrap(),
            call_data,
            None,
            None,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None))?;
        let total_assets = self
            .set_logic
            .contract
            .decode_output::<AccountingReturn>("accounting", result)?
            .asset_balance;

        if total_assets > 0 {
            let apy = total_market_return / total_assets;
            Ok(apy.as_u128() * SECONDS_IN_YEAR)
        } else {
            Ok(0 as u128)
        }
    }
}
