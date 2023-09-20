pub use flexible_trigger::*;
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
pub mod flexible_trigger {
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
                        name: ::std::borrow::ToOwned::to_owned("_boss"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_freezers"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address[]"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_isAutoTrigger"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bool"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_maxFreezeDuration"),
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
                    ::std::borrow::ToOwned::to_owned("acknowledge"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acknowledge"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                    ::std::borrow::ToOwned::to_owned("boss"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("boss"),
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
                    ::std::borrow::ToOwned::to_owned("freeze"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freeze"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("freezeTime"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freezeTime"),
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
                    ::std::borrow::ToOwned::to_owned("freezers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("freezers"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("isAutoTrigger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isAutoTrigger"),
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
                    ::std::borrow::ToOwned::to_owned("maxFreezeDuration"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("maxFreezeDuration"),
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
                    ::std::borrow::ToOwned::to_owned("publicTrigger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("publicTrigger"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resume"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resume"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                (
                    ::std::borrow::ToOwned::to_owned("trigger"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("trigger"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("FreezerAdded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("FreezerAdded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("freezer"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static FLEXIBLETRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R4\x80\x15b\0\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x16z8\x03\x80b\0\x16z\x839\x81\x01`@\x81\x90Rb\0\0\x81\x91b\0\x03dV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x80R\x84\x16`\xC0R`\xA0\x81\x90R`\x03\x80T`\xFF\x19\x90\x81\x16\x84\x15\x15\x17\x90\x91U`\0\x80T\x90\x91\x16\x81U\x83Q\x90[\x81\x81\x10\x15b\0\x01\x86W`\x01`\x04`\0\x87\x84\x81Q\x81\x10b\0\0\xDBWb\0\0\xDBb\0\x047V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UP\x7F\x01\x1Cf\x9B\xEEB\xBA\t*\x17\x0F9\xED\xDBF\xB7\xA7\xD1\x91W\x9C\x90\xCA|*\x8C\x84\x18\xC0\xCE\x85\xB3\x85\x82\x81Q\x81\x10b\0\x01PWb\0\x01Pb\0\x047V[` \x02` \x01\x01Q`@Qb\0\x01u\x91\x90`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA1`\x01\x01b\0\0\xB7V[PPPPPPPb\0\x04MV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xFBW`\0\x80\xFD[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0\x16Z\x839\x81Q\x91R`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x81\xFD[`\0\x82`\x1F\x83\x01\x12b\0\x02\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R`\0\x80Q` b\0\x16Z\x839\x81Q\x91R`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q` `\x01`\x01`@\x1B\x03\x80\x83\x11\x15b\0\x02\xD2Wb\0\x02\xD2b\0\x02\0V[\x82`\x05\x1B`@Q`\x1F\x19`?\x83\x01\x16\x81\x01\x81\x81\x10\x84\x82\x11\x17\x15b\0\x02\xFAWb\0\x02\xFAb\0\x02\0V[`@R\x93\x84R\x85\x81\x01\x83\x01\x93\x83\x81\x01\x92P\x87\x85\x11\x15b\0\x03\x1EWb\0\x03\x1Eb\0\x02\x16V[\x83\x87\x01\x91P[\x84\x82\x10\x15b\0\x03HWb\0\x038\x82b\0\x01\xE3V[\x83R\x91\x83\x01\x91\x90\x83\x01\x90b\0\x03$V[\x97\x96PPPPPPPV[\x80Q\x80\x15\x15\x81\x14b\0\x01\xFBW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x03\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[b\0\x03\xD3\x86b\0\x01\xE3V[\x94Pb\0\x03\xE3` \x87\x01b\0\x01\xE3V[`@\x87\x01Q\x90\x94P`\x01`\x01`@\x1B\x03\x81\x11\x15b\0\x04\x05Wb\0\x04\x05b\0\x01\x93V[b\0\x04\x13\x88\x82\x89\x01b\0\x02^V[\x93PPb\0\x04$``\x87\x01b\0\x03SV[\x91P`\x80\x86\x01Q\x90P\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\x80Q`\xA0Q`\xC0Qa\x11\xB3b\0\x04\xA7`\09`\0\x81\x81a\x03\xAD\x01R\x81\x81a\x04\x0B\x01R\x81\x81a\x05~\x01R\x81\x81a\x06\xDD\x01Ra\x07\xAF\x01R`\0\x81\x81a\x03\x86\x01Ra\x06\x06\x01R`\0\x81\x81a\x02\xD7\x01Ra\x08\xE0\x01Ra\x11\xB3`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xD3W`\x005`\xE0\x1C\x80c[\"\x7F\x9B\x11a\x01OW\x80c\xC2\xB7X\xE1\x11a\x01\x03W\x80c\xD5\x80\xDE\xD4\x11a\0\xE8W\x80c\xD5\x80\xDE\xD4\x14a\x03\xCFW\x80c\xE8cv\xC5\x14a\x03\xE2W\x80c\xFD~\x1B\xEE\x14a\x03\xEAWa\x01\xD3V[\x80c\xC2\xB7X\xE1\x14a\x03\x81W\x80c\xC7r\xAF9\x14a\x03\xA8Wa\x01\xD3V[\x80c\x7F\xEC\x8D8\x11a\x014W\x80c\x7F\xEC\x8D8\x14a\x03_W\x80c\xA2\xCE=I\x14a\x03gW\x80c\xC1\x9D\x93\xFB\x14a\x03tWa\x01\xD3V[\x80c[\"\x7F\x9B\x14a\x03DW\x80cb\xA5\xAF;\x14a\x03WWa\x01\xD3V[\x80c7\xA0\xAF\xC1\x11a\x01\xA6W\x80cI#=]\x11a\x01\x8BW\x80cI#=]\x14a\x03\x1EW\x80cO\x9C\xA8\xC5\x14a\x03&W\x80cYSqD\x14a\x03.Wa\x01\xD3V[\x80c7\xA0\xAF\xC1\x14a\x02\xBDW\x80cH\x1Cju\x14a\x02\xD2Wa\x01\xD3V[\x80c\x04o}\xA2\x14a\x02ZW\x80c\x08l)\x8D\x14a\x02dW\x80c\x1F\x81cR\x14a\x02\x85W\x80c,\xF7\xC51\x14a\x02\xA8W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02ba\x03\xF3V[\0[`\x03Ta\x01\0\x90\x04`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02pa\x02\x936`\x04a\x0F\xE2V[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xB0a\x04\xC4V[`@Qa\x02|\x91\x90a\x10\tV[a\x02\xC5a\x053V[`@Qa\x02|\x91\x90a\x10\x92V[a\x02\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02|V[a\x02ba\x05fV[a\x02ba\x06\x03V[a\x036`2\x81V[`@Q\x90\x81R` \x01a\x02|V[a\x02\xF9a\x03R6`\x04a\x10\xD3V[a\x06sV[a\x02ba\x06\xAAV[a\x02ba\x07\x97V[`\x03Ta\x02p\x90`\xFF\x16\x81V[`\0Ta\x02\xC5\x90`\xFF\x16\x81V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02pa\x03\xDD6`\x04a\x0F\xE2V[a\x08\x06V[`\x01Ta\x036V[a\x036`\x02T\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04bW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0T`\xFF\x16`\x02\x81\x11\x15a\x04{Wa\x04{a\x10cV[\x14a\x04\xB2W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xBC`\0a\x0BsV[P`\0`\x02UV[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xFEW[PPPPP\x90P\x90V[`\0\x80`\0T`\xFF\x16`\x02\x81\x11\x15a\x05MWa\x05Ma\x10cV[\x14a\x05\\WP`\0T`\xFF\x16\x90V[P`\0T`\xFF\x16\x90V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xD5W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02Ta\x062\x91\x90a\x10\xEFV[\x10a\x06iW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06qa\r\x84V[V[`\x01\x81\x81T\x81\x10a\x06\x83W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[3`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x15\x80\x15a\x07\0WP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a\x077W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x16`\x02\x81\x11\x15a\x07OWa\x07Oa\x10cV[\x14a\x07\x86W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x90`\x01a\x0BsV[PB`\x02UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06iW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x08WW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03Ta\x01\0\x90\x04`\xFF\x16a\x08\x98W`@Q\x7FH\xC0h\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDE\x91\x90a\x11)V[\x90P\x80a\n\x17W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\nTW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\n\xC3W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\n\x89Wa\n\x89a\x11NV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n\xBBWP`\x01\x94\x93PPPPV[`\x01\x01a\nWV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0\x80Ta\x0B\x84\x90`\xFF\x16\x83a\r\xDEV[a\x0B\xBAW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x0B\xF7Wa\x0B\xF7a\x10cV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\rAW`\x01\x81\x81T\x81\x10a\x0C\x1DWa\x0C\x1Da\x11NV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x0C|\x90\x87\x90`\x04\x01a\x10\x92V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r,W=`\0\x80>=`\0\xFD[PPPPa\r:\x81`\x01\x01\x90V[\x90Pa\x0C\x02V[P\x82`\x02\x81\x11\x15a\rTWa\rTa\x10cV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\x01`\0T`\xFF\x16`\x02\x81\x11\x15a\r\x9DWa\r\x9Da\x10cV[\x14a\r\xD4W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xBC`\x02a\x0BsV[`\0`\x02\x83`\x02\x81\x11\x15a\r\xF4Wa\r\xF4a\x10cV[\x03a\x0E\x01WP`\0a\x0F2V[\x81`\x02\x81\x11\x15a\x0E\x13Wa\x0E\x13a\x10cV[\x83`\x02\x81\x11\x15a\x0E%Wa\x0E%a\x10cV[\x03a\x0E2WP`\x01a\x0F2V[`\0\x83`\x02\x81\x11\x15a\x0EFWa\x0EFa\x10cV[\x14\x80\x15a\x0EdWP`\x01\x82`\x02\x81\x11\x15a\x0EbWa\x0Eba\x10cV[\x14[\x15a\x0EqWP`\x01a\x0F2V[`\x01\x83`\x02\x81\x11\x15a\x0E\x85Wa\x0E\x85a\x10cV[\x14\x80\x15a\x0E\xA3WP`\0\x82`\x02\x81\x11\x15a\x0E\xA1Wa\x0E\xA1a\x10cV[\x14[\x15a\x0E\xB0WP`\x01a\x0F2V[`\0\x83`\x02\x81\x11\x15a\x0E\xC4Wa\x0E\xC4a\x10cV[\x14\x80\x15a\x0E\xE2WP`\x02\x82`\x02\x81\x11\x15a\x0E\xE0Wa\x0E\xE0a\x10cV[\x14[\x15a\x0E\xEFWP`\x01a\x0F2V[`\x01\x83`\x02\x81\x11\x15a\x0F\x03Wa\x0F\x03a\x10cV[\x14\x80\x15a\x0F!WP`\x02\x82`\x02\x81\x11\x15a\x0F\x1FWa\x0F\x1Fa\x10cV[\x14[\x15a\x0F.WP`\x01a\x0F2V[P`\0[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xDFW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x0F\xF7Wa\x0F\xF7a\x0F8V[\x815a\x10\x02\x81a\x0F\xBDV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10WW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10%V[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x10\xCDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x10\xE8Wa\x10\xE8a\x0F8V[P5\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0F2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11>Wa\x11>a\x0F8V[\x81Q\x80\x15\x15\x81\x14a\x10\x02W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xBCQZ\xB3O^\x9D\xFA\xB5\x8D\x02\xAF\x93\xBCx\xFFP\xA3-y`\xF6z\xAD\x07Nj\x84>G$\x9DdsolcC\0\x08\x10\x003ABI decoding: invalid calldata a";
    /// The bytecode of the contract.
    pub static FLEXIBLETRIGGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\xD3W`\x005`\xE0\x1C\x80c[\"\x7F\x9B\x11a\x01OW\x80c\xC2\xB7X\xE1\x11a\x01\x03W\x80c\xD5\x80\xDE\xD4\x11a\0\xE8W\x80c\xD5\x80\xDE\xD4\x14a\x03\xCFW\x80c\xE8cv\xC5\x14a\x03\xE2W\x80c\xFD~\x1B\xEE\x14a\x03\xEAWa\x01\xD3V[\x80c\xC2\xB7X\xE1\x14a\x03\x81W\x80c\xC7r\xAF9\x14a\x03\xA8Wa\x01\xD3V[\x80c\x7F\xEC\x8D8\x11a\x014W\x80c\x7F\xEC\x8D8\x14a\x03_W\x80c\xA2\xCE=I\x14a\x03gW\x80c\xC1\x9D\x93\xFB\x14a\x03tWa\x01\xD3V[\x80c[\"\x7F\x9B\x14a\x03DW\x80cb\xA5\xAF;\x14a\x03WWa\x01\xD3V[\x80c7\xA0\xAF\xC1\x11a\x01\xA6W\x80cI#=]\x11a\x01\x8BW\x80cI#=]\x14a\x03\x1EW\x80cO\x9C\xA8\xC5\x14a\x03&W\x80cYSqD\x14a\x03.Wa\x01\xD3V[\x80c7\xA0\xAF\xC1\x14a\x02\xBDW\x80cH\x1Cju\x14a\x02\xD2Wa\x01\xD3V[\x80c\x04o}\xA2\x14a\x02ZW\x80c\x08l)\x8D\x14a\x02dW\x80c\x1F\x81cR\x14a\x02\x85W\x80c,\xF7\xC51\x14a\x02\xA8W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02ba\x03\xF3V[\0[`\x03Ta\x01\0\x90\x04`\xFF\x16[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02pa\x02\x936`\x04a\x0F\xE2V[`\x04` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[a\x02\xB0a\x04\xC4V[`@Qa\x02|\x91\x90a\x10\tV[a\x02\xC5a\x053V[`@Qa\x02|\x91\x90a\x10\x92V[a\x02\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02|V[a\x02ba\x05fV[a\x02ba\x06\x03V[a\x036`2\x81V[`@Q\x90\x81R` \x01a\x02|V[a\x02\xF9a\x03R6`\x04a\x10\xD3V[a\x06sV[a\x02ba\x06\xAAV[a\x02ba\x07\x97V[`\x03Ta\x02p\x90`\xFF\x16\x81V[`\0Ta\x02\xC5\x90`\xFF\x16\x81V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xF9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02pa\x03\xDD6`\x04a\x0F\xE2V[a\x08\x06V[`\x01Ta\x036V[a\x036`\x02T\x81V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04bW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\0T`\xFF\x16`\x02\x81\x11\x15a\x04{Wa\x04{a\x10cV[\x14a\x04\xB2W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xBC`\0a\x0BsV[P`\0`\x02UV[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x05)W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x04\xFEW[PPPPP\x90P\x90V[`\0\x80`\0T`\xFF\x16`\x02\x81\x11\x15a\x05MWa\x05Ma\x10cV[\x14a\x05\\WP`\0T`\xFF\x16\x90V[P`\0T`\xFF\x16\x90V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05\xD5W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\xFF\x16a\x01\0\x17\x90UV[B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x02Ta\x062\x91\x90a\x10\xEFV[\x10a\x06iW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x06qa\r\x84V[V[`\x01\x81\x81T\x81\x10a\x06\x83W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[3`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x15\x80\x15a\x07\0WP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a\x077W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x16`\x02\x81\x11\x15a\x07OWa\x07Oa\x10cV[\x14a\x07\x86W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\x90`\x01a\x0BsV[PB`\x02UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06iW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x08WW`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x03Ta\x01\0\x90\x04`\xFF\x16a\x08\x98W`@Q\x7FH\xC0h\xCD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\t\xA6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\t\xBAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xDE\x91\x90a\x11)V[\x90P\x80a\n\x17W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\nTW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\n\xC3W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\n\x89Wa\n\x89a\x11NV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\n\xBBWP`\x01\x94\x93PPPPV[`\x01\x01a\nWV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0\x80Ta\x0B\x84\x90`\xFF\x16\x83a\r\xDEV[a\x0B\xBAW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x0B\xF7Wa\x0B\xF7a\x10cV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\rAW`\x01\x81\x81T\x81\x10a\x0C\x1DWa\x0C\x1Da\x11NV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x0C|\x90\x87\x90`\x04\x01a\x10\x92V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\x18W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\r,W=`\0\x80>=`\0\xFD[PPPPa\r:\x81`\x01\x01\x90V[\x90Pa\x0C\x02V[P\x82`\x02\x81\x11\x15a\rTWa\rTa\x10cV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\x01`\0T`\xFF\x16`\x02\x81\x11\x15a\r\x9DWa\r\x9Da\x10cV[\x14a\r\xD4W`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xBC`\x02a\x0BsV[`\0`\x02\x83`\x02\x81\x11\x15a\r\xF4Wa\r\xF4a\x10cV[\x03a\x0E\x01WP`\0a\x0F2V[\x81`\x02\x81\x11\x15a\x0E\x13Wa\x0E\x13a\x10cV[\x83`\x02\x81\x11\x15a\x0E%Wa\x0E%a\x10cV[\x03a\x0E2WP`\x01a\x0F2V[`\0\x83`\x02\x81\x11\x15a\x0EFWa\x0EFa\x10cV[\x14\x80\x15a\x0EdWP`\x01\x82`\x02\x81\x11\x15a\x0EbWa\x0Eba\x10cV[\x14[\x15a\x0EqWP`\x01a\x0F2V[`\x01\x83`\x02\x81\x11\x15a\x0E\x85Wa\x0E\x85a\x10cV[\x14\x80\x15a\x0E\xA3WP`\0\x82`\x02\x81\x11\x15a\x0E\xA1Wa\x0E\xA1a\x10cV[\x14[\x15a\x0E\xB0WP`\x01a\x0F2V[`\0\x83`\x02\x81\x11\x15a\x0E\xC4Wa\x0E\xC4a\x10cV[\x14\x80\x15a\x0E\xE2WP`\x02\x82`\x02\x81\x11\x15a\x0E\xE0Wa\x0E\xE0a\x10cV[\x14[\x15a\x0E\xEFWP`\x01a\x0F2V[`\x01\x83`\x02\x81\x11\x15a\x0F\x03Wa\x0F\x03a\x10cV[\x14\x80\x15a\x0F!WP`\x02\x82`\x02\x81\x11\x15a\x0F\x1FWa\x0F\x1Fa\x10cV[\x14[\x15a\x0F.WP`\x01a\x0F2V[P`\0[\x92\x91PPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0F\xDFW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x0F\xF7Wa\x0F\xF7a\x0F8V[\x815a\x10\x02\x81a\x0F\xBDV[\x93\x92PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\x10WW\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x10%V[P\x90\x96\x95PPPPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x10\xCDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x10\xE8Wa\x10\xE8a\x0F8V[P5\x91\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0F2W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a\x11>Wa\x11>a\x0F8V[\x81Q\x80\x15\x15\x81\x14a\x10\x02W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xBCQZ\xB3O^\x9D\xFA\xB5\x8D\x02\xAF\x93\xBCx\xFFP\xA3-y`\xF6z\xAD\x07Nj\x84>G$\x9DdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static FLEXIBLETRIGGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FlexibleTrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for FlexibleTrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FlexibleTrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FlexibleTrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FlexibleTrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(FlexibleTrigger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FlexibleTrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    FLEXIBLETRIGGER_ABI.clone(),
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
                FLEXIBLETRIGGER_ABI.clone(),
                FLEXIBLETRIGGER_BYTECODE.clone().into(),
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
        ///Calls the contract's `acknowledge` (0x49233d5d) function
        pub fn acknowledge(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 35, 61, 93], ())
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
        ///Calls the contract's `boss` (0xc772af39) function
        pub fn boss(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([199, 114, 175, 57], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freeze` (0x62a5af3b) function
        pub fn freeze(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 165, 175, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezeTime` (0xfd7e1bee) function
        pub fn freeze_time(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 126, 27, 238], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `freezers` (0x1f816352) function
        pub fn freezers(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 129, 99, 82], p0)
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
        ///Calls the contract's `isAutoTrigger` (0xa2ce3d49) function
        pub fn is_auto_trigger(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([162, 206, 61, 73], ())
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
        ///Calls the contract's `maxFreezeDuration` (0xc2b758e1) function
        pub fn max_freeze_duration(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([194, 183, 88, 225], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `publicTrigger` (0x4f9ca8c5) function
        pub fn public_trigger(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 156, 168, 197], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resume` (0x046f7da2) function
        pub fn resume(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 111, 125, 162], ())
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
        ///Calls the contract's `trigger` (0x7fec8d38) function
        pub fn trigger(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 236, 141, 56], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `FreezerAdded` event
        pub fn freezer_added_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FreezerAddedFilter,
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
            FlexibleTriggerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for FlexibleTrigger<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FlexibleTriggerErrors {
        InvalidStateTransition(InvalidStateTransition),
        SetLimitReached(SetLimitReached),
        Unacknowledged(Unacknowledged),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FlexibleTriggerErrors {
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FlexibleTriggerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for FlexibleTriggerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FlexibleTriggerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidStateTransition(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetLimitReached(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unacknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FlexibleTriggerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for FlexibleTriggerErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<SetLimitReached> for FlexibleTriggerErrors {
        fn from(value: SetLimitReached) -> Self {
            Self::SetLimitReached(value)
        }
    }
    impl ::core::convert::From<Unacknowledged> for FlexibleTriggerErrors {
        fn from(value: Unacknowledged) -> Self {
            Self::Unacknowledged(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for FlexibleTriggerErrors {
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
    #[ethevent(name = "FreezerAdded", abi = "FreezerAdded(address)")]
    pub struct FreezerAddedFilter {
        pub freezer: ::ethers::core::types::Address,
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
    pub enum FlexibleTriggerEvents {
        FreezerAddedFilter(FreezerAddedFilter),
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for FlexibleTriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = FreezerAddedFilter::decode_log(log) {
                return Ok(FlexibleTriggerEvents::FreezerAddedFilter(decoded));
            }
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(FlexibleTriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(FlexibleTriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for FlexibleTriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::FreezerAddedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<FreezerAddedFilter> for FlexibleTriggerEvents {
        fn from(value: FreezerAddedFilter) -> Self {
            Self::FreezerAddedFilter(value)
        }
    }
    impl ::core::convert::From<SetAddedFilter> for FlexibleTriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for FlexibleTriggerEvents {
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
    ///Container type for all input parameters for the `acknowledge` function with signature `acknowledge()` and selector `0x49233d5d`
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
    #[ethcall(name = "acknowledge", abi = "acknowledge()")]
    pub struct AcknowledgeCall;
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
    ///Container type for all input parameters for the `boss` function with signature `boss()` and selector `0xc772af39`
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
    #[ethcall(name = "boss", abi = "boss()")]
    pub struct BossCall;
    ///Container type for all input parameters for the `freeze` function with signature `freeze()` and selector `0x62a5af3b`
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
    #[ethcall(name = "freeze", abi = "freeze()")]
    pub struct FreezeCall;
    ///Container type for all input parameters for the `freezeTime` function with signature `freezeTime()` and selector `0xfd7e1bee`
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
    #[ethcall(name = "freezeTime", abi = "freezeTime()")]
    pub struct FreezeTimeCall;
    ///Container type for all input parameters for the `freezers` function with signature `freezers(address)` and selector `0x1f816352`
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
    #[ethcall(name = "freezers", abi = "freezers(address)")]
    pub struct FreezersCall(pub ::ethers::core::types::Address);
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
    ///Container type for all input parameters for the `isAutoTrigger` function with signature `isAutoTrigger()` and selector `0xa2ce3d49`
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
    #[ethcall(name = "isAutoTrigger", abi = "isAutoTrigger()")]
    pub struct IsAutoTriggerCall;
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
    ///Container type for all input parameters for the `maxFreezeDuration` function with signature `maxFreezeDuration()` and selector `0xc2b758e1`
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
    #[ethcall(name = "maxFreezeDuration", abi = "maxFreezeDuration()")]
    pub struct MaxFreezeDurationCall;
    ///Container type for all input parameters for the `publicTrigger` function with signature `publicTrigger()` and selector `0x4f9ca8c5`
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
    #[ethcall(name = "publicTrigger", abi = "publicTrigger()")]
    pub struct PublicTriggerCall;
    ///Container type for all input parameters for the `resume` function with signature `resume()` and selector `0x046f7da2`
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
    #[ethcall(name = "resume", abi = "resume()")]
    pub struct ResumeCall;
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
    ///Container type for all input parameters for the `trigger` function with signature `trigger()` and selector `0x7fec8d38`
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
    #[ethcall(name = "trigger", abi = "trigger()")]
    pub struct TriggerCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FlexibleTriggerCalls {
        MaxSetLength(MaxSetLengthCall),
        Acknowledge(AcknowledgeCall),
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        Boss(BossCall),
        Freeze(FreezeCall),
        FreezeTime(FreezeTimeCall),
        Freezers(FreezersCall),
        GetSets(GetSetsCall),
        GetSetsLength(GetSetsLengthCall),
        IsAutoTrigger(IsAutoTriggerCall),
        Manager(ManagerCall),
        MaxFreezeDuration(MaxFreezeDurationCall),
        PublicTrigger(PublicTriggerCall),
        Resume(ResumeCall),
        RunProgrammaticCheck(RunProgrammaticCheckCall),
        Sets(SetsCall),
        State(StateCall),
        Trigger(TriggerCall),
    }
    impl ::ethers::core::abi::AbiDecode for FlexibleTriggerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MaxSetLengthCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxSetLength(decoded));
            }
            if let Ok(decoded)
                = <AcknowledgeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Acknowledge(decoded));
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
                = <BossCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Boss(decoded));
            }
            if let Ok(decoded)
                = <FreezeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Freeze(decoded));
            }
            if let Ok(decoded)
                = <FreezeTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FreezeTime(decoded));
            }
            if let Ok(decoded)
                = <FreezersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Freezers(decoded));
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
                = <IsAutoTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsAutoTrigger(decoded));
            }
            if let Ok(decoded)
                = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded)
                = <MaxFreezeDurationCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MaxFreezeDuration(decoded));
            }
            if let Ok(decoded)
                = <PublicTriggerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PublicTrigger(decoded));
            }
            if let Ok(decoded)
                = <ResumeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Resume(decoded));
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
            if let Ok(decoded)
                = <TriggerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Trigger(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FlexibleTriggerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxSetLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Acknowledge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Acknowledged(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Boss(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Freeze(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FreezeTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Freezers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSetsLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsAutoTrigger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxFreezeDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PublicTrigger(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Resume(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RunProgrammaticCheck(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Sets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Trigger(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for FlexibleTriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxSetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledge(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::Boss(element) => ::core::fmt::Display::fmt(element, f),
                Self::Freeze(element) => ::core::fmt::Display::fmt(element, f),
                Self::FreezeTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::Freezers(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSetsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsAutoTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxFreezeDuration(element) => ::core::fmt::Display::fmt(element, f),
                Self::PublicTrigger(element) => ::core::fmt::Display::fmt(element, f),
                Self::Resume(element) => ::core::fmt::Display::fmt(element, f),
                Self::RunProgrammaticCheck(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Sets(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::Trigger(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxSetLengthCall> for FlexibleTriggerCalls {
        fn from(value: MaxSetLengthCall) -> Self {
            Self::MaxSetLength(value)
        }
    }
    impl ::core::convert::From<AcknowledgeCall> for FlexibleTriggerCalls {
        fn from(value: AcknowledgeCall) -> Self {
            Self::Acknowledge(value)
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for FlexibleTriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for FlexibleTriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<BossCall> for FlexibleTriggerCalls {
        fn from(value: BossCall) -> Self {
            Self::Boss(value)
        }
    }
    impl ::core::convert::From<FreezeCall> for FlexibleTriggerCalls {
        fn from(value: FreezeCall) -> Self {
            Self::Freeze(value)
        }
    }
    impl ::core::convert::From<FreezeTimeCall> for FlexibleTriggerCalls {
        fn from(value: FreezeTimeCall) -> Self {
            Self::FreezeTime(value)
        }
    }
    impl ::core::convert::From<FreezersCall> for FlexibleTriggerCalls {
        fn from(value: FreezersCall) -> Self {
            Self::Freezers(value)
        }
    }
    impl ::core::convert::From<GetSetsCall> for FlexibleTriggerCalls {
        fn from(value: GetSetsCall) -> Self {
            Self::GetSets(value)
        }
    }
    impl ::core::convert::From<GetSetsLengthCall> for FlexibleTriggerCalls {
        fn from(value: GetSetsLengthCall) -> Self {
            Self::GetSetsLength(value)
        }
    }
    impl ::core::convert::From<IsAutoTriggerCall> for FlexibleTriggerCalls {
        fn from(value: IsAutoTriggerCall) -> Self {
            Self::IsAutoTrigger(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for FlexibleTriggerCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<MaxFreezeDurationCall> for FlexibleTriggerCalls {
        fn from(value: MaxFreezeDurationCall) -> Self {
            Self::MaxFreezeDuration(value)
        }
    }
    impl ::core::convert::From<PublicTriggerCall> for FlexibleTriggerCalls {
        fn from(value: PublicTriggerCall) -> Self {
            Self::PublicTrigger(value)
        }
    }
    impl ::core::convert::From<ResumeCall> for FlexibleTriggerCalls {
        fn from(value: ResumeCall) -> Self {
            Self::Resume(value)
        }
    }
    impl ::core::convert::From<RunProgrammaticCheckCall> for FlexibleTriggerCalls {
        fn from(value: RunProgrammaticCheckCall) -> Self {
            Self::RunProgrammaticCheck(value)
        }
    }
    impl ::core::convert::From<SetsCall> for FlexibleTriggerCalls {
        fn from(value: SetsCall) -> Self {
            Self::Sets(value)
        }
    }
    impl ::core::convert::From<StateCall> for FlexibleTriggerCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<TriggerCall> for FlexibleTriggerCalls {
        fn from(value: TriggerCall) -> Self {
            Self::Trigger(value)
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
    ///Container type for all return fields from the `boss` function with signature `boss()` and selector `0xc772af39`
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
    pub struct BossReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `freezeTime` function with signature `freezeTime()` and selector `0xfd7e1bee`
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
    pub struct FreezeTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `freezers` function with signature `freezers(address)` and selector `0x1f816352`
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
    pub struct FreezersReturn(pub bool);
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
    ///Container type for all return fields from the `isAutoTrigger` function with signature `isAutoTrigger()` and selector `0xa2ce3d49`
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
    pub struct IsAutoTriggerReturn(pub bool);
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
    ///Container type for all return fields from the `maxFreezeDuration` function with signature `maxFreezeDuration()` and selector `0xc2b758e1`
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
    pub struct MaxFreezeDurationReturn(pub ::ethers::core::types::U256);
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
