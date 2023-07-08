use std::borrow::Cow;

use eyre::Result;
use serde::{Deserialize, Deserializer};

use crate::cozy::EthersU256;

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

pub fn wad() -> EthersU256 {
    EthersU256::from(1e18 as u128)
}

pub fn float_to_wad(x: f64) -> EthersU256 {
    EthersU256::from((x * 1e18) as u128)
}

pub fn wad_to_float(x: EthersU256) -> f64 {
    x.as_u128() as f64 / 1e18
}

pub fn deserialize_string_to_u256<'de, D>(deserializer: D) -> Result<EthersU256, D::Error>
where
    D: Deserializer<'de>,
{
    let string_value: String = serde::Deserialize::deserialize(deserializer)?;
    let u256_value: EthersU256 =
        EthersU256::from_dec_str(string_value.as_str()).map_err(serde::de::Error::custom)?;
    Ok(u256_value)
}

pub fn deserialize_cow<'de, D>(deserializer: D) -> Result<Cow<'static, str>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(Cow::Owned(s.into()))
}

pub fn deserialize_cow_tuple_vec<'de, D, T>(
    deserializer: D,
) -> Result<Vec<(Cow<'static, str>, T)>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let vec: Vec<(String, T)> = Deserialize::deserialize(deserializer)?;
    let transformed_vec: Vec<(Cow<'static, str>, T)> = vec
        .into_iter()
        .map(|(s, v)| (Cow::Owned(s.into()), v))
        .collect();
    Ok(transformed_vec)
}
