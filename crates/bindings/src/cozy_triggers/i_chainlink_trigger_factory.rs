pub use i_chainlink_trigger_factory::*;
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
pub mod i_chainlink_trigger_factory {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"triggerConfigId\",\"type\":\"bytes32\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"truthOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"trackingOracle\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"priceTolerance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"truthFrequencyTolerance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"trackingFrequencyTolerance\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"category\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[],\"indexed\":false},{\"internalType\":\"string\",\"name\":\"logoURI\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TriggerDeployed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_price\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeFixedPriceAggregatorAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_truthOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_trackingOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_priceTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_truthFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_trackingFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_triggerCount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"computeTriggerAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"_address\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_price\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployFixedPriceAggregator\",\"outputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_truthOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_trackingOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_priceTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_truthFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_trackingFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct TriggerMetadata\",\"name\":\"_metadata\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"category\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"logoURI\",\"type\":\"string\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployTrigger\",\"outputs\":[{\"internalType\":\"contract IChainlinkTrigger\",\"name\":\"_trigger\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"_price\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"_decimals\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_trackingOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_priceTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_frequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct TriggerMetadata\",\"name\":\"_metadata\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"string\",\"name\":\"name\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"category\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"description\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"logoURI\",\"type\":\"string\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deployTrigger\",\"outputs\":[{\"internalType\":\"contract IChainlinkTrigger\",\"name\":\"_trigger\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_truthOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_trackingOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_priceTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_truthFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_trackingFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"findAvailableTrigger\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"contract IManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_truthOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract AggregatorV3Interface\",\"name\":\"_trackingOracle\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_priceTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_truthFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_trackingFrequencyTolerance\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerConfigId\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerCount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICHAINLINKTRIGGERFACTORY_ABI: ::ethers_contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct IChainlinkTriggerFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IChainlinkTriggerFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IChainlinkTriggerFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IChainlinkTriggerFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IChainlinkTriggerFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IChainlinkTriggerFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IChainlinkTriggerFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ICHAINLINKTRIGGERFACTORY_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `computeFixedPriceAggregatorAddress` (0xdba733f1) function
        pub fn compute_fixed_price_aggregator_address(
            &self,
            price: ::ethers::core::types::I256,
            decimals: u8,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([219, 167, 51, 241], (price, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeTriggerAddress` (0x83fd82f3) function
        pub fn compute_trigger_address(
            &self,
            truth_oracle: ::ethers::core::types::Address,
            tracking_oracle: ::ethers::core::types::Address,
            price_tolerance: ::ethers::core::types::U256,
            truth_frequency_tolerance: ::ethers::core::types::U256,
            tracking_frequency_tolerance: ::ethers::core::types::U256,
            trigger_count: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [131, 253, 130, 243],
                    (
                        truth_oracle,
                        tracking_oracle,
                        price_tolerance,
                        truth_frequency_tolerance,
                        tracking_frequency_tolerance,
                        trigger_count,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployFixedPriceAggregator` (0xa33520f2) function
        pub fn deploy_fixed_price_aggregator(
            &self,
            price: ::ethers::core::types::I256,
            decimals: u8,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([163, 53, 32, 242], (price, decimals))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployTrigger` (0x3c4ed745) function
        pub fn deploy_trigger(
            &self,
            truth_oracle: ::ethers::core::types::Address,
            tracking_oracle: ::ethers::core::types::Address,
            price_tolerance: ::ethers::core::types::U256,
            truth_frequency_tolerance: ::ethers::core::types::U256,
            tracking_frequency_tolerance: ::ethers::core::types::U256,
            metadata: TriggerMetadata,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [60, 78, 215, 69],
                    (
                        truth_oracle,
                        tracking_oracle,
                        price_tolerance,
                        truth_frequency_tolerance,
                        tracking_frequency_tolerance,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployTrigger` (0x5d81c77c) function
        pub fn deploy_trigger_with_price_and_decimals_and_tracking_oracle_and_price_tolerance_and_frequency_tolerance(
            &self,
            price: ::ethers::core::types::I256,
            decimals: u8,
            tracking_oracle: ::ethers::core::types::Address,
            price_tolerance: ::ethers::core::types::U256,
            frequency_tolerance: ::ethers::core::types::U256,
            metadata: TriggerMetadata,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [93, 129, 199, 124],
                    (
                        price,
                        decimals,
                        tracking_oracle,
                        price_tolerance,
                        frequency_tolerance,
                        metadata,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `findAvailableTrigger` (0x5d2f86bc) function
        pub fn find_available_trigger(
            &self,
            truth_oracle: ::ethers::core::types::Address,
            tracking_oracle: ::ethers::core::types::Address,
            price_tolerance: ::ethers::core::types::U256,
            truth_frequency_tolerance: ::ethers::core::types::U256,
            tracking_frequency_tolerance: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash(
                    [93, 47, 134, 188],
                    (
                        truth_oracle,
                        tracking_oracle,
                        price_tolerance,
                        truth_frequency_tolerance,
                        tracking_frequency_tolerance,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerConfigId` (0xe9ed41f5) function
        pub fn trigger_config_id(
            &self,
            truth_oracle: ::ethers::core::types::Address,
            tracking_oracle: ::ethers::core::types::Address,
            price_tolerance: ::ethers::core::types::U256,
            truth_frequency_tolerance: ::ethers::core::types::U256,
            tracking_frequency_tolerance: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [233, 237, 65, 245],
                    (
                        truth_oracle,
                        tracking_oracle,
                        price_tolerance,
                        truth_frequency_tolerance,
                        tracking_frequency_tolerance,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerCount` (0x33ae6662) function
        pub fn trigger_count(
            &self,
            p0: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([51, 174, 102, 98], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `TriggerDeployed` event
        pub fn trigger_deployed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TriggerDeployedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TriggerDeployedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for IChainlinkTriggerFactory<M> {
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
        name = "TriggerDeployed",
        abi = "TriggerDeployed(address,bytes32,address,address,uint256,uint256,uint256,string,string,string,string)"
    )]
    pub struct TriggerDeployedFilter {
        pub trigger: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub trigger_config_id: [u8; 32],
        #[ethevent(indexed)]
        pub truth_oracle: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub truth_frequency_tolerance: ::ethers::core::types::U256,
        pub tracking_frequency_tolerance: ::ethers::core::types::U256,
        pub name: ::std::string::String,
        pub category: ::std::string::String,
        pub description: ::std::string::String,
        pub logo_uri: ::std::string::String,
    }
    ///Container type for all input parameters for the `computeFixedPriceAggregatorAddress` function with signature `computeFixedPriceAggregatorAddress(int256,uint8)` and selector `0xdba733f1`
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
    #[ethcall(
        name = "computeFixedPriceAggregatorAddress",
        abi = "computeFixedPriceAggregatorAddress(int256,uint8)"
    )]
    pub struct ComputeFixedPriceAggregatorAddressCall {
        pub price: ::ethers::core::types::I256,
        pub decimals: u8,
    }
    ///Container type for all input parameters for the `computeTriggerAddress` function with signature `computeTriggerAddress(address,address,uint256,uint256,uint256,uint256)` and selector `0x83fd82f3`
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
    #[ethcall(
        name = "computeTriggerAddress",
        abi = "computeTriggerAddress(address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputeTriggerAddressCall {
        pub truth_oracle: ::ethers::core::types::Address,
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub truth_frequency_tolerance: ::ethers::core::types::U256,
        pub tracking_frequency_tolerance: ::ethers::core::types::U256,
        pub trigger_count: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployFixedPriceAggregator` function with signature `deployFixedPriceAggregator(int256,uint8)` and selector `0xa33520f2`
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
    #[ethcall(
        name = "deployFixedPriceAggregator",
        abi = "deployFixedPriceAggregator(int256,uint8)"
    )]
    pub struct DeployFixedPriceAggregatorCall {
        pub price: ::ethers::core::types::I256,
        pub decimals: u8,
    }
    ///Container type for all input parameters for the `deployTrigger` function with signature `deployTrigger(address,address,uint256,uint256,uint256,(string,string,string,string))` and selector `0x3c4ed745`
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
    #[ethcall(
        name = "deployTrigger",
        abi = "deployTrigger(address,address,uint256,uint256,uint256,(string,string,string,string))"
    )]
    pub struct DeployTriggerCall {
        pub truth_oracle: ::ethers::core::types::Address,
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub truth_frequency_tolerance: ::ethers::core::types::U256,
        pub tracking_frequency_tolerance: ::ethers::core::types::U256,
        pub metadata: TriggerMetadata,
    }
    ///Container type for all input parameters for the `deployTrigger` function with signature `deployTrigger(int256,uint8,address,uint256,uint256,(string,string,string,string))` and selector `0x5d81c77c`
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
    #[ethcall(
        name = "deployTrigger",
        abi = "deployTrigger(int256,uint8,address,uint256,uint256,(string,string,string,string))"
    )]
    pub struct DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceCall {
        pub price: ::ethers::core::types::I256,
        pub decimals: u8,
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub frequency_tolerance: ::ethers::core::types::U256,
        pub metadata: TriggerMetadata,
    }
    ///Container type for all input parameters for the `findAvailableTrigger` function with signature `findAvailableTrigger(address,address,uint256,uint256,uint256)` and selector `0x5d2f86bc`
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
    #[ethcall(
        name = "findAvailableTrigger",
        abi = "findAvailableTrigger(address,address,uint256,uint256,uint256)"
    )]
    pub struct FindAvailableTriggerCall {
        pub truth_oracle: ::ethers::core::types::Address,
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub truth_frequency_tolerance: ::ethers::core::types::U256,
        pub tracking_frequency_tolerance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `triggerConfigId` function with signature `triggerConfigId(address,address,uint256,uint256,uint256)` and selector `0xe9ed41f5`
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
    #[ethcall(
        name = "triggerConfigId",
        abi = "triggerConfigId(address,address,uint256,uint256,uint256)"
    )]
    pub struct TriggerConfigIdCall {
        pub truth_oracle: ::ethers::core::types::Address,
        pub tracking_oracle: ::ethers::core::types::Address,
        pub price_tolerance: ::ethers::core::types::U256,
        pub truth_frequency_tolerance: ::ethers::core::types::U256,
        pub tracking_frequency_tolerance: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `triggerCount` function with signature `triggerCount(bytes32)` and selector `0x33ae6662`
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
    #[ethcall(name = "triggerCount", abi = "triggerCount(bytes32)")]
    pub struct TriggerCountCall(pub [u8; 32]);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IChainlinkTriggerFactoryCalls {
        ComputeFixedPriceAggregatorAddress(ComputeFixedPriceAggregatorAddressCall),
        ComputeTriggerAddress(ComputeTriggerAddressCall),
        DeployFixedPriceAggregator(DeployFixedPriceAggregatorCall),
        DeployTrigger(DeployTriggerCall),
        DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyTolerance(
            DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceCall,
        ),
        FindAvailableTrigger(FindAvailableTriggerCall),
        Manager(ManagerCall),
        TriggerConfigId(TriggerConfigIdCall),
        TriggerCount(TriggerCountCall),
    }
    impl ::ethers::core::abi::AbiDecode for IChainlinkTriggerFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ComputeFixedPriceAggregatorAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ComputeFixedPriceAggregatorAddress(decoded));
            }
            if let Ok(decoded)
                = <ComputeTriggerAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ComputeTriggerAddress(decoded));
            }
            if let Ok(decoded)
                = <DeployFixedPriceAggregatorCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::DeployFixedPriceAggregator(decoded));
            }
            if let Ok(decoded)
                = <DeployTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeployTrigger(decoded));
            }
            if let Ok(decoded)
                = <DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyTolerance(
                        decoded,
                    ),
                );
            }
            if let Ok(decoded)
                = <FindAvailableTriggerCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FindAvailableTrigger(decoded));
            }
            if let Ok(decoded)
                = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded)
                = <TriggerConfigIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TriggerConfigId(decoded));
            }
            if let Ok(decoded)
                = <TriggerCountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TriggerCount(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IChainlinkTriggerFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeFixedPriceAggregatorAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeTriggerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployFixedPriceAggregator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployTrigger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyTolerance(
                    element,
                ) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FindAvailableTrigger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TriggerConfigId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TriggerCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IChainlinkTriggerFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeFixedPriceAggregatorAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeTriggerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployFixedPriceAggregator(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeployTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyTolerance(
                    element,
                ) => ::core::fmt::Display::fmt(element, f),
                Self::FindAvailableTrigger(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerConfigId(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerCount(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeFixedPriceAggregatorAddressCall>
    for IChainlinkTriggerFactoryCalls {
        fn from(value: ComputeFixedPriceAggregatorAddressCall) -> Self {
            Self::ComputeFixedPriceAggregatorAddress(value)
        }
    }
    impl ::core::convert::From<ComputeTriggerAddressCall>
    for IChainlinkTriggerFactoryCalls {
        fn from(value: ComputeTriggerAddressCall) -> Self {
            Self::ComputeTriggerAddress(value)
        }
    }
    impl ::core::convert::From<DeployFixedPriceAggregatorCall>
    for IChainlinkTriggerFactoryCalls {
        fn from(value: DeployFixedPriceAggregatorCall) -> Self {
            Self::DeployFixedPriceAggregator(value)
        }
    }
    impl ::core::convert::From<DeployTriggerCall> for IChainlinkTriggerFactoryCalls {
        fn from(value: DeployTriggerCall) -> Self {
            Self::DeployTrigger(value)
        }
    }
    impl ::core::convert::From<
        DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceCall,
    > for IChainlinkTriggerFactoryCalls {
        fn from(
            value: DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceCall,
        ) -> Self {
            Self::DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyTolerance(
                value,
            )
        }
    }
    impl ::core::convert::From<FindAvailableTriggerCall>
    for IChainlinkTriggerFactoryCalls {
        fn from(value: FindAvailableTriggerCall) -> Self {
            Self::FindAvailableTrigger(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for IChainlinkTriggerFactoryCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<TriggerConfigIdCall> for IChainlinkTriggerFactoryCalls {
        fn from(value: TriggerConfigIdCall) -> Self {
            Self::TriggerConfigId(value)
        }
    }
    impl ::core::convert::From<TriggerCountCall> for IChainlinkTriggerFactoryCalls {
        fn from(value: TriggerCountCall) -> Self {
            Self::TriggerCount(value)
        }
    }
    ///Container type for all return fields from the `computeFixedPriceAggregatorAddress` function with signature `computeFixedPriceAggregatorAddress(int256,uint8)` and selector `0xdba733f1`
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
    pub struct ComputeFixedPriceAggregatorAddressReturn(
        pub ::ethers::core::types::Address,
    );
    ///Container type for all return fields from the `computeTriggerAddress` function with signature `computeTriggerAddress(address,address,uint256,uint256,uint256,uint256)` and selector `0x83fd82f3`
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
    pub struct ComputeTriggerAddressReturn {
        pub address: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployFixedPriceAggregator` function with signature `deployFixedPriceAggregator(int256,uint8)` and selector `0xa33520f2`
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
    pub struct DeployFixedPriceAggregatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `deployTrigger` function with signature `deployTrigger(address,address,uint256,uint256,uint256,(string,string,string,string))` and selector `0x3c4ed745`
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
    pub struct DeployTriggerReturn {
        pub trigger: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `deployTrigger` function with signature `deployTrigger(int256,uint8,address,uint256,uint256,(string,string,string,string))` and selector `0x5d81c77c`
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
    pub struct DeployTriggerWithPriceAndDecimalsAndTrackingOracleAndPriceToleranceAndFrequencyToleranceReturn {
        pub trigger: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `findAvailableTrigger` function with signature `findAvailableTrigger(address,address,uint256,uint256,uint256)` and selector `0x5d2f86bc`
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
    pub struct FindAvailableTriggerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `triggerConfigId` function with signature `triggerConfigId(address,address,uint256,uint256,uint256)` and selector `0xe9ed41f5`
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
    pub struct TriggerConfigIdReturn(pub [u8; 32]);
    ///Container type for all return fields from the `triggerCount` function with signature `triggerCount(bytes32)` and selector `0x33ae6662`
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
    pub struct TriggerCountReturn(pub ::ethers::core::types::U256);
}
