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
use thiserror::Error;

mod simulate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
