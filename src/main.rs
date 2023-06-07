#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;
mod simulator;

pub fn main() -> Result<(), Box<dyn Error>> {
    simulator::cozy::run()
}
