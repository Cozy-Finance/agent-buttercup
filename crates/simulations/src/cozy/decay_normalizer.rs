use simulate::u256::U256;

use crate::cozy::{constants::SECONDS_IN_YEAR, utils::wad_to_float};

pub fn normalize_constant_decay_price(raw_price_percentage: f64, rate_per_second: U256) -> f64 {
    let rate_per_year = wad_to_float(rate_per_second * U256::from(SECONDS_IN_YEAR));
    // Integral of (1-x)^t for t = 0 to infty is 1/ln(1 - x)
    let survival_rate: f64 = 1. - rate_per_year;
    let effective_protection_duration = -1. / survival_rate.ln();
    raw_price_percentage / effective_protection_duration
}
