use std::fmt::{
  Formatter,
  Result,
  UpperHex
};

use ethers::types::H160;
use rand::{Rng};
use revm::primitives::B160;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Address {
  pub value: [u8; 20]
}

impl Address {
  pub fn random() -> Self {
    Address { value: rand::random::<[u8; 20]>() }
  }

  pub fn random_using<R>(rng: &mut R) -> Self
  where
    R: Rng + ?Sized,
  {
    let mut value: [u8; 20] = [0; 20];
    rng.fill(&mut value);
    Address { value }
  }
}

impl UpperHex for Address {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      for byte in &self.value {
          write!(f, "{:02X}", byte)?;
      }
      Ok(())
  }
}

impl From<H160> for Address {
  fn from(h160: H160) -> Self {
    Address { value: h160.into() }
  }
}

impl From<B160> for Address {
  fn from(b160: B160) -> Self {
    Address { value: b160.into() }
  }
}

impl Into<H160> for Address {
  fn into(self) -> H160 {
    self.value.into()
  }
}

impl Into<B160> for Address {
  fn into(self) -> B160 {
    self.value.into()
  }
}