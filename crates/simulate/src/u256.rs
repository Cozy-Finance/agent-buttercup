use ethers::prelude::U256 as EthersU256;
use revm::primitives::U256 as RevmU256;
use rand::{Rng};
use std::ops::Add;
use std::cmp::Ordering;

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

impl From<RevmU256> for U256 {
  fn from(revm_u256: RevmU256) -> Self {
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

impl Into<RevmU256> for U256 {
  fn into(self) -> RevmU256 {
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