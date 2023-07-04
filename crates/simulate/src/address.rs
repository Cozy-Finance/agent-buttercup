use std::{
    fmt::{Display, Formatter, UpperHex},
    str::FromStr,
};

use ethers::types::H160;
use rand::Rng;
use revm::primitives::B160;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Address {
    pub value: [u8; 20],
}

impl Address {
    pub fn random() -> Self {
        Address {
            value: rand::random::<[u8; 20]>(),
        }
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in &self.value {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<B160>::into(self.value))?;
        Ok(())
    }
}

impl FromStr for Address {
    type Err = rustc_hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address {
            value: s.parse::<B160>()?.into(),
        })
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

impl From<Address> for H160 {
    fn from(val: Address) -> Self {
        val.value.into()
    }
}

impl From<Address> for B160 {
    fn from(val: Address) -> Self {
        val.value.into()
    }
}
