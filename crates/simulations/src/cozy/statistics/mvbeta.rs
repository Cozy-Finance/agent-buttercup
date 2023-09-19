use std::{
    collections::HashMap,
    f64,
    hash::{Hash, Hasher},
};

use ::num_traits::abs;
use argmin::{
    core::{CostFunction, Error, Executor},
    solver::brent::BrentOpt,
};
use nalgebra::{Cholesky, DMatrix, DVector};
use ordered_float::OrderedFloat;
use rand::{distributions::Distribution, Rng};
use statrs::{
    distribution::{Beta, ContinuousCDF, Normal},
    Result as StatsResult, StatsError,
};

use super::mvnormal::MultivariateNormal;

#[derive(Debug, Clone)]
pub struct MultivariateBeta {
    dim: usize,
    pub mean: DVector<f64>,
    pub concentration: f64,
    beta_priors: Vec<Beta>,
}

impl MultivariateBeta {
    pub fn new(mean: Vec<f64>, concentration: f64) -> StatsResult<Self> {
        let mean = DVector::from_vec(mean);
        let dim = mean.len();

        // Check that `mean` is non-nan.
        if mean.iter().any(|f| f.is_nan())
        // Check that `mean` is between 0 and 1.
            || mean.max() > 1.
            || mean.min() < 0.
        // Check that `variance` is bigger than 1.
            || concentration <= 0.
        {
            return Err(StatsError::BadParams);
        }

        let mut beta_priors = vec![];
        for i in 0..dim {
            let alpha = mean[i] * concentration;
            let beta = (1. - mean[i]) * concentration;
            beta_priors.push(Beta::new(alpha, beta)?);
        }

        Ok(Self {
            dim,
            mean,
            concentration,
            beta_priors,
        })
    }
}

impl ::rand::distributions::Distribution<DVector<f64>> for MultivariateBeta {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DVector<f64> {
        DVector::from_vec(
            self.beta_priors
                .iter()
                .map(|beta| beta.sample(rng))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_sample() {
        let concentration = 100_000.0;
        let mean = vec![0.1, 0.2, 0.3, 0.5];
        let mv_beta = MultivariateBeta::new(mean, concentration).unwrap();

        let mut rng = rand::thread_rng();
        for _ in 0..99 {
            let _ = mv_beta.sample(&mut rng);
        }
    }
}
