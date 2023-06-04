#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::{
    collections::hash_map::DefaultHasher,
    error::Error,
    hash::Hasher,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Instant,
};

use ::simulate::stochastic::price_process::{PriceProcess, PriceProcessType};
use clap::{arg, command, CommandFactory, Parser, Subcommand};
use eyre::Result;
use itertools_num::linspace;
use revm::primitives::{Address, U256};
use thiserror::Error;

use binder::*;
use bindings::cozy_triggers::chainlink_trigger;

use std::fs;
use std::path::Path;

mod simulate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let path = Path::new("contracts/cozy-protocol-v2/out/Set.sol/Set.json");
    //let artifact = fs::read_to_string(&path)?;

}
