use nalgebra::{DMatrix, DVector};

#[derive(Debug, Clone)]
pub struct SetRiskModel {
    pub annual_probabilities: DVector<f64>,
    pub corr_matrix: DMatrix<f64>,
    pub cov_matrix: DMatrix<f64>,
    pub joint_prob: DMatrix<f64>,
    pub leverage: f64,
    pub market_weights: DVector<f64>,
    pub probability_other_markets_trigger: Vec<f64>,
    pub correlation_with_other_markets: Vec<f64>,
}

impl SetRiskModel {
    pub fn new(
        annual_probabilities: DVector<f64>,
        corr_matrix: DMatrix<f64>,
        leverage: f64,
        market_weights: DVector<f64>,
    ) -> Self {
        let n = annual_probabilities.len();

        // Compute corr and cov matrices.
        let std_devs = annual_probabilities
            .component_mul(&annual_probabilities.map(|x| 1.0 - x))
            .map(|x| x.sqrt());
        let cov_matrix = corr_matrix.map_with_location(|i, j, c| c * std_devs[i] * std_devs[j]);
        let joint_prob = cov_matrix
            .map_with_location(|i, j, c| c + annual_probabilities[i] * annual_probabilities[j]);

        // Compute risk from other markets.
        let total_weighted_prob = market_weights.dot(&annual_probabilities);
        let other_market_weights = (0..n).map(|i| 1. - market_weights[i]).collect::<Vec<f64>>();
        let probability_other_markets_trigger = (0..n)
            .map(|i| (total_weighted_prob - annual_probabilities[i] * &market_weights[i]))
            .collect::<Vec<f64>>();
        let correlation_with_other_markets = (0..n)
            .map(|i| {
                market_weights
                    .iter()
                    .enumerate()
                    .map(|(j, &w_j)| {
                        if j == i {
                            0.
                        } else {
                            w_j * corr_matrix.get((i, j)).unwrap()
                        }
                    })
                    .sum::<f64>()
                    / other_market_weights[i]
            })
            .collect::<Vec<f64>>();
        Self {
            annual_probabilities,
            corr_matrix,
            cov_matrix,
            joint_prob,
            leverage,
            market_weights,
            probability_other_markets_trigger,
            correlation_with_other_markets,
        }
    }

    pub fn set_risk_premium(
        &self,
        annual_market_apys: &DVector<f64>,
        portfolio_weights: &DVector<f64>,
    ) -> f64 {
        let annual_excess_returns = annual_market_apys / self.leverage - &self.annual_probabilities;
        portfolio_weights.dot(&annual_excess_returns)
    }

    pub fn expected_loss(&self, portfolio_weights: &DVector<f64>) -> f64 {
        portfolio_weights.dot(&self.annual_probabilities)
    }

    pub fn variance(&self, portfolio_weights: &DVector<f64>) -> f64 {
        portfolio_weights.dot(&(&self.cov_matrix * portfolio_weights))
    }
}
