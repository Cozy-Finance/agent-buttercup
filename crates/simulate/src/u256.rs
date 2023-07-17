use ethers::prelude::U256 as EthersU256;
use revm::primitives::U256 as EvmU256;
use rand::{Rng};
use std::ops::Add;
use std::ops::Sub;
use std::cmp::Ordering;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// Define the U256 struct with conversion implementations
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct U256 {
    value: EthersU256,
}

trait UnwrapOr<U> {
  fn unwrap_or(self, default: U) -> U;
}

impl U256 {
  pub const ZERO: Self = Self { value: EthersU256::zero() };

  pub fn random() -> Self {
    U256 { value: rand::random::<[u8; 32]>().into() }
  }

  pub fn random_using<R>(rng: &mut R) -> Self
  where
    R: Rng + ?Sized,
  {
    let mut bits = [0u8; 32];
    rng.fill(&mut bits);
    U256 { value: EthersU256::from_big_endian(&bits) }
  }
}

impl UnwrapOr<U256> for Option<U256> {
  fn unwrap_or(self, default: U256) -> U256 {
      self.unwrap_or(default)
  }
}

impl From<EthersU256> for U256 {
  fn from(ethers_u256: EthersU256) -> Self {
      U256 { value: ethers_u256.into() }
  }
}

impl From<EvmU256> for U256 {
  fn from(revm_u256: EvmU256) -> Self {
    U256 { value: revm_u256.into() }
  }
}

impl From<u64> for U256 {
  fn from(val: u64) -> Self {
    U256 { value: val.into() }
  }
}

impl Into<EthersU256> for U256 {
  fn into(self) -> EthersU256 {
    self.value
  }
}

impl Into<EvmU256> for U256 {
  fn into(self) -> EvmU256 {
    self.value.into()
  }
}

impl Add for U256 {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    U256 {
      value: self.value + other.value,
    }
  }
}

impl Sub for U256 {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    U256 {
      value: self.value - other.value,
    }
  }
}

impl Ord for U256 {
  fn cmp(&self, other: &Self) -> Ordering {
      self.value.cmp(&other.value)
  }
}

impl PartialOrd for U256 {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
  }
}

impl<'de> Deserialize<'de> for U256 {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
      D: Deserializer<'de>,
  {
      let value: String = Deserialize::deserialize(deserializer)?;
      let value = value.trim_start_matches("0x");
      let value = EthersU256::from_str_radix(value, 16)
          .map_err(|_| serde::de::Error::custom("Invalid U256 value"))?;
      Ok(U256 { value })
  }
}

impl Serialize for U256 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: Serializer,
  {
      serializer.serialize_str(&format!("{:#x}", self.value))
  }
}