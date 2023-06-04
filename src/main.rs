#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.


use bindings::cozy_protocol::{cozy_router, *, raw_abis::*};

mod simulate;
mod cozy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let path = Path::new("contracts/cozy-protocol-v2/out/Set.sol/Set.json");
    //let artifact = fs::read_to_string(&path)?;
  /*
    println!("{}", utils::get_fully_qualified_link(COZYROUTER_RAW_ABI.clone())?);
    println!("{}", utils::get_fully_qualified_link(SET_RAW_ABI.clone())?);
    println!("{}", utils::get_fully_qualified_link(SUPPLYSIDELIB_RAW_ABI.clone())?);
    println!("{}", utils::get_fully_qualified_link(COZYROUTER_RAW_ABI.clone())?);
    let configurator_address: [u8; 20] = hex::decode("83769BeEB7e5405ef0B7dc3C66C43E3a51A6d27f").unwrap().try_into().unwrap();
    let delay_address: [u8; 20] = hex::decode("00EFd0D4639191C49908A7BddbB9A11A994A8527").unwrap().try_into().unwrap();
    let demand_address: [u8; 20] = hex::decode("147B09A8C7d5E4A8253a3e01De4356D3c132010D").unwrap().try_into().unwrap();
    let redemption_address: [u8; 20] = hex::decode("062C88B4ba954955746eDA6f475C26eeaC04614B").unwrap().try_into().unwrap();
    let state_address: [u8; 20] = hex::decode("90A5b0DD8c4b06636A4BEf7BA82D9C58f44fAaAd").unwrap().try_into().unwrap();
    let supply_address: [u8; 20] = hex::decode("94EDc320466d68c0e80C3e6F454375Fb957e1038").unwrap().try_into().unwrap();

    let b = utils::build_linked_bytecode(
        SET_RAW_ABI.clone(), vec![
            (CONFIGURATORLIB_RAW_ABI.clone(), configurator_address.into()),
            (DELAYLIB_RAW_ABI.clone(), delay_address.into()), 
            (DEMANDSIDELIB_RAW_ABI.clone(), demand_address.into()), 
            (REDEMPTIONLIB_RAW_ABI.clone(), redemption_address.into()), 
            (STATETRANSITIONSLIB_RAW_ABI.clone(), state_address.into()),
            (SUPPLYSIDELIB_RAW_ABI.clone(), supply_address.into()),
    ]
    );
    let s = b.unwrap();
    assert
    println!("{:?}", s);
    */
    Ok(())
}
