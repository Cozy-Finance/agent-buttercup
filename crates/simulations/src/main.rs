#![warn(unsafe_code)]
//! Main lives in the `cli` crate so that we can do our input parsing.

use std::error::Error;
pub mod cozy;

pub fn main() -> Result<(), Box<dyn Error>> {
    cozy::run()
}
