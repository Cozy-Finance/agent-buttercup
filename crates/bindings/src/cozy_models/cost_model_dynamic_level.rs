pub use cost_model_dynamic_level::*;
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
pub mod cost_model_dynamic_level {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("uLow_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("uHigh_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("costFactorAtZeroUtilization_",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("costFactorAtFullUtilization_",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone_",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("optimalZoneRate_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("_min"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("_min"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("a"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("b"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("costFactor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactor"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fromUtilization_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("toUtilization_"),
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
                    ::std::borrow::ToOwned::to_owned("costFactorAtFullUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactorAtFullUtilization",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("costFactorAtZeroUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactorAtZeroUtilization",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone",),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("getUpdatedStorageParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getUpdatedStorageParams",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currentTime_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("utilization_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "newCostFactorInOptimalZone_",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newLastUpdateTime_",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("optimalZoneRate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("optimalZoneRate"),
                        inputs: ::std::vec![],
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
                                name: ::std::borrow::ToOwned::to_owned("fromUtilization_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("toUtilization_"),
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
                    ::std::borrow::ToOwned::to_owned("setAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setAddress"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uHigh"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("uLow"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uLow"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("uOpt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uOpt"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("update"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("utilization_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newUtilization_"),
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
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("UpdatedDynamicLevelModelParameters"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("UpdatedDynamicLevelModelParameters",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("lastUpdateTime"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidConfiguration"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidConfiguration",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReferencePoint"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidReferencePoint",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTime"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTime"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUtilization"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetAlreadyRegistered"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SetAlreadyRegistered",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COSTMODELDYNAMICLEVEL_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01@`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x13\xF78\x03\x80b\0\x13\xF7\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x01\xA2V[g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15b\0\0\xACW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x86\x11\x15b\0\0\xCEW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15b\0\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x10\x15b\0\x01\x1AW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x86\x90R`\xA0\x85\x90Rb\0\x01M`\x01`\x02b\0\x018\x88\x8Ab\0\x028V[b\0\x01s` \x1Bb\0\x07\x1F\x17\x90\x92\x91\x90` \x1CV[`\xC0R`\xE0\x93\x90\x93Ra\x01\0\x91\x90\x91Ra\x01 \x91\x90\x91R`\0UPPB`\x01Ub\0\x02`V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16b\0\x01\x8CW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x02\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15b\0\x02ZWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x10Yb\0\x03\x9E`\09`\0\x81\x81a\x02\x02\x01Ra\x07\xB5\x01R`\0\x81\x81a\x03B\x01R\x81\x81a\x07\xE6\x01Ra\r\xD8\x01R`\0\x81\x81a\x02\xEB\x01R\x81\x81a\x08A\x01R\x81\x81a\x08o\x01R\x81\x81a\tJ\x01R\x81\x81a\nC\x01Ra\r\x07\x01R`\0\x81\x81a\x02\x89\x01R\x81\x81a\x07S\x01R\x81\x81a\x07\x84\x01Ra\x08\x16\x01R`\0\x81\x81a\x03\x12\x01R\x81\x81a\x08\xA5\x01R\x81\x81a\x08\xDE\x01R\x81\x81a\n\x9E\x01R\x81\x81a\x0B\x1A\x01R\x81\x81a\x0BA\x01R\x81\x81a\x0B\x9C\x01R\x81\x81a\x0B\xD0\x01R\x81\x81a\x0B\xF7\x01R\x81\x81a\x0C \x01R\x81\x81a\rA\x01R\x81\x81a\ry\x01Ra\r\xA5\x01R`\0\x81\x81a\x02<\x01R\x81\x81a\t\x1C\x01R\x81\x81a\t\xBD\x01R\x81\x81a\t\xF2\x01R\x81\x81a\n\x1F\x01R\x81\x81a\nu\x01R\x81\x81a\n\xCB\x01R\x81\x81a\n\xF2\x01R\x81\x81a\x0Bi\x01R\x81\x81a\x0C\x88\x01R\x81\x81a\x0C\xB0\x01Ra\x0C\xDE\x01Ra\x10Y`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01vW`\x005`\xE0\x1C\x80c\xAC\x02;u\x11a\x01\x19W\x80c\xCF\xD0\xFA\xC9\x11a\0\xE8W\x80c\xCF\xD0\xFA\xC9\x14a\x03=W\x80c\xD7\xC8V\xB3\x14a\x03dW\x80c\xE05\xCB\xCA\x14a\x03wW\x80c\xE6V\x92u\x14a\x03\x8AWa\x01vV[\x80c\xAC\x02;u\x14a\x02\xD3W\x80c\xB9*b\x0F\x14a\x02\xE6W\x80c\xBC\xC6\xF0\x12\x14a\x03\rW\x80c\xC8\xF3<\x91\x14a\x034Wa\x01vV[\x80c;\x1Be \x11a\x01UW\x80c;\x1Be \x14a\x02sW\x80cu\xDF(&\x14a\x02{W\x80c\x80\xA3\xAF6\x14a\x02\x84W\x80c\x86O\xD1\x99\x14a\x02\xABWa\x01vV[\x80b=\xFE`\x14a\x01\xFDW\x80c\x04\xAB6\xC9\x14a\x027W\x80c/\xB5e\xE8\x14a\x02^W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02qa\x02l6`\x04a\x0E\xDAV[a\x03\xCFV[\0[a\x02qa\x04wV[a\x02$`\0T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBEa\x02\xB96`\x04a\x0E\xDAV[a\x05\x19V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02.V[a\x02$a\x02\xE16`\x04a\x0E\xDAV[a\x05vV[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$`\x01T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$a\x03r6`\x04a\x0E\xDAV[a\x05\x91V[a\x02$a\x03\x856`\x04a\x0E\xDAV[a\x06yV[`\x02Ta\x03\xAA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02.V[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04 W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*B\x82a\x05\x19V[`\x01\x81\x90U`\0\x82\x90U`@Q\x7F\xB7C\x86\xE08\xAC\xE5\x1C\xE2\x8E\x1A;\x1D|\xB7\xB2\x03\x8B\xC9\x19\x8A|z\x1C}\xBD\x9C\xCB\x88\x9El\x1D\x92a\x04k\x92\x90\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x80\x15\x90a\x04\xB5WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14\x15[\x15a\x04\xECW`@Q\x7F\x88i\xBE,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UV[`\x01T`\0\x90\x81\x90\x80\x85\x10\x15a\x05[W`@Q\x7Fo~\xAC&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05n\x84a\x05i\x83\x88a\x0F\xABV[a\x07MV[\x95\x93PPPPV[`\0\x81\x83\x10\x15a\x05\x86W\x82a\x05\x88V[\x81[\x90P[\x92\x91PPV[`\0\x82\x82\x10\x15a\x05\xCDW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x06\x0FW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x1BB\x85a\x05\x19V[P\x90P\x82\x84\x03a\x067Wa\x06/\x81\x84a\x08\xA1V[\x91PPa\x05\x8BV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x06L\x86\x86a\x0F\xABV[a\x06V\x91\x90a\x0F\xBEV[\x90Pa\x06p`\x01\x82a\x06i\x88\x88\x87a\t}V[\x91\x90a\x07\x1FV[\x92PPPa\x05\x8BV[`\0\x81\x83\x10\x15a\x06\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x06\xC4WP`\0a\x05\x8BV[`\0a\x06\xD0B\x85a\x05\x19V[P\x90P`\0a\x06\xE0\x84\x86\x84a\t}V[\x90P`\0a\x06\xF0`\0\x87\x85a\t}V[\x90P`\0a\x07\x06g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0F\xBEV[\x90P\x81a\x07\x13\x81\x83a\x0F\xD5V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x077W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x08\nWa\x06/a\x07\xDA\x84a\x07\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x0F\xABV[a\x07\xB3\x91\x90a\x0F\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x0CoV[a\x07\xE4\x90\x83a\x10\x10V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05vV[`\0a\x08:\x84a\x07\xA9\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[\x90Pa\x08f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0F\xABV[\x81\x11\x15a\x08\x97W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92PPPa\x05\x8BV[a\x06p\x81\x83a\x0F\xABV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\t\x1AW\x82a\t\ta\x08\xD9\x85\x85a\x0C\x84V[a\t\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0CoV[a\t\x13\x91\x90a\x10\x10V[\x90Pa\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\tHWP\x81a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\ta\tv\x85\x85a\x0C\x84V[\x84\x90a\x0CoV[`\0\x83\x83\x10\x15a\t\xB9W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x11a\nlWa\nga\t\xEF\x84\x87a\x0C\x84V[\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\n\x1DW\x86a\n?V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xFCV[a\noV[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x10\x80a\n\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x11[a\x0B\x93Wa\x0B\x8E`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x11a\x0B\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x18V[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x10a\x0BeW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0BgV[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\r\xFCV[a\x0B\x96V[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x0CJWa\x0CEa\x0B\xCE\x86\x88a\x0C\x84V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x11a\x0C\x1BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\x1DV[\x88[\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89a\r\xFCV[a\x0CMV[`\0[\x90P\x80a\x0CZ\x83\x85a\x10\x10V[a\x0Cd\x91\x90a\x10\x10V[\x97\x96PPPPPPPV[`\0a\x05\x88\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x1FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10\x15a\r?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r7Wa\r2a\r\x02`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[a\r,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0E\xC5V[a\t\x13V[P`\0a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\rnWP`\0a\x05\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\r7Wa\r2a\r\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xABV[a\r,\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[`\0\x82\x85\x10\x15a\x0E8W`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0ED\x86\x86a\x0F\xABV[\x90P`\0`\x02a\x0ET\x83\x8Aa\x0F\xBEV[a\x0E^\x90\x84a\x0F\xBEV[a\x0Eh\x91\x90a\x0F\xD5V[\x90P`\0\x88a\x0Ew\x87\x8Aa\x0F\xABV[a\x0E\x81\x91\x90a\x0F\xBEV[a\x0E\x93\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xBEV[a\x0E\x9D\x91\x90a\x10\x10V[\x90P`\0a\x0E\xAB\x84\x83a\x0F\xBEV[\x90Pa\x0E\xB7\x81\x84a\x10\x10V[\x9A\x99PPPPPPPPPPV[`\0a\x05\x88\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8BWa\x05\x8Ba\x0F|V[`\0\x82a\x10\x0BW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V\xFE\xA2dipfsX\"\x12 \x96\xE4\xC7\xB9\xD6j\xB7[<_M\xD49\x88\xEBY\xD8\xB3\xFCX\xB6|B<u\xC7\xC5x[d\r9dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static COSTMODELDYNAMICLEVEL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01vW`\x005`\xE0\x1C\x80c\xAC\x02;u\x11a\x01\x19W\x80c\xCF\xD0\xFA\xC9\x11a\0\xE8W\x80c\xCF\xD0\xFA\xC9\x14a\x03=W\x80c\xD7\xC8V\xB3\x14a\x03dW\x80c\xE05\xCB\xCA\x14a\x03wW\x80c\xE6V\x92u\x14a\x03\x8AWa\x01vV[\x80c\xAC\x02;u\x14a\x02\xD3W\x80c\xB9*b\x0F\x14a\x02\xE6W\x80c\xBC\xC6\xF0\x12\x14a\x03\rW\x80c\xC8\xF3<\x91\x14a\x034Wa\x01vV[\x80c;\x1Be \x11a\x01UW\x80c;\x1Be \x14a\x02sW\x80cu\xDF(&\x14a\x02{W\x80c\x80\xA3\xAF6\x14a\x02\x84W\x80c\x86O\xD1\x99\x14a\x02\xABWa\x01vV[\x80b=\xFE`\x14a\x01\xFDW\x80c\x04\xAB6\xC9\x14a\x027W\x80c/\xB5e\xE8\x14a\x02^W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02qa\x02l6`\x04a\x0E\xDAV[a\x03\xCFV[\0[a\x02qa\x04wV[a\x02$`\0T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBEa\x02\xB96`\x04a\x0E\xDAV[a\x05\x19V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02.V[a\x02$a\x02\xE16`\x04a\x0E\xDAV[a\x05vV[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$`\x01T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$a\x03r6`\x04a\x0E\xDAV[a\x05\x91V[a\x02$a\x03\x856`\x04a\x0E\xDAV[a\x06yV[`\x02Ta\x03\xAA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02.V[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04 W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*B\x82a\x05\x19V[`\x01\x81\x90U`\0\x82\x90U`@Q\x7F\xB7C\x86\xE08\xAC\xE5\x1C\xE2\x8E\x1A;\x1D|\xB7\xB2\x03\x8B\xC9\x19\x8A|z\x1C}\xBD\x9C\xCB\x88\x9El\x1D\x92a\x04k\x92\x90\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x80\x15\x90a\x04\xB5WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14\x15[\x15a\x04\xECW`@Q\x7F\x88i\xBE,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UV[`\x01T`\0\x90\x81\x90\x80\x85\x10\x15a\x05[W`@Q\x7Fo~\xAC&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05n\x84a\x05i\x83\x88a\x0F\xABV[a\x07MV[\x95\x93PPPPV[`\0\x81\x83\x10\x15a\x05\x86W\x82a\x05\x88V[\x81[\x90P[\x92\x91PPV[`\0\x82\x82\x10\x15a\x05\xCDW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x06\x0FW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x1BB\x85a\x05\x19V[P\x90P\x82\x84\x03a\x067Wa\x06/\x81\x84a\x08\xA1V[\x91PPa\x05\x8BV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x06L\x86\x86a\x0F\xABV[a\x06V\x91\x90a\x0F\xBEV[\x90Pa\x06p`\x01\x82a\x06i\x88\x88\x87a\t}V[\x91\x90a\x07\x1FV[\x92PPPa\x05\x8BV[`\0\x81\x83\x10\x15a\x06\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x06\xC4WP`\0a\x05\x8BV[`\0a\x06\xD0B\x85a\x05\x19V[P\x90P`\0a\x06\xE0\x84\x86\x84a\t}V[\x90P`\0a\x06\xF0`\0\x87\x85a\t}V[\x90P`\0a\x07\x06g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0F\xBEV[\x90P\x81a\x07\x13\x81\x83a\x0F\xD5V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x077W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x08\nWa\x06/a\x07\xDA\x84a\x07\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x0F\xABV[a\x07\xB3\x91\x90a\x0F\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x0CoV[a\x07\xE4\x90\x83a\x10\x10V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05vV[`\0a\x08:\x84a\x07\xA9\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[\x90Pa\x08f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0F\xABV[\x81\x11\x15a\x08\x97W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92PPPa\x05\x8BV[a\x06p\x81\x83a\x0F\xABV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\t\x1AW\x82a\t\ta\x08\xD9\x85\x85a\x0C\x84V[a\t\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0CoV[a\t\x13\x91\x90a\x10\x10V[\x90Pa\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\tHWP\x81a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\ta\tv\x85\x85a\x0C\x84V[\x84\x90a\x0CoV[`\0\x83\x83\x10\x15a\t\xB9W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x11a\nlWa\nga\t\xEF\x84\x87a\x0C\x84V[\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\n\x1DW\x86a\n?V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xFCV[a\noV[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x10\x80a\n\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x11[a\x0B\x93Wa\x0B\x8E`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x11a\x0B\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x18V[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x10a\x0BeW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0BgV[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\r\xFCV[a\x0B\x96V[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x0CJWa\x0CEa\x0B\xCE\x86\x88a\x0C\x84V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x11a\x0C\x1BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\x1DV[\x88[\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89a\r\xFCV[a\x0CMV[`\0[\x90P\x80a\x0CZ\x83\x85a\x10\x10V[a\x0Cd\x91\x90a\x10\x10V[\x97\x96PPPPPPPV[`\0a\x05\x88\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x1FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10\x15a\r?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r7Wa\r2a\r\x02`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[a\r,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0E\xC5V[a\t\x13V[P`\0a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\rnWP`\0a\x05\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\r7Wa\r2a\r\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xABV[a\r,\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[`\0\x82\x85\x10\x15a\x0E8W`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0ED\x86\x86a\x0F\xABV[\x90P`\0`\x02a\x0ET\x83\x8Aa\x0F\xBEV[a\x0E^\x90\x84a\x0F\xBEV[a\x0Eh\x91\x90a\x0F\xD5V[\x90P`\0\x88a\x0Ew\x87\x8Aa\x0F\xABV[a\x0E\x81\x91\x90a\x0F\xBEV[a\x0E\x93\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xBEV[a\x0E\x9D\x91\x90a\x10\x10V[\x90P`\0a\x0E\xAB\x84\x83a\x0F\xBEV[\x90Pa\x0E\xB7\x81\x84a\x10\x10V[\x9A\x99PPPPPPPPPPV[`\0a\x05\x88\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8BWa\x05\x8Ba\x0F|V[`\0\x82a\x10\x0BW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V\xFE\xA2dipfsX\"\x12 \x96\xE4\xC7\xB9\xD6j\xB7[<_M\xD49\x88\xEBY\xD8\xB3\xFCX\xB6|B<u\xC7\xC5x[d\r9dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static COSTMODELDYNAMICLEVEL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CostModelDynamicLevel<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CostModelDynamicLevel<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CostModelDynamicLevel<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CostModelDynamicLevel<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CostModelDynamicLevel<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CostModelDynamicLevel))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CostModelDynamicLevel<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COSTMODELDYNAMICLEVEL_ABI.clone(),
                client,
            ))
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
                COSTMODELDYNAMICLEVEL_ABI.clone(),
                COSTMODELDYNAMICLEVEL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `_min` (0xac023b75) function
        pub fn min(
            &self,
            a: ::ethers::core::types::U256,
            b: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([172, 2, 59, 117], (a, b))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `costFactor` (0xd7c856b3) function
        pub fn cost_factor(
            &self,
            from_utilization: ::ethers::core::types::U256,
            to_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([215, 200, 86, 179], (from_utilization, to_utilization))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `costFactorAtFullUtilization` (0xcfd0fac9) function
        pub fn cost_factor_at_full_utilization(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([207, 208, 250, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `costFactorAtZeroUtilization` (0xb92a620f) function
        pub fn cost_factor_at_zero_utilization(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([185, 42, 98, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `costFactorInOptimalZone` (0x75df2826) function
        pub fn cost_factor_in_optimal_zone(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([117, 223, 40, 38], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUpdatedStorageParams` (0x864fd199) function
        pub fn get_updated_storage_params(
            &self,
            current_time: ::ethers::core::types::U256,
            utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([134, 79, 209, 153], (current_time, utilization))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateTime` (0xc8f33c91) function
        pub fn last_update_time(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([200, 243, 60, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `optimalZoneRate` (0x003dfe60) function
        pub fn optimal_zone_rate(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 61, 254, 96], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundFactor` (0xe035cbca) function
        pub fn refund_factor(
            &self,
            from_utilization: ::ethers::core::types::U256,
            to_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([224, 53, 203, 202], (from_utilization, to_utilization))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `registerSet` (0x3b1b6520) function
        pub fn register_set(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 27, 101, 32], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setAddress` (0xe6569275) function
        pub fn set_address(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([230, 86, 146, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uHigh` (0xbcc6f012) function
        pub fn u_high(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([188, 198, 240, 18], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uLow` (0x04ab36c9) function
        pub fn u_low(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([4, 171, 54, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `uOpt` (0x80a3af36) function
        pub fn u_opt(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([128, 163, 175, 54], ())
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
        ///Gets the contract's `UpdatedDynamicLevelModelParameters` event
        pub fn updated_dynamic_level_model_parameters_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdatedDynamicLevelModelParametersFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpdatedDynamicLevelModelParametersFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for CostModelDynamicLevel<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidConfiguration` with signature `InvalidConfiguration()` and selector `0xc52a9bd3`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidConfiguration", abi = "InvalidConfiguration()")]
    pub struct InvalidConfiguration;
    ///Custom Error type `InvalidReferencePoint` with signature `InvalidReferencePoint()` and selector `0x030b4011`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidReferencePoint", abi = "InvalidReferencePoint()")]
    pub struct InvalidReferencePoint;
    ///Custom Error type `InvalidTime` with signature `InvalidTime()` and selector `0x6f7eac26`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTime", abi = "InvalidTime()")]
    pub struct InvalidTime;
    ///Custom Error type `InvalidUtilization` with signature `InvalidUtilization()` and selector `0x25062e25`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidUtilization", abi = "InvalidUtilization()")]
    pub struct InvalidUtilization;
    ///Custom Error type `SetAlreadyRegistered` with signature `SetAlreadyRegistered()` and selector `0x8869be2c`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SetAlreadyRegistered", abi = "SetAlreadyRegistered()")]
    pub struct SetAlreadyRegistered;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CostModelDynamicLevelErrors {
        InvalidConfiguration(InvalidConfiguration),
        InvalidReferencePoint(InvalidReferencePoint),
        InvalidTime(InvalidTime),
        InvalidUtilization(InvalidUtilization),
        SetAlreadyRegistered(SetAlreadyRegistered),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelDynamicLevelErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <InvalidConfiguration as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidConfiguration(decoded));
            }
            if let Ok(decoded) =
                <InvalidReferencePoint as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidReferencePoint(decoded));
            }
            if let Ok(decoded) = <InvalidTime as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTime(decoded));
            }
            if let Ok(decoded) =
                <InvalidUtilization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUtilization(decoded));
            }
            if let Ok(decoded) =
                <SetAlreadyRegistered as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetAlreadyRegistered(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CostModelDynamicLevelErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReferencePoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetAlreadyRegistered(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for CostModelDynamicLevelErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidConfiguration as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidReferencePoint as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidTime as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidUtilization as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <SetAlreadyRegistered as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Unauthorized as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CostModelDynamicLevelErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReferencePoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAlreadyRegistered(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CostModelDynamicLevelErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidConfiguration> for CostModelDynamicLevelErrors {
        fn from(value: InvalidConfiguration) -> Self {
            Self::InvalidConfiguration(value)
        }
    }
    impl ::core::convert::From<InvalidReferencePoint> for CostModelDynamicLevelErrors {
        fn from(value: InvalidReferencePoint) -> Self {
            Self::InvalidReferencePoint(value)
        }
    }
    impl ::core::convert::From<InvalidTime> for CostModelDynamicLevelErrors {
        fn from(value: InvalidTime) -> Self {
            Self::InvalidTime(value)
        }
    }
    impl ::core::convert::From<InvalidUtilization> for CostModelDynamicLevelErrors {
        fn from(value: InvalidUtilization) -> Self {
            Self::InvalidUtilization(value)
        }
    }
    impl ::core::convert::From<SetAlreadyRegistered> for CostModelDynamicLevelErrors {
        fn from(value: SetAlreadyRegistered) -> Self {
            Self::SetAlreadyRegistered(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for CostModelDynamicLevelErrors {
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
        Hash,
    )]
    #[ethevent(
        name = "UpdatedDynamicLevelModelParameters",
        abi = "UpdatedDynamicLevelModelParameters(uint256,uint256)"
    )]
    pub struct UpdatedDynamicLevelModelParametersFilter {
        pub cost_factor_in_optimal_zone: ::ethers::core::types::U256,
        pub last_update_time: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `_min` function with signature `_min(uint256,uint256)` and selector `0xac023b75`
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
    #[ethcall(name = "_min", abi = "_min(uint256,uint256)")]
    pub struct MinCall {
        pub a: ::ethers::core::types::U256,
        pub b: ::ethers::core::types::U256,
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
        pub from_utilization: ::ethers::core::types::U256,
        pub to_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `costFactorAtFullUtilization` function with signature `costFactorAtFullUtilization()` and selector `0xcfd0fac9`
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
        name = "costFactorAtFullUtilization",
        abi = "costFactorAtFullUtilization()"
    )]
    pub struct CostFactorAtFullUtilizationCall;
    ///Container type for all input parameters for the `costFactorAtZeroUtilization` function with signature `costFactorAtZeroUtilization()` and selector `0xb92a620f`
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
        name = "costFactorAtZeroUtilization",
        abi = "costFactorAtZeroUtilization()"
    )]
    pub struct CostFactorAtZeroUtilizationCall;
    ///Container type for all input parameters for the `costFactorInOptimalZone` function with signature `costFactorInOptimalZone()` and selector `0x75df2826`
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
    #[ethcall(name = "costFactorInOptimalZone", abi = "costFactorInOptimalZone()")]
    pub struct CostFactorInOptimalZoneCall;
    ///Container type for all input parameters for the `getUpdatedStorageParams` function with signature `getUpdatedStorageParams(uint256,uint256)` and selector `0x864fd199`
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
        name = "getUpdatedStorageParams",
        abi = "getUpdatedStorageParams(uint256,uint256)"
    )]
    pub struct GetUpdatedStorageParamsCall {
        pub current_time: ::ethers::core::types::U256,
        pub utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
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
    #[ethcall(name = "lastUpdateTime", abi = "lastUpdateTime()")]
    pub struct LastUpdateTimeCall;
    ///Container type for all input parameters for the `optimalZoneRate` function with signature `optimalZoneRate()` and selector `0x003dfe60`
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
    #[ethcall(name = "optimalZoneRate", abi = "optimalZoneRate()")]
    pub struct OptimalZoneRateCall;
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
        pub from_utilization: ::ethers::core::types::U256,
        pub to_utilization: ::ethers::core::types::U256,
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
    ///Container type for all input parameters for the `setAddress` function with signature `setAddress()` and selector `0xe6569275`
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
    #[ethcall(name = "setAddress", abi = "setAddress()")]
    pub struct SetAddressCall;
    ///Container type for all input parameters for the `uHigh` function with signature `uHigh()` and selector `0xbcc6f012`
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
    #[ethcall(name = "uHigh", abi = "uHigh()")]
    pub struct UhighCall;
    ///Container type for all input parameters for the `uLow` function with signature `uLow()` and selector `0x04ab36c9`
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
    #[ethcall(name = "uLow", abi = "uLow()")]
    pub struct UlowCall;
    ///Container type for all input parameters for the `uOpt` function with signature `uOpt()` and selector `0x80a3af36`
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
    #[ethcall(name = "uOpt", abi = "uOpt()")]
    pub struct UoptCall;
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
    pub enum CostModelDynamicLevelCalls {
        Min(MinCall),
        CostFactor(CostFactorCall),
        CostFactorAtFullUtilization(CostFactorAtFullUtilizationCall),
        CostFactorAtZeroUtilization(CostFactorAtZeroUtilizationCall),
        CostFactorInOptimalZone(CostFactorInOptimalZoneCall),
        GetUpdatedStorageParams(GetUpdatedStorageParamsCall),
        LastUpdateTime(LastUpdateTimeCall),
        OptimalZoneRate(OptimalZoneRateCall),
        RefundFactor(RefundFactorCall),
        RegisterSet(RegisterSetCall),
        SetAddress(SetAddressCall),
        Uhigh(UhighCall),
        Ulow(UlowCall),
        Uopt(UoptCall),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelDynamicLevelCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <MinCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <CostFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CostFactor(decoded));
            }
            if let Ok(decoded) =
                <CostFactorAtFullUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorAtFullUtilization(decoded));
            }
            if let Ok(decoded) =
                <CostFactorAtZeroUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorAtZeroUtilization(decoded));
            }
            if let Ok(decoded) =
                <CostFactorInOptimalZoneCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorInOptimalZone(decoded));
            }
            if let Ok(decoded) =
                <GetUpdatedStorageParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetUpdatedStorageParams(decoded));
            }
            if let Ok(decoded) =
                <LastUpdateTimeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastUpdateTime(decoded));
            }
            if let Ok(decoded) =
                <OptimalZoneRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OptimalZoneRate(decoded));
            }
            if let Ok(decoded) = <RefundFactorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RefundFactor(decoded));
            }
            if let Ok(decoded) = <RegisterSetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RegisterSet(decoded));
            }
            if let Ok(decoded) = <SetAddressCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetAddress(decoded));
            }
            if let Ok(decoded) = <UhighCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Uhigh(decoded));
            }
            if let Ok(decoded) = <UlowCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ulow(decoded));
            }
            if let Ok(decoded) = <UoptCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Uopt(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CostModelDynamicLevelCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CostFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CostFactorAtFullUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CostFactorAtZeroUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CostFactorInOptimalZone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUpdatedStorageParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastUpdateTime(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OptimalZoneRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uhigh(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ulow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uopt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CostModelDynamicLevelCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorAtFullUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorAtZeroUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorInOptimalZone(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUpdatedStorageParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastUpdateTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::OptimalZoneRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uhigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ulow(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uopt(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MinCall> for CostModelDynamicLevelCalls {
        fn from(value: MinCall) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<CostFactorCall> for CostModelDynamicLevelCalls {
        fn from(value: CostFactorCall) -> Self {
            Self::CostFactor(value)
        }
    }
    impl ::core::convert::From<CostFactorAtFullUtilizationCall> for CostModelDynamicLevelCalls {
        fn from(value: CostFactorAtFullUtilizationCall) -> Self {
            Self::CostFactorAtFullUtilization(value)
        }
    }
    impl ::core::convert::From<CostFactorAtZeroUtilizationCall> for CostModelDynamicLevelCalls {
        fn from(value: CostFactorAtZeroUtilizationCall) -> Self {
            Self::CostFactorAtZeroUtilization(value)
        }
    }
    impl ::core::convert::From<CostFactorInOptimalZoneCall> for CostModelDynamicLevelCalls {
        fn from(value: CostFactorInOptimalZoneCall) -> Self {
            Self::CostFactorInOptimalZone(value)
        }
    }
    impl ::core::convert::From<GetUpdatedStorageParamsCall> for CostModelDynamicLevelCalls {
        fn from(value: GetUpdatedStorageParamsCall) -> Self {
            Self::GetUpdatedStorageParams(value)
        }
    }
    impl ::core::convert::From<LastUpdateTimeCall> for CostModelDynamicLevelCalls {
        fn from(value: LastUpdateTimeCall) -> Self {
            Self::LastUpdateTime(value)
        }
    }
    impl ::core::convert::From<OptimalZoneRateCall> for CostModelDynamicLevelCalls {
        fn from(value: OptimalZoneRateCall) -> Self {
            Self::OptimalZoneRate(value)
        }
    }
    impl ::core::convert::From<RefundFactorCall> for CostModelDynamicLevelCalls {
        fn from(value: RefundFactorCall) -> Self {
            Self::RefundFactor(value)
        }
    }
    impl ::core::convert::From<RegisterSetCall> for CostModelDynamicLevelCalls {
        fn from(value: RegisterSetCall) -> Self {
            Self::RegisterSet(value)
        }
    }
    impl ::core::convert::From<SetAddressCall> for CostModelDynamicLevelCalls {
        fn from(value: SetAddressCall) -> Self {
            Self::SetAddress(value)
        }
    }
    impl ::core::convert::From<UhighCall> for CostModelDynamicLevelCalls {
        fn from(value: UhighCall) -> Self {
            Self::Uhigh(value)
        }
    }
    impl ::core::convert::From<UlowCall> for CostModelDynamicLevelCalls {
        fn from(value: UlowCall) -> Self {
            Self::Ulow(value)
        }
    }
    impl ::core::convert::From<UoptCall> for CostModelDynamicLevelCalls {
        fn from(value: UoptCall) -> Self {
            Self::Uopt(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for CostModelDynamicLevelCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    ///Container type for all return fields from the `_min` function with signature `_min(uint256,uint256)` and selector `0xac023b75`
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
    pub struct MinReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `costFactorAtFullUtilization` function with signature `costFactorAtFullUtilization()` and selector `0xcfd0fac9`
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
    pub struct CostFactorAtFullUtilizationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `costFactorAtZeroUtilization` function with signature `costFactorAtZeroUtilization()` and selector `0xb92a620f`
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
    pub struct CostFactorAtZeroUtilizationReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `costFactorInOptimalZone` function with signature `costFactorInOptimalZone()` and selector `0x75df2826`
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
    pub struct CostFactorInOptimalZoneReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getUpdatedStorageParams` function with signature `getUpdatedStorageParams(uint256,uint256)` and selector `0x864fd199`
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
    pub struct GetUpdatedStorageParamsReturn {
        pub new_cost_factor_in_optimal_zone: ::ethers::core::types::U256,
        pub new_last_update_time: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `lastUpdateTime` function with signature `lastUpdateTime()` and selector `0xc8f33c91`
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
    pub struct LastUpdateTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `optimalZoneRate` function with signature `optimalZoneRate()` and selector `0x003dfe60`
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
    pub struct OptimalZoneRateReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `setAddress` function with signature `setAddress()` and selector `0xe6569275`
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
    pub struct SetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `uHigh` function with signature `uHigh()` and selector `0xbcc6f012`
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
    pub struct UhighReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `uLow` function with signature `uLow()` and selector `0x04ab36c9`
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
    pub struct UlowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `uOpt` function with signature `uOpt()` and selector `0x80a3af36`
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
    pub struct UoptReturn(pub ::ethers::core::types::U256);
}
