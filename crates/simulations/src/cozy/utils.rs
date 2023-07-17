use std::borrow::Cow;

use serde::Deserialize;
use simulate::u256::U256;

pub struct Counter {
    count: u64,
}

impl Counter {
    pub fn new(start_value: u64) -> Counter {
        Counter { count: start_value }
    }

    pub fn get_and_increment_count(&mut self) -> u64 {
        let count: u64 = self.count;
        self.count += 1;
        count
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }
}

pub fn wad() -> U256 {
    U256::from(1e18 as u128)
}

pub fn float_to_wad(x: f64) -> U256 {
    U256::from((x * 1e18) as u128)
}

pub fn wad_to_float(x: U256) -> f64 {
    x.as_u128() as f64 / 1e18
}

pub fn deserialize_cow<'de, D>(deserializer: D) -> Result<Cow<'static, str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(Cow::Owned(s))
}

pub fn deserialize_cow_tuple_vec<'de, D, T>(
    deserializer: D,
) -> Result<Vec<(Cow<'static, str>, T)>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec: Vec<(String, T)> = Deserialize::deserialize(deserializer)?;
    let transformed_vec: Vec<(Cow<'static, str>, T)> =
        vec.into_iter().map(|(s, v)| (Cow::Owned(s), v)).collect();
    Ok(transformed_vec)
}
