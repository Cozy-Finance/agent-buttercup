use std::{
    fmt::{Display, Formatter, UpperHex},
    str::FromStr,
};

use ethers::abi::{InvalidOutputType, Token, Tokenizable};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{EthersAddress, EvmAddress};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address {
    #[serde(with = "hex")]
    pub value: [u8; 20],
}

impl Address {
    pub fn zero() -> Self {
        Address { value: [0; 20] }
    }

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
        write!(f, "{}", Into::<EvmAddress>::into(self.value))?;
        Ok(())
    }
}

impl FromStr for Address {
    type Err = rustc_hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Address {
            value: s.parse::<EvmAddress>()?.into(),
        })
    }
}

impl From<EthersAddress> for Address {
    fn from(h160: EthersAddress) -> Self {
        Address { value: h160.into() }
    }
}

impl From<EvmAddress> for Address {
    fn from(b160: EvmAddress) -> Self {
        Address { value: b160.into() }
    }
}

impl From<Address> for EthersAddress {
    fn from(val: Address) -> Self {
        val.value.into()
    }
}

impl From<Address> for EvmAddress {
    fn from(val: Address) -> Self {
        val.value.into()
    }
}

impl Tokenizable for Address {
    fn from_token(token: Token) -> Result<Self, InvalidOutputType> {
        EthersAddress::from_token(token).map(Address::from)
    }

    fn into_token(self) -> Token {
        Token::Address(self.into())
    }
}
