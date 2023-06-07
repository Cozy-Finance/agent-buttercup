pub use i_common_errors::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod i_common_errors {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidStateTransition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RoundsToZero\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICOMMONERRORS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ICommonErrors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICommonErrors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICommonErrors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICommonErrors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICommonErrors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICommonErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ICommonErrors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ICOMMONERRORS_ABI.clone(),
                client,
            ))
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ICommonErrors<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidState` with signature `InvalidState()` and selector `0xbaf3f0f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidState", abi = "InvalidState()")]
    pub struct InvalidState;
    ///Custom Error type `InvalidStateTransition` with signature `InvalidStateTransition()` and selector `0x8f9a780c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidStateTransition", abi = "InvalidStateTransition()")]
    pub struct InvalidStateTransition;
    ///Custom Error type `RoundsToZero` with signature `RoundsToZero()` and selector `0xc440e0aa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RoundsToZero", abi = "RoundsToZero()")]
    pub struct RoundsToZero;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ICommonErrorsErrors {
        InvalidState(InvalidState),
        InvalidStateTransition(InvalidStateTransition),
        RoundsToZero(RoundsToZero),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ICommonErrorsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidState as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidState(decoded));
            }
            if let Ok(decoded) =
                <InvalidStateTransition as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidStateTransition(decoded));
            }
            if let Ok(decoded) = <RoundsToZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundsToZero(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ICommonErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidStateTransition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundsToZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ICommonErrorsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <InvalidState as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidStateTransition as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <RoundsToZero as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ICommonErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidState(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStateTransition(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundsToZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ICommonErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidState> for ICommonErrorsErrors {
        fn from(value: InvalidState) -> Self {
            Self::InvalidState(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for ICommonErrorsErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<RoundsToZero> for ICommonErrorsErrors {
        fn from(value: RoundsToZero) -> Self {
            Self::RoundsToZero(value)
        }
    }
}
