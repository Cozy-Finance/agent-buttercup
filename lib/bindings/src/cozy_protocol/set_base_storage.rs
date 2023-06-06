pub use set_base_storage::*;
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
pub mod set_base_storage {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accounting\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"assetBalance\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedSetOwnerFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyReserveFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyBackstopFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalPurchasesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalSalesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"assetsPendingRedemption\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"asset\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"backstop\",\"outputs\":[{\"internalType\":\"contract IBackstop\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"contract IManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"contract IPToken\",\"name\":\"ptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct MarketConfigStorage\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"enum MarketState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"activeProtection\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDecayRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDripRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"purchasesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"salesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastDecayTime\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ptokenFactory\",\"outputs\":[{\"internalType\":\"contract IPTokenFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setState\",\"outputs\":[{\"internalType\":\"enum SetState\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ITrigger\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerLookups\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"marketExists\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"marketId\",\"type\":\"uint16\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static SETBASESTORAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct SetBaseStorage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SetBaseStorage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SetBaseStorage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SetBaseStorage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SetBaseStorage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SetBaseStorage)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SetBaseStorage<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SETBASESTORAGE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accounting` (0x9624e83e) function
        pub fn accounting(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u128, u128, u128, u128, u128, u128, u128),
        > {
            self.0
                .method_hash([150, 36, 232, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `backstop` (0x7dea1817) function
        pub fn backstop(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 234, 24, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markets` (0xb1283e77) function
        pub fn markets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                MarketConfigStorage,
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
                u128,
                u64,
            ),
        > {
            self.0
                .method_hash([177, 40, 62, 119], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ptokenFactory` (0x764d6892) function
        pub fn ptoken_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([118, 77, 104, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x8b673e7a) function
        pub fn set_config(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u32, u16)> {
            self.0
                .method_hash([139, 103, 62, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setState` (0x1203402f) function
        pub fn set_state(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([18, 3, 64, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerLookups` (0x58cfaac7) function
        pub fn trigger_lookups(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, u16)> {
            self.0
                .method_hash([88, 207, 170, 199], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SetBaseStorage<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
    #[ethcall(name = "accounting", abi = "accounting()")]
    pub struct AccountingCall;
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `backstop` function with signature `backstop()` and selector `0x7dea1817`
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
    #[ethcall(name = "backstop", abi = "backstop()")]
    pub struct BackstopCall;
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    #[ethcall(name = "markets", abi = "markets(uint256)")]
    pub struct MarketsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    #[ethcall(name = "ptokenFactory", abi = "ptokenFactory()")]
    pub struct PtokenFactoryCall;
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    #[ethcall(name = "setConfig", abi = "setConfig()")]
    pub struct SetConfigCall;
    ///Container type for all input parameters for the `setState` function with signature `setState()` and selector `0x1203402f`
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
    #[ethcall(name = "setState", abi = "setState()")]
    pub struct SetStateCall;
    ///Container type for all input parameters for the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    #[ethcall(name = "triggerLookups", abi = "triggerLookups(address)")]
    pub struct TriggerLookupsCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SetBaseStorageCalls {
        Accounting(AccountingCall),
        Asset(AssetCall),
        Backstop(BackstopCall),
        Manager(ManagerCall),
        Markets(MarketsCall),
        PtokenFactory(PtokenFactoryCall),
        SetConfig(SetConfigCall),
        SetState(SetStateCall),
        TriggerLookups(TriggerLookupsCall),
    }
    impl ::ethers::core::abi::AbiDecode for SetBaseStorageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AccountingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Accounting(decoded));
            }
            if let Ok(decoded)
                = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded)
                = <BackstopCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Backstop(decoded));
            }
            if let Ok(decoded)
                = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded)
                = <MarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Markets(decoded));
            }
            if let Ok(decoded)
                = <PtokenFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PtokenFactory(decoded));
            }
            if let Ok(decoded)
                = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded)
                = <SetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetState(decoded));
            }
            if let Ok(decoded)
                = <TriggerLookupsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TriggerLookups(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetBaseStorageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Accounting(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Backstop(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Markets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PtokenFactory(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TriggerLookups(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for SetBaseStorageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Accounting(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Backstop(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Markets(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtokenFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerLookups(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccountingCall> for SetBaseStorageCalls {
        fn from(value: AccountingCall) -> Self {
            Self::Accounting(value)
        }
    }
    impl ::core::convert::From<AssetCall> for SetBaseStorageCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BackstopCall> for SetBaseStorageCalls {
        fn from(value: BackstopCall) -> Self {
            Self::Backstop(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for SetBaseStorageCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<MarketsCall> for SetBaseStorageCalls {
        fn from(value: MarketsCall) -> Self {
            Self::Markets(value)
        }
    }
    impl ::core::convert::From<PtokenFactoryCall> for SetBaseStorageCalls {
        fn from(value: PtokenFactoryCall) -> Self {
            Self::PtokenFactory(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for SetBaseStorageCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for SetBaseStorageCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<TriggerLookupsCall> for SetBaseStorageCalls {
        fn from(value: TriggerLookupsCall) -> Self {
            Self::TriggerLookups(value)
        }
    }
    ///Container type for all return fields from the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
    pub struct AccountingReturn {
        pub asset_balance: u128,
        pub accrued_set_owner_fees: u128,
        pub accrued_cozy_reserve_fees: u128,
        pub accrued_cozy_backstop_fees: u128,
        pub total_purchases_fees: u128,
        pub total_sales_fees: u128,
        pub assets_pending_redemption: u128,
    }
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `backstop` function with signature `backstop()` and selector `0x7dea1817`
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
    pub struct BackstopReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    pub struct MarketsReturn {
        pub ptoken: ::ethers::core::types::Address,
        pub trigger: ::ethers::core::types::Address,
        pub config: MarketConfigStorage,
        pub state: u8,
        pub active_protection: ::ethers::core::types::U256,
        pub last_decay_rate: ::ethers::core::types::U256,
        pub last_drip_rate: ::ethers::core::types::U256,
        pub purchases_fee_pool: u128,
        pub sales_fee_pool: u128,
        pub last_decay_time: u64,
    }
    ///Container type for all return fields from the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    pub struct PtokenFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    pub struct SetConfigReturn {
        pub leverage_factor: u32,
        pub deposit_fee: u16,
    }
    ///Container type for all return fields from the `setState` function with signature `setState()` and selector `0x1203402f`
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
    pub struct SetStateReturn(pub u8);
    ///Container type for all return fields from the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    pub struct TriggerLookupsReturn {
        pub market_exists: bool,
        pub market_id: u16,
    }
}
