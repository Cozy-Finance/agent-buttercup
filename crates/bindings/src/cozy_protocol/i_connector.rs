pub use i_connector::*;
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
pub mod i_connector {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"baseAsset\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToBaseAssetsNeeded\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToWrappedAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unwrapWrappedAsset\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"recipient_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"wrapBaseAsset\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICONNECTOR_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IConnector<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IConnector<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IConnector<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IConnector<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IConnector<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IConnector))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IConnector<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                ICONNECTOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], account)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `baseAsset` (0xcdf456e1) function
        pub fn base_asset(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([205, 244, 86, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToBaseAssetsNeeded` (0x366ccb22) function
        pub fn convert_to_base_assets_needed(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 108, 203, 34], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToWrappedAssets` (0x47525990) function
        pub fn convert_to_wrapped_assets(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 82, 89, 144], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unwrapWrappedAsset` (0x4f30e30b) function
        pub fn unwrap_wrapped_asset(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 48, 227, 11], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `wrapBaseAsset` (0x6dc3414a) function
        pub fn wrap_base_asset(
            &self,
            recipient: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 195, 65, 74], (recipient, amount))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for IConnector<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `baseAsset` function with signature `baseAsset()` and selector `0xcdf456e1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "baseAsset", abi = "baseAsset()")]
    pub struct BaseAssetCall;
    ///Container type for all input parameters for the `convertToBaseAssetsNeeded` function with signature `convertToBaseAssetsNeeded(uint256)` and selector `0x366ccb22`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "convertToBaseAssetsNeeded",
        abi = "convertToBaseAssetsNeeded(uint256)"
    )]
    pub struct ConvertToBaseAssetsNeededCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToWrappedAssets` function with signature `convertToWrappedAssets(uint256)` and selector `0x47525990`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "convertToWrappedAssets",
        abi = "convertToWrappedAssets(uint256)"
    )]
    pub struct ConvertToWrappedAssetsCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `unwrapWrappedAsset` function with signature `unwrapWrappedAsset(address,uint256)` and selector `0x4f30e30b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "unwrapWrappedAsset",
        abi = "unwrapWrappedAsset(address,uint256)"
    )]
    pub struct UnwrapWrappedAssetCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `wrapBaseAsset` function with signature `wrapBaseAsset(address,uint256)` and selector `0x6dc3414a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "wrapBaseAsset", abi = "wrapBaseAsset(address,uint256)")]
    pub struct WrapBaseAssetCall {
        pub recipient: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IConnectorCalls {
        BalanceOf(BalanceOfCall),
        BaseAsset(BaseAssetCall),
        ConvertToBaseAssetsNeeded(ConvertToBaseAssetsNeededCall),
        ConvertToWrappedAssets(ConvertToWrappedAssetsCall),
        UnwrapWrappedAsset(UnwrapWrappedAssetCall),
        WrapBaseAsset(WrapBaseAssetCall),
    }
    impl ::ethers::core::abi::AbiDecode for IConnectorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <BaseAssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BaseAsset(decoded));
            }
            if let Ok(decoded) =
                <ConvertToBaseAssetsNeededCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToBaseAssetsNeeded(decoded));
            }
            if let Ok(decoded) =
                <ConvertToWrappedAssetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToWrappedAssets(decoded));
            }
            if let Ok(decoded) =
                <UnwrapWrappedAssetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnwrapWrappedAsset(decoded));
            }
            if let Ok(decoded) = <WrapBaseAssetCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrapBaseAsset(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IConnectorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BaseAsset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConvertToBaseAssetsNeeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToWrappedAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnwrapWrappedAsset(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WrapBaseAsset(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IConnectorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BaseAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToBaseAssetsNeeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToWrappedAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnwrapWrappedAsset(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrapBaseAsset(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BalanceOfCall> for IConnectorCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BaseAssetCall> for IConnectorCalls {
        fn from(value: BaseAssetCall) -> Self {
            Self::BaseAsset(value)
        }
    }
    impl ::core::convert::From<ConvertToBaseAssetsNeededCall> for IConnectorCalls {
        fn from(value: ConvertToBaseAssetsNeededCall) -> Self {
            Self::ConvertToBaseAssetsNeeded(value)
        }
    }
    impl ::core::convert::From<ConvertToWrappedAssetsCall> for IConnectorCalls {
        fn from(value: ConvertToWrappedAssetsCall) -> Self {
            Self::ConvertToWrappedAssets(value)
        }
    }
    impl ::core::convert::From<UnwrapWrappedAssetCall> for IConnectorCalls {
        fn from(value: UnwrapWrappedAssetCall) -> Self {
            Self::UnwrapWrappedAsset(value)
        }
    }
    impl ::core::convert::From<WrapBaseAssetCall> for IConnectorCalls {
        fn from(value: WrapBaseAssetCall) -> Self {
            Self::WrapBaseAsset(value)
        }
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `baseAsset` function with signature `baseAsset()` and selector `0xcdf456e1`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BaseAssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `convertToBaseAssetsNeeded` function with signature `convertToBaseAssetsNeeded(uint256)` and selector `0x366ccb22`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ConvertToBaseAssetsNeededReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToWrappedAssets` function with signature `convertToWrappedAssets(uint256)` and selector `0x47525990`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ConvertToWrappedAssetsReturn(pub ::ethers::core::types::U256);
}
