pub use i_cost_model::*;
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
pub mod i_cost_model {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("costFactor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactor"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("utilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundFactor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("refundFactor"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("utilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("registerSet"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("registerSet"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("update"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("utilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ICOSTMODEL_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    pub struct ICostModel<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICostModel<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICostModel<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICostModel<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICostModel<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ICostModel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ICostModel<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                ICOSTMODEL_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `costFactor` (0xd7c856b3) function
        pub fn cost_factor(
            &self,
            utilization: ::ethers::core::types::U256,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 200, 86, 179], (utilization, new_utilization))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundFactor` (0xe035cbca) function
        pub fn refund_factor(
            &self,
            utilization: ::ethers::core::types::U256,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([224, 53, 203, 202], (utilization, new_utilization))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSet` (0x3b1b6520) function
        pub fn register_set(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 27, 101, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x2fb565e8) function
        pub fn update(
            &self,
            utilization: ::ethers::core::types::U256,
            new_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 181, 101, 232], (utilization, new_utilization))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for ICostModel<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `costFactor` function with signature `costFactor(uint256,uint256)` and selector `0xd7c856b3`
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
    #[ethcall(name = "costFactor", abi = "costFactor(uint256,uint256)")]
    pub struct CostFactorCall {
        pub utilization: ::ethers::core::types::U256,
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `refundFactor` function with signature `refundFactor(uint256,uint256)` and selector `0xe035cbca`
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
    #[ethcall(name = "refundFactor", abi = "refundFactor(uint256,uint256)")]
    pub struct RefundFactorCall {
        pub utilization: ::ethers::core::types::U256,
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `registerSet` function with signature `registerSet()` and selector `0x3b1b6520`
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
    #[ethcall(name = "registerSet", abi = "registerSet()")]
    pub struct RegisterSetCall;
    ///Container type for all input parameters for the `update` function with signature `update(uint256,uint256)` and selector `0x2fb565e8`
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
    #[ethcall(name = "update", abi = "update(uint256,uint256)")]
    pub struct UpdateCall {
        pub utilization: ::ethers::core::types::U256,
        pub new_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ICostModelCalls {
        CostFactor(CostFactorCall),
        RefundFactor(RefundFactorCall),
        RegisterSet(RegisterSetCall),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ICostModelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CostFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CostFactor(decoded));
            }
            if let Ok(decoded) = <RefundFactorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RefundFactor(decoded));
            }
            if let Ok(decoded) = <RegisterSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterSet(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ICostModelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CostFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ICostModelCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CostFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CostFactorCall> for ICostModelCalls {
        fn from(value: CostFactorCall) -> Self {
            Self::CostFactor(value)
        }
    }
    impl ::core::convert::From<RefundFactorCall> for ICostModelCalls {
        fn from(value: RefundFactorCall) -> Self {
            Self::RefundFactor(value)
        }
    }
    impl ::core::convert::From<RegisterSetCall> for ICostModelCalls {
        fn from(value: RegisterSetCall) -> Self {
            Self::RegisterSet(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for ICostModelCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    ///Container type for all return fields from the `costFactor` function with signature `costFactor(uint256,uint256)` and selector `0xd7c856b3`
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
    pub struct CostFactorReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `refundFactor` function with signature `refundFactor(uint256,uint256)` and selector `0xe035cbca`
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
    pub struct RefundFactorReturn(pub ::ethers::core::types::U256);
}
