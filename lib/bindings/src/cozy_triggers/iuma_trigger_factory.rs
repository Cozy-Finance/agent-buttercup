pub use iuma_trigger_factory::*;
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
pub mod iuma_trigger_factory {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"triggerConfigId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"umaOracleFinder\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"string\",\"name\":\"query\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"rewardToken\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"rewardAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"refundRecipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"bondAmount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"proposalDisputeWindow\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"category\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"logoURI\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TriggerDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_query\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_rewardToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_refundRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_bondAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_proposalDisputeWindow\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_triggerCount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeTriggerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_query\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_rewardToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_refundRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_bondAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_proposalDisputeWindow\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct TriggerMetadata\",\"name\":\"_metadata\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"category\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"logoURI\",\"type\":\"string\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployTrigger\",\"outputs\":[{\"internalType\":\"contract IUMATrigger\",\"name\":\"_trigger\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_query\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_rewardToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_refundRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_bondAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_proposalDisputeWindow\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"findAvailableTrigger\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleFinder\",\"outputs\":[{\"internalType\":\"contract FinderInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"_query\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"_rewardToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_rewardAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"_refundRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_bondAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_proposalDisputeWindow\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerConfigId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUMATRIGGERFACTORY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IUMATriggerFactory<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUMATriggerFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUMATriggerFactory<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUMATriggerFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUMATriggerFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUMATriggerFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUMATriggerFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                IUMATRIGGERFACTORY_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `computeTriggerAddress` (0x9d616791) function
        pub fn compute_trigger_address(
            &self,
            query: ::std::string::String,
            reward_token: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
            refund_recipient: ::ethers::core::types::Address,
            bond_amount: ::ethers::core::types::U256,
            proposal_dispute_window: ::ethers::core::types::U256,
            trigger_count: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [157, 97, 103, 145],
                    (
                        query,
                        reward_token,
                        reward_amount,
                        refund_recipient,
                        bond_amount,
                        proposal_dispute_window,
                        trigger_count,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployTrigger` (0x6922a512) function
        pub fn deploy_trigger(
            &self,
            query: ::std::string::String,
            reward_token: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
            refund_recipient: ::ethers::core::types::Address,
            bond_amount: ::ethers::core::types::U256,
            proposal_dispute_window: ::ethers::core::types::U256,
            metadata: TriggerMetadata,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [105, 34, 165, 18],
                    (
                        query,
                        reward_token,
                        reward_amount,
                        refund_recipient,
                        bond_amount,
                        proposal_dispute_window,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findAvailableTrigger` (0xe7ad052b) function
        pub fn find_available_trigger(
            &self,
            query: ::std::string::String,
            reward_token: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
            refund_recipient: ::ethers::core::types::Address,
            bond_amount: ::ethers::core::types::U256,
            proposal_dispute_window: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [231, 173, 5, 43],
                    (
                        query,
                        reward_token,
                        reward_amount,
                        refund_recipient,
                        bond_amount,
                        proposal_dispute_window,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracleFinder` (0x9ceb3ea2) function
        pub fn oracle_finder(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([156, 235, 62, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerConfigId` (0xdf86c083) function
        pub fn trigger_config_id(
            &self,
            query: ::std::string::String,
            reward_token: ::ethers::core::types::Address,
            reward_amount: ::ethers::core::types::U256,
            refund_recipient: ::ethers::core::types::Address,
            bond_amount: ::ethers::core::types::U256,
            proposal_dispute_window: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [223, 134, 192, 131],
                    (
                        query,
                        reward_token,
                        reward_amount,
                        refund_recipient,
                        bond_amount,
                        proposal_dispute_window,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerCount` (0x33ae6662) function
        pub fn trigger_count(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([51, 174, 102, 98], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TriggerDeployed` event
        pub fn trigger_deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TriggerDeployedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TriggerDeployedFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for IUMATriggerFactory<M>
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
        name = "TriggerDeployed",
        abi = "TriggerDeployed(address,bytes32,address,string,address,uint256,address,uint256,uint256,string,string,string,string)"
    )]
    pub struct TriggerDeployedFilter {
        pub trigger: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trigger_config_id: [u8; 32],
        #[ethevent(indexed)]
        pub uma_oracle_finder: ::ethers::core::types::Address,
        pub query: ::std::string::String,
        #[ethevent(indexed)]
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub refund_recipient: ::ethers::core::types::Address,
        pub bond_amount: ::ethers::core::types::U256,
        pub proposal_dispute_window: ::ethers::core::types::U256,
        pub name: ::std::string::String,
        pub category: ::std::string::String,
        pub description: ::std::string::String,
        pub logo_uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `computeTriggerAddress` function with signature `computeTriggerAddress(string,address,uint256,address,uint256,uint256,uint256)` and selector `0x9d616791`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "computeTriggerAddress",
        abi = "computeTriggerAddress(string,address,uint256,address,uint256,uint256,uint256)"
    )]
    pub struct ComputeTriggerAddressCall {
        pub query: ::std::string::String,
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub refund_recipient: ::ethers::core::types::Address,
        pub bond_amount: ::ethers::core::types::U256,
        pub proposal_dispute_window: ::ethers::core::types::U256,
        pub trigger_count: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployTrigger` function with signature `deployTrigger(string,address,uint256,address,uint256,uint256,(string,string,string,string))` and selector `0x6922a512`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deployTrigger",
        abi = "deployTrigger(string,address,uint256,address,uint256,uint256,(string,string,string,string))"
    )]
    pub struct DeployTriggerCall {
        pub query: ::std::string::String,
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub refund_recipient: ::ethers::core::types::Address,
        pub bond_amount: ::ethers::core::types::U256,
        pub proposal_dispute_window: ::ethers::core::types::U256,
        pub metadata: TriggerMetadata,
    }
    ///Container type for all input parameters for the `findAvailableTrigger` function with signature `findAvailableTrigger(string,address,uint256,address,uint256,uint256)` and selector `0xe7ad052b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "findAvailableTrigger",
        abi = "findAvailableTrigger(string,address,uint256,address,uint256,uint256)"
    )]
    pub struct FindAvailableTriggerCall {
        pub query: ::std::string::String,
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub refund_recipient: ::ethers::core::types::Address,
        pub bond_amount: ::ethers::core::types::U256,
        pub proposal_dispute_window: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `oracleFinder` function with signature `oracleFinder()` and selector `0x9ceb3ea2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "oracleFinder", abi = "oracleFinder()")]
    pub struct OracleFinderCall;
    ///Container type for all input parameters for the `triggerConfigId` function with signature `triggerConfigId(string,address,uint256,address,uint256,uint256)` and selector `0xdf86c083`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "triggerConfigId",
        abi = "triggerConfigId(string,address,uint256,address,uint256,uint256)"
    )]
    pub struct TriggerConfigIdCall {
        pub query: ::std::string::String,
        pub reward_token: ::ethers::core::types::Address,
        pub reward_amount: ::ethers::core::types::U256,
        pub refund_recipient: ::ethers::core::types::Address,
        pub bond_amount: ::ethers::core::types::U256,
        pub proposal_dispute_window: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `triggerCount` function with signature `triggerCount(bytes32)` and selector `0x33ae6662`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "triggerCount", abi = "triggerCount(bytes32)")]
    pub struct TriggerCountCall(pub [u8; 32]);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUMATriggerFactoryCalls {
        ComputeTriggerAddress(ComputeTriggerAddressCall),
        DeployTrigger(DeployTriggerCall),
        FindAvailableTrigger(FindAvailableTriggerCall),
        Manager(ManagerCall),
        OracleFinder(OracleFinderCall),
        TriggerConfigId(TriggerConfigIdCall),
        TriggerCount(TriggerCountCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUMATriggerFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ComputeTriggerAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeTriggerAddress(decoded));
            }
            if let Ok(decoded) = <DeployTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeployTrigger(decoded));
            }
            if let Ok(decoded) =
                <FindAvailableTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FindAvailableTrigger(decoded));
            }
            if let Ok(decoded) = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded) = <OracleFinderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OracleFinder(decoded));
            }
            if let Ok(decoded) =
                <TriggerConfigIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TriggerConfigId(decoded));
            }
            if let Ok(decoded) = <TriggerCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TriggerCount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUMATriggerFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeTriggerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployTrigger(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindAvailableTrigger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleFinder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TriggerConfigId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TriggerCount(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IUMATriggerFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeTriggerAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::FindAvailableTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleFinder(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerConfigId(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerCount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeTriggerAddressCall> for IUMATriggerFactoryCalls {
        fn from(value: ComputeTriggerAddressCall) -> Self {
            Self::ComputeTriggerAddress(value)
        }
    }
    impl ::core::convert::From<DeployTriggerCall> for IUMATriggerFactoryCalls {
        fn from(value: DeployTriggerCall) -> Self {
            Self::DeployTrigger(value)
        }
    }
    impl ::core::convert::From<FindAvailableTriggerCall> for IUMATriggerFactoryCalls {
        fn from(value: FindAvailableTriggerCall) -> Self {
            Self::FindAvailableTrigger(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for IUMATriggerFactoryCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<OracleFinderCall> for IUMATriggerFactoryCalls {
        fn from(value: OracleFinderCall) -> Self {
            Self::OracleFinder(value)
        }
    }
    impl ::core::convert::From<TriggerConfigIdCall> for IUMATriggerFactoryCalls {
        fn from(value: TriggerConfigIdCall) -> Self {
            Self::TriggerConfigId(value)
        }
    }
    impl ::core::convert::From<TriggerCountCall> for IUMATriggerFactoryCalls {
        fn from(value: TriggerCountCall) -> Self {
            Self::TriggerCount(value)
        }
    }
    ///Container type for all return fields from the `computeTriggerAddress` function with signature `computeTriggerAddress(string,address,uint256,address,uint256,uint256,uint256)` and selector `0x9d616791`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ComputeTriggerAddressReturn {
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployTrigger` function with signature `deployTrigger(string,address,uint256,address,uint256,uint256,(string,string,string,string))` and selector `0x6922a512`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DeployTriggerReturn {
        pub trigger: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `findAvailableTrigger` function with signature `findAvailableTrigger(string,address,uint256,address,uint256,uint256)` and selector `0xe7ad052b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct FindAvailableTriggerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `oracleFinder` function with signature `oracleFinder()` and selector `0x9ceb3ea2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OracleFinderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `triggerConfigId` function with signature `triggerConfigId(string,address,uint256,address,uint256,uint256)` and selector `0xdf86c083`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TriggerConfigIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `triggerCount` function with signature `triggerCount(bytes32)` and selector `0x33ae6662`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TriggerCountReturn(pub ::ethers::core::types::U256);
}
