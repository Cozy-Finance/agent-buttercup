pub use i_manager::*;
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
pub mod i_manager {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"reserveAmount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"backstopAmount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CozyFeesClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct Delays\",\"name\":\"delays_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"configUpdateDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"configUpdateGracePeriod\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minDepositDuration\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redemptionDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"purchaseDelay\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelaysUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"depositCap_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositCapUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct Fees\",\"name\":\"fees_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"depositFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFeeBackstop\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFeeBackstop\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFeeBackstop\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"amount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetFeesClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowedMarketsPerSet\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"configUpdateDelay\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"configUpdateGracePeriod\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"depositFees\",\"outputs\":[{\"internalType\":\"struct ProtocolFees\",\"name\":\"protocolFees_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"reserveFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"backstopFee\",\"type\":\"uint16\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getDepositCap\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isSet\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"minDepositDuration\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ISet[]\",\"name\":\"sets_\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauser\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"purchaseDelay\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"purchaseFees\",\"outputs\":[{\"internalType\":\"struct ProtocolFees\",\"name\":\"protocolFees_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"reserveFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"backstopFee\",\"type\":\"uint16\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"redemptionDelay\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"redemptionDelay_\",\"type\":\"uint32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"saleFees\",\"outputs\":[{\"internalType\":\"struct ProtocolFees\",\"name\":\"protocolFees_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"reserveFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"backstopFee\",\"type\":\"uint16\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"contract ISet[]\",\"name\":\"sets_\",\"type\":\"address[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IManager)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IMANAGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `allowedMarketsPerSet` (0x9aeae457) function
        pub fn allowed_markets_per_set(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 234, 228, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configUpdateDelay` (0xc5f755f0) function
        pub fn config_update_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([197, 247, 85, 240], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configUpdateGracePeriod` (0x31cd4c9c) function
        pub fn config_update_grace_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([49, 205, 76, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositFees` (0x26741e7d) function
        pub fn deposit_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProtocolFees> {
            self.0
                .method_hash([38, 116, 30, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDepositCap` (0x2ec137a8) function
        pub fn get_deposit_cap(
            &self,
            asset: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 193, 55, 168], asset)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isSet` (0x74ebe3ec) function
        pub fn is_set(
            &self,
            set: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([116, 235, 227, 236], set)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minDepositDuration` (0x03814568) function
        pub fn min_deposit_duration(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([3, 129, 69, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x9755c6a7) function
        pub fn pause(
            &self,
            sets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 85, 198, 167], sets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauser` (0x9fd0506d) function
        pub fn pauser(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([159, 208, 80, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `purchaseDelay` (0xb2eafefa) function
        pub fn purchase_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([178, 234, 254, 250], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `purchaseFees` (0xb996d0a1) function
        pub fn purchase_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProtocolFees> {
            self.0
                .method_hash([185, 150, 208, 161], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redemptionDelay` (0x9b31c3a8) function
        pub fn redemption_delay(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([155, 49, 195, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `saleFees` (0x9c2d7d41) function
        pub fn sale_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ProtocolFees> {
            self.0
                .method_hash([156, 45, 125, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x03821452) function
        pub fn unpause(
            &self,
            sets: ::std::vec::Vec<::ethers::core::types::Address>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 130, 20, 82], sets)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `CozyFeesClaimed` event
        pub fn cozy_fees_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CozyFeesClaimedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DelaysUpdated` event
        pub fn delays_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DelaysUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DepositCapUpdated` event
        pub fn deposit_cap_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DepositCapUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeesUpdated` event
        pub fn fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeesUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetFeesClaimed` event
        pub fn set_fees_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetFeesClaimedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IManager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "CozyFeesClaimed",
        abi = "CozyFeesClaimed(address,uint128,uint128)"
    )]
    pub struct CozyFeesClaimedFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        pub reserve_amount: u128,
        pub backstop_amount: u128,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "DelaysUpdated",
        abi = "DelaysUpdated((uint256,uint256,uint256,uint256,uint256))"
    )]
    pub struct DelaysUpdatedFilter {
        pub delays: Delays,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "DepositCapUpdated", abi = "DepositCapUpdated(address,uint256)")]
    pub struct DepositCapUpdatedFilter {
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub deposit_cap: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "FeesUpdated",
        abi = "FeesUpdated((uint16,uint16,uint16,uint16,uint16,uint16))"
    )]
    pub struct FeesUpdatedFilter {
        pub fees: Fees,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetFeesClaimed", abi = "SetFeesClaimed(address,address,uint128)")]
    pub struct SetFeesClaimedFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: u128,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IManagerEvents {
        CozyFeesClaimedFilter(CozyFeesClaimedFilter),
        DelaysUpdatedFilter(DelaysUpdatedFilter),
        DepositCapUpdatedFilter(DepositCapUpdatedFilter),
        FeesUpdatedFilter(FeesUpdatedFilter),
        SetFeesClaimedFilter(SetFeesClaimedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CozyFeesClaimedFilter::decode_log(log) {
                return Ok(IManagerEvents::CozyFeesClaimedFilter(decoded));
            }
            if let Ok(decoded) = DelaysUpdatedFilter::decode_log(log) {
                return Ok(IManagerEvents::DelaysUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DepositCapUpdatedFilter::decode_log(log) {
                return Ok(IManagerEvents::DepositCapUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(IManagerEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SetFeesClaimedFilter::decode_log(log) {
                return Ok(IManagerEvents::SetFeesClaimedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CozyFeesClaimedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DelaysUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositCapUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FeesUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesClaimedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CozyFeesClaimedFilter> for IManagerEvents {
        fn from(value: CozyFeesClaimedFilter) -> Self {
            Self::CozyFeesClaimedFilter(value)
        }
    }
    impl ::core::convert::From<DelaysUpdatedFilter> for IManagerEvents {
        fn from(value: DelaysUpdatedFilter) -> Self {
            Self::DelaysUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositCapUpdatedFilter> for IManagerEvents {
        fn from(value: DepositCapUpdatedFilter) -> Self {
            Self::DepositCapUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeesUpdatedFilter> for IManagerEvents {
        fn from(value: FeesUpdatedFilter) -> Self {
            Self::FeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SetFeesClaimedFilter> for IManagerEvents {
        fn from(value: SetFeesClaimedFilter) -> Self {
            Self::SetFeesClaimedFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowedMarketsPerSet` function with signature `allowedMarketsPerSet()` and selector `0x9aeae457`
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
    #[ethcall(name = "allowedMarketsPerSet", abi = "allowedMarketsPerSet()")]
    pub struct AllowedMarketsPerSetCall;
    ///Container type for all input parameters for the `configUpdateDelay` function with signature `configUpdateDelay()` and selector `0xc5f755f0`
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
    #[ethcall(name = "configUpdateDelay", abi = "configUpdateDelay()")]
    pub struct ConfigUpdateDelayCall;
    ///Container type for all input parameters for the `configUpdateGracePeriod` function with signature `configUpdateGracePeriod()` and selector `0x31cd4c9c`
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
    #[ethcall(name = "configUpdateGracePeriod", abi = "configUpdateGracePeriod()")]
    pub struct ConfigUpdateGracePeriodCall;
    ///Container type for all input parameters for the `depositFees` function with signature `depositFees()` and selector `0x26741e7d`
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
    #[ethcall(name = "depositFees", abi = "depositFees()")]
    pub struct DepositFeesCall;
    ///Container type for all input parameters for the `getDepositCap` function with signature `getDepositCap(address)` and selector `0x2ec137a8`
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
    #[ethcall(name = "getDepositCap", abi = "getDepositCap(address)")]
    pub struct GetDepositCapCall {
        pub asset: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isSet` function with signature `isSet(address)` and selector `0x74ebe3ec`
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
    #[ethcall(name = "isSet", abi = "isSet(address)")]
    pub struct IsSetCall {
        pub set: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `minDepositDuration` function with signature `minDepositDuration()` and selector `0x03814568`
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
    #[ethcall(name = "minDepositDuration", abi = "minDepositDuration()")]
    pub struct MinDepositDurationCall;
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(address[])` and selector `0x9755c6a7`
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
    #[ethcall(name = "pause", abi = "pause(address[])")]
    pub struct PauseCall {
        pub sets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all input parameters for the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    #[ethcall(name = "pauser", abi = "pauser()")]
    pub struct PauserCall;
    ///Container type for all input parameters for the `purchaseDelay` function with signature `purchaseDelay()` and selector `0xb2eafefa`
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
    #[ethcall(name = "purchaseDelay", abi = "purchaseDelay()")]
    pub struct PurchaseDelayCall;
    ///Container type for all input parameters for the `purchaseFees` function with signature `purchaseFees()` and selector `0xb996d0a1`
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
    #[ethcall(name = "purchaseFees", abi = "purchaseFees()")]
    pub struct PurchaseFeesCall;
    ///Container type for all input parameters for the `redemptionDelay` function with signature `redemptionDelay()` and selector `0x9b31c3a8`
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
    #[ethcall(name = "redemptionDelay", abi = "redemptionDelay()")]
    pub struct RedemptionDelayCall;
    ///Container type for all input parameters for the `saleFees` function with signature `saleFees()` and selector `0x9c2d7d41`
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
    #[ethcall(name = "saleFees", abi = "saleFees()")]
    pub struct SaleFeesCall;
    ///Container type for all input parameters for the `unpause` function with signature `unpause(address[])` and selector `0x03821452`
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
    #[ethcall(name = "unpause", abi = "unpause(address[])")]
    pub struct UnpauseCall {
        pub sets: ::std::vec::Vec<::ethers::core::types::Address>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IManagerCalls {
        AllowedMarketsPerSet(AllowedMarketsPerSetCall),
        ConfigUpdateDelay(ConfigUpdateDelayCall),
        ConfigUpdateGracePeriod(ConfigUpdateGracePeriodCall),
        DepositFees(DepositFeesCall),
        GetDepositCap(GetDepositCapCall),
        IsSet(IsSetCall),
        MinDepositDuration(MinDepositDurationCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Pauser(PauserCall),
        PurchaseDelay(PurchaseDelayCall),
        PurchaseFees(PurchaseFeesCall),
        RedemptionDelay(RedemptionDelayCall),
        SaleFees(SaleFeesCall),
        Unpause(UnpauseCall),
    }
    impl ::ethers::core::abi::AbiDecode for IManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AllowedMarketsPerSetCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AllowedMarketsPerSet(decoded));
            }
            if let Ok(decoded)
                = <ConfigUpdateDelayCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigUpdateDelay(decoded));
            }
            if let Ok(decoded)
                = <ConfigUpdateGracePeriodCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConfigUpdateGracePeriod(decoded));
            }
            if let Ok(decoded)
                = <DepositFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DepositFees(decoded));
            }
            if let Ok(decoded)
                = <GetDepositCapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDepositCap(decoded));
            }
            if let Ok(decoded)
                = <IsSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsSet(decoded));
            }
            if let Ok(decoded)
                = <MinDepositDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinDepositDuration(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded)
                = <PauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pauser(decoded));
            }
            if let Ok(decoded)
                = <PurchaseDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PurchaseDelay(decoded));
            }
            if let Ok(decoded)
                = <PurchaseFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PurchaseFees(decoded));
            }
            if let Ok(decoded)
                = <RedemptionDelayCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RedemptionDelay(decoded));
            }
            if let Ok(decoded)
                = <SaleFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SaleFees(decoded));
            }
            if let Ok(decoded)
                = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllowedMarketsPerSet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigUpdateDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConfigUpdateGracePeriod(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDepositCap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinDepositDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PurchaseDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PurchaseFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedemptionDelay(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SaleFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllowedMarketsPerSet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigUpdateDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigUpdateGracePeriod(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDepositCap(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinDepositDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::PurchaseDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::PurchaseFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionDelay(element) => ::core::fmt::Display::fmt(element, f),
                Self::SaleFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllowedMarketsPerSetCall> for IManagerCalls {
        fn from(value: AllowedMarketsPerSetCall) -> Self {
            Self::AllowedMarketsPerSet(value)
        }
    }
    impl ::core::convert::From<ConfigUpdateDelayCall> for IManagerCalls {
        fn from(value: ConfigUpdateDelayCall) -> Self {
            Self::ConfigUpdateDelay(value)
        }
    }
    impl ::core::convert::From<ConfigUpdateGracePeriodCall> for IManagerCalls {
        fn from(value: ConfigUpdateGracePeriodCall) -> Self {
            Self::ConfigUpdateGracePeriod(value)
        }
    }
    impl ::core::convert::From<DepositFeesCall> for IManagerCalls {
        fn from(value: DepositFeesCall) -> Self {
            Self::DepositFees(value)
        }
    }
    impl ::core::convert::From<GetDepositCapCall> for IManagerCalls {
        fn from(value: GetDepositCapCall) -> Self {
            Self::GetDepositCap(value)
        }
    }
    impl ::core::convert::From<IsSetCall> for IManagerCalls {
        fn from(value: IsSetCall) -> Self {
            Self::IsSet(value)
        }
    }
    impl ::core::convert::From<MinDepositDurationCall> for IManagerCalls {
        fn from(value: MinDepositDurationCall) -> Self {
            Self::MinDepositDuration(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for IManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for IManagerCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauserCall> for IManagerCalls {
        fn from(value: PauserCall) -> Self {
            Self::Pauser(value)
        }
    }
    impl ::core::convert::From<PurchaseDelayCall> for IManagerCalls {
        fn from(value: PurchaseDelayCall) -> Self {
            Self::PurchaseDelay(value)
        }
    }
    impl ::core::convert::From<PurchaseFeesCall> for IManagerCalls {
        fn from(value: PurchaseFeesCall) -> Self {
            Self::PurchaseFees(value)
        }
    }
    impl ::core::convert::From<RedemptionDelayCall> for IManagerCalls {
        fn from(value: RedemptionDelayCall) -> Self {
            Self::RedemptionDelay(value)
        }
    }
    impl ::core::convert::From<SaleFeesCall> for IManagerCalls {
        fn from(value: SaleFeesCall) -> Self {
            Self::SaleFees(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for IManagerCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    ///Container type for all return fields from the `allowedMarketsPerSet` function with signature `allowedMarketsPerSet()` and selector `0x9aeae457`
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
    pub struct AllowedMarketsPerSetReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `configUpdateDelay` function with signature `configUpdateDelay()` and selector `0xc5f755f0`
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
    pub struct ConfigUpdateDelayReturn(pub u32);
    ///Container type for all return fields from the `configUpdateGracePeriod` function with signature `configUpdateGracePeriod()` and selector `0x31cd4c9c`
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
    pub struct ConfigUpdateGracePeriodReturn(pub u32);
    ///Container type for all return fields from the `depositFees` function with signature `depositFees()` and selector `0x26741e7d`
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
    pub struct DepositFeesReturn {
        pub protocol_fees: ProtocolFees,
    }
    ///Container type for all return fields from the `getDepositCap` function with signature `getDepositCap(address)` and selector `0x2ec137a8`
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
    pub struct GetDepositCapReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `isSet` function with signature `isSet(address)` and selector `0x74ebe3ec`
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
    pub struct IsSetReturn(pub bool);
    ///Container type for all return fields from the `minDepositDuration` function with signature `minDepositDuration()` and selector `0x03814568`
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
    pub struct MinDepositDurationReturn(pub u32);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    pub struct PauserReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `purchaseDelay` function with signature `purchaseDelay()` and selector `0xb2eafefa`
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
    pub struct PurchaseDelayReturn(pub u32);
    ///Container type for all return fields from the `purchaseFees` function with signature `purchaseFees()` and selector `0xb996d0a1`
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
    pub struct PurchaseFeesReturn {
        pub protocol_fees: ProtocolFees,
    }
    ///Container type for all return fields from the `redemptionDelay` function with signature `redemptionDelay()` and selector `0x9b31c3a8`
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
    pub struct RedemptionDelayReturn {
        pub redemption_delay: u32,
    }
    ///Container type for all return fields from the `saleFees` function with signature `saleFees()` and selector `0x9c2d7d41`
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
    pub struct SaleFeesReturn {
        pub protocol_fees: ProtocolFees,
    }
    ///`ProtocolFees(uint16,uint16)`
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
    pub struct ProtocolFees {
        pub reserve_fee: u16,
        pub backstop_fee: u16,
    }
}
