pub use i_trigger::*;
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
pub mod i_trigger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acknowledged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acknowledged"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addSet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("addSet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("set"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISet"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("state"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("state"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("enum MarketState"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("SetAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("SetAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("set"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TriggerStateUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TriggerStateUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("state"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ITRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    pub struct ITrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ITrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ITrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ITrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ITrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ITrigger)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ITrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    ITRIGGER_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `acknowledged` (0x086c298d) function
        pub fn acknowledged(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([8, 108, 41, 141], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addSet` (0xd580ded4) function
        pub fn add_set(
            &self,
            set: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([213, 128, 222, 212], set)
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
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TriggerStateUpdatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ITriggerEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for ITrigger<M> {
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
        Hash
    )]
    #[ethevent(name = "TriggerStateUpdated", abi = "TriggerStateUpdated(uint8)")]
    pub struct TriggerStateUpdatedFilter {
        #[ethevent(indexed)]
        pub state: u8,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ITriggerEvents {
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for ITriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(ITriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(ITriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ITriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SetAddedFilter> for ITriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for ITriggerEvents {
        fn from(value: TriggerStateUpdatedFilter) -> Self {
            Self::TriggerStateUpdatedFilter(value)
        }
    }
    ///Container type for all input parameters for the `acknowledged` function with signature `acknowledged()` and selector `0x086c298d`
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
        Hash
    )]
    #[ethcall(name = "addSet", abi = "addSet(address)")]
    pub struct AddSetCall {
        pub set: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `state` function with signature `state()` and selector `0xc19d93fb`
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
    #[ethcall(name = "state", abi = "state()")]
    pub struct StateCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ITriggerCalls {
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        State(StateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ITriggerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AcknowledgedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Acknowledged(decoded));
            }
            if let Ok(decoded)
                = <AddSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddSet(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ITriggerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Acknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ITriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for ITriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for ITriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<StateCall> for ITriggerCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    ///Container type for all return fields from the `acknowledged` function with signature `acknowledged()` and selector `0x086c298d`
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
    pub struct AcknowledgedReturn(pub bool);
    ///Container type for all return fields from the `addSet` function with signature `addSet(address)` and selector `0xd580ded4`
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
    pub struct AddSetReturn(pub bool);
    ///Container type for all return fields from the `state` function with signature `state()` and selector `0xc19d93fb`
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
    pub struct StateReturn(pub u8);
}
