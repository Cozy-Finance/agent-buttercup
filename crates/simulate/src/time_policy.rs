//! The block time policy that is used to fast forward time in the simulation is handled here.

use ethers::types::U256 as EthersU256;
use revm::primitives::U256;
use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct TimeEnv {
    pub number: U256,
    pub timestamp: U256,
}

pub trait TimePolicy: Sync + Send {
    /// Returns the current blocktime env.
    fn current_time_env(&self) -> TimeEnv;

    /// Takes 1 step in the policy and returns the new blocktime env.
    fn step(&mut self) -> TimeEnv;

    /// Returns `true` if the policy is active and `false` if finished.
    fn is_active(&self) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct FixedTimePolicy {
    current_time_env: TimeEnv,
    /// Each new block moves the timestamp forward by `time_per_block`
    pub time_per_block: u64,
    /// Each step moves the block number forward by `blocks_per_step`
    pub blocks_per_step: u64,
    /// Total number of blocks to generate before becoming inactive
    blocks_to_generate: Option<u64>,
    /// Total amount of time to generate before becoming inactive
    time_to_generate: Option<u64>,
    /// Total number of generated active blocks
    generated_blocks: u64,
    /// Total number of generated active time
    generated_time: u64,
}

#[derive(Error, Debug)]
pub enum FixedTimePolicyError {
    #[error("Both blocks_to_generate and time_to_generate cannot be None.")]
    UnspecifiedEndTime,
}

impl FixedTimePolicy {
    pub fn new(
        start_block_number: u64,
        start_block_timestamp: u64,
        time_per_block: u64,
        blocks_per_step: u64,
        blocks_to_generate: Option<u64>,
        time_to_generate: Option<u64>,
    ) -> Result<Self, FixedTimePolicyError> {
        if blocks_to_generate.is_none() & time_to_generate.is_none() {
            return Err(FixedTimePolicyError::UnspecifiedEndTime);
        }
        Ok(FixedTimePolicy {
            current_time_env: TimeEnv {
                number: EthersU256::from(start_block_number).into(),
                timestamp: EthersU256::from(start_block_timestamp).into(),
            },
            time_per_block,
            blocks_per_step,
            blocks_to_generate,
            time_to_generate,
            generated_blocks: 0,
            generated_time: 0,
        })
    }
}

impl TimePolicy for FixedTimePolicy {
    fn current_time_env(&self) -> TimeEnv {
        self.current_time_env
    }

    fn step(&mut self) -> TimeEnv {
        if self.is_active() {
            let time_delta = self.time_per_block * self.blocks_per_step;
            self.generated_blocks += self.blocks_per_step;
            self.generated_time += time_delta;
            self.current_time_env = TimeEnv {
                number: self.current_time_env.number + U256::from(self.blocks_per_step),
                timestamp: self.current_time_env.timestamp + U256::from(time_delta),
            };
        }
        self.current_time_env
    }

    fn is_active(&self) -> bool {
        (self.generated_blocks < self.blocks_to_generate.unwrap_or(u64::MAX))
            & (self.generated_time < self.time_to_generate.unwrap_or(u64::MAX))
    }
}
