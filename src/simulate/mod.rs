#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::fs;

use clap::Parser;
use serde::{Deserialize, Serialize};
use simulate::stochastic::price_process::PriceProcess;

pub mod cozy;
