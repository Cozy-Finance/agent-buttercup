pub use i_asset_redeemer_errors::*;
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
pub mod i_asset_redeemer_errors {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"DelayNotElapsed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RedemptionNotFound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxRedeemableShares_\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"RedemptionRequestExceedsMax\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IASSETREDEEMERERRORS_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IAssetRedeemerErrors<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IAssetRedeemerErrors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IAssetRedeemerErrors<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IAssetRedeemerErrors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IAssetRedeemerErrors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IAssetRedeemerErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IAssetRedeemerErrors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IASSETREDEEMERERRORS_ABI.clone(),
                client,
            ))
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for IAssetRedeemerErrors<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DelayNotElapsed` with signature `DelayNotElapsed()` and selector `0x27836e1f`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DelayNotElapsed", abi = "DelayNotElapsed()")]
    pub struct DelayNotElapsed;
    ///Custom Error type `RedemptionNotFound` with signature `RedemptionNotFound()` and selector `0x986a1905`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RedemptionNotFound", abi = "RedemptionNotFound()")]
    pub struct RedemptionNotFound;
    ///Custom Error type `RedemptionRequestExceedsMax` with signature `RedemptionRequestExceedsMax(uint256)` and selector `0x59818bce`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "RedemptionRequestExceedsMax",
        abi = "RedemptionRequestExceedsMax(uint256)"
    )]
    pub struct RedemptionRequestExceedsMax {
        pub max_redeemable_shares: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IAssetRedeemerErrorsErrors {
        DelayNotElapsed(DelayNotElapsed),
        RedemptionNotFound(RedemptionNotFound),
        RedemptionRequestExceedsMax(RedemptionRequestExceedsMax),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for IAssetRedeemerErrorsErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DelayNotElapsed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelayNotElapsed(decoded));
            }
            if let Ok(decoded) =
                <RedemptionNotFound as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RedemptionNotFound(decoded));
            }
            if let Ok(decoded) =
                <RedemptionRequestExceedsMax as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RedemptionRequestExceedsMax(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IAssetRedeemerErrorsErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DelayNotElapsed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RedemptionNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedemptionRequestExceedsMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for IAssetRedeemerErrorsErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DelayNotElapsed as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <RedemptionNotFound as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RedemptionRequestExceedsMax as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for IAssetRedeemerErrorsErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelayNotElapsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionRequestExceedsMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for IAssetRedeemerErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DelayNotElapsed> for IAssetRedeemerErrorsErrors {
        fn from(value: DelayNotElapsed) -> Self {
            Self::DelayNotElapsed(value)
        }
    }
    impl ::core::convert::From<RedemptionNotFound> for IAssetRedeemerErrorsErrors {
        fn from(value: RedemptionNotFound) -> Self {
            Self::RedemptionNotFound(value)
        }
    }
    impl ::core::convert::From<RedemptionRequestExceedsMax> for IAssetRedeemerErrorsErrors {
        fn from(value: RedemptionRequestExceedsMax) -> Self {
            Self::RedemptionRequestExceedsMax(value)
        }
    }
}
