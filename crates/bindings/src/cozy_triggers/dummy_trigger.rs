pub use dummy_trigger::*;
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
pub mod dummy_trigger {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("manager_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IManager"),
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
    pub static DUMMYTRIGGER_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> = ::ethers_contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\r>8\x03\x80a\r>\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x8DV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\x01\x08V[`\0` \x82\x84\x03\x12\x15a\0\xEAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\x01W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x0C\x14a\x01*`\09`\0\x81\x81a\x01\xDE\x01Ra\x03\xE1\x01Ra\x0C\x14`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80c[\"\x7F\x9B\x11a\0\xF8W\x80c\xC1\x9D\x93\xFB\x11a\0\xDDW\x80c\xC1\x9D\x93\xFB\x14a\x02cW\x80c\xD5\x80\xDE\xD4\x14a\x02pW\x80c\xE8cv\xC5\x14a\x02\x83Wa\x01%V[\x80c[\"\x7F\x9B\x14a\x02;W\x80c\x7F\xEC\x8D8\x14a\x02NWa\x01%V[\x80c\x08l)\x8D\x14a\x01\xACW\x80c,\xF7\xC51\x14a\x01\xC4W\x80cH\x1Cju\x14a\x01\xD9W\x80cYSqD\x14a\x02%W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x01[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCCa\x02\x8BV[`@Qa\x01\xBB\x91\x90a\t\xDFV[a\x02\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBBV[a\x02-`2\x81V[`@Q\x90\x81R` \x01a\x01\xBBV[a\x02\0a\x02I6`\x04a\n\xBEV[a\x02\xFAV[a\x02Va\x031V[`@Qa\x01\xBB\x91\x90a\x0B\tV[`\0Ta\x02V\x90`\xFF\x16\x81V[a\x01\xAFa\x02~6`\x04a\x0BJV[a\x03HV[`\x01Ta\x02-V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\xF0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02\xC5W[PPPPP\x90P\x90V[`\x01\x81\x81T\x81\x10a\x03\nW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\0a\x03=`\x02a\x06tV[PP`\0T`\xFF\x16\x90V[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x03\x99W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDF\x91\x90a\x0B\x8AV[\x90P\x80a\x05\x18W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x05UW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x05\xC4W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x05\x8AWa\x05\x8Aa\x0B\xAFV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xBCWP`\x01\x94\x93PPPPV[`\x01\x01a\x05XV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0\x80Ta\x06\x85\x90`\xFF\x16\x83a\x08\x85V[a\x06\xBBW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x06\xF8Wa\x06\xF8a\n\xDAV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x08BW`\x01\x81\x81T\x81\x10a\x07\x1EWa\x07\x1Ea\x0B\xAFV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x07}\x90\x87\x90`\x04\x01a\x0B\tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08-W=`\0\x80>=`\0\xFD[PPPPa\x08;\x81`\x01\x01\x90V[\x90Pa\x07\x03V[P\x82`\x02\x81\x11\x15a\x08UWa\x08Ua\n\xDAV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\0`\x02\x83`\x02\x81\x11\x15a\x08\x9BWa\x08\x9Ba\n\xDAV[\x03a\x08\xA8WP`\0a\t\xD9V[\x81`\x02\x81\x11\x15a\x08\xBAWa\x08\xBAa\n\xDAV[\x83`\x02\x81\x11\x15a\x08\xCCWa\x08\xCCa\n\xDAV[\x03a\x08\xD9WP`\x01a\t\xD9V[`\0\x83`\x02\x81\x11\x15a\x08\xEDWa\x08\xEDa\n\xDAV[\x14\x80\x15a\t\x0BWP`\x01\x82`\x02\x81\x11\x15a\t\tWa\t\ta\n\xDAV[\x14[\x15a\t\x18WP`\x01a\t\xD9V[`\x01\x83`\x02\x81\x11\x15a\t,Wa\t,a\n\xDAV[\x14\x80\x15a\tJWP`\0\x82`\x02\x81\x11\x15a\tHWa\tHa\n\xDAV[\x14[\x15a\tWWP`\x01a\t\xD9V[`\0\x83`\x02\x81\x11\x15a\tkWa\tka\n\xDAV[\x14\x80\x15a\t\x89WP`\x02\x82`\x02\x81\x11\x15a\t\x87Wa\t\x87a\n\xDAV[\x14[\x15a\t\x96WP`\x01a\t\xD9V[`\x01\x83`\x02\x81\x11\x15a\t\xAAWa\t\xAAa\n\xDAV[\x14\x80\x15a\t\xC8WP`\x02\x82`\x02\x81\x11\x15a\t\xC6Wa\t\xC6a\n\xDAV[\x14[\x15a\t\xD5WP`\x01a\t\xD9V[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\n-W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\xFBV[P\x90\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\n\xD3Wa\n\xD3a\n9V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x0BDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x0B_Wa\x0B_a\n9V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x83W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x9FWa\x0B\x9Fa\n9V[\x81Q\x80\x15\x15\x81\x14a\x0B\x83W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA3\xF3\xB1B\x1E\xBE+\xAE\x88!\x0C\xFF\x85\xADv\xD6\xB7\xBAi<\xE7A\xE9\x8F\xE1\xDB\xEF\xF5\x9F-\xB4\x0FdsolcC\0\x08\x10\x003";
    /// The bytecode of the contract.
    pub static DUMMYTRIGGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01%W`\x005`\xE0\x1C\x80c[\"\x7F\x9B\x11a\0\xF8W\x80c\xC1\x9D\x93\xFB\x11a\0\xDDW\x80c\xC1\x9D\x93\xFB\x14a\x02cW\x80c\xD5\x80\xDE\xD4\x14a\x02pW\x80c\xE8cv\xC5\x14a\x02\x83Wa\x01%V[\x80c[\"\x7F\x9B\x14a\x02;W\x80c\x7F\xEC\x8D8\x14a\x02NWa\x01%V[\x80c\x08l)\x8D\x14a\x01\xACW\x80c,\xF7\xC51\x14a\x01\xC4W\x80cH\x1Cju\x14a\x01\xD9W\x80cYSqD\x14a\x02%W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[`\x01[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCCa\x02\x8BV[`@Qa\x01\xBB\x91\x90a\t\xDFV[a\x02\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xBBV[a\x02-`2\x81V[`@Q\x90\x81R` \x01a\x01\xBBV[a\x02\0a\x02I6`\x04a\n\xBEV[a\x02\xFAV[a\x02Va\x031V[`@Qa\x01\xBB\x91\x90a\x0B\tV[`\0Ta\x02V\x90`\xFF\x16\x81V[a\x01\xAFa\x02~6`\x04a\x0BJV[a\x03HV[`\x01Ta\x02-V[```\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x02\xF0W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x02\xC5W[PPPPP\x90P\x90V[`\x01\x81\x81T\x81\x10a\x03\nW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x81V[`\0a\x03=`\x02a\x06tV[PP`\0T`\xFF\x16\x90V[`\x003s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x14a\x03\x99W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q\x7Ft\xEB\xE3\xEC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R`\0\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90ct\xEB\xE3\xEC\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x04\xA7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xFA\x15\x80\x15a\x04\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xDF\x91\x90a\x0B\x8AV[\x90P\x80a\x05\x18W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01T`2\x81\x10a\x05UW`@Q\x7F\x86\xBF\xB2\xC8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x05\xC4W\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01\x82\x81T\x81\x10a\x05\x8AWa\x05\x8Aa\x0B\xAFV[`\0\x91\x82R` \x90\x91 \x01Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x05\xBCWP`\x01\x94\x93PPPPV[`\x01\x01a\x05XV[P`\x01\x80T\x80\x82\x01\x82U`\0\x91\x90\x91R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\xF4K\xF3`R:\xACY\x84\xDD\x85\xFA\xB3M`9`\x8A\x1D\r\xF0\xF6\xC45AOR\xC2\xB80\xC0\xE3\x90` \x01`@Q\x80\x91\x03\x90\xA1P`\x01\x93\x92PPPV[`\0\x80Ta\x06\x85\x90`\xFF\x16\x83a\x08\x85V[a\x06\xBBW`@Q\x7F\x8F\x9Ax\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80T\x83\x91\x90\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x83`\x02\x81\x11\x15a\x06\xF8Wa\x06\xF8a\n\xDAV[\x02\x17\x90UP`\x01T`\0[\x81\x81\x10\x15a\x08BW`\x01\x81\x81T\x81\x10a\x07\x1EWa\x07\x1Ea\x0B\xAFV[`\0\x91\x82R` \x90\x91 \x01T`@Q\x7Fzu\xFD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90czu\xFD\xDE\x90a\x07}\x90\x87\x90`\x04\x01a\x0B\tV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x19W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01\x90\x81R\x7F code\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[PZ\xF1\x15\x80\x15a\x08-W=`\0\x80>=`\0\xFD[PPPPa\x08;\x81`\x01\x01\x90V[\x90Pa\x07\x03V[P\x82`\x02\x81\x11\x15a\x08UWa\x08Ua\n\xDAV[`@Q\x7F\xE2\xAE\x03\xA0\xD0b{4\x15\"\x001\x16\x93\xBA@\x93\x93\xC7\xD1\xEB\xDD\n$k\xA3\xC0%:\xBC&?\x90`\0\x90\xA2P\x90\x91\x90PV[`\0`\x02\x83`\x02\x81\x11\x15a\x08\x9BWa\x08\x9Ba\n\xDAV[\x03a\x08\xA8WP`\0a\t\xD9V[\x81`\x02\x81\x11\x15a\x08\xBAWa\x08\xBAa\n\xDAV[\x83`\x02\x81\x11\x15a\x08\xCCWa\x08\xCCa\n\xDAV[\x03a\x08\xD9WP`\x01a\t\xD9V[`\0\x83`\x02\x81\x11\x15a\x08\xEDWa\x08\xEDa\n\xDAV[\x14\x80\x15a\t\x0BWP`\x01\x82`\x02\x81\x11\x15a\t\tWa\t\ta\n\xDAV[\x14[\x15a\t\x18WP`\x01a\t\xD9V[`\x01\x83`\x02\x81\x11\x15a\t,Wa\t,a\n\xDAV[\x14\x80\x15a\tJWP`\0\x82`\x02\x81\x11\x15a\tHWa\tHa\n\xDAV[\x14[\x15a\tWWP`\x01a\t\xD9V[`\0\x83`\x02\x81\x11\x15a\tkWa\tka\n\xDAV[\x14\x80\x15a\t\x89WP`\x02\x82`\x02\x81\x11\x15a\t\x87Wa\t\x87a\n\xDAV[\x14[\x15a\t\x96WP`\x01a\t\xD9V[`\x01\x83`\x02\x81\x11\x15a\t\xAAWa\t\xAAa\n\xDAV[\x14\x80\x15a\t\xC8WP`\x02\x82`\x02\x81\x11\x15a\t\xC6Wa\t\xC6a\n\xDAV[\x14[\x15a\t\xD5WP`\x01a\t\xD9V[P`\0[\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a\n-W\x83Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\t\xFBV[P\x90\x96\x95PPPPPPV[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\n\xD3Wa\n\xD3a\n9V[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[` \x81\x01`\x03\x83\x10a\x0BDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`!`\x04R`$`\0\xFD[\x91\x90R\x90V[`\0` \x82\x84\x03\x12\x15a\x0B_Wa\x0B_a\n9V[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0B\x83W`\0\x80\xFD[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x0B\x9FWa\x0B\x9Fa\n9V[\x81Q\x80\x15\x15\x81\x14a\x0B\x83W`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`2`\x04R`$`\0\xFD\xFE\xA2dipfsX\"\x12 \xA3\xF3\xB1B\x1E\xBE+\xAE\x88!\x0C\xFF\x85\xADv\xD6\xB7\xBAi<\xE7A\xE9\x8F\xE1\xDB\xEF\xF5\x9F-\xB4\x0FdsolcC\0\x08\x10\x003";
    /// The deployed bytecode of the contract.
    pub static DUMMYTRIGGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DummyTrigger<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for DummyTrigger<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DummyTrigger<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DummyTrigger<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DummyTrigger<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DummyTrigger))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DummyTrigger<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    DUMMYTRIGGER_ABI.clone(),
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
                DUMMYTRIGGER_ABI.clone(),
                DUMMYTRIGGER_BYTECODE.clone().into(),
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
        pub fn trigger(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([127, 236, 141, 56], ())
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
            DummyTriggerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for DummyTrigger<M> {
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
    pub enum DummyTriggerErrors {
        InvalidStateTransition(InvalidStateTransition),
        SetLimitReached(SetLimitReached),
        Unacknowledged(Unacknowledged),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DummyTriggerErrors {
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
    impl ::ethers::core::abi::AbiEncode for DummyTriggerErrors {
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
    impl ::ethers_contract::ContractRevert for DummyTriggerErrors {
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
    impl ::core::fmt::Display for DummyTriggerErrors {
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
    impl ::core::convert::From<::std::string::String> for DummyTriggerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for DummyTriggerErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<SetLimitReached> for DummyTriggerErrors {
        fn from(value: SetLimitReached) -> Self {
            Self::SetLimitReached(value)
        }
    }
    impl ::core::convert::From<Unacknowledged> for DummyTriggerErrors {
        fn from(value: Unacknowledged) -> Self {
            Self::Unacknowledged(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for DummyTriggerErrors {
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
    pub enum DummyTriggerEvents {
        SetAddedFilter(SetAddedFilter),
        TriggerStateUpdatedFilter(TriggerStateUpdatedFilter),
    }
    impl ::ethers_contract::EthLogDecode for DummyTriggerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = SetAddedFilter::decode_log(log) {
                return Ok(DummyTriggerEvents::SetAddedFilter(decoded));
            }
            if let Ok(decoded) = TriggerStateUpdatedFilter::decode_log(log) {
                return Ok(DummyTriggerEvents::TriggerStateUpdatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DummyTriggerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::SetAddedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerStateUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<SetAddedFilter> for DummyTriggerEvents {
        fn from(value: SetAddedFilter) -> Self {
            Self::SetAddedFilter(value)
        }
    }
    impl ::core::convert::From<TriggerStateUpdatedFilter> for DummyTriggerEvents {
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
    pub enum DummyTriggerCalls {
        MaxSetLength(MaxSetLengthCall),
        Acknowledged(AcknowledgedCall),
        AddSet(AddSetCall),
        GetSets(GetSetsCall),
        GetSetsLength(GetSetsLengthCall),
        Manager(ManagerCall),
        Sets(SetsCall),
        State(StateCall),
        Trigger(TriggerCall),
    }
    impl ::ethers::core::abi::AbiDecode for DummyTriggerCalls {
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
    impl ::ethers::core::abi::AbiEncode for DummyTriggerCalls {
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
                Self::Sets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::State(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Trigger(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DummyTriggerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxSetLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Acknowledged(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSets(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSetsLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sets(element) => ::core::fmt::Display::fmt(element, f),
                Self::State(element) => ::core::fmt::Display::fmt(element, f),
                Self::Trigger(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxSetLengthCall> for DummyTriggerCalls {
        fn from(value: MaxSetLengthCall) -> Self {
            Self::MaxSetLength(value)
        }
    }
    impl ::core::convert::From<AcknowledgedCall> for DummyTriggerCalls {
        fn from(value: AcknowledgedCall) -> Self {
            Self::Acknowledged(value)
        }
    }
    impl ::core::convert::From<AddSetCall> for DummyTriggerCalls {
        fn from(value: AddSetCall) -> Self {
            Self::AddSet(value)
        }
    }
    impl ::core::convert::From<GetSetsCall> for DummyTriggerCalls {
        fn from(value: GetSetsCall) -> Self {
            Self::GetSets(value)
        }
    }
    impl ::core::convert::From<GetSetsLengthCall> for DummyTriggerCalls {
        fn from(value: GetSetsLengthCall) -> Self {
            Self::GetSetsLength(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for DummyTriggerCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<SetsCall> for DummyTriggerCalls {
        fn from(value: SetsCall) -> Self {
            Self::Sets(value)
        }
    }
    impl ::core::convert::From<StateCall> for DummyTriggerCalls {
        fn from(value: StateCall) -> Self {
            Self::State(value)
        }
    }
    impl ::core::convert::From<TriggerCall> for DummyTriggerCalls {
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
    ///Container type for all return fields from the `trigger` function with signature `trigger()` and selector `0x7fec8d38`
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
    pub struct TriggerReturn(pub u8);
}
