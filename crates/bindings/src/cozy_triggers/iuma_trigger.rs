pub use iuma_trigger::*;
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
pub mod iuma_trigger {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetAdded\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"enum MarketState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TriggerStateUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_SET_LENGTH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"acknowledged\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"addSet\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bondAmount\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOracle\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSets\",\"outputs\":[{\"internalType\":\"address[]\",\"name\":\"\",\"type\":\"address[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSetsLength\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"oracleFinder\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"priceProposed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"_identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"_timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"_ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"_answer\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"priceSettled\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"proposalDisputeWindow\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"query\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"queryIdentifier\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requestTimestamp\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rewardToken\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"runProgrammaticCheck\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sets\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"shouldTrigger\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"state\",\"outputs\":[{\"internalType\":\"enum MarketState\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static IUMATRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct IUMATrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for IUMATrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IUMATrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IUMATrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IUMATrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IUMATrigger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IUMATrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                IUMATRIGGER_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `MAX_SET_LENGTH` (0x59537144) function
        pub fn max_set_length(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 83, 113, 68], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acknowledged` (0x086c298d) function
        pub fn acknowledged(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([8, 108, 41, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addSet` (0xd580ded4) function
        pub fn add_set(
            &self,
            set: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 128, 222, 212], set)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bondAmount` (0x80f323a7) function
        pub fn bond_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 243, 35, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOracle` (0x833b1fce) function
        pub fn get_oracle(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([131, 59, 31, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSets` (0x2cf7c531) function
        pub fn get_sets(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([44, 247, 197, 49], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSetsLength` (0xe86376c5) function
        pub fn get_sets_length(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([232, 99, 118, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracleFinder` (0x9ceb3ea2) function
        pub fn oracle_finder(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([156, 235, 62, 162], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceProposed` (0x9c2fd1df) function
        pub fn price_proposed(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 47, 209, 223], (identifier, timestamp, ancillary_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceSettled` (0x04cc1fd5) function
        pub fn price_settled(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            answer: ::ethers::core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [4, 204, 31, 213],
                    (identifier, timestamp, ancillary_data, answer),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposalDisputeWindow` (0xb365441b) function
        pub fn proposal_dispute_window(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([179, 101, 68, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `query` (0x2c46b205) function
        pub fn query(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([44, 70, 178, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryIdentifier` (0x51119862) function
        pub fn query_identifier(&self) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([81, 17, 152, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestTimestamp` (0x3e66a647) function
        pub fn request_timestamp(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([62, 102, 166, 71], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rewardToken` (0xf7c618c1) function
        pub fn reward_token(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `runProgrammaticCheck` (0x37a0afc1) function
        pub fn run_programmatic_check(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([55, 160, 175, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sets` (0x5b227f9b) function
        pub fn sets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([91, 34, 127, 155], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `shouldTrigger` (0x4f4ab8b0) function
        pub fn should_trigger(&self) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([79, 74, 184, 176], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0xc19d93fb) function
        pub fn state(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 157, 147, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `SetAdded` event
        pub fn set_added_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SetAddedFilter> {
            self.0.event()
        }
        ///Gets the contract's `TriggerStateUpdated` event
        pub fn trigger_state_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TriggerStateUpdatedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, IUMATriggerEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for IUMATrigger<M> {
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
    #[ethevent(name = "SetAdded", abi = "SetAdded(address)")]
    pub struct SetAddedFilter {
        pub set: ::ethers::core::types::Address,
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
    #[ethevent(name = "TriggerStateUpdated", abi = "TriggerStateUpdated(uint8)")]
    pub struct TriggerStateUpdatedFilter {
        #[ethevent(indexed)]
        pub state: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUMATriggerEvents {
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for IUMATriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(IUMATriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(IUMATriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IUMATriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<SetAddedFilter> for IUMATriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for IUMATriggerEvents {
        fn from(value: TriggerStateUpdatedFilter) -> Self {
            Self::TriggerStateUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_SET_LENGTH` function with signature `MAX_SET_LENGTH()` and selector `0x59537144`
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
    #[ethcall(name = "MAX_SET_LENGTH", abi = "MAX_SET_LENGTH()")]
    pub struct MaxSetLengthCall;
    ///Container type for all input parameters for the `acknowledged` function with signature `acknowledged()` and selector `0x086c298d`
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
    #[ethcall(name = "acknowledged", abi = "acknowledged()")]
    pub struct AcknowledgedCall;
    ///Container type for all input parameters for the `addSet` function with signature `addSet(address)` and selector `0xd580ded4`
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
    #[ethcall(name = "addSet", abi = "addSet(address)")]
    pub struct AddSetCall {
        pub set: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `bondAmount` function with signature `bondAmount()` and selector `0x80f323a7`
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
    #[ethcall(name = "bondAmount", abi = "bondAmount()")]
    pub struct BondAmountCall;
    ///Container type for all input parameters for the `getOracle` function with signature `getOracle()` and selector `0x833b1fce`
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
    #[ethcall(name = "getOracle", abi = "getOracle()")]
    pub struct GetOracleCall;
    ///Container type for all input parameters for the `getSets` function with signature `getSets()` and selector `0x2cf7c531`
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
    #[ethcall(name = "getSets", abi = "getSets()")]
    pub struct GetSetsCall;
    ///Container type for all input parameters for the `getSetsLength` function with signature `getSetsLength()` and selector `0xe86376c5`
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
    #[ethcall(name = "getSetsLength", abi = "getSetsLength()")]
    pub struct GetSetsLengthCall;
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `oracleFinder` function with signature `oracleFinder()` and selector `0x9ceb3ea2`
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
    #[ethcall(name = "oracleFinder", abi = "oracleFinder()")]
    pub struct OracleFinderCall;
    ///Container type for all input parameters for the `priceProposed` function with signature `priceProposed(bytes32,uint256,bytes)` and selector `0x9c2fd1df`
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
    #[ethcall(name = "priceProposed", abi = "priceProposed(bytes32,uint256,bytes)")]
    pub struct PriceProposedCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `priceSettled` function with signature `priceSettled(bytes32,uint256,bytes,int256)` and selector `0x04cc1fd5`
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
    #[ethcall(
        name = "priceSettled",
        abi = "priceSettled(bytes32,uint256,bytes,int256)"
    )]
    pub struct PriceSettledCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub answer: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `proposalDisputeWindow` function with signature `proposalDisputeWindow()` and selector `0xb365441b`
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
    #[ethcall(name = "proposalDisputeWindow", abi = "proposalDisputeWindow()")]
    pub struct ProposalDisputeWindowCall;
    ///Container type for all input parameters for the `query` function with signature `query()` and selector `0x2c46b205`
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
    #[ethcall(name = "query", abi = "query()")]
    pub struct QueryCall;
    ///Container type for all input parameters for the `queryIdentifier` function with signature `queryIdentifier()` and selector `0x51119862`
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
    #[ethcall(name = "queryIdentifier", abi = "queryIdentifier()")]
    pub struct QueryIdentifierCall;
    ///Container type for all input parameters for the `requestTimestamp` function with signature `requestTimestamp()` and selector `0x3e66a647`
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
    #[ethcall(name = "requestTimestamp", abi = "requestTimestamp()")]
    pub struct RequestTimestampCall;
    ///Container type for all input parameters for the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
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
    #[ethcall(name = "rewardToken", abi = "rewardToken()")]
    pub struct RewardTokenCall;
    ///Container type for all input parameters for the `runProgrammaticCheck` function with signature `runProgrammaticCheck()` and selector `0x37a0afc1`
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
    #[ethcall(name = "runProgrammaticCheck", abi = "runProgrammaticCheck()")]
    pub struct RunProgrammaticCheckCall;
    ///Container type for all input parameters for the `sets` function with signature `sets(uint256)` and selector `0x5b227f9b`
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
    #[ethcall(name = "sets", abi = "sets(uint256)")]
    pub struct SetsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `shouldTrigger` function with signature `shouldTrigger()` and selector `0x4f4ab8b0`
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
    #[ethcall(name = "shouldTrigger", abi = "shouldTrigger()")]
    pub struct ShouldTriggerCall;
    ///Container type for all input parameters for the `state` function with signature `state()` and selector `0xc19d93fb`
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
    #[ethcall(name = "state", abi = "state()")]
    pub struct StateCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IUMATriggerCalls {
        MaxSetLength(MaxSetLengthCall),
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        BondAmount(BondAmountCall),
        GetOracle(GetOracleCall),
        GetSets(GetSetsCall),
        GetSetsLength(GetSetsLengthCall),
        Manager(ManagerCall),
        OracleFinder(OracleFinderCall),
        PriceProposed(PriceProposedCall),
        PriceSettled(PriceSettledCall),
        ProposalDisputeWindow(ProposalDisputeWindowCall),
        Query(QueryCall),
        QueryIdentifier(QueryIdentifierCall),
        RequestTimestamp(RequestTimestampCall),
        RewardToken(RewardTokenCall),
        RunProgrammaticCheck(RunProgrammaticCheckCall),
        Sets(SetsCall),
        ShouldTrigger(ShouldTriggerCall),
        State(StateCall),
    }
    impl ::ethers::core::abi::AbiDecode for IUMATriggerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MaxSetLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxSetLength(decoded));
            }
            if let Ok(decoded) = <AcknowledgedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Acknowledged(decoded));
            }
            if let Ok(decoded) = <AddSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddSet(decoded));
            }
            if let Ok(decoded) = <BondAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BondAmount(decoded));
            }
            if let Ok(decoded) = <GetOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOracle(decoded));
            }
            if let Ok(decoded) = <GetSetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSets(decoded));
            }
            if let Ok(decoded) = <GetSetsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSetsLength(decoded));
            }
            if let Ok(decoded) = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded) = <OracleFinderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OracleFinder(decoded));
            }
            if let Ok(decoded) = <PriceProposedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriceProposed(decoded));
            }
            if let Ok(decoded) = <PriceSettledCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriceSettled(decoded));
            }
            if let Ok(decoded) =
                <ProposalDisputeWindowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProposalDisputeWindow(decoded));
            }
            if let Ok(decoded) = <QueryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Query(decoded));
            }
            if let Ok(decoded) =
                <QueryIdentifierCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QueryIdentifier(decoded));
            }
            if let Ok(decoded) =
                <RequestTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestTimestamp(decoded));
            }
            if let Ok(decoded) = <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardToken(decoded));
            }
            if let Ok(decoded) =
                <RunProgrammaticCheckCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RunProgrammaticCheck(decoded));
            }
            if let Ok(decoded) = <SetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sets(decoded));
            }
            if let Ok(decoded) = <ShouldTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ShouldTrigger(decoded));
            }
            if let Ok(decoded) = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IUMATriggerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxSetLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Acknowledged(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AddSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BondAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetOracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSetsLength(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleFinder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PriceProposed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PriceSettled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProposalDisputeWindow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Query(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryIdentifier(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestTimestamp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RewardToken(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RunProgrammaticCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ShouldTrigger(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for IUMATriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxSetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BondAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSetsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleFinder(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceProposed(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceSettled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDisputeWindow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Query(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryIdentifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RunProgrammaticCheck(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ShouldTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxSetLengthCall> for IUMATriggerCalls {
        fn from(value: MaxSetLengthCall) -> Self {
            Self::MaxSetLength(value)
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for IUMATriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for IUMATriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<BondAmountCall> for IUMATriggerCalls {
        fn from(value: BondAmountCall) -> Self {
            Self::BondAmount(value)
        }
    }
    impl ::core::convert::From<GetOracleCall> for IUMATriggerCalls {
        fn from(value: GetOracleCall) -> Self {
            Self::GetOracle(value)
        }
    }
    impl ::core::convert::From<GetSetsCall> for IUMATriggerCalls {
        fn from(value: GetSetsCall) -> Self {
            Self::GetSets(value)
        }
    }
    impl ::core::convert::From<GetSetsLengthCall> for IUMATriggerCalls {
        fn from(value: GetSetsLengthCall) -> Self {
            Self::GetSetsLength(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for IUMATriggerCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<OracleFinderCall> for IUMATriggerCalls {
        fn from(value: OracleFinderCall) -> Self {
            Self::OracleFinder(value)
        }
    }
    impl ::core::convert::From<PriceProposedCall> for IUMATriggerCalls {
        fn from(value: PriceProposedCall) -> Self {
            Self::PriceProposed(value)
        }
    }
    impl ::core::convert::From<PriceSettledCall> for IUMATriggerCalls {
        fn from(value: PriceSettledCall) -> Self {
            Self::PriceSettled(value)
        }
    }
    impl ::core::convert::From<ProposalDisputeWindowCall> for IUMATriggerCalls {
        fn from(value: ProposalDisputeWindowCall) -> Self {
            Self::ProposalDisputeWindow(value)
        }
    }
    impl ::core::convert::From<QueryCall> for IUMATriggerCalls {
        fn from(value: QueryCall) -> Self {
            Self::Query(value)
        }
    }
    impl ::core::convert::From<QueryIdentifierCall> for IUMATriggerCalls {
        fn from(value: QueryIdentifierCall) -> Self {
            Self::QueryIdentifier(value)
        }
    }
    impl ::core::convert::From<RequestTimestampCall> for IUMATriggerCalls {
        fn from(value: RequestTimestampCall) -> Self {
            Self::RequestTimestamp(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for IUMATriggerCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<RunProgrammaticCheckCall> for IUMATriggerCalls {
        fn from(value: RunProgrammaticCheckCall) -> Self {
            Self::RunProgrammaticCheck(value)
        }
    }
    impl ::core::convert::From<SetsCall> for IUMATriggerCalls {
        fn from(value: SetsCall) -> Self {
            Self::Sets(value)
        }
    }
    impl ::core::convert::From<ShouldTriggerCall> for IUMATriggerCalls {
        fn from(value: ShouldTriggerCall) -> Self {
            Self::ShouldTrigger(value)
        }
    }
    impl ::core::convert::From<StateCall> for IUMATriggerCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    ///Container type for all return fields from the `MAX_SET_LENGTH` function with signature `MAX_SET_LENGTH()` and selector `0x59537144`
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
    pub struct MaxSetLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `acknowledged` function with signature `acknowledged()` and selector `0x086c298d`
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
    pub struct AcknowledgedReturn(pub bool);
    ///Container type for all return fields from the `bondAmount` function with signature `bondAmount()` and selector `0x80f323a7`
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
    pub struct BondAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getOracle` function with signature `getOracle()` and selector `0x833b1fce`
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
    pub struct GetOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getSets` function with signature `getSets()` and selector `0x2cf7c531`
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
    pub struct GetSetsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `getSetsLength` function with signature `getSetsLength()` and selector `0xe86376c5`
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
    pub struct GetSetsLengthReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `oracleFinder` function with signature `oracleFinder()` and selector `0x9ceb3ea2`
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
    pub struct OracleFinderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proposalDisputeWindow` function with signature `proposalDisputeWindow()` and selector `0xb365441b`
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
    pub struct ProposalDisputeWindowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `query` function with signature `query()` and selector `0x2c46b205`
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
    pub struct QueryReturn(pub ::std::string::String);
    ///Container type for all return fields from the `queryIdentifier` function with signature `queryIdentifier()` and selector `0x51119862`
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
    pub struct QueryIdentifierReturn(pub [u8; 32]);
    ///Container type for all return fields from the `requestTimestamp` function with signature `requestTimestamp()` and selector `0x3e66a647`
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
    pub struct RequestTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `rewardToken` function with signature `rewardToken()` and selector `0xf7c618c1`
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
    pub struct RewardTokenReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `runProgrammaticCheck` function with signature `runProgrammaticCheck()` and selector `0x37a0afc1`
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
    pub struct RunProgrammaticCheckReturn(pub u8);
    ///Container type for all return fields from the `sets` function with signature `sets(uint256)` and selector `0x5b227f9b`
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
    pub struct SetsReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `shouldTrigger` function with signature `shouldTrigger()` and selector `0x4f4ab8b0`
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
    pub struct ShouldTriggerReturn(pub bool);
    ///Container type for all return fields from the `state` function with signature `state()` and selector `0xc19d93fb`
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
    pub struct StateReturn(pub u8);
}
