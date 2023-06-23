use bindings::{
    backstop::*,
    chainlink_trigger_factory::*,
    configurator_lib::*,
    cost_model_dynamic_level_factory::*,
    cost_model_jump_rate_factory::*,
    cozy_models::metadata::*,
    cozy_protocol::metadata::*,
    cozy_router::*,
    cozy_simulation::metadata::*,
    cozy_triggers::metadata::*,
    delay_lib::*,
    demand_side_lib::*,
    drip_decay_model_constant_factory::*,
    dummy_token::*,
    dummy_trigger::*,
    manager::*,
    p_token::*,
    p_token_factory::*,
    redemption_lib::*,
    set::*,
    set_factory::*,
    state_transitions_lib::*,
    supply_side_lib::*,
    uma_trigger_factory::*,
    weth::{metadata::*, weth9::*},
};
use lazy_static::lazy_static;

use crate::cozy::{EthersBytes, EthersContract};

#[derive(Debug)]
pub struct BindingsWrapper {
    pub abi: &'static EthersContract,
    pub bytecode: Option<&'static EthersBytes>,
    pub unlinked_bytecode: Option<&'static str>,
    pub name: &'static str,
    pub path: &'static str,
}

lazy_static! {
    pub static ref CONFIGURATORLIB: BindingsWrapper = BindingsWrapper {
        abi: &CONFIGURATORLIB_ABI,
        bytecode: Some(&CONFIGURATORLIB_BYTECODE),
        unlinked_bytecode: None,
        name: CONFIGURATORLIB_NAME,
        path: CONFIGURATORLIB_PATH
    };
    pub static ref DELAYLIB: BindingsWrapper = BindingsWrapper {
        abi: &DELAYLIB_ABI,
        bytecode: Some(&DELAYLIB_BYTECODE),
        unlinked_bytecode: None,
        name: DELAYLIB_NAME,
        path: DELAYLIB_PATH
    };
    pub static ref DEMANDSIDELIB: BindingsWrapper = BindingsWrapper {
        abi: &DEMANDSIDELIB_ABI,
        bytecode: Some(&DEMANDSIDELIB_BYTECODE),
        unlinked_bytecode: None,
        name: DEMANDSIDELIB_NAME,
        path: DEMANDSIDELIB_PATH
    };
    pub static ref REDEMPTIONLIB: BindingsWrapper = BindingsWrapper {
        abi: &REDEMPTIONLIB_ABI,
        bytecode: Some(&REDEMPTIONLIB_BYTECODE),
        unlinked_bytecode: None,
        name: REDEMPTIONLIB_NAME,
        path: REDEMPTIONLIB_PATH
    };
    pub static ref STATETRANSITIONSLIB: BindingsWrapper = BindingsWrapper {
        abi: &STATETRANSITIONSLIB_ABI,
        bytecode: Some(&STATETRANSITIONSLIB_BYTECODE),
        unlinked_bytecode: None,
        name: STATETRANSITIONSLIB_NAME,
        path: STATETRANSITIONSLIB_PATH
    };
    pub static ref SUPPLYSIDELIB: BindingsWrapper = BindingsWrapper {
        abi: &SUPPLYSIDELIB_ABI,
        bytecode: Some(&SUPPLYSIDELIB_BYTECODE),
        unlinked_bytecode: None,
        name: SUPPLYSIDELIB_NAME,
        path: SUPPLYSIDELIB_PATH
    };
    pub static ref MANAGER: BindingsWrapper = BindingsWrapper {
        abi: &MANAGER_ABI,
        bytecode: Some(&MANAGER_BYTECODE),
        unlinked_bytecode: None,
        name: MANAGER_NAME,
        path: MANAGER_PATH
    };
    pub static ref SET: BindingsWrapper = BindingsWrapper {
        abi: &SET_ABI,
        bytecode: None,
        unlinked_bytecode: Some(SET_RAW_BYTECODE),
        name: SET_NAME,
        path: SET_PATH
    };
    pub static ref SETFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &SETFACTORY_ABI,
        bytecode: Some(&SETFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: SETFACTORY_NAME,
        path: SETFACTORY_PATH
    };
    pub static ref PTOKEN: BindingsWrapper = BindingsWrapper {
        abi: &PTOKEN_ABI,
        bytecode: Some(&PTOKEN_BYTECODE),
        unlinked_bytecode: None,
        name: PTOKEN_NAME,
        path: PTOKEN_PATH
    };
    pub static ref PTOKENFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &PTOKENFACTORY_ABI,
        bytecode: Some(&PTOKENFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: PTOKENFACTORY_NAME,
        path: PTOKENFACTORY_PATH
    };
    pub static ref BACKSTOP: BindingsWrapper = BindingsWrapper {
        abi: &BACKSTOP_ABI,
        bytecode: Some(&BACKSTOP_BYTECODE),
        unlinked_bytecode: None,
        name: BACKSTOP_NAME,
        path: BACKSTOP_PATH
    };
    pub static ref COZYROUTER: BindingsWrapper = BindingsWrapper {
        abi: &COZYROUTER_ABI,
        bytecode: Some(&COZYROUTER_BYTECODE),
        unlinked_bytecode: None,
        name: COZYROUTER_NAME,
        path: COZYROUTER_PATH
    };
    pub static ref COSTMODELJUMPRATEFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &COSTMODELJUMPRATEFACTORY_ABI,
        bytecode: Some(&COSTMODELJUMPRATEFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: COSTMODELJUMPRATEFACTORY_NAME,
        path: COSTMODELJUMPRATEFACTORY_PATH
    };
    pub static ref COSTMODELDYNAMICLEVELFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &COSTMODELDYNAMICLEVELFACTORY_ABI,
        bytecode: Some(&COSTMODELDYNAMICLEVELFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: COSTMODELDYNAMICLEVELFACTORY_NAME,
        path: COSTMODELDYNAMICLEVELFACTORY_PATH
    };
    pub static ref DRIPDECAYMODELCONSTANTFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &DRIPDECAYMODELCONSTANTFACTORY_ABI,
        bytecode: Some(&DRIPDECAYMODELCONSTANTFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: DRIPDECAYMODELCONSTANTFACTORY_NAME,
        path: DRIPDECAYMODELCONSTANTFACTORY_PATH
    };
    pub static ref UMATRIGGERFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &UMATRIGGERFACTORY_ABI,
        bytecode: Some(&UMATRIGGERFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: UMATRIGGERFACTORY_NAME,
        path: UMATRIGGERFACTORY_PATH
    };
    pub static ref CHAINLINKTRIGGERFACTORY: BindingsWrapper = BindingsWrapper {
        abi: &CHAINLINKTRIGGERFACTORY_ABI,
        bytecode: Some(&CHAINLINKTRIGGERFACTORY_BYTECODE),
        unlinked_bytecode: None,
        name: CHAINLINKTRIGGERFACTORY_NAME,
        path: CHAINLINKTRIGGERFACTORY_PATH
    };
    pub static ref DUMMYTRIGGER: BindingsWrapper = BindingsWrapper {
        abi: &DUMMYTRIGGER_ABI,
        bytecode: Some(&DUMMYTRIGGER_BYTECODE),
        unlinked_bytecode: None,
        name: DUMMYTRIGGER_NAME,
        path: DUMMYTRIGGER_PATH
    };
    pub static ref WETH: BindingsWrapper = BindingsWrapper {
        abi: &WETH9_ABI,
        bytecode: Some(&WETH9_BYTECODE),
        unlinked_bytecode: None,
        name: WETH9_NAME,
        path: WETH9_PATH
    };
    pub static ref DUMMYTOKEN: BindingsWrapper = BindingsWrapper {
        abi: &DUMMYTOKEN_ABI,
        bytecode: Some(&DUMMYTOKEN_BYTECODE),
        unlinked_bytecode: None,
        name: DUMMYTOKEN_NAME,
        path: DUMMYTOKEN_PATH
    };
}
