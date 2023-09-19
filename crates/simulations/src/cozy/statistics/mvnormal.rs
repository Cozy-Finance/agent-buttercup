use std::{f64, f64::consts::PI};

use nalgebra::{Cholesky, DMatrix, DVector};
use primes::{PrimeSet, Sieve};
use rand::{distributions::Distribution, Rng};
use statrs::{
    distribution::{ContinuousCDF, Normal, Uniform},
    Result as StatsResult, StatsError,
};

#[derive(Debug, Clone)]
pub struct MultivariateNormal {
    dim: usize,
    cov_chol_decomp: DMatrix<f64>,
    mu: DVector<f64>,
    cov: DMatrix<f64>,
    precision: DMatrix<f64>,
    pdf_const: f64,
}

impl MultivariateNormal {
    /// Constructs a new multivariate normal distribution with a mean of `mean`
    /// and covariance matrix `cov`.
    ///
    /// # Errors
    ///
    /// Returns an error if `cov` is not symmetric and PD.
    pub fn new(mean: Vec<f64>, cov: Vec<f64>) -> StatsResult<Self> {
        let mean = DVector::from_vec(mean);
        let cov = DMatrix::from_vec(mean.len(), mean.len(), cov);
        let dim = mean.len();

        // Check that the provided covariance matrix is symmetric.
        if cov.lower_triangle() != cov.upper_triangle().transpose()
        // Check that mean and covariance do not contain NaN.
            || mean.iter().any(|f| f.is_nan())
            || cov.iter().any(|f| f.is_nan())
        // Check that the dimensions match.
            || mean.nrows() != cov.nrows() || cov.nrows() != cov.ncols()
        {
            return Err(StatsError::BadParams);
        }

        let cov_det = cov.determinant();
        let pdf_const = ((2. * PI).powi(mean.nrows() as i32) * cov_det.abs())
            .recip()
            .sqrt();

        // Store the Cholesky decomposition of the covariance matrix for sampling
        // and error if `cov` not PD.
        match Cholesky::new(cov.clone()) {
            None => Err(StatsError::BadParams),
            Some(cholesky_decomp) => {
                let precision = cholesky_decomp.inverse();
                Ok(MultivariateNormal {
                    dim,
                    cov_chol_decomp: cholesky_decomp.unpack(),
                    mu: mean,
                    cov,
                    precision,
                    pdf_const,
                })
            }
        }
    }

    /// Getter for `mu`.
    pub fn mu(&self) -> DVector<f64> {
        self.mu.clone()
    }

    /// Getter for `cov`.
    pub fn cov(&self) -> DMatrix<f64> {
        self.cov.clone()
    }

    /// Returns the lower triangular cholesky decomposition of self.cov
    /// wrt. switching of rows and columns of the matrix as well as mutating
    /// the input `a` and `b` with the row switches.
    ///
    /// Algorithm explained in 4.1.3 in ´Computation of Multivariate
    /// Normal and t Probabilities´, Alan Genz.
    fn chol_chrows(&self, a: &mut DVector<f64>, b: &mut DVector<f64>) -> DMatrix<f64> {
        let mut cov = self.cov.clone();
        let mut chol_lower: DMatrix<f64> = DMatrix::zeros(self.dim, self.dim);
        let mut y: DVector<f64> = DVector::zeros(self.dim);

        let std_normal = Normal::new(0., 1.).unwrap();
        for i in 0..self.dim {
            let mut cdf_diff = f64::INFINITY;
            let mut new_i = i;
            let mut a_tilde = a[i];
            let mut b_tilde = b[i];
            // Find the index of which to switch rows with
            for j in i..self.dim {
                let mut num = 0.;
                let mut den = cov[(j, j)].sqrt();
                if i > 0 {
                    // Numerator:
                    num = (chol_lower.index((j, ..i)) * y.index((..i, 0)))[0];
                    // Denominator:
                    den = (cov[(j, j)]
                        - (chol_lower.index((j, ..i)).transpose() * chol_lower.index((j, ..i)))[0])
                        .sqrt();
                }
                let pot_a_tilde = (a[j] - num) / den;
                let pot_b_tilde = (b[j] - num) / den;
                let cdf_a = std_normal.cdf(pot_a_tilde);
                let cdf_b = std_normal.cdf(pot_b_tilde);

                let pot_cdf_diff = cdf_b - cdf_a; // Potential minimum
                if pot_cdf_diff < cdf_diff {
                    new_i = j;
                    cdf_diff = pot_cdf_diff;
                    a_tilde = pot_a_tilde;
                    b_tilde = pot_b_tilde;
                }
            }
            if i != new_i {
                cov.swap_rows(i, new_i);
                cov.swap_columns(i, new_i);
                a.swap_rows(i, new_i);
                b.swap_rows(i, new_i);
                chol_lower.swap_rows(i, new_i);
                chol_lower.swap_columns(i, new_i);
            }

            // Get the expected values:
            y[i] = ((-a_tilde.powi(2) / 2.).exp() - (-b_tilde.powi(2) / 2.).exp())
                / ((2. * PI).sqrt() * cdf_diff);

            // Cholesky decomposition algorithm with the new changed row
            let mut ids = chol_lower.index_mut((.., ..i + 1)); // Get only the relevant indices
            ids[(i, i)] =
                (cov[(i, i)] - (ids.index((i, ..i)) * ids.index((i, ..i)).transpose())[0]).sqrt();
            for j in i + 1..self.dim {
                ids[(j, i)] = (cov[(j, i)]
                    - (ids.index((i, ..i)) * ids.index((j, ..i)).transpose())[0])
                    / ids[(i, i)];
            }
        }
        chol_lower
    }

    fn integrate_pdf(&self, a: &mut DVector<f64>, b: &mut DVector<f64>) -> (f64, f64) {
        let chol_lower = self.chol_chrows(a, b);

        // Generate first `dim` primes, Ricthmyer generators.
        let mut sqrt_primes = DVector::zeros(self.dim);
        let mut pset = Sieve::new();
        for (i, n) in pset.iter().enumerate().take(self.dim) {
            sqrt_primes[i] = (n as f64).sqrt();
        }

        let n_samples = 15;
        let n_points = 1000 * self.dim;
        let unif = Uniform::new(0., 1.).unwrap();
        let std_normal = Normal::new(0., 1.).unwrap();
        let mut rng = rand::thread_rng();

        let one = DVector::from_vec(vec![1.; self.dim]);
        let mut y: DVector<f64> = DVector::zeros(self.dim - 1);

        let alpha = 3.;
        let mut err = 0.;
        let mut err_help = 0.;
        let mut p = 0.; // The cdf probability.
        for i in 0..n_samples {
            let rnd_points = DVector::from(
                (0..self.dim)
                    .map(|_| unif.sample(&mut rng))
                    .collect::<Vec<_>>(),
            );
            let mut sum_i = 0.;
            for j in 0..n_points {
                let w =
                    (2. * DVector::from_vec(
                        ((j as f64) * &sqrt_primes + &rnd_points)
                            .iter()
                            .map(|x| x % 1.)
                            .collect::<Vec<f64>>(),
                    ) - &one)
                        .abs();
                let mut di = std_normal.cdf(a[0] / chol_lower[(0, 0)]);
                let mut ei = std_normal.cdf(b[0] / chol_lower[(0, 0)]);
                let mut fi = ei - di;
                for m in 1..self.dim {
                    y[m - 1] = std_normal.inverse_cdf(di + w[m - 1] * (ei - di));
                    let mut num = (chol_lower.index((m, ..m)) * y.index((..m, 0)))[0];
                    let den = chol_lower[(m, m)];
                    if num.is_nan() {
                        // Either -inf, 0 or inf, comes when yᵢ = -inf and chol_lowerₘᵢ = 0
                        num = 0.;
                    }
                    di = std_normal.cdf((a[m] - num) / den);
                    ei = std_normal.cdf((b[m] - num) / den);
                    fi *= ei - di;
                }
                sum_i += (fi - sum_i) / ((j + 1) as f64);
            }
            let delta = (sum_i - p) / ((i + 1) as f64);
            p += delta;
            err_help = (((i - 1) as f64) * err_help / ((i + 1) as f64)) + delta.powi(2);
            err = alpha * err_help.sqrt()
        }
        (p, err)
    }

    /// Calculates the probability density function for the multivariate
    /// normal distribution at `x`.
    ///
    /// # Formula
    ///
    /// ```ignore
    /// (2 * π) ^ (-k / 2) * det(Σ) ^ (1 / 2) * e ^ ( -(1 / 2) * transpose(x - μ) * inv(Σ) * (x - μ))
    /// ```
    ///
    /// where `μ` is the mean, `inv(Σ)` is the precision matrix, `det(Σ)` is the determinant
    /// of the covariance matrix, and `k` is the dimension of the distribution.
    pub fn pdf(&self, x: &DVector<f64>) -> f64 {
        let dv = x - &self.mu;
        let exp_term = -0.5
            * *(&dv.transpose() * &self.precision * &dv)
                .get((0, 0))
                .unwrap();
        self.pdf_const * exp_term.exp()
    }

    /// Calculates the log probability density function for the multivariate
    /// normal distribution at `x`. Equivalent to pdf(x).ln().
    pub fn ln_pdf(&self, x: &DVector<f64>) -> f64 {
        let dv = x - &self.mu;
        let exp_term = -0.5
            * *(&dv.transpose() * &self.precision * &dv)
                .get((0, 0))
                .unwrap();
        self.pdf_const.ln() + exp_term
    }

    /// Returns the cumulative distribution function at `x` for the
    /// multivariate normal distribution
    pub fn cdf(&self, mut x: DVector<f64>) -> f64 {
        // Shift integration limit wrt. mean
        x -= &self.mu;
        let (p, _) = self.integrate_pdf(
            &mut DVector::from_vec(vec![f64::NEG_INFINITY; self.dim]),
            &mut x,
        );
        p
    }

    /// Returns the survival function at `x` for the
    /// multivariate normal distribution, using approximation with
    /// `N` points.
    pub fn sf(&self, x: DVector<f64>) -> f64 {
        1. - self.cdf(x)
    }
}

impl ::rand::distributions::Distribution<DVector<f64>> for MultivariateNormal {
    /// Samples from the multivariate normal distribution
    ///
    /// # Formula
    /// L * Z + μ
    ///
    /// where `L` is the Cholesky decomposition of the covariance matrix,
    /// `Z` is a vector of normally distributed random variables, and
    /// `μ` is the mean vector.

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DVector<f64> {
        let d = Normal::new(0., 1.).unwrap();
        let z = DVector::<f64>::from_distribution(self.dim, &d, rng);
        (&self.cov_chol_decomp * z) + &self.mu
    }
}
