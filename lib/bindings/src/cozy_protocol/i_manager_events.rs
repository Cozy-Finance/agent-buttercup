pub use i_manager_events::*;
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
pub mod i_manager_events {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint128\",\"name\":\"reserveAmount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"backstopAmount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"CozyFeesClaimed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct Delays\",\"name\":\"delays_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"configUpdateDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"configUpdateGracePeriod\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minDepositDuration\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"redemptionDelay\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"purchaseDelay\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"DelaysUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"depositCap_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DepositCapUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct Fees\",\"name\":\"fees_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint16\",\"name\":\"depositFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFeeBackstop\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFeeBackstop\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFeeReserves\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFeeBackstop\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"amount_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetFeesClaimed\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static IMANAGEREVENTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IManagerEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IManagerEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IManagerEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IManagerEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IManagerEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IManagerEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IManagerEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IMANAGEREVENTS_ABI.clone(),
                client,
            ))
        }
        ///Gets the contract's `CozyFeesClaimed` event
        pub fn cozy_fees_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CozyFeesClaimedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DelaysUpdated` event
        pub fn delays_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DelaysUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `DepositCapUpdated` event
        pub fn deposit_cap_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositCapUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeesUpdated` event
        pub fn fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeesUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `SetFeesClaimed` event
        pub fn set_fees_claimed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetFeesClaimedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IManagerEventsEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IManagerEvents<M>
    {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(
        name = "SetFeesClaimed",
        abi = "SetFeesClaimed(address,address,uint128)"
    )]
    pub struct SetFeesClaimedFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub amount: u128,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IManagerEventsEvents {
        CozyFeesClaimedFilter(CozyFeesClaimedFilter),
        DelaysUpdatedFilter(DelaysUpdatedFilter),
        DepositCapUpdatedFilter(DepositCapUpdatedFilter),
        FeesUpdatedFilter(FeesUpdatedFilter),
        SetFeesClaimedFilter(SetFeesClaimedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IManagerEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CozyFeesClaimedFilter::decode_log(log) {
                return Ok(IManagerEventsEvents::CozyFeesClaimedFilter(decoded));
            }
            if let Ok(decoded) = DelaysUpdatedFilter::decode_log(log) {
                return Ok(IManagerEventsEvents::DelaysUpdatedFilter(decoded));
            }
            if let Ok(decoded) = DepositCapUpdatedFilter::decode_log(log) {
                return Ok(IManagerEventsEvents::DepositCapUpdatedFilter(decoded));
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(IManagerEventsEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SetFeesClaimedFilter::decode_log(log) {
                return Ok(IManagerEventsEvents::SetFeesClaimedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IManagerEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CozyFeesClaimedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelaysUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositCapUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFeesClaimedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CozyFeesClaimedFilter> for IManagerEventsEvents {
        fn from(value: CozyFeesClaimedFilter) -> Self {
            Self::CozyFeesClaimedFilter(value)
        }
    }
    impl ::core::convert::From<DelaysUpdatedFilter> for IManagerEventsEvents {
        fn from(value: DelaysUpdatedFilter) -> Self {
            Self::DelaysUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<DepositCapUpdatedFilter> for IManagerEventsEvents {
        fn from(value: DepositCapUpdatedFilter) -> Self {
            Self::DepositCapUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<FeesUpdatedFilter> for IManagerEventsEvents {
        fn from(value: FeesUpdatedFilter) -> Self {
            Self::FeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SetFeesClaimedFilter> for IManagerEventsEvents {
        fn from(value: SetFeesClaimedFilter) -> Self {
            Self::SetFeesClaimedFilter(value)
        }
    }
}
