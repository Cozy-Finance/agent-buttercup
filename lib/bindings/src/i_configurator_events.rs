pub use i_configurator_events::*;
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
pub mod i_configurator_events {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateTime_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateDeadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesQueued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"ptokenAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketCreated\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static ICONFIGURATOREVENTS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IConfiguratorEvents<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IConfiguratorEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IConfiguratorEvents<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IConfiguratorEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IConfiguratorEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IConfiguratorEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IConfiguratorEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ICONFIGURATOREVENTS_ABI.clone(),
                client,
            ))
        }
        ///Gets the contract's `ConfigUpdatesFinalized` event
        pub fn config_updates_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesFinalizedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConfigUpdatesQueued` event
        pub fn config_updates_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesQueuedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MarketCreated` event
        pub fn market_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarketCreatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IConfiguratorEventsEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IConfiguratorEvents<M>
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
        name = "ConfigUpdatesFinalized",
        abi = "ConfigUpdatesFinalized((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct ConfigUpdatesFinalizedFilter {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
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
        name = "ConfigUpdatesQueued",
        abi = "ConfigUpdatesQueued((uint32,uint16),(address,address,address,uint16,uint16,uint16)[],uint256,uint256)"
    )]
    pub struct ConfigUpdatesQueuedFilter {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
        pub update_time: ::ethers::core::types::U256,
        pub update_deadline: ::ethers::core::types::U256,
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
    #[ethevent(name = "MarketCreated", abi = "MarketCreated(uint16,address)")]
    pub struct MarketCreatedFilter {
        #[ethevent(indexed)]
        pub market_id: u16,
        pub ptoken_address: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IConfiguratorEventsEvents {
        ConfigUpdatesFinalizedFilter(ConfigUpdatesFinalizedFilter),
        ConfigUpdatesQueuedFilter(ConfigUpdatesQueuedFilter),
        MarketCreatedFilter(MarketCreatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for IConfiguratorEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ConfigUpdatesFinalizedFilter::decode_log(log) {
                return Ok(IConfiguratorEventsEvents::ConfigUpdatesFinalizedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ConfigUpdatesQueuedFilter::decode_log(log) {
                return Ok(IConfiguratorEventsEvents::ConfigUpdatesQueuedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = MarketCreatedFilter::decode_log(log) {
                return Ok(IConfiguratorEventsEvents::MarketCreatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IConfiguratorEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConfigUpdatesFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigUpdatesQueuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConfigUpdatesFinalizedFilter> for IConfiguratorEventsEvents {
        fn from(value: ConfigUpdatesFinalizedFilter) -> Self {
            Self::ConfigUpdatesFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<ConfigUpdatesQueuedFilter> for IConfiguratorEventsEvents {
        fn from(value: ConfigUpdatesQueuedFilter) -> Self {
            Self::ConfigUpdatesQueuedFilter(value)
        }
    }
    impl ::core::convert::From<MarketCreatedFilter> for IConfiguratorEventsEvents {
        fn from(value: MarketCreatedFilter) -> Self {
            Self::MarketCreatedFilter(value)
        }
    }
}
