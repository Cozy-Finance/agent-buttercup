use nalgebra::{Cholesky, DMatrix, Dynamic};
use rand::Rng;
use statrs::{
    distribution::{ChiSquared, Normal},
    Result, StatsError,
};

#[derive(Debug, Clone)]
pub struct Wishart {
    df: f64,
    scale: DMatrix<f64>,
    chol: Cholesky<f64, Dynamic>,
}

impl Wishart {
    pub fn new(df: f64, scale: DMatrix<f64>) -> Result<Self> {
        if scale.nrows() != scale.ncols()
            || df <= 0.0
            || df.is_nan()
            || df <= scale.nrows() as f64 - 1.0
        {
            return Err(StatsError::BadParams);
        }
        match Cholesky::new(scale.clone()) {
            None => Err(StatsError::BadParams),
            Some(chol) => Ok(Wishart { df, scale, chol }),
        }
    }

    pub fn dim(&self) -> usize {
        self.scale.nrows()
    }
}

impl ::rand::distributions::Distribution<DMatrix<f64>> for Wishart {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DMatrix<f64> {
        let dim = self.dim();
        let mut a = DMatrix::zeros(dim, dim);

        for i in 0..dim {
            a[(i, i)] = ChiSquared::new(self.df - i as f64).unwrap().sample(rng);
        }

        let std_normal = Normal::new(0.0, 1.0).unwrap();
        for i in 1..dim {
            for j in 0..i {
                a[(i, j)] = std_normal.sample(rng);
            }
        }

        let l = self.chol.l();
        &l * &a * a.transpose() * l.transpose()
    }
}

#[derive(Debug, Clone)]
pub struct WishartCorrelation {
    wishart: Wishart,
}

impl WishartCorrelation {
    pub fn new(df: f64, scale: DMatrix<f64>) -> Result<Self> {
        let wishart = Wishart::new(df, scale)?;
        Ok(WishartCorrelation { wishart })
    }
}

impl ::rand::distributions::Distribution<DMatrix<f64>> for WishartCorrelation {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DMatrix<f64> {
        let wishart_sample = self.wishart.sample(rng);
        let scaled_diagonal =
            DMatrix::from_diagonal(&wishart_sample.diagonal().map(|x| 1.0 / x.sqrt()));
        &scaled_diagonal * &wishart_sample * scaled_diagonal
    }
}

#[cfg(test)]
mod tests {
    use rand::distributions::Distribution;

    use super::*;

    #[test]
    #[ignore]
    fn test_sample() {
        let df = 3.0;
        let scale = DMatrix::from_row_slice(3, 3, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        let wishart_correlation = WishartCorrelation::new(df, scale).unwrap();

        let mut rng = rand::thread_rng();
        for _ in 0..99 {
            let _ = wishart_correlation.sample(&mut rng);
        }
    }
}
