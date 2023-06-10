pub use i_ownable::*;
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
pub mod i_ownable {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IOWNABLE_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IOwnable<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IOwnable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IOwnable<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IOwnable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IOwnable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IOwnable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IOwnable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IOWNABLE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for IOwnable<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
}
