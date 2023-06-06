#![warn(unsafe_code)]
//! Lib crate for describing simulations.

pub mod agent;
pub mod block_time_policy;
pub mod contract;
pub mod environment;
pub mod manager;
pub mod sim_env_data;
pub mod stochastic;
pub mod utils;

pub use ethers::types::{Bytes as EthersBytes, H160 as EthersAddress};
pub use revm::primitives::{Bytes as EvmBytes, B160 as EvmAddress};
