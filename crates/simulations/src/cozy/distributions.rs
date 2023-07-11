use rand::Rng;
use rand_distr::{num_traits::ToPrimitive, Distribution, Exp, Normal};
use serde::Deserialize;

use crate::cozy::{constants::*, utils::deserialize_string_to_u256, EthersU256};

pub trait CozyDistribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

#[derive(Debug, Clone, Deserialize)]
pub struct U256UniformRange {
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub min: EthersU256,
    #[serde(deserialize_with = "deserialize_string_to_u256")]
    pub max: EthersU256,
}

impl CozyDistribution<EthersU256> for U256UniformRange {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EthersU256 {
        let mut sample = [0_u8; 32];
        rng.fill(&mut sample[..]);
        EthersU256::from(sample) % (self.max - self.min) + self.min
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum TimeUnit {
    Second,
    Hour,
    Day,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Exponential {
    pub rate: f64,
    pub time_unit: TimeUnit,
}

impl CozyDistribution<f64> for Exponential {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let exp = Exp::new(self.rate).unwrap();
        exp.sample(rng)
    }
}

impl Exponential {
    pub fn sample_in_secs<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let sample = self.sample(rng);
        match &self.time_unit {
            TimeUnit::Second => sample,
            TimeUnit::Hour => sample * SECONDS_IN_MINUTE.to_f64().unwrap(),
            TimeUnit::Day => sample * SECONDS_IN_DAY.to_f64().unwrap(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TriggerProbModelSettings {
    pub starting_prob: f64,
    pub step_in_secs: u64,
    pub annualized_logit_std: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "TriggerProbModelSettings")]
pub struct TriggerProbModel {
    pub starting_prob: f64,
    pub current_prob: f64,
    pub current_logit: f64,
    pub step_in_secs: u64,
    pub annualized_logit_std: f64,
}

impl From<TriggerProbModelSettings> for TriggerProbModel {
    fn from(settings: TriggerProbModelSettings) -> Self {
        TriggerProbModel {
            starting_prob: settings.starting_prob,
            current_prob: settings.starting_prob,
            current_logit: logit(settings.starting_prob),
            step_in_secs: settings.step_in_secs,
            annualized_logit_std: settings.annualized_logit_std,
        }
    }
}

pub fn logit(p: f64) -> f64 {
    (p / (1.0 - p)).ln()
}

pub fn logistic(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

impl TriggerProbModel {
    pub fn new(starting_prob: f64, step_in_secs: u64, annualized_logit_std: f64) -> Self {
        TriggerProbModel {
            starting_prob,
            current_prob: starting_prob,
            current_logit: logit(starting_prob),
            step_in_secs,
            annualized_logit_std,
        }
    }

    pub fn step<R: Rng + ?Sized>(&mut self, rng: &mut R) -> f64 {
        if self.annualized_logit_std == 0.0 {
            return self.current_prob;
        }
        let normal = Normal::new(
            self.current_logit,
            self.annualized_logit_std
                * (self.step_in_secs as f64 / SECONDS_IN_YEAR as f64)
                    .to_f64()
                    .unwrap()
                    .sqrt(),
        )
        .unwrap();
        self.current_logit = normal.sample(rng);
        self.current_prob = logistic(self.current_logit);
        self.current_prob
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TruncatedNorm {
    pub mean: f64,
    pub std: f64,
    pub lower_bd: f64,
    pub upper_bd: f64,
}

impl TruncatedNorm {
    pub fn new(mean: f64, std: f64, lower_bd: f64, upper_bd: f64) -> Self {
        TruncatedNorm {
            mean,
            std,
            lower_bd,
            upper_bd,
        }
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let normal = Normal::new(self.mean, self.std).unwrap();

        let mut sample = normal.sample(rng);
        while !(sample >= self.lower_bd && sample <= self.upper_bd) {
            sample = normal.sample(rng);
        }

        sample
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProbTruncatedNorm {
    pub mean: f64,
    pub std: f64,
}

impl ProbTruncatedNorm {
    pub fn new(mean: f64, std: f64) -> Self {
        ProbTruncatedNorm { mean, std }
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        TruncatedNorm::new(self.mean, self.std, 0.0, 1.0).sample(rng)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LinkedProbTruncatedNorm {
    pub std: f64,
}

impl LinkedProbTruncatedNorm {
    pub fn new(std: f64) -> Self {
        LinkedProbTruncatedNorm { std }
    }

    pub fn sample<R: Rng + ?Sized>(&self, mean: f64, rng: &mut R) -> f64 {
        ProbTruncatedNorm::new(mean, self.std).sample(rng)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct JumpProbTruncatedNorm {
    mean1: f64,
    mean2: f64,
    std: f64,
    jump_prob: f64,
}

impl JumpProbTruncatedNorm {
    pub fn new(mean1: f64, mean2: f64, std: f64, jump_prob: f64) -> Self {
        JumpProbTruncatedNorm {
            mean1,
            mean2,
            std,
            jump_prob,
        }
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        if rng.gen_bool(self.jump_prob) {
            ProbTruncatedNorm::new(self.mean2, self.std).sample(rng)
        } else {
            ProbTruncatedNorm::new(self.mean1, self.std).sample(rng)
        }
    }
}
