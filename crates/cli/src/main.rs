use std::{env, str::FromStr, sync::Arc, io::Read};

use bytes::Bytes;
use clairvoyance::uniswap::{get_pool, Pool};
use clap::{Parser, Subcommand};
use ethabi::ParamType;
use ethers::{
    abi::parse_abi,
    prelude::BaseContract,
    providers::{Http, Provider},
    types::{BlockId, H160 as eH160, H256, U256 as eU256, Call},
};
use ethers_providers::Middleware;
// use ethers_contract::Call::ContractCall;
use eyre::Result;
use revm::{AccountInfo, Bytecode, TransactOut, TransactTo};
use simulate::{price_simulation::PriceSimulation, testbed::Testbed};
use tokio::join;
use utils::chain_tools::get_provider;

use ethers_solc::Solc;
use utils::tokens::{get_tokens, Token};

use bindings::hello_world::{HelloWorld, self};

mod config;

#[tokio::main]
async fn main() -> Result<()> {
    // Do a transaction using revm
    // CLIENT STUFF WE NEED TO GET RID OF
    let client = get_provider().await;
    

    // create a testbed where we can run sims
    let mut testbed = Testbed::new();

    // insert a default user
    let user_addr = eH160::from_str("0x0000000000000000000000000000000000000001")?;
    let user_acc_info = AccountInfo::new(
        eU256::from(1293874298374982736983074_u128),
        0,
        Bytecode::new(),
    );
    testbed.create_user(user_addr);
    testbed
        .evm
        .db()
        .unwrap()
        .insert_account_info(user_addr, user_acc_info);

    // deploy a local Hellow World pool
    let contract_addr = eH160::from_str("0x1111111111111111111111111111111111111111")?;

    let contract_bytes = bindings::hello_world::HELLOWORLD_BYTECODE.to_owned();
    println!("{:#?}", contract_bytes);
    let contract_bytes = contract_bytes.to_vec();
    let contract_bytes = Bytes::from(contract_bytes);

    let contract_bytecode = Bytecode::new_raw(contract_bytes);
    println!("{:#?}", contract_bytecode);

    let contract_acc_info = AccountInfo::new(eU256::zero(), 0_u64, contract_bytecode);
    testbed
        .evm
        .db()
        .unwrap()
        .insert_account_info(contract_addr, contract_acc_info);

    testbed
            .evm
            .db()
            .unwrap()
                .insert_account_storage(contract_addr, eU256::zero(), eU256::zero())
                .unwrap();

    println!(
        "Database after adding user and factory contract: {:#?}",
        testbed.evm.db()
    );

    let new_contract = HelloWorld::new(contract_addr, client); // want remove clients
    let calldata = new_contract.greeting().calldata().unwrap();
    let calldata = calldata.to_vec();
    let calldata = Bytes::from(calldata);
    println!("Calldata sent to EVM: {:#?}", calldata);
    let contract_abi = hello_world::HELLOWORLD_ABI.to_owned();
    println!("{:#?}", contract_abi);
    let abi = BaseContract::from(
        parse_abi(&[
            "function greet() public view returns (string memory greeting)",
        ])?);
    println!("Printing parse_abi: {:#?}", abi);
    let encoded = abi.encode("greet", ())?;
    let encoded = Bytes::from(hex::decode(hex::encode(encoded))?);
    println!("Printing encoded: {:#?}", encoded);

    // Maybe add a fallback and send no data and see if we get the same thing
    // perform a transaction
    testbed.evm.env.tx.caller = user_addr;
    testbed.evm.env.tx.transact_to = TransactTo::Call(contract_addr);
    // testbed.evm.env.tx.data = calldata; //Is this also correct?
    testbed.evm.env.tx.data = encoded; // This field could be the problem
    testbed.evm.env.tx.value = eU256::zero();
    let result = testbed.evm.transact().0.out;


    let value = match result {
        TransactOut::Call(value) => Some(value),
        _ => None,
    };
    let value = value.unwrap();

    println!("Printing value from TransactOut: {:#?}", value);
    
    
    // let result = abi.decode("greet", value)?;
    // abi.decode_raw(name, bytes)
    let output: String =
        abi.decode_output("greet", value)?; // invalid data error comes from this line
    // let result = ethabi::decode_whole(&[ParamType::String], &value);
    println!("Transaction result: {:#?}", output);
    Ok(())
}
