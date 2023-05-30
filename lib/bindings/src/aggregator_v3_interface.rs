pub use aggregator_v3_interface::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod aggregator_v3_interface {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"description\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint80\",\"name\":\"_roundId\",\"type\":\"uint80\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"latestRoundData\",\"outputs\":[{\"internalType\":\"uint80\",\"name\":\"roundId\",\"type\":\"uint80\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"answer\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"updatedAt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint80\",\"name\":\"answeredInRound\",\"type\":\"uint80\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"version\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static AGGREGATORV3INTERFACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct AggregatorV3Interface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AggregatorV3Interface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AggregatorV3Interface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AggregatorV3Interface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AggregatorV3Interface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(AggregatorV3Interface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AggregatorV3Interface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AGGREGATORV3INTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `description` (0x7284e416) function
        pub fn description(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([114, 132, 228, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRoundData` (0x9a6fc8f5) function
        pub fn get_round_data(
            &self,
            round_id: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([154, 111, 200, 245], round_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestRoundData` (0xfeaf968c) function
        pub fn latest_round_data(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
            ),
        > {
            self.0
                .method_hash([254, 175, 150, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `version` (0x54fd4d50) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 253, 77, 80], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AggregatorV3Interface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `description` function with signature `description()` and selector `0x7284e416`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "description", abi = "description()")]
    pub struct DescriptionCall;
    ///Container type for all input parameters for the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRoundData", abi = "getRoundData(uint80)")]
    pub struct GetRoundDataCall {
        pub round_id: u128,
    }
    ///Container type for all input parameters for the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "latestRoundData", abi = "latestRoundData()")]
    pub struct LatestRoundDataCall;
    ///Container type for all input parameters for the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "version", abi = "version()")]
    pub struct VersionCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AggregatorV3InterfaceCalls {
        Decimals(DecimalsCall),
        Description(DescriptionCall),
        GetRoundData(GetRoundDataCall),
        LatestRoundData(LatestRoundDataCall),
        Version(VersionCall),
    }
    impl ::ethers::core::abi::AbiDecode for AggregatorV3InterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded)
                = <DescriptionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Description(decoded));
            }
            if let Ok(decoded)
                = <GetRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRoundData(decoded));
            }
            if let Ok(decoded)
                = <LatestRoundDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LatestRoundData(decoded));
            }
            if let Ok(decoded)
                = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AggregatorV3InterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Decimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Description(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestRoundData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AggregatorV3InterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Description(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestRoundData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DecimalsCall> for AggregatorV3InterfaceCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DescriptionCall> for AggregatorV3InterfaceCalls {
        fn from(value: DescriptionCall) -> Self {
            Self::Description(value)
        }
    }
    impl ::core::convert::From<GetRoundDataCall> for AggregatorV3InterfaceCalls {
        fn from(value: GetRoundDataCall) -> Self {
            Self::GetRoundData(value)
        }
    }
    impl ::core::convert::From<LatestRoundDataCall> for AggregatorV3InterfaceCalls {
        fn from(value: LatestRoundDataCall) -> Self {
            Self::LatestRoundData(value)
        }
    }
    impl ::core::convert::From<VersionCall> for AggregatorV3InterfaceCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `description` function with signature `description()` and selector `0x7284e416`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DescriptionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getRoundData` function with signature `getRoundData(uint80)` and selector `0x9a6fc8f5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `latestRoundData` function with signature `latestRoundData()` and selector `0xfeaf968c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestRoundDataReturn {
        pub round_id: u128,
        pub answer: ::ethers::core::types::I256,
        pub started_at: ::ethers::core::types::U256,
        pub updated_at: ::ethers::core::types::U256,
        pub answered_in_round: u128,
    }
    ///Container type for all return fields from the `version` function with signature `version()` and selector `0x54fd4d50`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct VersionReturn(pub ::ethers::core::types::U256);
}
