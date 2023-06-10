pub use base_model_factory::*;
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
pub mod base_model_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isDeployed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static BASEMODELFACTORY_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct BaseModelFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for BaseModelFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for BaseModelFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for BaseModelFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for BaseModelFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(BaseModelFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> BaseModelFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                BASEMODELFACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `isDeployed` (0x90184b02) function
        pub fn is_deployed(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([144, 24, 75, 2], p0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for BaseModelFactory<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `isDeployed` function with signature `isDeployed(address)` and selector `0x90184b02`
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
    #[ethcall(name = "isDeployed", abi = "isDeployed(address)")]
    pub struct IsDeployedCall(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isDeployed` function with signature `isDeployed(address)` and selector `0x90184b02`
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
    pub struct IsDeployedReturn(pub bool);
}
