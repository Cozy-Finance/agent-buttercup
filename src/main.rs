#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use bindings::cozy_protocol::metadata::*;
use bindings::cozy_protocol::*;
use revm::primitives::B160;
use simulate::contract::utils;
use std::error::Error;
use std::fs;
use std::path::Path;

mod simulator;
use crate::simulator::cozy::bindings_wrapper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{:?}", *CONFIGURATORLIB);

    let configurator_address: [u8; 20] = hex::decode("83769BeEB7e5405ef0B7dc3C66C43E3a51A6d27f")
        .unwrap()
        .try_into()
        .unwrap();
    let delay_address: [u8; 20] = hex::decode("00EFd0D4639191C49908A7BddbB9A11A994A8527")
        .unwrap()
        .try_into()
        .unwrap();
    let demand_address: [u8; 20] = hex::decode("147B09A8C7d5E4A8253a3e01De4356D3c132010D")
        .unwrap()
        .try_into()
        .unwrap();
    let redemption_address: [u8; 20] = hex::decode("062C88B4ba954955746eDA6f475C26eeaC04614B")
        .unwrap()
        .try_into()
        .unwrap();
    let state_address: [u8; 20] = hex::decode("90A5b0DD8c4b06636A4BEf7BA82D9C58f44fAaAd")
        .unwrap()
        .try_into()
        .unwrap();
    let supply_address: [u8; 20] = hex::decode("94EDc320466d68c0e80C3e6F454375Fb957e1038")
        .unwrap()
        .try_into()
        .unwrap();

    let b = utils::build_linked_bytecode(
        SET_RAW_BYTECODE.clone(),
        vec![
            (
                CONFIGURATORLIB_PATH,
                CONFIGURATORLIB_NAME,
                B160::from(configurator_address).into(),
            ),
            (
                DEMANDSIDELIB_PATH,
                DEMANDSIDELIB_NAME,
                B160::from(demand_address).into(),
            ),
            (
                DELAYLIB_PATH,
                DELAYLIB_NAME,
                B160::from(delay_address).into(),
            ),
            (
                REDEMPTIONLIB_PATH,
                REDEMPTIONLIB_NAME,
                B160::from(redemption_address).into(),
            ),
            (
                STATETRANSITIONSLIB_PATH,
                STATETRANSITIONSLIB_NAME,
                B160::from(state_address).into(),
            ),
            (
                SUPPLYSIDELIB_PATH,
                SUPPLYSIDELIB_NAME,
                B160::from(supply_address).into(),
            ),
        ],
    );
    let s = b.unwrap();
    println!("{:?}", s.to_owned());
    Ok(())
}
