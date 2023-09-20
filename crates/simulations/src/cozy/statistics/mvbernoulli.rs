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
#[allow(unused_imports)]
use rand::{distributions::Distribution, Rng};
use statrs::{
    distribution::{ContinuousCDF, Normal},
    Result as StatsResult, StatsError,
};

use super::mvnormal::MultivariateNormal;

#[derive(Debug, Clone)]
pub struct MultivariateBernoulli {
    dim: usize,
    pub mean: DVector<f64>,
    pub corr: DMatrix<f64>,
    pub prob_matrix: DMatrix<f64>,
    mvn: MultivariateNormal,
}

impl MultivariateBernoulli {
    /// Constructs a new multivariate Bernoulli distribution with a mean of `mean`
    /// and correlations matrix `corr`.
    ///
    /// # Errors
    ///
    /// Returns as error if:
    ///     - `mean` values are not between 0 and 1
    ///     - `corr` is not symmetric and PD
    pub fn new(mean: Vec<f64>, corr: Vec<f64>) -> StatsResult<Self> {
        let mean = DVector::from_vec(mean);
        let corr = DMatrix::from_vec(mean.len(), mean.len(), corr);
        let dim = mean.len();

        // Check that `corr` is symmetric.
        if corr.lower_triangle() != corr.upper_triangle().transpose()
        // Check that `mean` and `corr` do not contain NaNs.
            || mean.iter().any(|f| f.is_nan())
            || corr.iter().any(|f| f.is_nan())
        // Check that `mean` is between 0 and 1.
            || mean.max() > 1.
            || mean.min() < 0.
        // Check that the `corr` is PD.
            || Cholesky::new(corr.clone()).is_none()
        // Check that the dimensions match.
            || mean.nrows() != corr.nrows() || corr.nrows() != corr.ncols()
        {
            return Err(StatsError::BadParams);
        }

        // Compute the `prob_matrix`.
        let std_devs = mean.component_mul(&mean.map(|x| 1.0 - x)).map(|x| x.sqrt());
        let cov = corr.map_with_location(|i, j, c| c * std_devs[i] * std_devs[j]);
        let prob_matrix = cov + &mean * mean.transpose();

        // Compute the corresponding `mvn_mu` and `mvn_cov` used for sampling.
        let standard_normal = Normal::new(0., 1.)?;
        let mvn_mu = mean.map(&|x| standard_normal.inverse_cdf(x));

        let mut opt_failure = false;
        let mut mvn_corr_solns = HashMap::new();
        let mut mvn_corr = prob_matrix.map_with_location(|i, j, prob_ij| {
            if j > i {
                let problem = MvbToMvnCorrSolver::new(mean[i], mean[j], prob_ij);
                if let Some(soln) = mvn_corr_solns.get(&problem) {
                    return *soln;
                }
                let soln = problem.run().unwrap_or_else(|_| {
                    opt_failure = true;
                    f64::NAN
                });
                mvn_corr_solns.insert(problem, soln);
                soln
            } else {
                1.
            }
        });
        if opt_failure {
            return Err(StatsError::ComputationFailedToConverge);
        }
        mvn_corr.fill_lower_triangle_with_upper_triangle();

        Ok(Self {
            dim,
            mean,
            corr,
            prob_matrix,
            mvn: MultivariateNormal::new(mvn_mu.as_slice().to_vec(), mvn_corr.as_slice().to_vec())?,
        })
    }
}

impl ::rand::distributions::Distribution<DVector<f64>> for MultivariateBernoulli {
    /// Samples from the multivariate Bernoulli distribution.
    ///
    /// # Formula
    /// x ~ MVN(μ, Σ)
    /// z = I(x > 0)
    ///
    /// where `μ` is the MVN mean vector and `Σ` is the MVN covariance matrix.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DVector<f64> {
        let x = self.mvn.sample(rng);
        x.map(|x| if x > 0. { 1. } else { 0. })
    }
}

/// Used to find the correlation in a bivariate normal that corresponds to simulating
/// a bivariate Bernoulli with a given joint probability `prob_ij`.
///
/// # Formula
/// mvn_mu_i = Φ^-1(prob_i)
/// mvn_mu_j = Φ^-1(prob_j)
/// prob_ij = 1 - Φ(-mvn_mu_i) - Φ(-mvn_mu_j) + Φ(-mvn_mu_i, -mvn_mu_j, mvn_corr_ij)
///
/// where Φ is the standard normal CDF. We use the Brent optimization algorithm
/// to find the `mvn_corr_ij` that satisfies the above.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MvbToMvnCorrSolver {
    prob_i: OrderedFloat<f64>,
    prob_j: OrderedFloat<f64>,
    prob_ij: OrderedFloat<f64>,
}

impl MvbToMvnCorrSolver {
    pub fn new(prob_i: f64, prob_j: f64, prob_ij: f64) -> Self {
        Self {
            prob_i: prob_i.into(),
            prob_j: prob_j.into(),
            prob_ij: prob_ij.into(),
        }
    }

    pub fn run(self) -> StatsResult<f64> {
        let result = Executor::new(self, BrentOpt::new(-0.99999999, 0.99999999))
            .run()
            .unwrap();
        match result.state().best_param {
            Some(corr) => Ok(corr),
            None => Err(StatsError::ComputationFailedToConverge),
        }
    }
}

impl CostFunction for MvbToMvnCorrSolver {
    type Param = f64;
    type Output = f64;

    fn cost(&self, mvn_corr_ij: &Self::Param) -> Result<Self::Output, Error> {
        let mvn = MultivariateNormal::new(vec![0., 0.], vec![1., *mvn_corr_ij, *mvn_corr_ij, 1.])
            .unwrap();
        let standard_normal = Normal::new(0., 1.)?;
        let mvn_mu_i = standard_normal.inverse_cdf(self.prob_i.into_inner());
        let mvn_mu_j = standard_normal.inverse_cdf(self.prob_j.into_inner());
        let target_value = 1. - standard_normal.cdf(-mvn_mu_i) - standard_normal.cdf(-mvn_mu_j)
            + mvn.cdf(DVector::from_vec(vec![-mvn_mu_i, -mvn_mu_j]));
        let deviation = abs(target_value - *self.prob_ij);
        Ok(deviation)
    }
}

impl Hash for MvbToMvnCorrSolver {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted_probs = vec![self.prob_i, self.prob_j];
        sorted_probs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted_probs.hash(state);
        self.prob_ij.to_bits().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    pub fn test_mvb_to_mvn_corr_solver() {
        let compute_prob_ij = |prob_i: f64, prob_j: f64, corr_ij: f64| {
            corr_ij * (prob_i * prob_j * (1. - prob_i) * (1. - prob_j)).sqrt() + prob_i * prob_j
        };

        let mut prob_i = 0.05;
        let mut prob_j = 0.1;
        let mut corr_ij = 0.3;
        let mut mvn_corr_finder =
            MvbToMvnCorrSolver::new(prob_i, prob_j, compute_prob_ij(prob_i, prob_j, corr_ij));
        assert!((mvn_corr_finder.run().unwrap() - 0.61).abs() < 1e-2);

        prob_i = 0.10;
        prob_j = 0.20;
        corr_ij = 0.2;
        mvn_corr_finder =
            MvbToMvnCorrSolver::new(prob_i, prob_j, compute_prob_ij(prob_i, prob_j, corr_ij));
        assert!((mvn_corr_finder.run().unwrap() - 0.40).abs() < 1e-2);
    }

    #[test]
    #[ignore]
    pub fn test_mvbernoulli() {
        let mean = vec![0.05, 0.1, 0.15, 0.20];
        let corrs = vec![
            1., 0.3, 0.2, 0.1, 0.3, 1., 0.3, 0.2, 0.2, 0.3, 1., 0.3, 0.1, 0.2, 0.3, 1.,
        ];

        let mvbernoulli = MultivariateBernoulli::new(mean.clone(), corrs.clone()).unwrap();
        assert!(mvbernoulli.dim == 4);
        assert!(mvbernoulli.mean == DVector::from_vec(mean));
        assert!(mvbernoulli.corr == DMatrix::from_vec(4, 4, corrs));

        let expected_corrs = vec![
            1., 0.61, 0.46, 0.26, 0.61, 1., 0.56, 0.40, 0.46, 0.56, 1., 0.53, 0.26, 0.40, 0.53, 1.,
        ];
        let diff_in_corrs = mvbernoulli.mvn.cov() - DMatrix::from_vec(4, 4, expected_corrs);
        assert!(diff_in_corrs.iter().all(|x| x.abs() < 1e-2));

        let mut rng = rand::thread_rng();
        let sample = mvbernoulli.sample(&mut rng);
        assert!(sample.iter().all(|x| *x == 0. || *x == 1.));
    }
}
