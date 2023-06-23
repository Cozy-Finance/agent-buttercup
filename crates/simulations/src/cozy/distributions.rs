use rand::Rng;
use serde::Deserialize;

use crate::cozy::EthersU256;

pub trait Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> T;
}

#[derive(Debug, Clone, Deserialize)]
pub struct UniformRange<T> {
    pub min: T,
    pub max: T,
}

impl Distribution<EthersU256> for UniformRange<EthersU256> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EthersU256 {
        let mut sample = [0_u8; 32];
        rng.fill(&mut sample[..]);
        EthersU256::from(sample) % (self.max - self.min) + self.min
    }
}
