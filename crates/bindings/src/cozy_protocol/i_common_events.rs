pub use i_common_events::*;
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
pub mod i_common_events {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"reserveFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"backstopFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"setOwnerFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesAccrued\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static ICOMMONEVENTS_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct ICommonEvents<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICommonEvents<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICommonEvents<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICommonEvents<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICommonEvents<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICommonEvents))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ICommonEvents<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                ICOMMONEVENTS_ABI.clone(),
                client,
            ))
        }
        ///Gets the contract's `FeesAccrued` event
        pub fn fees_accrued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, FeesAccruedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, FeesAccruedFilter> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for ICommonEvents<M> {
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
    #[ethevent(name = "FeesAccrued", abi = "FeesAccrued(uint128,uint128,uint128)")]
    pub struct FeesAccruedFilter {
        pub reserve_fees: u128,
        pub backstop_fees: u128,
        pub set_owner_fees: u128,
    }
}
