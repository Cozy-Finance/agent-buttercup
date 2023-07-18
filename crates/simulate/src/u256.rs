use ethers::prelude::U256 as EthersU256;
use revm::primitives::U256 as EvmU256;
use serde::{Deserializer, Serializer};

pub type U256 = EthersU256;

pub fn f64_to_u256(x: f64) -> U256 {
    EvmU256::from(x).into()
}

pub fn deserialize_string_to_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let string_value: String = serde::Deserialize::deserialize(deserializer)?;
    let u256_value: U256 =
        U256::from_dec_str(string_value.as_str()).map_err(serde::de::Error::custom)?;
    Ok(u256_value)
}

pub fn serialize_u256_to_u128<S>(value: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let u128_value = value.as_u128();
    serializer.serialize_u128(u128_value)
}
