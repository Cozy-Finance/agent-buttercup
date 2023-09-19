use std::{fmt::Debug, sync::Arc};

use nalgebra::DVector;
use simulate::state::{update::Update, World};

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub set_analytics: Arc<SetAnalytics>,
}

impl Default for CozyWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl CozyWorld {
    pub fn new() -> Self {
        log::info!("Creating Cozy World");
        CozyWorld {
            set_analytics: Arc::new(SetAnalytics::new(
                0.,
                DVector::<f64>::zeros(0),
                DVector::<f64>::zeros(0),
                DVector::<f64>::zeros(0),
            )),
        }
    }
}

impl World for CozyWorld {
    type WorldUpdate = CozyUpdate;
    fn execute(&mut self, update: Self::WorldUpdate) -> Option<Self::WorldUpdate> {
        match update {
            CozyUpdate::UpdateSetAnalytics(new_set_analytics) => {
                self.set_analytics = Arc::new(new_set_analytics)
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum CozyUpdate {
    UpdateSetAnalytics(SetAnalytics),
}

impl Update for CozyUpdate {}

#[derive(Debug, Clone)]
pub struct SetAnalytics {
    pub set_apy: f64,
    pub market_apys: DVector<f64>,
    pub portfolio_weights: DVector<f64>,
    pub utilizations: DVector<f64>,
}

impl SetAnalytics {
    pub fn new(
        set_apy: f64,
        market_apys: DVector<f64>,
        portfolio_weights: DVector<f64>,
        utilizations: DVector<f64>,
    ) -> Self {
        Self {
            set_apy,
            market_apys,
            portfolio_weights,
            utilizations,
        }
    }
}
