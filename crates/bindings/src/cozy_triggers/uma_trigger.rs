pub use uma_trigger::*;
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
pub mod uma_trigger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_manager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_oracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract OptimisticOracleV2Interface",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_query"),
                        kind: ::ethers::core::abi::ethabi::ParamType::String,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("string"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_rewardToken"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IERC20"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_refundRecipient"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bondAmount"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_proposalDisputeWindow"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_SET_LENGTH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("MAX_SET_LENGTH"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                                    name: ::std::borrow::ToOwned::to_owned("_set"),
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
                    ::std::borrow::ToOwned::to_owned("bondAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("bondAmount"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSets"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISet[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSetsLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSetsLength"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("manager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("manager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("oracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract OptimisticOracleV2Interface",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceDisputed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceDisputed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_identifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ancillaryData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceProposed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceProposed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_identifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ancillaryData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("priceSettled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceSettled"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_identifier"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_ancillaryData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_answer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposalDisputeWindow"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "proposalDisputeWindow",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("query"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("query"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("queryIdentifier"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("queryIdentifier"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundRecipient"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("refundRecipient"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("requestTimestamp"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rewardToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rewardToken"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IERC20"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("runProgrammaticCheck"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "runProgrammaticCheck",
                            ),
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
                (
                    ::std::borrow::ToOwned::to_owned("sets"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sets"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract ISet"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ProposalDisputed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ProposalDisputed"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("QueryResubmitted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("QueryResubmitted"),
                            inputs: ::std::vec![],
                            anonymous: false,
                        },
                    ],
                ),
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
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidProposal"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidProposal"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidStateTransition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidStateTransition",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetLimitReached"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SetLimitReached"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unacknowledged"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unacknowledged"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unsettleable"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unsettleable"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static UMATRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01 `@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0;.8\x03\x80b\0;.\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x08yV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\x80R\x86\x16`\xA0R`\x02b\0\0\xA3\x86\x82b\0\t\xFCV[P`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\xC0R`\x04\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\xE0\x82\x90Ra\x01\0\x81\x90Rb\0\0\xDEb\0\0\xEBV[PPPPPPPb\0\x0CwV[`\xC0Q`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x01oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x01\x84W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\xAA\x91\x90b\0\n\xC8V[`\xC0Q`\xA0Q`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x92\x93P\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x02<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x02QW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02w\x91\x90b\0\n\xE7V[PB`\x03\x81\x90U`\xA0Q`\xC0Q`@Qc\x11\xDF\x92\xF1`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\x11\xDF\x92\xF1\x92b\0\x02\xCC\x92nYES_OR_NO_QUERY`\x88\x1B\x92\x91`\x02\x91\x90\x88\x90`\x04\x01b\0\x0B\x9CV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x03&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x03;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x03a\x91\x90b\0\n\xC8V[P`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\x12\x06\x98\xAFnYES_OR_NO_QUERY`\x88\x1B`\x03T`\x02`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x03\xAB\x93\x92\x91\x90b\0\x0B\xDAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x04\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x04\x1AW=`\0\x80>=`\0\xFD[PP`\xA0Q`\x03T`\xE0Q`@QcV\xAD:\xAD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x94Pc\xADZuZ\x93Pb\0\x04l\x92nYES_OR_NO_QUERY`\x88\x1B\x92\x91`\x02\x91`\x04\x01b\0\x0C\x04V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x04\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x04\xDBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x05\x01\x91\x90b\0\n\xC8V[P`\xA0Q`\x03Ta\x01\0Q`@Qc#\x9E\"\xFF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92cG<E\xFE\x92b\0\x05Q\x92nYES_OR_NO_QUERY`\x88\x1B\x92`\x02\x91\x90`\x04\x01b\0\x0C\x04V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x05\xABW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x05\xC0W=`\0\x80>=`\0\xFD[PPPP`\xA0Q`\x01`\x01`\xA0\x1B\x03\x16c\xF3'\xB0unYES_OR_NO_QUERY`\x88\x1B`\x03T`\x02`\x01\x80`\x01`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01b\0\x06\x15\x96\x95\x94\x93\x92\x91\x90b\0\x0C6V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0;\x0E\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x06\x84W=`\0\x80>=`\0\xFD[PPPPPV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x06\xF1W`\0\x80\xFD[PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15b\0\x07'W\x81\x81\x01Q\x83\x82\x01R` \x01b\0\x07\rV[PP`\0\x91\x01RV[`\0\x82`\x1F\x83\x01\x12b\0\x07\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x07\xB3Wb\0\x07\xB3b\0\x06\xF4V[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15b\0\x07\xDEWb\0\x07\xDEb\0\x06\xF4V[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15b\0\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R\x92P`\x84\x83\xFD[b\0\x08]\x84` \x83\x01` \x89\x01b\0\x07\nV[\x96\x95PPPPPPV[\x80Qb\0\x08t\x81b\0\x06\xDBV[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\x08\x9AWb\0\x08\x9Ab\0\x06\x8BV[\x87Qb\0\x08\xA7\x81b\0\x06\xDBV[` \x89\x01Q\x90\x97Pb\0\x08\xBA\x81b\0\x06\xDBV[`@\x89\x01Q\x90\x96P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\t\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\t0\x8A\x82\x8B\x01b\0\x070V[\x95PPb\0\tA``\x89\x01b\0\x08gV[\x93Pb\0\tQ`\x80\x89\x01b\0\x08gV[\x92P`\xA0\x88\x01Q\x91P`\xC0\x88\x01Q\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\x01\x81\x81\x1C\x90\x82\x16\x80b\0\t\x82W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\t\xA3WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\x1F\x82\x11\x15b\0\t\xF7W`\0\x81\x81R` \x81 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15b\0\t\xD2WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15b\0\t\xF3W\x82\x81U`\x01\x01b\0\t\xDEV[PPP[PPPV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\n\x18Wb\0\n\x18b\0\x06\xF4V[b\0\n0\x81b\0\n)\x84Tb\0\tmV[\x84b\0\t\xA9V[` \x80`\x1F\x83\x11`\x01\x81\x14b\0\nhW`\0\x84\x15b\0\nOWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ub\0\t\xF3V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15b\0\n\x99W\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01b\0\nxV[P\x85\x82\x10\x15b\0\n\xB8W\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0` \x82\x84\x03\x12\x15b\0\n\xE0Wb\0\n\xE0b\0\x06\x8BV[PQ\x91\x90PV[`\0` \x82\x84\x03\x12\x15b\0\n\xFFWb\0\n\xFFb\0\x06\x8BV[\x81Q\x80\x15\x15\x81\x14b\0\x0B\x10W`\0\x80\xFD[\x93\x92PPPV[`\0\x81Tb\0\x0B&\x81b\0\tmV[\x80\x85R` `\x01\x83\x81\x16\x80\x15b\0\x0BFW`\x01\x81\x14b\0\x0BaWb\0\x0B\x91V[`\xFF\x19\x85\x16\x88\x84\x01R\x83\x15\x15`\x05\x1B\x88\x01\x83\x01\x95Pb\0\x0B\x91V[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15b\0\x0B\x89W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01b\0\x0BlV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0b\0\x0B\xBD`\xA0\x83\x01\x86b\0\x0B\x17V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0b\0\x0B\xFB``\x83\x01\x84b\0\x0B\x17V[\x95\x94PPPPPV[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0b\0\x0C%`\x80\x83\x01\x85b\0\x0B\x17V[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0b\0\x0CW`\xC0\x83\x01\x87b\0\x0B\x17V[\x94\x15\x15``\x83\x01RP\x91\x15\x15`\x80\x83\x01R\x15\x15`\xA0\x90\x91\x01R\x93\x92PPPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa-\xBFb\0\rO`\09`\0\x81\x81a\x04\x1F\x01Ra\x1F)\x01R`\0\x81\x81a\x03\xE5\x01Ra\x1D\x9A\x01R`\0\x81\x81a\x04n\x01R\x81\x81a\x05\xA1\x01R\x81\x81a\x06\xD8\x01R\x81\x81a\x17\xA7\x01R\x81\x81a\x19'\x01Ra\x1A\xBD\x01R`\0\x81\x81a\x03\xBE\x01R\x81\x81a\x04\xA8\x01R\x81\x81a\x07m\x01R\x81\x81a\t\xBF\x01R\x81\x81a\x0Bg\x01R\x81\x81a\x0C\xF4\x01R\x81\x81a\x0E\x94\x01R\x81\x81a\x10\x16\x01R\x81\x81a\x18\xF5\x01R\x81\x81a\x1Ak\x01R\x81\x81a\x1B\xFC\x01R\x81\x81a\x1DF\x01R\x81\x81a\x1E\xD7\x01Ra C\x01R`\0\x81\x81a\x03U\x01Ra\x12\x15\x01Ra-\xBF`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xEEW`\x005`\xE0\x1C\x80cYSqD\x11a\x01OW\x80c\xB3eD\x1B\x11a\x01\x03W\x80c\xD5\x80\xDE\xD4\x11a\0\xE8W\x80c\xD5\x80\xDE\xD4\x14a\x04NW\x80c\xE8cv\xC5\x14a\x04aW\x80c\xF7\xC6\x18\xC1\x14a\x04iWa\x01\xEEV[\x80c\xB3eD\x1B\x14a\x04\x1AW\x80c\xC1\x9D\x93\xFB\x14a\x04AWa\x01\xEEV[\x80c}\xC0\xD1\xD0\x11a\x014W\x80c}\xC0\xD1\xD0\x14a\x03\xB9W\x80c\x80\xF3#\xA7\x14a\x03\xE0W\x80c\x9C/\xD1\xDF\x14a\x04\x07Wa\x01\xEEV[\x80cYSqD\x14a\x03\x9EW\x80c[\"\x7F\x9B\x14a\x03\xA6Wa\x01\xEEV[\x80c,\xF7\xC51\x11a\x01\xA6W\x80c>f\xA6G\x11a\x01\x8BW\x80c>f\xA6G\x14a\x039W\x80cH\x1Cju\x14a\x03PW\x80cQ\x11\x98b\x14a\x03wWa\x01\xEEV[\x80c,\xF7\xC51\x14a\x03\x0FW\x80c7\xA0\xAF\xC1\x14a\x03$Wa\x01\xEEV[\x80c\x08l)\x8D\x11a\x01\xD7W\x80c\x08l)\x8D\x14a\x02\xD4W\x80c\r\x8F#r\x14a\x02\xE7W\x80c,F\xB2\x05\x14a\x02\xFAWa\x01\xEEV[\x80c\x01\xF5\xADZ\x14a\x02uW\x80c\x04\xCC\x1F\xD5\x14a\x02\xBFW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x04Ta\x02\x95\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD2a\x02\xCD6`\x04a&\x0EV[a\x04\x90V[\0[`\x01[`@Q\x90\x15\x15\x81R` \x01a\x02\xB6V[a\x02\xD2a\x02\xF56`\x04a&\x0EV[a\x07UV[a\x03\x02a\x08UV[`@Qa\x02\xB6\x91\x90a&\xD0V[a\x03\x17a\x08\xE3V[`@Qa\x02\xB6\x91\x90a&\xEAV[a\x03,a\tRV[`@Qa\x02\xB6\x91\x90a'sV[a\x03B`\x03T\x81V[`@Q\x90\x81R` \x01a\x02\xB6V[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B`2\x81V[a\x02\x95a\x03\xB46`\x04a'\xB4V[a\x0EEV[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xD2a\x04\x156`\x04a'\xD0V[a\x0E|V[a\x03B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Ta\x03,\x90`\xFF\x16\x81V[a\x02\xD7a\x04\\6`\x04a(KV[a\x11|V[`\x01Ta\x03BV[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x04\xD7WP`\x03T\x83\x14\x15[\x80a\x04\xFFWP`\x02`@Qa\x04\xEC\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x82\x80Q\x90` \x01 \x14\x15[\x80a\x05*WP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14\x15[\x15a\x05aW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x07\x12W`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a)RV[\x90P\x80\x15a\x07\x01W`\x04Ta\x07\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91\x16\x83a\x14\xA8V[a\x07\x0B`\x02a\x15eV[PPa\x07OV[a\x07\x1C`\0a\x15eV[Pa\x07%a\x17vV[`@Q\x7F\x10\xC3<\xAC\xC4r\xB3\xAF\x94\xB3\x15\x85\xDB$\xF8\xB5Q\xA6%\0\x98\xB5z\xD0\x95\x8AI8\x0CW\x90.\x90`\0\x90\xA1[PPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x07\x9CWP`\x03T\x83\x14\x15[\x80a\x07\xC4WP`\x02`@Qa\x07\xB1\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x82\x80Q\x90` \x01 \x14\x15[\x80a\x07\xEFWP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14\x15[\x15a\x08&W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xAC\x97%\xFF8\xA9\x8A\xFEg\x17\xE8/S\x10\xBDEC\xFD\x0C\x0F\xD0\x104\x13\\\xB3~B\xAAg\xFE-\x90`\0\x90\xA1PPPPV[`\x02\x80Ta\x08b\x90a(kV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x8E\x90a(kV[\x80\x15a\x08\xDBW\x80`\x1F\x10a\x08\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xDBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xBEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tHW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x1DW[PPPPP\x90P\x90V[`\0`\x02`\0T`\xFF\x16`\x02\x81\x11\x15a\tmWa\tma'DV[\x03a\t|WP`\0T`\xFF\x16\x90V[`\x03T`@Q\x7F\xBCX\xCC\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xBCX\xCC\xAA\x91a\n\x19\x910\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a*\tV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEB\x91\x90a*cV[\x90P\x80a\x0B$W`@Q\x7FF\x10G\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`@Q\x7F\xA9\x90O\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xA9\x90O\x9B\x91a\x0B\xC1\x910\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a*\tV[a\x02\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x94\x91\x90a+\x91V[\x90P\x80``\x01Qa\x0E8W`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x81U`\x03T`@Q\x7F^\x9Ay\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92c^\x9Ay\xA9\x92a\rb\x920\x92\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91`\x02\x91\x01a*\tV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E6\x91\x90a)RV[P[PP`\0T`\xFF\x16\x91\x90PV[`\x01\x81\x81T\x81\x10a\x0EUW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x0E\xC3WP`\x03T\x82\x14\x15[\x80a\x0E\xEBWP`\x02`@Qa\x0E\xD8\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x15[\x80a\x0F\x16WP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x15[\x15a\x0FMW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xD9`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R\x84Q`\xE0\x81\x01\x86R\x83\x81R\x91\x82\x01\x83\x90R\x93\x81\x01\x82\x90R\x92\x83\x01\x81\x90R`\x80\x83\x81\x01\x82\x90R`\xA0\x84\x01\x82\x90R`\xC0\x84\x01\x91\x90\x91R\x90\x91\x90\x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x7F\xA9\x90O\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x90O\x9B\x90a\x10Q\x900\x90\x88\x90\x88\x90\x88\x90`\x04\x01a,?V[a\x02\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11$\x91\x90a+\x91V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\xA0\x01Q\x14a\x11kW`@Q\x7F\xEE\x03(\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11u`\x01a\x15eV[PPPPPV[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x11\xCDW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x13\x91\x90a*cV[\x90P\x80a\x13LW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x13\x89W`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x13\xF8W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x13\xBEWa\x13\xBEa,zV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x13\xF0WP`\x01\x94\x93PPPPV[`\x01\x01a\x13\x8CV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x15v\x90`\xFF\x16\x83a!TV[a\x15\xACW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x15\xE9Wa\x15\xE9a'DV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x173W`\x01\x81\x81T\x81\x10a\x16\x0FWa\x16\x0Fa,zV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x16n\x90\x87\x90`\x04\x01a'sV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x1EW=`\0\x80>=`\0\xFD[PPPPa\x17,\x81`\x01\x01\x90V[\x90Pa\x15\xF4V[P\x82`\x02\x81\x11\x15a\x17FWa\x17Fa'DV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x80W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xB8\x91\x90a)RV[`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xEFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A'\x91\x90a*cV[PB`\x03\x81\x90U`@Q\x7F\x11\xDF\x92\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x11\xDF\x92\xF1\x91a\x1A\xE7\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x88\x90`\x04\x01a,\xA9V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x83W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xBB\x91\x90a)RV[P`\x03T`@Q\x7F\x12\x06\x98\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x12\x06\x98\xAF\x91a\x1CT\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a,\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x04W=`\0\x80>=`\0\xFD[PP`\x03T`@Q\x7F\xADZuZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\xADZuZ\x92Pa\x1D\xC2\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a-\x1AV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1ErW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x96\x91\x90a)RV[P`\x03T`@Q\x7FG<E\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cG<E\xFE\x91a\x1FQ\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a-\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \x01W=`\0\x80>=`\0\xFD[PP`\x03T`@Q\x7F\xF3'\xB0u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\xF3'\xB0u\x92Pa \xA4\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x01\x90\x81\x90\x81\x90`\x04\x01a-JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11uW=`\0\x80>=`\0\xFD[`\0`\x02\x83`\x02\x81\x11\x15a!jWa!ja'DV[\x03a!wWP`\0a\"\xA8V[\x81`\x02\x81\x11\x15a!\x89Wa!\x89a'DV[\x83`\x02\x81\x11\x15a!\x9BWa!\x9Ba'DV[\x03a!\xA8WP`\x01a\"\xA8V[`\0\x83`\x02\x81\x11\x15a!\xBCWa!\xBCa'DV[\x14\x80\x15a!\xDAWP`\x01\x82`\x02\x81\x11\x15a!\xD8Wa!\xD8a'DV[\x14[\x15a!\xE7WP`\x01a\"\xA8V[`\x01\x83`\x02\x81\x11\x15a!\xFBWa!\xFBa'DV[\x14\x80\x15a\"\x19WP`\0\x82`\x02\x81\x11\x15a\"\x17Wa\"\x17a'DV[\x14[\x15a\"&WP`\x01a\"\xA8V[`\0\x83`\x02\x81\x11\x15a\":Wa\":a'DV[\x14\x80\x15a\"XWP`\x02\x82`\x02\x81\x11\x15a\"VWa\"Va'DV[\x14[\x15a\"eWP`\x01a\"\xA8V[`\x01\x83`\x02\x81\x11\x15a\"yWa\"ya'DV[\x14\x80\x15a\"\x97WP`\x02\x82`\x02\x81\x11\x15a\"\x95Wa\"\x95a'DV[\x14[\x15a\"\xA4WP`\x01a\"\xA8V[P`\0[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\nWa$\na#\xB8V[`@R\x90V[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\nWa$\na#\xB8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a${Wa${a#\xB8V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a%\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%0Wa%0a#\xB8V[a%`\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a$4V[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a%\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&'Wa&'a\"\xAEV[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&OWa&Oa#3V[a&[\x87\x82\x88\x01a$\x83V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a&\x92W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&vV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a&\xE3` \x83\x01\x84a&lV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'8W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a'\x06V[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a'\xAEW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a'\xC9Wa'\xC9a\"\xAEV[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xE8Wa'\xE8a\"\xAEV[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x10Wa(\x10a#3V[a(\x1C\x86\x82\x87\x01a$\x83V[\x91PP\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(HW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(`Wa(`a\"\xAEV[\x815a&\xE3\x81a(&V[`\x01\x81\x81\x1C\x90\x82\x16\x80a(\x7FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a(\xB8W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta(\xCC\x81a(kV[`\x01\x82\x81\x16\x80\x15a(\xE4W`\x01\x81\x14a)\x17Wa)FV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa)FV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a)=W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a)$V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a)gWa)ga\"\xAEV[PQ\x91\x90PV[`\0\x81Ta){\x81a(kV[\x80\x85R` `\x01\x83\x81\x16\x80\x15a)\x98W`\x01\x81\x14a)\xD0Wa)\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x89\x01R\x82\x84\x15\x15`\x05\x1B\x89\x01\x01\x95Pa)\xFEV[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a)\xF6W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a)\xDBV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a*D`\x80\x83\x01\x84a)nV[\x96\x95PPPPPPV[\x80Q\x80\x15\x15\x81\x14a*^W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a*xWa*xa\"\xAEV[a&\xE3\x82a*NV[\x80Qa*^\x81a(&V[`\0`\xE0\x82\x84\x03\x12\x15a+\x1EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a+&a#\xE7V[\x90Pa+1\x82a*NV[\x81Ra+?` \x83\x01a*NV[` \x82\x01Ra+P`@\x83\x01a*NV[`@\x82\x01Ra+a``\x83\x01a*NV[``\x82\x01Ra+r`\x80\x83\x01a*NV[`\x80\x82\x01R`\xA0\x82\x01Q`\xA0\x82\x01R`\xC0\x82\x01Q`\xC0\x82\x01R\x92\x91PPV[`\0a\x02\0\x82\x84\x03\x12\x15a+\xA7Wa+\xA7a\"\xAEV[a+\xAFa$\x10V[a+\xB8\x83a*\x81V[\x81Ra+\xC6` \x84\x01a*\x81V[` \x82\x01Ra+\xD7`@\x84\x01a*\x81V[`@\x82\x01Ra+\xE8``\x84\x01a*NV[``\x82\x01Ra+\xFA\x84`\x80\x85\x01a*\x8CV[`\x80\x82\x01Ra\x01`\x83\x01Q`\xA0\x82\x01Ra\x01\x80\x83\x01Q`\xC0\x82\x01Ra\x01\xA0\x83\x01Q`\xE0\x82\x01Ra\x01\xC0\x83\x01Qa\x01\0\x82\x01Ra\x01\xE0\x90\x92\x01Qa\x01 \x83\x01RP\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a*D`\x80\x83\x01\x84a&lV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a,\xC8`\xA0\x83\x01\x86a)nV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a-\x11``\x83\x01\x84a)nV[\x95\x94PPPPPV[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a-9`\x80\x83\x01\x85a)nV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a-i`\xC0\x83\x01\x87a)nV[\x94\x15\x15``\x83\x01RP\x91\x15\x15`\x80\x83\x01R\x15\x15`\xA0\x90\x91\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 2\xF4\xF9\xF1\x92\xE3\x1BC\xE5\xC7Dg\xD9t\xA6\xE8\xFAl\xAE\xDA\xFEf\xE21\xC9Bkn=j\xC08dsolcC\0\x08\x10\x003Target contract does not contain";
    /// The bytecode of the contract.
    pub static UMATRIGGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xEEW`\x005`\xE0\x1C\x80cYSqD\x11a\x01OW\x80c\xB3eD\x1B\x11a\x01\x03W\x80c\xD5\x80\xDE\xD4\x11a\0\xE8W\x80c\xD5\x80\xDE\xD4\x14a\x04NW\x80c\xE8cv\xC5\x14a\x04aW\x80c\xF7\xC6\x18\xC1\x14a\x04iWa\x01\xEEV[\x80c\xB3eD\x1B\x14a\x04\x1AW\x80c\xC1\x9D\x93\xFB\x14a\x04AWa\x01\xEEV[\x80c}\xC0\xD1\xD0\x11a\x014W\x80c}\xC0\xD1\xD0\x14a\x03\xB9W\x80c\x80\xF3#\xA7\x14a\x03\xE0W\x80c\x9C/\xD1\xDF\x14a\x04\x07Wa\x01\xEEV[\x80cYSqD\x14a\x03\x9EW\x80c[\"\x7F\x9B\x14a\x03\xA6Wa\x01\xEEV[\x80c,\xF7\xC51\x11a\x01\xA6W\x80c>f\xA6G\x11a\x01\x8BW\x80c>f\xA6G\x14a\x039W\x80cH\x1Cju\x14a\x03PW\x80cQ\x11\x98b\x14a\x03wWa\x01\xEEV[\x80c,\xF7\xC51\x14a\x03\x0FW\x80c7\xA0\xAF\xC1\x14a\x03$Wa\x01\xEEV[\x80c\x08l)\x8D\x11a\x01\xD7W\x80c\x08l)\x8D\x14a\x02\xD4W\x80c\r\x8F#r\x14a\x02\xE7W\x80c,F\xB2\x05\x14a\x02\xFAWa\x01\xEEV[\x80c\x01\xF5\xADZ\x14a\x02uW\x80c\x04\xCC\x1F\xD5\x14a\x02\xBFW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x04Ta\x02\x95\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\xD2a\x02\xCD6`\x04a&\x0EV[a\x04\x90V[\0[`\x01[`@Q\x90\x15\x15\x81R` \x01a\x02\xB6V[a\x02\xD2a\x02\xF56`\x04a&\x0EV[a\x07UV[a\x03\x02a\x08UV[`@Qa\x02\xB6\x91\x90a&\xD0V[a\x03\x17a\x08\xE3V[`@Qa\x02\xB6\x91\x90a&\xEAV[a\x03,a\tRV[`@Qa\x02\xB6\x91\x90a'sV[a\x03B`\x03T\x81V[`@Q\x90\x81R` \x01a\x02\xB6V[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B`2\x81V[a\x02\x95a\x03\xB46`\x04a'\xB4V[a\x0EEV[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xD2a\x04\x156`\x04a'\xD0V[a\x0E|V[a\x03B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Ta\x03,\x90`\xFF\x16\x81V[a\x02\xD7a\x04\\6`\x04a(KV[a\x11|V[`\x01Ta\x03BV[a\x02\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x04\xD7WP`\x03T\x83\x14\x15[\x80a\x04\xFFWP`\x02`@Qa\x04\xEC\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x82\x80Q\x90` \x01 \x14\x15[\x80a\x05*WP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14\x15[\x15a\x05aW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x03a\x07\x12W`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06zW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a)RV[\x90P\x80\x15a\x07\x01W`\x04Ta\x07\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91\x16\x83a\x14\xA8V[a\x07\x0B`\x02a\x15eV[PPa\x07OV[a\x07\x1C`\0a\x15eV[Pa\x07%a\x17vV[`@Q\x7F\x10\xC3<\xAC\xC4r\xB3\xAF\x94\xB3\x15\x85\xDB$\xF8\xB5Q\xA6%\0\x98\xB5z\xD0\x95\x8AI8\x0CW\x90.\x90`\0\x90\xA1[PPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x07\x9CWP`\x03T\x83\x14\x15[\x80a\x07\xC4WP`\x02`@Qa\x07\xB1\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x82\x80Q\x90` \x01 \x14\x15[\x80a\x07\xEFWP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x14\x15[\x15a\x08&W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7F\xAC\x97%\xFF8\xA9\x8A\xFEg\x17\xE8/S\x10\xBDEC\xFD\x0C\x0F\xD0\x104\x13\\\xB3~B\xAAg\xFE-\x90`\0\x90\xA1PPPPV[`\x02\x80Ta\x08b\x90a(kV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\x8E\x90a(kV[\x80\x15a\x08\xDBW\x80`\x1F\x10a\x08\xB0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\xDBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xBEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tHW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\x1DW[PPPPP\x90P\x90V[`\0`\x02`\0T`\xFF\x16`\x02\x81\x11\x15a\tmWa\tma'DV[\x03a\t|WP`\0T`\xFF\x16\x90V[`\x03T`@Q\x7F\xBCX\xCC\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xBCX\xCC\xAA\x91a\n\x19\x910\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a*\tV[` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\n\xB3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\n\xC7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xEB\x91\x90a*cV[\x90P\x80a\x0B$W`@Q\x7FF\x10G\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03T`@Q\x7F\xA9\x90O\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\xA9\x90O\x9B\x91a\x0B\xC1\x910\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a*\tV[a\x02\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\\W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x0CpW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x94\x91\x90a+\x91V[\x90P\x80``\x01Qa\x0E8W`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x81U`\x03T`@Q\x7F^\x9Ay\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92c^\x9Ay\xA9\x92a\rb\x920\x92\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92\x91`\x02\x91\x01a*\tV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0E\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E6\x91\x90a)RV[P[PP`\0T`\xFF\x16\x91\x90PV[`\x01\x81\x81T\x81\x10a\x0EUW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15\x80a\x0E\xC3WP`\x03T\x82\x14\x15[\x80a\x0E\xEBWP`\x02`@Qa\x0E\xD8\x91\x90a(\xBEV[`@Q\x80\x91\x03\x90 \x81\x80Q\x90` \x01 \x14\x15[\x80a\x0F\x16WP\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x14\x15[\x15a\x0FMW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xD9`@\x80Qa\x01@\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R\x84Q`\xE0\x81\x01\x86R\x83\x81R\x91\x82\x01\x83\x90R\x93\x81\x01\x82\x90R\x92\x83\x01\x81\x90R`\x80\x83\x81\x01\x82\x90R`\xA0\x84\x01\x82\x90R`\xC0\x84\x01\x91\x90\x91R\x90\x91\x90\x82\x01\x90\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`@Q\x7F\xA9\x90O\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x90O\x9B\x90a\x10Q\x900\x90\x88\x90\x88\x90\x88\x90`\x04\x01a,?V[a\x02\0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x10\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x11\0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11$\x91\x90a+\x91V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81`\xA0\x01Q\x14a\x11kW`@Q\x7F\xEE\x03(\x08\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x11u`\x01a\x15eV[PPPPPV[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x11\xCDW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xDBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x12\xEFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x13\x91\x90a*cV[\x90P\x80a\x13LW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x13\x89W`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x13\xF8W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x13\xBEWa\x13\xBEa,zV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x13\xF0WP`\x01\x94\x93PPPPV[`\x01\x01a\x13\x8CV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x83`\x04\x82\x01R\x82`$\x82\x01R` `\0`D\x83`\0\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91PP\x80a\x07OW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0F`$\x82\x01R\x7FTRANSFER_FAILED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x15v\x90`\xFF\x16\x83a!TV[a\x15\xACW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x15\xE9Wa\x15\xE9a'DV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x173W`\x01\x81\x81T\x81\x10a\x16\x0FWa\x16\x0Fa,zV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x16n\x90\x87\x90`\x04\x01a'sV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x17\nW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x17\x1EW=`\0\x80>=`\0\xFD[PPPPa\x17,\x81`\x01\x01\x90V[\x90Pa\x15\xF4V[P\x82`\x02\x81\x11\x15a\x17FWa\x17Fa'DV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\0\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x18\x80W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x18\x94W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xB8\x91\x90a)RV[`@Q\x7F\t^\xA7\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^\xA7\xB3\x90`D\x01` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xEFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1A\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A'\x91\x90a*cV[PB`\x03\x81\x90U`@Q\x7F\x11\xDF\x92\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x11\xDF\x92\xF1\x91a\x1A\xE7\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x88\x90`\x04\x01a,\xA9V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\x83W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1B\x97W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xBB\x91\x90a)RV[P`\x03T`@Q\x7F\x12\x06\x98\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91c\x12\x06\x98\xAF\x91a\x1CT\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x04\x01a,\xF2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1C\xF0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1D\x04W=`\0\x80>=`\0\xFD[PP`\x03T`@Q\x7F\xADZuZ\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\xADZuZ\x92Pa\x1D\xC2\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a-\x1AV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1E^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x1ErW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x96\x91\x90a)RV[P`\x03T`@Q\x7FG<E\xFE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91cG<E\xFE\x91a\x1FQ\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x04\x01a-\x1AV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1F\xEDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a \x01W=`\0\x80>=`\0\xFD[PP`\x03T`@Q\x7F\xF3'\xB0u\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93Pc\xF3'\xB0u\x92Pa \xA4\x91\x7FYES_OR_NO_QUERY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91`\x02\x90`\x01\x90\x81\x90\x81\x90`\x04\x01a-JV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a!@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x11uW=`\0\x80>=`\0\xFD[`\0`\x02\x83`\x02\x81\x11\x15a!jWa!ja'DV[\x03a!wWP`\0a\"\xA8V[\x81`\x02\x81\x11\x15a!\x89Wa!\x89a'DV[\x83`\x02\x81\x11\x15a!\x9BWa!\x9Ba'DV[\x03a!\xA8WP`\x01a\"\xA8V[`\0\x83`\x02\x81\x11\x15a!\xBCWa!\xBCa'DV[\x14\x80\x15a!\xDAWP`\x01\x82`\x02\x81\x11\x15a!\xD8Wa!\xD8a'DV[\x14[\x15a!\xE7WP`\x01a\"\xA8V[`\x01\x83`\x02\x81\x11\x15a!\xFBWa!\xFBa'DV[\x14\x80\x15a\"\x19WP`\0\x82`\x02\x81\x11\x15a\"\x17Wa\"\x17a'DV[\x14[\x15a\"&WP`\x01a\"\xA8V[`\0\x83`\x02\x81\x11\x15a\":Wa\":a'DV[\x14\x80\x15a\"XWP`\x02\x82`\x02\x81\x11\x15a\"VWa\"Va'DV[\x14[\x15a\"eWP`\x01a\"\xA8V[`\x01\x83`\x02\x81\x11\x15a\"yWa\"ya'DV[\x14\x80\x15a\"\x97WP`\x02\x82`\x02\x81\x11\x15a\"\x95Wa\"\x95a'DV[\x14[\x15a\"\xA4WP`\x01a\"\xA8V[P`\0[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01R\x7Fet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\nWa$\na#\xB8V[`@R\x90V[`@Qa\x01@\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a$\nWa$\na#\xB8V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a${Wa${a#\xB8V[`@R\x91\x90PV[`\0\x82`\x1F\x83\x01\x12a%\x14W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01R\x7Frray offset\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x815` g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%0Wa%0a#\xB8V[a%`\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01a$4V[\x82\x81R\x85\x82\x84\x87\x01\x01\x11\x15a%\xF3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x82`\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01R\x7F length\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[\x82\x82\x86\x01\x83\x83\x017`\0\x92\x81\x01\x90\x91\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a&'Wa&'a\"\xAEV[\x845\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&OWa&Oa#3V[a&[\x87\x82\x88\x01a$\x83V[\x94\x97\x93\x96P\x93\x94``\x015\x93PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a&\x92W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a&vV[P`\0` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a&\xE3` \x83\x01\x84a&lV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a'8W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a'\x06V[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a'\xAEW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a'\xC9Wa'\xC9a\"\xAEV[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a'\xE8Wa'\xE8a\"\xAEV[\x835\x92P` \x84\x015\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a(\x10Wa(\x10a#3V[a(\x1C\x86\x82\x87\x01a$\x83V[\x91PP\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a(HW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a(`Wa(`a\"\xAEV[\x815a&\xE3\x81a(&V[`\x01\x81\x81\x1C\x90\x82\x16\x80a(\x7FW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a(\xB8W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80\x83Ta(\xCC\x81a(kV[`\x01\x82\x81\x16\x80\x15a(\xE4W`\x01\x81\x14a)\x17Wa)FV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa)FV[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a)=W\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a)$V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a)gWa)ga\"\xAEV[PQ\x91\x90PV[`\0\x81Ta){\x81a(kV[\x80\x85R` `\x01\x83\x81\x16\x80\x15a)\x98W`\x01\x81\x14a)\xD0Wa)\xFEV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x85\x16\x83\x89\x01R\x82\x84\x15\x15`\x05\x1B\x89\x01\x01\x95Pa)\xFEV[\x86`\0R\x82`\0 `\0[\x85\x81\x10\x15a)\xF6W\x81T\x8A\x82\x01\x86\x01R\x90\x83\x01\x90\x84\x01a)\xDBV[\x89\x01\x84\x01\x96PP[PPPPP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a*D`\x80\x83\x01\x84a)nV[\x96\x95PPPPPPV[\x80Q\x80\x15\x15\x81\x14a*^W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a*xWa*xa\"\xAEV[a&\xE3\x82a*NV[\x80Qa*^\x81a(&V[`\0`\xE0\x82\x84\x03\x12\x15a+\x1EW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01R\x7Fort\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[a+&a#\xE7V[\x90Pa+1\x82a*NV[\x81Ra+?` \x83\x01a*NV[` \x82\x01Ra+P`@\x83\x01a*NV[`@\x82\x01Ra+a``\x83\x01a*NV[``\x82\x01Ra+r`\x80\x83\x01a*NV[`\x80\x82\x01R`\xA0\x82\x01Q`\xA0\x82\x01R`\xC0\x82\x01Q`\xC0\x82\x01R\x92\x91PPV[`\0a\x02\0\x82\x84\x03\x12\x15a+\xA7Wa+\xA7a\"\xAEV[a+\xAFa$\x10V[a+\xB8\x83a*\x81V[\x81Ra+\xC6` \x84\x01a*\x81V[` \x82\x01Ra+\xD7`@\x84\x01a*\x81V[`@\x82\x01Ra+\xE8``\x84\x01a*NV[``\x82\x01Ra+\xFA\x84`\x80\x85\x01a*\x8CV[`\x80\x82\x01Ra\x01`\x83\x01Q`\xA0\x82\x01Ra\x01\x80\x83\x01Q`\xC0\x82\x01Ra\x01\xA0\x83\x01Q`\xE0\x82\x01Ra\x01\xC0\x83\x01Qa\x01\0\x82\x01Ra\x01\xE0\x90\x92\x01Qa\x01 \x83\x01RP\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a*D`\x80\x83\x01\x84a&lV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x85\x81R\x84` \x82\x01R`\xA0`@\x82\x01R`\0a,\xC8`\xA0\x83\x01\x86a)nV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x90\x94\x16``\x83\x01RP`\x80\x01R\x93\x92PPPV[\x83\x81R\x82` \x82\x01R```@\x82\x01R`\0a-\x11``\x83\x01\x84a)nV[\x95\x94PPPPPV[\x84\x81R\x83` \x82\x01R`\x80`@\x82\x01R`\0a-9`\x80\x83\x01\x85a)nV[\x90P\x82``\x83\x01R\x95\x94PPPPPV[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0a-i`\xC0\x83\x01\x87a)nV[\x94\x15\x15``\x83\x01RP\x91\x15\x15`\x80\x83\x01R\x15\x15`\xA0\x90\x91\x01R\x93\x92PPPV\xFE\xA2dipfsX\"\x12 2\xF4\xF9\xF1\x92\xE3\x1BC\xE5\xC7Dg\xD9t\xA6\xE8\xFAl\xAE\xDA\xFEf\xE21\xC9Bkn=j\xC08dsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static UMATRIGGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UMATrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for UMATrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UMATrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UMATrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UMATrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(UMATrigger)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UMATrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    UMATRIGGER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers_contract::builders::ContractDeployer<M, Self>,
            ::ethers_contract::ContractError<M>,
        > {
            let factory = ::ethers_contract::ContractFactory::new(
                UMATRIGGER_ABI.clone(),
                UMATRIGGER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `bondAmount` (0x80f323a7) function
        pub fn bond_amount(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 243, 35, 167], ())
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
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `oracle` (0x7dc0d1d0) function
        pub fn oracle(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([125, 192, 209, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceDisputed` (0x0d8f2372) function
        pub fn price_disputed(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [13, 143, 35, 114],
                    (identifier, timestamp, ancillary_data, p3),
                )
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
                .method_hash(
                    [156, 47, 209, 223],
                    (identifier, timestamp, ancillary_data),
                )
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
        pub fn query(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([44, 70, 178, 5], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `queryIdentifier` (0x51119862) function
        pub fn query_identifier(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([81, 17, 152, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundRecipient` (0x01f5ad5a) function
        pub fn refund_recipient(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([1, 245, 173, 90], ())
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
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([247, 198, 24, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `runProgrammaticCheck` (0x37a0afc1) function
        pub fn run_programmatic_check(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([55, 160, 175, 193], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sets` (0x5b227f9b) function
        pub fn sets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([91, 34, 127, 155], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `state` (0xc19d93fb) function
        pub fn state(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([193, 157, 147, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ProposalDisputed` event
        pub fn proposal_disputed_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposalDisputedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `QueryResubmitted` event
        pub fn query_resubmitted_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            QueryResubmittedFilter,
        > {
            self.0.event()
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
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UMATriggerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for UMATrigger<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidProposal` with signature `InvalidProposal()` and selector `0xee032808`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidProposal", abi = "InvalidProposal()")]
    pub struct InvalidProposal;
    ///Custom Error type `InvalidStateTransition` with signature `InvalidStateTransition()` and selector `0x8f9a780c`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidStateTransition", abi = "InvalidStateTransition()")]
    pub struct InvalidStateTransition;
    ///Custom Error type `SetLimitReached` with signature `SetLimitReached()` and selector `0x86bfb2c8`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "SetLimitReached", abi = "SetLimitReached()")]
    pub struct SetLimitReached;
    ///Custom Error type `Unacknowledged` with signature `Unacknowledged()` and selector `0x48c068cd`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Unacknowledged", abi = "Unacknowledged()")]
    pub struct Unacknowledged;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Custom Error type `Unsettleable` with signature `Unsettleable()` and selector `0x461047af`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "Unsettleable", abi = "Unsettleable()")]
    pub struct Unsettleable;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum UMATriggerErrors {
        InvalidProposal(InvalidProposal),
        InvalidStateTransition(InvalidStateTransition),
        SetLimitReached(SetLimitReached),
        Unacknowledged(Unacknowledged),
        Unauthorized(Unauthorized),
        Unsettleable(Unsettleable),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for UMATriggerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <InvalidProposal as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProposal(decoded));
            }
            if let Ok(decoded)
                = <InvalidStateTransition as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidStateTransition(decoded));
            }
            if let Ok(decoded)
                = <SetLimitReached as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetLimitReached(decoded));
            }
            if let Ok(decoded)
                = <Unacknowledged as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unacknowledged(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded)
                = <Unsettleable as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unsettleable(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UMATriggerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidProposal(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidStateTransition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLimitReached(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unacknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unsettleable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for UMATriggerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidProposal as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidStateTransition as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SetLimitReached as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unacknowledged as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <Unsettleable as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for UMATriggerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidProposal(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStateTransition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLimitReached(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unacknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unsettleable(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for UMATriggerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidProposal> for UMATriggerErrors {
        fn from(value: InvalidProposal) -> Self {
            Self::InvalidProposal(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for UMATriggerErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<SetLimitReached> for UMATriggerErrors {
        fn from(value: SetLimitReached) -> Self {
            Self::SetLimitReached(value)
        }
    }
    impl ::core::convert::From<Unacknowledged> for UMATriggerErrors {
        fn from(value: Unacknowledged) -> Self {
            Self::Unacknowledged(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for UMATriggerErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<Unsettleable> for UMATriggerErrors {
        fn from(value: Unsettleable) -> Self {
            Self::Unsettleable(value)
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
    #[ethevent(name = "ProposalDisputed", abi = "ProposalDisputed()")]
    pub struct ProposalDisputedFilter;
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
    #[ethevent(name = "QueryResubmitted", abi = "QueryResubmitted()")]
    pub struct QueryResubmittedFilter;
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
    pub enum UMATriggerEvents {
        ProposalDisputedFilter(ProposalDisputedFilter),
        QueryResubmittedFilter(QueryResubmittedFilter),
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for UMATriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ProposalDisputedFilter::decode_log(log) {
                return Ok(UMATriggerEvents::ProposalDisputedFilter(decoded));
            }
            if let Ok(decoded) = QueryResubmittedFilter::decode_log(log) {
                return Ok(UMATriggerEvents::QueryResubmittedFilter(decoded));
            }
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(UMATriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(UMATriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for UMATriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ProposalDisputedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QueryResubmittedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<ProposalDisputedFilter> for UMATriggerEvents {
        fn from(value: ProposalDisputedFilter) -> Self {
            Self::ProposalDisputedFilter(value)
        }
    }
    impl ::core::convert::From<QueryResubmittedFilter> for UMATriggerEvents {
        fn from(value: QueryResubmittedFilter) -> Self {
            Self::QueryResubmittedFilter(value)
        }
    }
    impl ::core::convert::From<SetAddedFilter> for UMATriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for UMATriggerEvents {
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
        Hash
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
    ///Container type for all input parameters for the `bondAmount` function with signature `bondAmount()` and selector `0x80f323a7`
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
    #[ethcall(name = "bondAmount", abi = "bondAmount()")]
    pub struct BondAmountCall;
    ///Container type for all input parameters for the `getSets` function with signature `getSets()` and selector `0x2cf7c531`
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    #[ethcall(name = "oracle", abi = "oracle()")]
    pub struct OracleCall;
    ///Container type for all input parameters for the `priceDisputed` function with signature `priceDisputed(bytes32,uint256,bytes,uint256)` and selector `0x0d8f2372`
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
        name = "priceDisputed",
        abi = "priceDisputed(bytes32,uint256,bytes,uint256)"
    )]
    pub struct PriceDisputedCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub p3: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `priceProposed` function with signature `priceProposed(bytes32,uint256,bytes)` and selector `0x9c2fd1df`
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
        Hash
    )]
    #[ethcall(name = "priceSettled", abi = "priceSettled(bytes32,uint256,bytes,int256)")]
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "queryIdentifier", abi = "queryIdentifier()")]
    pub struct QueryIdentifierCall;
    ///Container type for all input parameters for the `refundRecipient` function with signature `refundRecipient()` and selector `0x01f5ad5a`
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
    #[ethcall(name = "refundRecipient", abi = "refundRecipient()")]
    pub struct RefundRecipientCall;
    ///Container type for all input parameters for the `requestTimestamp` function with signature `requestTimestamp()` and selector `0x3e66a647`
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "sets", abi = "sets(uint256)")]
    pub struct SetsCall(pub ::ethers::core::types::U256);
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
    pub enum UMATriggerCalls {
        MaxSetLength(MaxSetLengthCall),
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        BondAmount(BondAmountCall),
        GetSets(GetSetsCall),
        GetSetsLength(GetSetsLengthCall),
        Manager(ManagerCall),
        Oracle(OracleCall),
        PriceDisputed(PriceDisputedCall),
        PriceProposed(PriceProposedCall),
        PriceSettled(PriceSettledCall),
        ProposalDisputeWindow(ProposalDisputeWindowCall),
        Query(QueryCall),
        QueryIdentifier(QueryIdentifierCall),
        RefundRecipient(RefundRecipientCall),
        RequestTimestamp(RequestTimestampCall),
        RewardToken(RewardTokenCall),
        RunProgrammaticCheck(RunProgrammaticCheckCall),
        Sets(SetsCall),
        State(StateCall),
    }
    impl ::ethers::core::abi::AbiDecode for UMATriggerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MaxSetLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxSetLength(decoded));
            }
            if let Ok(decoded)
                = <AcknowledgedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Acknowledged(decoded));
            }
            if let Ok(decoded)
                = <AddSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddSet(decoded));
            }
            if let Ok(decoded)
                = <BondAmountCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BondAmount(decoded));
            }
            if let Ok(decoded)
                = <GetSetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSets(decoded));
            }
            if let Ok(decoded)
                = <GetSetsLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSetsLength(decoded));
            }
            if let Ok(decoded)
                = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded)
                = <OracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Oracle(decoded));
            }
            if let Ok(decoded)
                = <PriceDisputedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PriceDisputed(decoded));
            }
            if let Ok(decoded)
                = <PriceProposedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PriceProposed(decoded));
            }
            if let Ok(decoded)
                = <PriceSettledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PriceSettled(decoded));
            }
            if let Ok(decoded)
                = <ProposalDisputeWindowCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProposalDisputeWindow(decoded));
            }
            if let Ok(decoded)
                = <QueryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Query(decoded));
            }
            if let Ok(decoded)
                = <QueryIdentifierCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QueryIdentifier(decoded));
            }
            if let Ok(decoded)
                = <RefundRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RefundRecipient(decoded));
            }
            if let Ok(decoded)
                = <RequestTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RequestTimestamp(decoded));
            }
            if let Ok(decoded)
                = <RewardTokenCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RewardToken(decoded));
            }
            if let Ok(decoded)
                = <RunProgrammaticCheckCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RunProgrammaticCheck(decoded));
            }
            if let Ok(decoded)
                = <SetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sets(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for UMATriggerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxSetLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Acknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BondAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSetsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Oracle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PriceDisputed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceProposed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceSettled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposalDisputeWindow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Query(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QueryIdentifier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RefundRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RewardToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RunProgrammaticCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for UMATriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxSetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::BondAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSetsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Oracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceDisputed(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceProposed(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceSettled(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposalDisputeWindow(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Query(element) => ::core::fmt::Display::fmt(element, f),
                Self::QueryIdentifier(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::RewardToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RunProgrammaticCheck(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Sets(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxSetLengthCall> for UMATriggerCalls {
        fn from(value: MaxSetLengthCall) -> Self {
            Self::MaxSetLength(value)
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for UMATriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for UMATriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<BondAmountCall> for UMATriggerCalls {
        fn from(value: BondAmountCall) -> Self {
            Self::BondAmount(value)
        }
    }
    impl ::core::convert::From<GetSetsCall> for UMATriggerCalls {
        fn from(value: GetSetsCall) -> Self {
            Self::GetSets(value)
        }
    }
    impl ::core::convert::From<GetSetsLengthCall> for UMATriggerCalls {
        fn from(value: GetSetsLengthCall) -> Self {
            Self::GetSetsLength(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for UMATriggerCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<OracleCall> for UMATriggerCalls {
        fn from(value: OracleCall) -> Self {
            Self::Oracle(value)
        }
    }
    impl ::core::convert::From<PriceDisputedCall> for UMATriggerCalls {
        fn from(value: PriceDisputedCall) -> Self {
            Self::PriceDisputed(value)
        }
    }
    impl ::core::convert::From<PriceProposedCall> for UMATriggerCalls {
        fn from(value: PriceProposedCall) -> Self {
            Self::PriceProposed(value)
        }
    }
    impl ::core::convert::From<PriceSettledCall> for UMATriggerCalls {
        fn from(value: PriceSettledCall) -> Self {
            Self::PriceSettled(value)
        }
    }
    impl ::core::convert::From<ProposalDisputeWindowCall> for UMATriggerCalls {
        fn from(value: ProposalDisputeWindowCall) -> Self {
            Self::ProposalDisputeWindow(value)
        }
    }
    impl ::core::convert::From<QueryCall> for UMATriggerCalls {
        fn from(value: QueryCall) -> Self {
            Self::Query(value)
        }
    }
    impl ::core::convert::From<QueryIdentifierCall> for UMATriggerCalls {
        fn from(value: QueryIdentifierCall) -> Self {
            Self::QueryIdentifier(value)
        }
    }
    impl ::core::convert::From<RefundRecipientCall> for UMATriggerCalls {
        fn from(value: RefundRecipientCall) -> Self {
            Self::RefundRecipient(value)
        }
    }
    impl ::core::convert::From<RequestTimestampCall> for UMATriggerCalls {
        fn from(value: RequestTimestampCall) -> Self {
            Self::RequestTimestamp(value)
        }
    }
    impl ::core::convert::From<RewardTokenCall> for UMATriggerCalls {
        fn from(value: RewardTokenCall) -> Self {
            Self::RewardToken(value)
        }
    }
    impl ::core::convert::From<RunProgrammaticCheckCall> for UMATriggerCalls {
        fn from(value: RunProgrammaticCheckCall) -> Self {
            Self::RunProgrammaticCheck(value)
        }
    }
    impl ::core::convert::From<SetsCall> for UMATriggerCalls {
        fn from(value: SetsCall) -> Self {
            Self::Sets(value)
        }
    }
    impl ::core::convert::From<StateCall> for UMATriggerCalls {
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
        Hash
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
    ///Container type for all return fields from the `bondAmount` function with signature `bondAmount()` and selector `0x80f323a7`
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
    pub struct BondAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSets` function with signature `getSets()` and selector `0x2cf7c531`
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
        Hash
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
        Hash
    )]
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `oracle` function with signature `oracle()` and selector `0x7dc0d1d0`
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
    pub struct OracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proposalDisputeWindow` function with signature `proposalDisputeWindow()` and selector `0xb365441b`
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
        Hash
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
        Hash
    )]
    pub struct QueryIdentifierReturn(pub [u8; 32]);
    ///Container type for all return fields from the `refundRecipient` function with signature `refundRecipient()` and selector `0x01f5ad5a`
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
    pub struct RefundRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `requestTimestamp` function with signature `requestTimestamp()` and selector `0x3e66a647`
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct SetsReturn(pub ::ethers::core::types::Address);
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
