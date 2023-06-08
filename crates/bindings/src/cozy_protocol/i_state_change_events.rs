pub use i_state_change_events::*;
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
pub mod i_state_change_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum MarketState\",\"name\":\"updatedTo_\",\"type\":\"uint8\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MarketStateUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"enum SetState\",\"name\":\"updatedTo_\",\"type\":\"uint8\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SetStateUpdated\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static ISTATECHANGEEVENTS_ABI: ::ethers_contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IStateChangeEvents<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IStateChangeEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IStateChangeEvents<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IStateChangeEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IStateChangeEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IStateChangeEvents)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IStateChangeEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ISTATECHANGEEVENTS_ABI.clone(),
                    client,
                ),
            )
        }
        ///Gets the contract's `MarketStateUpdated` event
        pub fn market_state_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MarketStateUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SetStateUpdated` event
        pub fn set_state_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SetStateUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IStateChangeEventsEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for IStateChangeEvents<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "MarketStateUpdated",
        abi = "MarketStateUpdated(uint16,address,uint8)"
    )]
    pub struct MarketStateUpdatedFilter {
        #[ethevent(indexed)]
        pub market_id: u16,
        #[ethevent(indexed)]
        pub trigger: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub updated_to: u8,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "SetStateUpdated", abi = "SetStateUpdated(uint8)")]
    pub struct SetStateUpdatedFilter {
        #[ethevent(indexed)]
        pub updated_to: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IStateChangeEventsEvents {
        MarketStateUpdatedFilter(MarketStateUpdatedFilter),
        SetStateUpdatedFilter(SetStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for IStateChangeEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = MarketStateUpdatedFilter::decode_log(log) {
                return Ok(IStateChangeEventsEvents::MarketStateUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SetStateUpdatedFilter::decode_log(log) {
                return Ok(IStateChangeEventsEvents::SetStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IStateChangeEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MarketStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<MarketStateUpdatedFilter> for IStateChangeEventsEvents {
        fn from(value: MarketStateUpdatedFilter) -> Self {
            Self::MarketStateUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SetStateUpdatedFilter> for IStateChangeEventsEvents {
        fn from(value: SetStateUpdatedFilter) -> Self {
            Self::SetStateUpdatedFilter(value)
        }
    }
}
