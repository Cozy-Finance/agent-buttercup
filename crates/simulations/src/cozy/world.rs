use std::{borrow::Cow, collections::HashMap, fmt::Debug, sync::Arc};

use auto_impl::auto_impl;
use bindings::cozy_protocol::cozy_router;
use eyre::Result;
use revm::primitives::{ExecutionResult, TxEnv};
use simulate::{
    address::Address,
    contract::sim_contract::SimContract,
    state::{update::UpdateData, world::World},
    utils::build_call_tx,
};

use crate::cozy::world_contracts::{
    CozyBackstop, CozyBaseToken, CozyChainlinkTriggerFactory, CozyDripDecayConstantFactory,
    CozyDynamicLevelFactory, CozyJumpRateFactory, CozyManager, CozyPtoken, CozyPtokenFactory,
    CozyRouter, CozySetFactory, CozySetLogic, CozyUmaTriggerFactory, Weth,
};

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    AddCozyRouter(Arc<CozyRouter>),
    AddCozyBaseToken(Arc<CozyBaseToken>),
    AddCozySetLogic(Arc<CozySetLogic>),
    AddCozyJumpRateFactory(Arc<CozyJumpRateFactory>),
    AddCozyDynamicLevelFactory(Arc<CozyDynamicLevelFactory>),
    AddCozyDripDecayConstantFactory(Arc<CozyDripDecayConstantFactory>),
    AddCozyUmaTriggerFactory(Arc<CozyUmaTriggerFactory>),
    AddCozyChainlinkTriggerFactory(Arc<CozyChainlinkTriggerFactory>),
    AddCozyManager(Arc<CozyManager>),
    AddCozyBackstop(Arc<CozyBackstop>),
    AddCozySetFactory(Arc<CozySetFactory>),
    AddCozyPtoken(Arc<CozyPtoken>),
    AddCozyPtokenFactory(Arc<CozyPtokenFactory>),
    AddWeth(Arc<Weth>),
    AddToSets(CozySet),
    AddToCostModels(Arc<CozyCostModel>),
    AddToDripDecayModels(Arc<CozyDripDecayModel>),
    AddToTriggers(CozyTrigger),
    UpdateSetData(Cow<'static, str>, f64),
    UpdateTriggerData(Cow<'static, str>, f64),
    Triggered(Cow<'static, str>),
}

impl UpdateData for CozyUpdate {}

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub cozy_router: Option<Arc<CozyRouter>>,
    pub base_token: Option<Arc<CozyBaseToken>>,
    pub set_logic: Option<Arc<CozySetLogic>>,
    pub jump_rate_factory: Option<Arc<CozyJumpRateFactory>>,
    pub dynamic_level_factory: Option<Arc<CozyDynamicLevelFactory>>,
    pub drip_decay_constant_factory: Option<Arc<CozyDripDecayConstantFactory>>,
    pub uma_trigger_factory: Option<Arc<CozyUmaTriggerFactory>>,
    pub chainlink_trigger_factory: Option<Arc<CozyChainlinkTriggerFactory>>,
    pub manager: Option<Arc<CozyManager>>,
    pub backstop: Option<Arc<CozyBackstop>>,
    pub set_factory: Option<Arc<CozySetFactory>>,
    pub ptoken: Option<Arc<CozyPtoken>>,
    pub ptoken_factory: Option<Arc<CozyPtokenFactory>>,
    pub weth: Option<Arc<Weth>>,
    pub sets: CozyMap<CozySet>,
    pub cost_models: CozyMap<Arc<CozyCostModel>>,
    pub drip_decay_models: CozyMap<Arc<CozyDripDecayModel>>,
    pub triggers: CozyMap<CozyTrigger>,
}

impl CozyWorld {
    pub fn new() -> Self {
        log::info!("Creating Cozy World");
        CozyWorld {
            cozy_router: None,
            base_token: None,
            set_logic: None,
            jump_rate_factory: None,
            dynamic_level_factory: None,
            drip_decay_constant_factory: None,
            uma_trigger_factory: None,
            chainlink_trigger_factory: None,
            manager: None,
            backstop: None,
            set_factory: None,
            ptoken: None,
            ptoken_factory: None,
            weth: None,
            sets: CozyMap::new(),
            cost_models: CozyMap::new(),
            drip_decay_models: CozyMap::new(),
            triggers: CozyMap::new(),
        }
    }
}

impl Default for CozyWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: &Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
        match update {
            CozyUpdate::AddCozyRouter(cozy_router) => {
                self.cozy_router = Some(cozy_router.clone());
            }
            CozyUpdate::AddCozyBaseToken(base_token) => {
                self.base_token = Some(base_token.clone());
            }
            CozyUpdate::AddCozySetLogic(set_logic) => {
                self.set_logic = Some(set_logic.clone());
            }
            CozyUpdate::AddCozyJumpRateFactory(factory) => {
                self.jump_rate_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyDynamicLevelFactory(factory) => {
                self.dynamic_level_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyDripDecayConstantFactory(factory) => {
                self.drip_decay_constant_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyUmaTriggerFactory(factory) => {
                self.uma_trigger_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyChainlinkTriggerFactory(factory) => {
                self.chainlink_trigger_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyManager(manager) => {
                self.manager = Some(manager.clone());
            }
            CozyUpdate::AddCozyBackstop(backstop) => {
                self.backstop = Some(backstop.clone());
            }
            CozyUpdate::AddCozySetFactory(factory) => {
                self.set_factory = Some(factory.clone());
            }
            CozyUpdate::AddCozyPtoken(ptoken) => {
                self.ptoken = Some(ptoken.clone());
            }
            CozyUpdate::AddCozyPtokenFactory(factory) => {
                self.ptoken_factory = Some(factory.clone());
            }
            CozyUpdate::AddWeth(weth) => {
                self.weth = Some(weth.clone());
            }
            CozyUpdate::AddToSets(set) => {
                let _ = self.sets.insert(set.clone());
            }
            CozyUpdate::AddToCostModels(cost_model) => {
                let _ = self.cost_models.insert(cost_model.clone());
            }
            CozyUpdate::AddToDripDecayModels(drip_decay_model) => {
                let _ = self.drip_decay_models.insert(drip_decay_model.clone());
            }
            CozyUpdate::AddToTriggers(trigger) => {
                let _ = self.triggers.insert(trigger.clone());
            }
            CozyUpdate::UpdateSetData(name, new_apy) => {
                let mut set = self.sets.get_mut_by_name(name).unwrap();
                set.apy = *new_apy;
            }
            CozyUpdate::UpdateTriggerData(name, new_prob) => {
                let mut trigger = self.triggers.get_mut_by_name(name).unwrap();
                trigger.current_prob = *new_prob;
            }
            CozyUpdate::Triggered(name) => {
                let mut trigger = self.triggers.get_mut_by_name(name).unwrap();
                trigger.triggered = true;
            }
        }
        None
    }
}

#[auto_impl(&, Arc)]
pub trait CozyMapId {
    fn name(&self) -> Cow<'static, str>;
    fn address(&self) -> Address;
}

#[macro_export]
macro_rules! impl_cozy_map_id {
    ($struct_name:ident) => {
        impl CozyMapId for $struct_name {
            fn name(&self) -> Cow<'static, str> {
                self.name.clone()
            }

            fn address(&self) -> Address {
                self.address
            }
        }
    };
}

#[derive(Debug, Clone)]
pub struct CozySet {
    pub name: Cow<'static, str>,
    pub address: Address,
    pub trigger_lookup: HashMap<Address, u16>,
    pub apy: f64,
}

impl CozySet {
    pub fn new(
        name: Cow<'static, str>,
        address: Address,
        trigger_lookup: HashMap<Address, u16>,
    ) -> Self {
        CozySet {
            name,
            address,
            trigger_lookup,
            apy: 0.0,
        }
    }
}

impl_cozy_map_id!(CozySet);

#[derive(Debug, Clone)]
pub struct CozyCostModel {
    pub name: Cow<'static, str>,
    pub address: Address,
}

impl CozyCostModel {
    pub fn new(name: Cow<'static, str>, address: Address) -> Arc<Self> {
        Arc::new(CozyCostModel { name, address })
    }
}

impl_cozy_map_id!(CozyCostModel);

#[derive(Debug, Clone)]
pub struct CozyDripDecayModel {
    pub name: Cow<'static, str>,
    pub address: Address,
}

impl CozyDripDecayModel {
    pub fn new(name: Cow<'static, str>, address: Address) -> Arc<Self> {
        Arc::new(CozyDripDecayModel { name, address })
    }
}

impl_cozy_map_id!(CozyDripDecayModel);

#[derive(Debug, Clone)]
pub struct CozyTrigger {
    pub name: Cow<'static, str>,
    pub address: Address,
    pub current_prob: f64,
    pub triggered: bool,
}

impl CozyTrigger {
    pub fn new(name: Cow<'static, str>, address: Address, current_prob: f64) -> Self {
        CozyTrigger {
            name,
            address,
            current_prob,
            triggered: false,
        }
    }
}

impl_cozy_map_id!(CozyTrigger);

#[derive(Debug, Clone)]
pub struct CozyMap<T: CozyMapId> {
    name_to_idx_map: HashMap<Cow<'static, str>, usize>,
    addr_to_idx_map: HashMap<Address, usize>,
    pub items: Vec<T>,
}

impl<T: CozyMapId> CozyMap<T> {
    pub fn new() -> Self {
        CozyMap {
            name_to_idx_map: HashMap::new(),
            addr_to_idx_map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn get_by_addr(&self, addr: &Address) -> Option<&T> {
        let idx = self.addr_to_idx_map.get(addr)?;
        self.items.get(*idx)
    }

    pub fn get_mut_by_addr(&mut self, addr: &Address) -> Option<&mut T> {
        let idx = self.addr_to_idx_map.get(addr)?;
        self.items.get_mut(*idx)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        let idx = self.name_to_idx_map.get(name)?;
        self.items.get(*idx)
    }

    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut T> {
        let idx = self.name_to_idx_map.get(name)?;
        self.items.get_mut(*idx)
    }

    pub fn get_addr(&self, name: &str) -> Option<Address> {
        let idx = self.name_to_idx_map.get(name)?;
        let addr = self.items[*idx].address();
        Some(addr)
    }

    pub fn get_name(&self, addr: &Address) -> Option<Cow<'static, str>> {
        let idx = self.addr_to_idx_map.get(addr)?;
        let name = self.items[*idx].name();
        Some(name)
    }

    pub fn insert(&mut self, val: T) -> Result<(), String> {
        let name = val.name();
        let addr = val.address();

        if self.name_to_idx_map.contains_key(&name) {
            return Err(format!("Name {} already exists in map", name));
        }
        if self.addr_to_idx_map.contains_key(&addr) {
            return Err(format!("Address {:?} already exists in map", addr));
        }

        let idx = self.items.len();
        self.name_to_idx_map.insert(name, idx);
        self.addr_to_idx_map.insert(addr, idx);
        self.items.push(val);
        Ok(())
    }

    pub fn values(&self) -> &Vec<T> {
        &self.items
    }
}

impl<T: CozyMapId> Default for CozyMap<T> {
    fn default() -> Self {
        Self::new()
    }
}
