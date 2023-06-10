pub use stub::*;
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
pub mod stub {
    #[rustfmt::skip]
    const __ABI: &str = "[]";
    ///The parsed JSON ABI of the contract.
    pub static STUB_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct Stub<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for Stub<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Stub<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Stub<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Stub<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Stub))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Stub<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                STUB_ABI.clone(),
                client,
            ))
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for Stub<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
