pub use chainlink_trigger::*;
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
pub mod chainlink_trigger {
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
                        name: ::std::borrow::ToOwned::to_owned("_truthOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract AggregatorV3Interface",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_trackingOracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned(
                                "contract AggregatorV3Interface",
                            ),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_priceTolerance"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_truthFrequencyTolerance",
                        ),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned(
                            "_trackingFrequencyTolerance",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("oracleToScale"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("oracleToScale"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum ChainlinkTrigger.OracleToScale",
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
                    ::std::borrow::ToOwned::to_owned("priceTolerance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("priceTolerance"),
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
                    ::std::borrow::ToOwned::to_owned("scaleFactor"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("scaleFactor"),
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
                (
                    ::std::borrow::ToOwned::to_owned("trackingFrequencyTolerance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "trackingFrequencyTolerance",
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
                    ::std::borrow::ToOwned::to_owned("trackingOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trackingOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
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
                    ::std::borrow::ToOwned::to_owned("truthFrequencyTolerance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "truthFrequencyTolerance",
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
                    ::std::borrow::ToOwned::to_owned("truthOracle"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("truthOracle"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "contract AggregatorV3Interface",
                                        ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPrice"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPriceTolerance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidPriceTolerance",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTimestamp"),
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
                    ::std::borrow::ToOwned::to_owned("StaleOraclePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("StaleOraclePrice"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CHAINLINKTRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\x80`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x1Ff8\x03\x80b\0\x1Ff\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\tGV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\x80Ra'\x10\x83\x10b\0\0\xB2W`@QcP\x8F9?`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\xA0\x81\x90R\x90\x85\x16`\xC0R`\xE0\x84\x90Ra\x01\0\x83\x90Ra\x01 \x82\x90R`@\x80Qc1<\xE5g`\xE0\x1B\x81R\x90Q`\0\x92\x83\x92\x83\x92c1<\xE5g\x91`\x04\x80\x82\x01\x92` \x92\x90\x91\x90\x82\x90\x03\x01\x81\x86\x80;\x15\x80\x15b\0\x01VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0\x1FF\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x01kW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x01\x91\x91\x90b\0\t\xBCV[`\xFF\x16\x90P`\0`\xC0Q`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x02\x13W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0\x1FF\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x02(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x02N\x91\x90b\0\t\xBCV[`\xFF\x16\x90P\x81\x81\x10\x15b\0\x02\x81W`\x02\x93Pb\0\x02l\x81\x83b\0\n\x03V[b\0\x02y\x90`\nb\0\x0B\x16V[\x92Pb\0\x02\xAAV[\x80\x82\x10\x15b\0\x02\xAAW`\x01\x93Pb\0\x02\x9A\x82\x82b\0\n\x03V[b\0\x02\xA7\x90`\nb\0\x0B\x16V[\x92P[\x83`\x02\x81\x11\x15b\0\x02\xBFWb\0\x02\xBFb\0\x0B$V[a\x01`\x81`\x02\x81\x11\x15b\0\x02\xD7Wb\0\x02\xD7b\0\x0B$V[\x90RPa\x01@\x83\x90Rb\0\x02\xEAb\0\x02\xFBV[PPPPPPPPPPPb\0\x0C\x12V[`\0\x80`\0T`\xFF\x16`\x02\x81\x11\x15b\0\x03\x18Wb\0\x03\x18b\0\x0B$V[\x14b\0\x03(WP`\0T`\xFF\x16\x90V[b\0\x032b\0\x03SV[\x15b\0\x03IWb\0\x03D`\x02b\0\x04[V[\x90P\x90V[P`\0T`\xFF\x16\x90V[`\0\x80b\0\x03m`\xA0Qa\x01\0Qb\0\x05\xE5` \x1B` \x1CV[\x90P`\0b\0\x03\x88`\xC0Qa\x01 Qb\0\x05\xE5` \x1B` \x1CV[\x90P`\x01a\x01`Q`\x02\x81\x11\x15b\0\x03\xA4Wb\0\x03\xA4b\0\x0B$V[\x03b\0\x03\xC2Wa\x01@Qb\0\x03\xBA\x90\x83b\0\x0B:V[\x91Pb\0\x03\xF5V[`\x02a\x01`Q`\x02\x81\x11\x15b\0\x03\xDCWb\0\x03\xDCb\0\x0B$V[\x03b\0\x03\xF5Wa\x01@Qb\0\x03\xF2\x90\x82b\0\x0B:V[\x90P[`\0\x81\x83\x11b\0\x04\x11Wb\0\x04\x0B\x83\x83b\0\n\x03V[b\0\x04\x1DV[b\0\x04\x1D\x82\x84b\0\n\x03V[\x90P`\0\x83\x11b\0\x040W`\x01b\0\x04SV[`\xE0Qb\0\x04Qa'\x10\x85\x84b\0\x07\x1E` \x1Bb\0\x08\x1A\x17\x90\x92\x91\x90` \x1CV[\x11[\x93PPPP\x90V[`\0\x80Tb\0\x04n\x90`\xFF\x16\x83b\0\x07MV[b\0\x04\x8CW`@Qc#\xE6\x9E\x03`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90`\xFF\x19\x16`\x01\x83`\x02\x81\x11\x15b\0\x04\xAEWb\0\x04\xAEb\0\x0B$V[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15b\0\x05\x9FW`\x01\x81\x81T\x81\x10b\0\x04\xD8Wb\0\x04\xD8b\0\x0B\\V[`\0\x91\x82R` \x90\x91 \x01T`@Qc=:\xFE\xEF`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90czu\xFD\xDE\x90b\0\x05\x13\x90\x87\x90`\x04\x01b\0\x0BrV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\x05mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0\x1FF\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15b\0\x05\x82W=`\0\x80>=`\0\xFD[PPPPb\0\x05\x97\x81b\0\x08\xD8` \x1B` \x1CV[\x90Pb\0\x04\xB9V[P\x82`\x02\x81\x11\x15b\0\x05\xB5Wb\0\x05\xB5b\0\x0B$V[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\0\x80`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15b\0\x06cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R`\0\x80Q` b\0\x1FF\x839\x81Q\x91R`D\x82\x01\x90\x81Rd code`\xD8\x1B`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15b\0\x06xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90b\0\x06\x9E\x91\x90b\0\x0B\xB8V[P\x93PP\x92PPB\x81\x11\x15b\0\x06\xC7W`@Qc\xB7\xD0\x94\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83b\0\x06\xD4\x82Bb\0\n\x03V[\x11\x15b\0\x06\xF4W`@Qc\x15\x10\xFE[`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15b\0\x07\x16W`@Qb\xBF\xC9!`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16b\0\x077W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0`\x02\x83`\x02\x81\x11\x15b\0\x07fWb\0\x07fb\0\x0B$V[\x03b\0\x07uWP`\0b\0\x08\xD2V[\x81`\x02\x81\x11\x15b\0\x07\x8AWb\0\x07\x8Ab\0\x0B$V[\x83`\x02\x81\x11\x15b\0\x07\x9FWb\0\x07\x9Fb\0\x0B$V[\x03b\0\x07\xAEWP`\x01b\0\x08\xD2V[`\0\x83`\x02\x81\x11\x15b\0\x07\xC5Wb\0\x07\xC5b\0\x0B$V[\x14\x80\x15b\0\x07\xE7WP`\x01\x82`\x02\x81\x11\x15b\0\x07\xE5Wb\0\x07\xE5b\0\x0B$V[\x14[\x15b\0\x07\xF6WP`\x01b\0\x08\xD2V[`\x01\x83`\x02\x81\x11\x15b\0\x08\rWb\0\x08\rb\0\x0B$V[\x14\x80\x15b\0\x08/WP`\0\x82`\x02\x81\x11\x15b\0\x08-Wb\0\x08-b\0\x0B$V[\x14[\x15b\0\x08>WP`\x01b\0\x08\xD2V[`\0\x83`\x02\x81\x11\x15b\0\x08UWb\0\x08Ub\0\x0B$V[\x14\x80\x15b\0\x08wWP`\x02\x82`\x02\x81\x11\x15b\0\x08uWb\0\x08ub\0\x0B$V[\x14[\x15b\0\x08\x86WP`\x01b\0\x08\xD2V[`\x01\x83`\x02\x81\x11\x15b\0\x08\x9DWb\0\x08\x9Db\0\x0B$V[\x14\x80\x15b\0\x08\xBFWP`\x02\x82`\x02\x81\x11\x15b\0\x08\xBDWb\0\x08\xBDb\0\x0B$V[\x14[\x15b\0\x08\xCEWP`\x01b\0\x08\xD2V[P`\0[\x92\x91PPV[`\x01\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\tDW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\tfWb\0\tfb\0\x08\xDEV[\x86Qb\0\ts\x81b\0\t.V[` \x88\x01Q\x90\x96Pb\0\t\x86\x81b\0\t.V[`@\x88\x01Q\x90\x95Pb\0\t\x99\x81b\0\t.V[\x80\x94PP``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15b\0\t\xD4Wb\0\t\xD4b\0\x08\xDEV[\x81Q`\xFF\x81\x16\x81\x14b\0\t\xE6W`\0\x80\xFD[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15b\0\x08\xD2Wb\0\x08\xD2b\0\t\xEDV[`\x01\x81\x81[\x80\x85\x11\x15b\0\nZW\x81`\0\x19\x04\x82\x11\x15b\0\n>Wb\0\n>b\0\t\xEDV[\x80\x85\x16\x15b\0\nLW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90b\0\n\x1EV[P\x92P\x92\x90PV[`\0\x82b\0\nsWP`\x01b\0\x08\xD2V[\x81b\0\n\x82WP`\0b\0\x08\xD2V[\x81`\x01\x81\x14b\0\n\x9BW`\x02\x81\x14b\0\n\xA6Wb\0\n\xC6V[`\x01\x91PPb\0\x08\xD2V[`\xFF\x84\x11\x15b\0\n\xBAWb\0\n\xBAb\0\t\xEDV[PP`\x01\x82\x1Bb\0\x08\xD2V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15b\0\n\xEBWP\x81\x81\nb\0\x08\xD2V[b\0\n\xF7\x83\x83b\0\n\x19V[\x80`\0\x19\x04\x82\x11\x15b\0\x0B\x0EWb\0\x0B\x0Eb\0\t\xEDV[\x02\x93\x92PPPV[`\0b\0\t\xE6\x83\x83b\0\nbV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15b\0\x0BWWb\0\x0BWb\0\t\xEDV[P\x02\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10b\0\x0B\x95WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[\x80Q`\x01`\x01`P\x1B\x03\x81\x16\x81\x14b\0\x0B\xB3W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x0B\xD6Wb\0\x0B\xD6b\0\x08\xDEV[b\0\x0B\xE1\x86b\0\x0B\x9BV[\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pb\0\x0C\x06`\x80\x87\x01b\0\x0B\x9BV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`Qa\x12\x88b\0\x0C\xBE`\09`\0\x81\x81a\x03:\x01R\x81\x81a\x08\xE9\x01Ra\tS\x01R`\0\x81\x81a\x02\xEC\x01R\x81\x81a\t#\x01Ra\t\x8D\x01R`\0\x81\x81a\x03\xCF\x01Ra\x08\xBF\x01R`\0\x81\x81a\x03a\x01Ra\x08q\x01R`\0\x81\x81a\x02\x9C\x01Ra\t\xEA\x01R`\0\x81\x81a\x03\x88\x01Ra\x08\x9E\x01R`\0\x81\x81a\x03\x13\x01Ra\x08P\x01R`\0\x81\x81a\x02P\x01Ra\x05\x87\x01Ra\x12\x88`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x82W`\x005`\xE0\x1C\x80c\x81_\xE9'\x11a\x01\x19W\x80c\xC1\x9D\x93\xFB\x11a\0\xE8W\x80c\xC1\x9D\x93\xFB\x14a\x03\xAAW\x80c\xD5\x80\xDE\xD4\x14a\x03\xB7W\x80c\xE5f\x1E\0\x14a\x03\xCAW\x80c\xE8cv\xC5\x14a\x03\xF1Wa\x01\x82V[\x80c\x81_\xE9'\x14a\x03\x0EW\x80c\x8C\x9C\xC0<\x14a\x035W\x80c\xA1l\xB4t\x14a\x03\\W\x80c\xAE\xC9\xC3\xDD\x14a\x03\x83Wa\x01\x82V[\x80cY\x01\x1C\xD1\x11a\x01UW\x80cY\x01\x1C\xD1\x14a\x02\x97W\x80cYSqD\x14a\x02\xCCW\x80c[\"\x7F\x9B\x14a\x02\xD4W\x80ch=\xD1\x91\x14a\x02\xE7Wa\x01\x82V[\x80c\x08l)\x8D\x14a\x02\tW\x80c,\xF7\xC51\x14a\x02!W\x80c7\xA0\xAF\xC1\x14a\x026W\x80cH\x1Cju\x14a\x02KW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x01[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02)a\x03\xF9V[`@Qa\x02\x18\x91\x90a\x0FVV[a\x02>a\x04hV[`@Qa\x02\x18\x91\x90a\x10\x19V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x18V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x18V[a\x02\xBE`2\x81V[a\x02ra\x02\xE26`\x04a\x10\xB1V[a\x04\xB7V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Ta\x02>\x90`\xFF\x16\x81V[a\x02\x0Ca\x03\xC56`\x04a\x10\xCDV[a\x04\xEEV[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02\xBEV[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04^W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x043W[PPPPP\x90P\x90V[`\0\x80`\0T`\xFF\x16`\x02\x81\x11\x15a\x04\x82Wa\x04\x82a\x0F\xB0V[\x14a\x04\x91WP`\0T`\xFF\x16\x90V[a\x04\x99a\x08HV[\x15a\x04\xADWa\x04\xA8`\x02a\n V[\x90P\x90V[P`\0T`\xFF\x16\x90V[`\x01\x81\x81T\x81\x10a\x04\xC7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x05?W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06MW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x85\x91\x90a\x11\rV[\x90P\x80a\x06\xBEW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x06\xFBW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x07jW\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x070Wa\x070a\x112V[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07bWP`\x01\x94\x93PPPPV[`\x01\x01a\x06\xFEV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x082W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x08\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C1V[\x90P`\0a\x08\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C1V[\x90P`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x81\x11\x15a\t\x19Wa\t\x19a\x0F\xB0V[\x03a\tOWa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x11\x90V[\x91Pa\t\xB5V[`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x81\x11\x15a\t\x83Wa\t\x83a\x0F\xB0V[\x03a\t\xB5Wa\t\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x11\x90V[\x90P[`\0\x81\x83\x11a\t\xCDWa\t\xC8\x83\x83a\x11\xCDV[a\t\xD7V[a\t\xD7\x82\x84a\x11\xCDV[\x90P`\0\x83\x11a\t\xE8W`\x01a\n\x18V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x16\x82a'\x10\x86a\x08\x1AV[\x11[\x93PPPP\x90V[`\0\x80Ta\n1\x90`\xFF\x16\x83a\r\xFCV[a\ngW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\n\xA4Wa\n\xA4a\x0F\xB0V[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x0B\xEEW`\x01\x81\x81T\x81\x10a\n\xCAWa\n\xCAa\x112V[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x0B)\x90\x87\x90`\x04\x01a\x10\x19V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\xD9W=`\0\x80>=`\0\xFD[PPPPa\x0B\xE7\x81`\x01\x01\x90V[\x90Pa\n\xAFV[P\x82`\x02\x81\x11\x15a\x0C\x01Wa\x0C\x01a\x0F\xB0V[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\0\x80`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r6\x91\x90a\x11\xFFV[P\x93PP\x92PPB\x81\x11\x15a\rwW`@Q\x7F\xB7\xD0\x94\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\r\x82\x82Ba\x11\xCDV[\x11\x15a\r\xBAW`@Q\x7F\xA8\x87\xF2\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\r\xF4W`@Q~\xBF\xC9!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x93\x92PPPV[`\0`\x02\x83`\x02\x81\x11\x15a\x0E\x12Wa\x0E\x12a\x0F\xB0V[\x03a\x0E\x1FWP`\0a\x0FPV[\x81`\x02\x81\x11\x15a\x0E1Wa\x0E1a\x0F\xB0V[\x83`\x02\x81\x11\x15a\x0ECWa\x0ECa\x0F\xB0V[\x03a\x0EPWP`\x01a\x0FPV[`\0\x83`\x02\x81\x11\x15a\x0EdWa\x0Eda\x0F\xB0V[\x14\x80\x15a\x0E\x82WP`\x01\x82`\x02\x81\x11\x15a\x0E\x80Wa\x0E\x80a\x0F\xB0V[\x14[\x15a\x0E\x8FWP`\x01a\x0FPV[`\x01\x83`\x02\x81\x11\x15a\x0E\xA3Wa\x0E\xA3a\x0F\xB0V[\x14\x80\x15a\x0E\xC1WP`\0\x82`\x02\x81\x11\x15a\x0E\xBFWa\x0E\xBFa\x0F\xB0V[\x14[\x15a\x0E\xCEWP`\x01a\x0FPV[`\0\x83`\x02\x81\x11\x15a\x0E\xE2Wa\x0E\xE2a\x0F\xB0V[\x14\x80\x15a\x0F\0WP`\x02\x82`\x02\x81\x11\x15a\x0E\xFEWa\x0E\xFEa\x0F\xB0V[\x14[\x15a\x0F\rWP`\x01a\x0FPV[`\x01\x83`\x02\x81\x11\x15a\x0F!Wa\x0F!a\x0F\xB0V[\x14\x80\x15a\x0F?WP`\x02\x82`\x02\x81\x11\x15a\x0F=Wa\x0F=a\x0F\xB0V[\x14[\x15a\x0FLWP`\x01a\x0FPV[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xA4W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0FrV[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x10\x16W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01a\x10&\x83a\x0F\xDFV[\x91\x90R\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xC6Wa\x10\xC6a\x10,V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\xE2Wa\x10\xE2a\x10,V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x06W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11\"Wa\x11\"a\x10,V[\x81Q\x80\x15\x15\x81\x14a\x11\x06W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x11\xC8Wa\x11\xC8a\x11aV[P\x02\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0FPWa\x0FPa\x11aV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xFAW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12\x1AWa\x12\x1Aa\x10,V[a\x12#\x86a\x11\xE0V[\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa\x12F`\x80\x87\x01a\x11\xE0V[\x90P\x92\x95P\x92\x95\x90\x93PV\xFE\xA2dipfsX\"\x12 Q\xE4)\xB6;\xD7_\x8C\x0E\xB5\xC75g\xB2\xEEi\xF9\x17\xBA\xB2\xA0\x99\xF8\x11\xB9\xF7\x81H\xB2v\xBA\x1EdsolcC\0\x08\x10\x003Target contract does not contain";
    /// The bytecode of the contract.
    pub static CHAINLINKTRIGGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\x82W`\x005`\xE0\x1C\x80c\x81_\xE9'\x11a\x01\x19W\x80c\xC1\x9D\x93\xFB\x11a\0\xE8W\x80c\xC1\x9D\x93\xFB\x14a\x03\xAAW\x80c\xD5\x80\xDE\xD4\x14a\x03\xB7W\x80c\xE5f\x1E\0\x14a\x03\xCAW\x80c\xE8cv\xC5\x14a\x03\xF1Wa\x01\x82V[\x80c\x81_\xE9'\x14a\x03\x0EW\x80c\x8C\x9C\xC0<\x14a\x035W\x80c\xA1l\xB4t\x14a\x03\\W\x80c\xAE\xC9\xC3\xDD\x14a\x03\x83Wa\x01\x82V[\x80cY\x01\x1C\xD1\x11a\x01UW\x80cY\x01\x1C\xD1\x14a\x02\x97W\x80cYSqD\x14a\x02\xCCW\x80c[\"\x7F\x9B\x14a\x02\xD4W\x80ch=\xD1\x91\x14a\x02\xE7Wa\x01\x82V[\x80c\x08l)\x8D\x14a\x02\tW\x80c,\xF7\xC51\x14a\x02!W\x80c7\xA0\xAF\xC1\x14a\x026W\x80cH\x1Cju\x14a\x02KW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x01[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02)a\x03\xF9V[`@Qa\x02\x18\x91\x90a\x0FVV[a\x02>a\x04hV[`@Qa\x02\x18\x91\x90a\x10\x19V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x18V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01a\x02\x18V[a\x02\xBE`2\x81V[a\x02ra\x02\xE26`\x04a\x10\xB1V[a\x04\xB7V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0Ta\x02>\x90`\xFF\x16\x81V[a\x02\x0Ca\x03\xC56`\x04a\x10\xCDV[a\x04\xEEV[a\x02\xBE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\x01Ta\x02\xBEV[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x04^W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x043W[PPPPP\x90P\x90V[`\0\x80`\0T`\xFF\x16`\x02\x81\x11\x15a\x04\x82Wa\x04\x82a\x0F\xB0V[\x14a\x04\x91WP`\0T`\xFF\x16\x90V[a\x04\x99a\x08HV[\x15a\x04\xADWa\x04\xA8`\x02a\n V[\x90P\x90V[P`\0T`\xFF\x16\x90V[`\x01\x81\x81T\x81\x10a\x04\xC7W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x05?W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x06MW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x06aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x85\x91\x90a\x11\rV[\x90P\x80a\x06\xBEW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x06\xFBW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x07jW\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x070Wa\x070a\x112V[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x07bWP`\x01\x94\x93PPPPV[`\x01\x01a\x06\xFEV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x082W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80a\x08\x95\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C1V[\x90P`\0a\x08\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C1V[\x90P`\x01\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x81\x11\x15a\t\x19Wa\t\x19a\x0F\xB0V[\x03a\tOWa\tH\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x11\x90V[\x91Pa\t\xB5V[`\x02\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02\x81\x11\x15a\t\x83Wa\t\x83a\x0F\xB0V[\x03a\t\xB5Wa\t\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a\x11\x90V[\x90P[`\0\x81\x83\x11a\t\xCDWa\t\xC8\x83\x83a\x11\xCDV[a\t\xD7V[a\t\xD7\x82\x84a\x11\xCDV[\x90P`\0\x83\x11a\t\xE8W`\x01a\n\x18V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\x16\x82a'\x10\x86a\x08\x1AV[\x11[\x93PPPP\x90V[`\0\x80Ta\n1\x90`\xFF\x16\x83a\r\xFCV[a\ngW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\n\xA4Wa\n\xA4a\x0F\xB0V[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x0B\xEEW`\x01\x81\x81T\x81\x10a\n\xCAWa\n\xCAa\x112V[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x0B)\x90\x87\x90`\x04\x01a\x10\x19V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xC5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x0B\xD9W=`\0\x80>=`\0\xFD[PPPPa\x0B\xE7\x81`\x01\x01\x90V[\x90Pa\n\xAFV[P\x82`\x02\x81\x11\x15a\x0C\x01Wa\x0C\x01a\x0F\xB0V[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\0\x80`\0\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xFE\xAF\x96\x8C`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\xA0`@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x0C\xFEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\r\x12W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r6\x91\x90a\x11\xFFV[P\x93PP\x92PPB\x81\x11\x15a\rwW`@Q\x7F\xB7\xD0\x94\x97\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\r\x82\x82Ba\x11\xCDV[\x11\x15a\r\xBAW`@Q\x7F\xA8\x87\xF2\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\r\xF4W`@Q~\xBF\xC9!\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x93\x92PPPV[`\0`\x02\x83`\x02\x81\x11\x15a\x0E\x12Wa\x0E\x12a\x0F\xB0V[\x03a\x0E\x1FWP`\0a\x0FPV[\x81`\x02\x81\x11\x15a\x0E1Wa\x0E1a\x0F\xB0V[\x83`\x02\x81\x11\x15a\x0ECWa\x0ECa\x0F\xB0V[\x03a\x0EPWP`\x01a\x0FPV[`\0\x83`\x02\x81\x11\x15a\x0EdWa\x0Eda\x0F\xB0V[\x14\x80\x15a\x0E\x82WP`\x01\x82`\x02\x81\x11\x15a\x0E\x80Wa\x0E\x80a\x0F\xB0V[\x14[\x15a\x0E\x8FWP`\x01a\x0FPV[`\x01\x83`\x02\x81\x11\x15a\x0E\xA3Wa\x0E\xA3a\x0F\xB0V[\x14\x80\x15a\x0E\xC1WP`\0\x82`\x02\x81\x11\x15a\x0E\xBFWa\x0E\xBFa\x0F\xB0V[\x14[\x15a\x0E\xCEWP`\x01a\x0FPV[`\0\x83`\x02\x81\x11\x15a\x0E\xE2Wa\x0E\xE2a\x0F\xB0V[\x14\x80\x15a\x0F\0WP`\x02\x82`\x02\x81\x11\x15a\x0E\xFEWa\x0E\xFEa\x0F\xB0V[\x14[\x15a\x0F\rWP`\x01a\x0FPV[`\x01\x83`\x02\x81\x11\x15a\x0F!Wa\x0F!a\x0F\xB0V[\x14\x80\x15a\x0F?WP`\x02\x82`\x02\x81\x11\x15a\x0F=Wa\x0F=a\x0F\xB0V[\x14[\x15a\x0FLWP`\x01a\x0FPV[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x0F\xA4W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0FrV[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a\x10\x16W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[PV[` \x81\x01a\x10&\x83a\x0F\xDFV[\x91\x90R\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x10\xC6Wa\x10\xC6a\x10,V[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x10\xE2Wa\x10\xE2a\x10,V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\x06W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x11\"Wa\x11\"a\x10,V[\x81Q\x80\x15\x15\x81\x14a\x11\x06W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x83\x11\x82\x15\x15\x16\x15a\x11\xC8Wa\x11\xC8a\x11aV[P\x02\x90V[\x81\x81\x03\x81\x81\x11\x15a\x0FPWa\x0FPa\x11aV[\x80Qi\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x11\xFAW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x12\x1AWa\x12\x1Aa\x10,V[a\x12#\x86a\x11\xE0V[\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa\x12F`\x80\x87\x01a\x11\xE0V[\x90P\x92\x95P\x92\x95\x90\x93PV\xFE\xA2dipfsX\"\x12 Q\xE4)\xB6;\xD7_\x8C\x0E\xB5\xC75g\xB2\xEEi\xF9\x17\xBA\xB2\xA0\x99\xF8\x11\xB9\xF7\x81H\xB2v\xBA\x1EdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static CHAINLINKTRIGGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ChainlinkTrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for ChainlinkTrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ChainlinkTrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ChainlinkTrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ChainlinkTrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ChainlinkTrigger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ChainlinkTrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    CHAINLINKTRIGGER_ABI.clone(),
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
                CHAINLINKTRIGGER_ABI.clone(),
                CHAINLINKTRIGGER_BYTECODE.clone().into(),
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
        ///Calls the contract's `oracleToScale` (0x8c9cc03c) function
        pub fn oracle_to_scale(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([140, 156, 192, 60], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `priceTolerance` (0x59011cd1) function
        pub fn price_tolerance(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([89, 1, 28, 209], ())
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
        ///Calls the contract's `scaleFactor` (0x683dd191) function
        pub fn scale_factor(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 61, 209, 145], ())
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
        ///Calls the contract's `trackingFrequencyTolerance` (0xe5661e00) function
        pub fn tracking_frequency_tolerance(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 102, 30, 0], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trackingOracle` (0xaec9c3dd) function
        pub fn tracking_oracle(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([174, 201, 195, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `truthFrequencyTolerance` (0xa16cb474) function
        pub fn truth_frequency_tolerance(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([161, 108, 180, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `truthOracle` (0x815fe927) function
        pub fn truth_oracle(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([129, 95, 233, 39], ())
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
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ChainlinkTriggerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for ChainlinkTrigger<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidPrice` with signature `InvalidPrice()` and selector `0x00bfc921`
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
    #[etherror(name = "InvalidPrice", abi = "InvalidPrice()")]
    pub struct InvalidPrice;
    ///Custom Error type `InvalidPriceTolerance` with signature `InvalidPriceTolerance()` and selector `0x508f393f`
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
    #[etherror(name = "InvalidPriceTolerance", abi = "InvalidPriceTolerance()")]
    pub struct InvalidPriceTolerance;
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
    ///Custom Error type `InvalidTimestamp` with signature `InvalidTimestamp()` and selector `0xb7d09497`
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
    #[etherror(name = "InvalidTimestamp", abi = "InvalidTimestamp()")]
    pub struct InvalidTimestamp;
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
    ///Custom Error type `StaleOraclePrice` with signature `StaleOraclePrice()` and selector `0xa887f2d8`
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
    #[etherror(name = "StaleOraclePrice", abi = "StaleOraclePrice()")]
    pub struct StaleOraclePrice;
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ChainlinkTriggerErrors {
        InvalidPrice(InvalidPrice),
        InvalidPriceTolerance(InvalidPriceTolerance),
        InvalidStateTransition(InvalidStateTransition),
        InvalidTimestamp(InvalidTimestamp),
        SetLimitReached(SetLimitReached),
        StaleOraclePrice(StaleOraclePrice),
        Unacknowledged(Unacknowledged),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ChainlinkTriggerErrors {
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
                = <InvalidPrice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPrice(decoded));
            }
            if let Ok(decoded)
                = <InvalidPriceTolerance as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidPriceTolerance(decoded));
            }
            if let Ok(decoded)
                = <InvalidStateTransition as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidStateTransition(decoded));
            }
            if let Ok(decoded)
                = <InvalidTimestamp as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTimestamp(decoded));
            }
            if let Ok(decoded)
                = <SetLimitReached as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetLimitReached(decoded));
            }
            if let Ok(decoded)
                = <StaleOraclePrice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::StaleOraclePrice(decoded));
            }
            if let Ok(decoded)
                = <Unacknowledged as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unacknowledged(decoded));
            }
            if let Ok(decoded)
                = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ChainlinkTriggerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPriceTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidStateTransition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetLimitReached(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StaleOraclePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unacknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for ChainlinkTriggerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidPrice as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidPriceTolerance as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidStateTransition as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTimestamp as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SetLimitReached as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <StaleOraclePrice as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unacknowledged as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Unauthorized as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ChainlinkTriggerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPriceTolerance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidStateTransition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetLimitReached(element) => ::core::fmt::Display::fmt(element, f),
                Self::StaleOraclePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unacknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ChainlinkTriggerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidPrice> for ChainlinkTriggerErrors {
        fn from(value: InvalidPrice) -> Self {
            Self::InvalidPrice(value)
        }
    }
    impl ::core::convert::From<InvalidPriceTolerance> for ChainlinkTriggerErrors {
        fn from(value: InvalidPriceTolerance) -> Self {
            Self::InvalidPriceTolerance(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for ChainlinkTriggerErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<InvalidTimestamp> for ChainlinkTriggerErrors {
        fn from(value: InvalidTimestamp) -> Self {
            Self::InvalidTimestamp(value)
        }
    }
    impl ::core::convert::From<SetLimitReached> for ChainlinkTriggerErrors {
        fn from(value: SetLimitReached) -> Self {
            Self::SetLimitReached(value)
        }
    }
    impl ::core::convert::From<StaleOraclePrice> for ChainlinkTriggerErrors {
        fn from(value: StaleOraclePrice) -> Self {
            Self::StaleOraclePrice(value)
        }
    }
    impl ::core::convert::From<Unacknowledged> for ChainlinkTriggerErrors {
        fn from(value: Unacknowledged) -> Self {
            Self::Unacknowledged(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for ChainlinkTriggerErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    pub enum ChainlinkTriggerEvents {
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for ChainlinkTriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(ChainlinkTriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(ChainlinkTriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ChainlinkTriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SetAddedFilter> for ChainlinkTriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for ChainlinkTriggerEvents {
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
    ///Container type for all input parameters for the `oracleToScale` function with signature `oracleToScale()` and selector `0x8c9cc03c`
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
    #[ethcall(name = "oracleToScale", abi = "oracleToScale()")]
    pub struct OracleToScaleCall;
    ///Container type for all input parameters for the `priceTolerance` function with signature `priceTolerance()` and selector `0x59011cd1`
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
    #[ethcall(name = "priceTolerance", abi = "priceTolerance()")]
    pub struct PriceToleranceCall;
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
    ///Container type for all input parameters for the `scaleFactor` function with signature `scaleFactor()` and selector `0x683dd191`
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
    #[ethcall(name = "scaleFactor", abi = "scaleFactor()")]
    pub struct ScaleFactorCall;
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
    ///Container type for all input parameters for the `trackingFrequencyTolerance` function with signature `trackingFrequencyTolerance()` and selector `0xe5661e00`
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
    #[ethcall(name = "trackingFrequencyTolerance", abi = "trackingFrequencyTolerance()")]
    pub struct TrackingFrequencyToleranceCall;
    ///Container type for all input parameters for the `trackingOracle` function with signature `trackingOracle()` and selector `0xaec9c3dd`
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
    #[ethcall(name = "trackingOracle", abi = "trackingOracle()")]
    pub struct TrackingOracleCall;
    ///Container type for all input parameters for the `truthFrequencyTolerance` function with signature `truthFrequencyTolerance()` and selector `0xa16cb474`
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
    #[ethcall(name = "truthFrequencyTolerance", abi = "truthFrequencyTolerance()")]
    pub struct TruthFrequencyToleranceCall;
    ///Container type for all input parameters for the `truthOracle` function with signature `truthOracle()` and selector `0x815fe927`
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
    #[ethcall(name = "truthOracle", abi = "truthOracle()")]
    pub struct TruthOracleCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ChainlinkTriggerCalls {
        MaxSetLength(MaxSetLengthCall),
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        GetSets(GetSetsCall),
        GetSetsLength(GetSetsLengthCall),
        Manager(ManagerCall),
        OracleToScale(OracleToScaleCall),
        PriceTolerance(PriceToleranceCall),
        RunProgrammaticCheck(RunProgrammaticCheckCall),
        ScaleFactor(ScaleFactorCall),
        Sets(SetsCall),
        State(StateCall),
        TrackingFrequencyTolerance(TrackingFrequencyToleranceCall),
        TrackingOracle(TrackingOracleCall),
        TruthFrequencyTolerance(TruthFrequencyToleranceCall),
        TruthOracle(TruthOracleCall),
    }
    impl ::ethers::core::abi::AbiDecode for ChainlinkTriggerCalls {
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
                = <OracleToScaleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OracleToScale(decoded));
            }
            if let Ok(decoded)
                = <PriceToleranceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PriceTolerance(decoded));
            }
            if let Ok(decoded)
                = <RunProgrammaticCheckCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RunProgrammaticCheck(decoded));
            }
            if let Ok(decoded)
                = <ScaleFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ScaleFactor(decoded));
            }
            if let Ok(decoded)
                = <SetsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sets(decoded));
            }
            if let Ok(decoded)
                = <StateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::State(decoded));
            }
            if let Ok(decoded)
                = <TrackingFrequencyToleranceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TrackingFrequencyTolerance(decoded));
            }
            if let Ok(decoded)
                = <TrackingOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TrackingOracle(decoded));
            }
            if let Ok(decoded)
                = <TruthFrequencyToleranceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TruthFrequencyTolerance(decoded));
            }
            if let Ok(decoded)
                = <TruthOracleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TruthOracle(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ChainlinkTriggerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxSetLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Acknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSetsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleToScale(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RunProgrammaticCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ScaleFactor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TrackingFrequencyTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrackingOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TruthFrequencyTolerance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TruthOracle(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ChainlinkTriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxSetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSetsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleToScale(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceTolerance(element) => ::core::fmt::Display::fmt(element, f),
                Self::RunProgrammaticCheck(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ScaleFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sets(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrackingFrequencyTolerance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TrackingOracle(element) => ::core::fmt::Display::fmt(element, f),
                Self::TruthFrequencyTolerance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TruthOracle(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxSetLengthCall> for ChainlinkTriggerCalls {
        fn from(value: MaxSetLengthCall) -> Self {
            Self::MaxSetLength(value)
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for ChainlinkTriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for ChainlinkTriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<GetSetsCall> for ChainlinkTriggerCalls {
        fn from(value: GetSetsCall) -> Self {
            Self::GetSets(value)
        }
    }
    impl ::core::convert::From<GetSetsLengthCall> for ChainlinkTriggerCalls {
        fn from(value: GetSetsLengthCall) -> Self {
            Self::GetSetsLength(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for ChainlinkTriggerCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<OracleToScaleCall> for ChainlinkTriggerCalls {
        fn from(value: OracleToScaleCall) -> Self {
            Self::OracleToScale(value)
        }
    }
    impl ::core::convert::From<PriceToleranceCall> for ChainlinkTriggerCalls {
        fn from(value: PriceToleranceCall) -> Self {
            Self::PriceTolerance(value)
        }
    }
    impl ::core::convert::From<RunProgrammaticCheckCall> for ChainlinkTriggerCalls {
        fn from(value: RunProgrammaticCheckCall) -> Self {
            Self::RunProgrammaticCheck(value)
        }
    }
    impl ::core::convert::From<ScaleFactorCall> for ChainlinkTriggerCalls {
        fn from(value: ScaleFactorCall) -> Self {
            Self::ScaleFactor(value)
        }
    }
    impl ::core::convert::From<SetsCall> for ChainlinkTriggerCalls {
        fn from(value: SetsCall) -> Self {
            Self::Sets(value)
        }
    }
    impl ::core::convert::From<StateCall> for ChainlinkTriggerCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<TrackingFrequencyToleranceCall>
    for ChainlinkTriggerCalls {
        fn from(value: TrackingFrequencyToleranceCall) -> Self {
            Self::TrackingFrequencyTolerance(value)
        }
    }
    impl ::core::convert::From<TrackingOracleCall> for ChainlinkTriggerCalls {
        fn from(value: TrackingOracleCall) -> Self {
            Self::TrackingOracle(value)
        }
    }
    impl ::core::convert::From<TruthFrequencyToleranceCall> for ChainlinkTriggerCalls {
        fn from(value: TruthFrequencyToleranceCall) -> Self {
            Self::TruthFrequencyTolerance(value)
        }
    }
    impl ::core::convert::From<TruthOracleCall> for ChainlinkTriggerCalls {
        fn from(value: TruthOracleCall) -> Self {
            Self::TruthOracle(value)
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
    ///Container type for all return fields from the `oracleToScale` function with signature `oracleToScale()` and selector `0x8c9cc03c`
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
    pub struct OracleToScaleReturn(pub u8);
    ///Container type for all return fields from the `priceTolerance` function with signature `priceTolerance()` and selector `0x59011cd1`
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
    pub struct PriceToleranceReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `scaleFactor` function with signature `scaleFactor()` and selector `0x683dd191`
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
    pub struct ScaleFactorReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `trackingFrequencyTolerance` function with signature `trackingFrequencyTolerance()` and selector `0xe5661e00`
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
    pub struct TrackingFrequencyToleranceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `trackingOracle` function with signature `trackingOracle()` and selector `0xaec9c3dd`
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
    pub struct TrackingOracleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `truthFrequencyTolerance` function with signature `truthFrequencyTolerance()` and selector `0xa16cb474`
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
    pub struct TruthFrequencyToleranceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `truthOracle` function with signature `truthOracle()` and selector `0x815fe927`
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
    pub struct TruthOracleReturn(pub ::ethers::core::types::Address);
}
