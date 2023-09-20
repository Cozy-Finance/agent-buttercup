use crate::u256::U256;

#[derive(Debug, Copy, Clone)]
pub struct TimeEnv {
    pub block_number: U256,
    pub block_timestamp: U256,
}

pub trait TimePolicy: Sync + Send {
    /// Returns the current time env.
    fn current_time_env(&self) -> TimeEnv;

    /// Takes one step forward in the policy and returns the new time env.
    fn step(&mut self) -> TimeEnv;

    /// Returns `true` if the policy is active and `false` otherwise.
    fn is_active(&self) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct FixedTimePolicy {
    current_time_env: TimeEnv,
    /// Each step moves the timestamp forward by `time_per_block` and block number forward by 1.
    pub time_per_step: U256,
    /// Total amount of time to generate before becoming inactive.
    time_to_generate: U256,
    /// Total amount of generated time.
    generated_time: U256,
}

impl FixedTimePolicy {
    pub fn new(current_time_env: TimeEnv, time_per_step: U256, time_to_generate: U256) -> Self {
        Self {
            current_time_env,
            time_per_step,
            time_to_generate,
            generated_time: U256::zero(),
        }
    }
}

impl TimePolicy for FixedTimePolicy {
    fn current_time_env(&self) -> TimeEnv {
        self.current_time_env
    }

    fn step(&mut self) -> TimeEnv {
        if self.is_active() {
            self.generated_time += self.time_per_step;
            self.current_time_env = TimeEnv {
                block_number: self.current_time_env.block_number + 1,
                block_timestamp: self.current_time_env.block_timestamp + self.time_per_step,
            };
        }
        self.current_time_env
    }

    fn is_active(&self) -> bool {
        self.generated_time < self.time_to_generate
    }
}
