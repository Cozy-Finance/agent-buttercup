pub use ip_token_factory::*;
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
pub mod ip_token_factory {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IPToken\",\"name\":\"ptoken_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"PTokenDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"decimals_\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployPToken\",\"outputs\":[{\"internalType\":\"contract IPToken\",\"name\":\"ptoken_\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IPTOKENFACTORY_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IPTokenFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IPTokenFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IPTokenFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IPTokenFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IPTokenFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IPTokenFactory)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IPTokenFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    IPTOKENFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `deployPToken` (0x330b9319) function
        pub fn deploy_p_token(
            &self,
            decimals: u8,
            trigger: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([51, 11, 147, 25], (decimals, trigger))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PTokenDeployed` event
        pub fn p_token_deployed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PtokenDeployedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PtokenDeployedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for IPTokenFactory<M> {
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
        name = "PTokenDeployed",
        abi = "PTokenDeployed(address,address,address,uint8)"
    )]
    pub struct PtokenDeployedFilter {
        pub ptoken: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trigger: ::ethers::core::types::Address,
        pub decimals: u8,
    }
    ///Container type for all input parameters for the `deployPToken` function with signature `deployPToken(uint8,address)` and selector `0x330b9319`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deployPToken", abi = "deployPToken(uint8,address)")]
    pub struct DeployPTokenCall {
        pub decimals: u8,
        pub trigger: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployPToken` function with signature `deployPToken(uint8,address)` and selector `0x330b9319`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeployPTokenReturn {
        pub ptoken: ::ethers::core::types::Address,
    }
}
