pub use i_backstop_events::*;
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
pub mod i_backstop_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"status_\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BackstopApprovalStatusUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Claim\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static IBACKSTOPEVENTS_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IBackstopEvents<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBackstopEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBackstopEvents<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBackstopEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBackstopEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IBackstopEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IBackstopEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IBACKSTOPEVENTS_ABI.clone(),
                client,
            ))
        }
        ///Gets the contract's `BackstopApprovalStatusUpdated` event
        pub fn backstop_approval_status_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BackstopApprovalStatusUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Claim` event
        pub fn claim_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ClaimFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IBackstopEventsEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for IBackstopEvents<M>
    {
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
        Hash,
    )]
    #[ethevent(
        name = "BackstopApprovalStatusUpdated",
        abi = "BackstopApprovalStatusUpdated(address,bool)"
    )]
    pub struct BackstopApprovalStatusUpdatedFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        pub status: bool,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Claim", abi = "Claim(address,address,uint256)")]
    pub struct ClaimFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBackstopEventsEvents {
        BackstopApprovalStatusUpdatedFilter(BackstopApprovalStatusUpdatedFilter),
        ClaimFilter(ClaimFilter),
    }
    impl ::ethers_contract::EthLogDecode for IBackstopEventsEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BackstopApprovalStatusUpdatedFilter::decode_log(log) {
                return Ok(IBackstopEventsEvents::BackstopApprovalStatusUpdatedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = ClaimFilter::decode_log(log) {
                return Ok(IBackstopEventsEvents::ClaimFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IBackstopEventsEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BackstopApprovalStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BackstopApprovalStatusUpdatedFilter> for IBackstopEventsEvents {
        fn from(value: BackstopApprovalStatusUpdatedFilter) -> Self {
            Self::BackstopApprovalStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFilter> for IBackstopEventsEvents {
        fn from(value: ClaimFilter) -> Self {
            Self::ClaimFilter(value)
        }
    }
}
