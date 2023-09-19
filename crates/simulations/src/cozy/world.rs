use std::{fmt::Debug, sync::Arc};

use nalgebra::DVector;
use simulate::state::{update::UpdateData, world::World};

#[derive(Debug, Clone)]
pub struct CozyWorld {
    pub set_analytics: Arc<SetAnalytics>,
}

impl CozyWorld {
    pub fn new() -> Self {
        log::info!("Creating Cozy World");
        CozyWorld {
            set_analytics: Arc::new(SetAnalytics::new(
                0.,
                DVector::zeros(0),
                DVector::zeros(0),
                DVector::zeros(0),
            )),
        }
    }
}

impl World for CozyWorld {
    type WorldUpdateData = CozyUpdate;
    fn execute(&mut self, update: Self::WorldUpdateData) -> Option<Self::WorldUpdateData> {
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

impl UpdateData for CozyUpdate {}

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
