pub use cost_model_jump_rate::*;
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
pub mod cost_model_jump_rate {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_kink"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_costFactorAtZeroUtilization",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_costFactorAtKinkUtilization",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_costFactorAtFullUtilization",),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("costFactor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactor"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_fromUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("costFactorAtKinkUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("costFactorAtKinkUtilization",),
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
                    ::std::borrow::ToOwned::to_owned("kink"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("kink"),
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
                                name: ::std::borrow::ToOwned::to_owned("_fromUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toUtilization"),
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
                                name: ::std::borrow::ToOwned::to_owned("_fromUtilization"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_toUtilization"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidUtilization"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUtilization"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COSTMODELJUMPRATE_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\0`@R4\x80\x15a\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x0E@8\x03\x80a\x0E@\x839\x81\x01`@\x81\x90Ra\0}\x91a\x018V[g\r\xE0\xB6\xB3\xA7d\0\0\x84\x11\x15a\0\xA6W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\0\xCFW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x01!W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xE0\x93\x90\x93R`\x80\x91\x90\x91R`\xA0R`\xC0Ra\x01\xB9V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x0B\x91a\x02\xAF`\09`\0\x81\x81a\x02W\x01R\x81\x81a\x04J\x01R\x81\x81a\x04\x9A\x01R\x81\x81a\x04\xC4\x01R\x81\x81a\x04\xF6\x01R\x81\x81a\x05\xDD\x01R\x81\x81a\x06\x04\x01R\x81\x81a\x06,\x01R\x81\x81a\x06S\x01R\x81\x81a\x06\xBB\x01R\x81\x81a\x06\xE2\x01R\x81\x81a\x07\n\x01R\x81\x81a\x071\x01R\x81\x81a\x07Y\x01R\x81\x81a\x07\xDC\x01R\x81\x81a\x08\x0B\x01Ra\x08\x87\x01R`\0\x81\x81a\x02\n\x01R\x81\x81a\x04$\x01Ra\x08\xDA\x01R`\0\x81\x81a\x01\xAA\x01R\x81\x81a\x04r\x01R\x81\x81a\x05\x1D\x01R\x81\x81a\x07z\x01R\x81\x81a\x08U\x01Ra\x08\xB9\x01R`\0\x81\x81a\x01\xE3\x01R\x81\x81a\x03\xEE\x01R\x81\x81a\x05C\x01R\x81\x81a\x06}\x01Ra\x084\x01Ra\x0B\x91`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\xCF\xD0\xFA\xC9\x11a\0\xDDW\x80c\xCF\xD0\xFA\xC9\x14a\x02\x05W\x80c\xD7\xC8V\xB3\x14a\x02,W\x80c\xE05\xCB\xCA\x14a\x02?W\x80c\xFD-\xA39\x14a\x02RWa\x01\nV[\x80c/\xB5e\xE8\x14a\x01\x91W\x80c;\x1Be \x14a\x01\xA3W\x80c\xA2aN\x9F\x14a\x01\xA5W\x80c\xB9*b\x0F\x14a\x01\xDEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xA3a\x01\x9F6`\x04a\n\x12V[PPV[\0[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCCa\x02:6`\x04a\n\x12V[a\x02yV[a\x01\xCCa\x02M6`\x04a\n\x12V[a\x03PV[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x82\x10\x15a\x02\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x02\xF7W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x0EWa\x03\x07\x82a\x03\xE4V[\x90Pa\x03JV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x03#\x85\x85a\n\xE3V[a\x03-\x91\x90a\n\xF6V[\x90Pa\x03F`\x01\x82a\x03?\x87\x87a\x05\x90V[\x91\x90a\x07\xAAV[\x91PP[\x92\x91PPV[`\0\x81\x83\x10\x15a\x03\x8CW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x9BWP`\0a\x03JV[`\0a\x03\xA7\x83\x85a\x05\x90V[\x90P`\0a\x03\xB6`\0\x86a\x05\x90V[\x90P`\0a\x03\xCCg\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xF6V[\x90P\x81a\x03\xD9\x81\x83a\x0B\rV[\x97\x96PPPPPPPV[`\0\x81a\x04\x12WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x04HWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x04\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10a\x04\xEEWa\x04\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\n\xE3V[a\x04\xF0V[\x82[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x05AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P`\0a\x05p\x85a\x07\xD8V[\x90P\x81a\x05}\x84\x83a\x08\xFEV[a\x05\x87\x91\x90a\x0BHV[\x95\x94PPPPPV[`\0\x82\x82\x10\x15a\x05\xCCW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xA1a\x05\xDB`\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06(W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06*V[\x85[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06yV[\x85[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90P`\0a\x07\x9Ea\x06\xB9g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x08V[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07UW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07WV[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90Pa\x05\x87\x81\x83a\x0BHV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x07\xC2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\x08\x7FWa\x03Ja\x08/`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[\x90a\t\xE3V[a\x03Ja\x08\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[`\0a\t\x13\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xAAV[\x93\x92PPPV[`\0\x82\x85\x10\x15a\tVW`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\tb\x86\x86a\n\xE3V[\x90P`\0`\x02a\tr\x83\x8Aa\n\xF6V[a\t|\x90\x84a\n\xF6V[a\t\x86\x91\x90a\x0B\rV[\x90P`\0\x88a\t\x95\x87\x8Aa\n\xE3V[a\t\x9F\x91\x90a\n\xF6V[a\t\xB1\x86g\r\xE0\xB6\xB3\xA7d\0\0a\n\xF6V[a\t\xBB\x91\x90a\x0BHV[\x90P`\0a\t\xC9\x84\x83a\n\xF6V[\x90Pa\t\xD5\x81\x84a\x0BHV[\x9A\x99PPPPPPPPPPV[`\0a\t\x13\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\x0BW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03JWa\x03Ja\n\xB4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03JWa\x03Ja\n\xB4V[`\0\x82a\x0BCW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03JWa\x03Ja\n\xB4V\xFE\xA2dipfsX\"\x12 \ty\xC9O:Gg\xCF\x0E\xFB\x8A\x10g\x16;\xBF\xAFX\x94\xD2\x91\t\xEC\xD5\xF5\xD4\xDD\x9B\x1E\xFA\xB2\xEBdsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static COSTMODELJUMPRATE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\xCF\xD0\xFA\xC9\x11a\0\xDDW\x80c\xCF\xD0\xFA\xC9\x14a\x02\x05W\x80c\xD7\xC8V\xB3\x14a\x02,W\x80c\xE05\xCB\xCA\x14a\x02?W\x80c\xFD-\xA39\x14a\x02RWa\x01\nV[\x80c/\xB5e\xE8\x14a\x01\x91W\x80c;\x1Be \x14a\x01\xA3W\x80c\xA2aN\x9F\x14a\x01\xA5W\x80c\xB9*b\x0F\x14a\x01\xDEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xA3a\x01\x9F6`\x04a\n\x12V[PPV[\0[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCCa\x02:6`\x04a\n\x12V[a\x02yV[a\x01\xCCa\x02M6`\x04a\n\x12V[a\x03PV[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x82\x10\x15a\x02\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x02\xF7W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x0EWa\x03\x07\x82a\x03\xE4V[\x90Pa\x03JV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x03#\x85\x85a\n\xE3V[a\x03-\x91\x90a\n\xF6V[\x90Pa\x03F`\x01\x82a\x03?\x87\x87a\x05\x90V[\x91\x90a\x07\xAAV[\x91PP[\x92\x91PPV[`\0\x81\x83\x10\x15a\x03\x8CW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x9BWP`\0a\x03JV[`\0a\x03\xA7\x83\x85a\x05\x90V[\x90P`\0a\x03\xB6`\0\x86a\x05\x90V[\x90P`\0a\x03\xCCg\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xF6V[\x90P\x81a\x03\xD9\x81\x83a\x0B\rV[\x97\x96PPPPPPPV[`\0\x81a\x04\x12WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x04HWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x04\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10a\x04\xEEWa\x04\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\n\xE3V[a\x04\xF0V[\x82[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x05AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P`\0a\x05p\x85a\x07\xD8V[\x90P\x81a\x05}\x84\x83a\x08\xFEV[a\x05\x87\x91\x90a\x0BHV[\x95\x94PPPPPV[`\0\x82\x82\x10\x15a\x05\xCCW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xA1a\x05\xDB`\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06(W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06*V[\x85[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06yV[\x85[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90P`\0a\x07\x9Ea\x06\xB9g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x08V[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07UW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07WV[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90Pa\x05\x87\x81\x83a\x0BHV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x07\xC2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\x08\x7FWa\x03Ja\x08/`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[\x90a\t\xE3V[a\x03Ja\x08\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[`\0a\t\x13\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xAAV[\x93\x92PPPV[`\0\x82\x85\x10\x15a\tVW`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\tb\x86\x86a\n\xE3V[\x90P`\0`\x02a\tr\x83\x8Aa\n\xF6V[a\t|\x90\x84a\n\xF6V[a\t\x86\x91\x90a\x0B\rV[\x90P`\0\x88a\t\x95\x87\x8Aa\n\xE3V[a\t\x9F\x91\x90a\n\xF6V[a\t\xB1\x86g\r\xE0\xB6\xB3\xA7d\0\0a\n\xF6V[a\t\xBB\x91\x90a\x0BHV[\x90P`\0a\t\xC9\x84\x83a\n\xF6V[\x90Pa\t\xD5\x81\x84a\x0BHV[\x9A\x99PPPPPPPPPPV[`\0a\t\x13\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\x0BW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03JWa\x03Ja\n\xB4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03JWa\x03Ja\n\xB4V[`\0\x82a\x0BCW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03JWa\x03Ja\n\xB4V\xFE\xA2dipfsX\"\x12 \ty\xC9O:Gg\xCF\x0E\xFB\x8A\x10g\x16;\xBF\xAFX\x94\xD2\x91\t\xEC\xD5\xF5\xD4\xDD\x9B\x1E\xFA\xB2\xEBdsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static COSTMODELJUMPRATE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CostModelJumpRate<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CostModelJumpRate<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CostModelJumpRate<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CostModelJumpRate<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CostModelJumpRate<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CostModelJumpRate))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CostModelJumpRate<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COSTMODELJUMPRATE_ABI.clone(),
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
                COSTMODELJUMPRATE_ABI.clone(),
                COSTMODELJUMPRATE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        ///Calls the contract's `costFactorAtKinkUtilization` (0xa2614e9f) function
        pub fn cost_factor_at_kink_utilization(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([162, 97, 78, 159], ())
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
        ///Calls the contract's `kink` (0xfd2da339) function
        pub fn kink(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([253, 45, 163, 57], ())
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
        ///Calls the contract's `update` (0x2fb565e8) function
        pub fn update(
            &self,
            from_utilization: ::ethers::core::types::U256,
            to_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 181, 101, 232], (from_utilization, to_utilization))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for CostModelJumpRate<M>
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CostModelJumpRateErrors {
        InvalidConfiguration(InvalidConfiguration),
        InvalidReferencePoint(InvalidReferencePoint),
        InvalidUtilization(InvalidUtilization),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelJumpRateErrors {
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
            if let Ok(decoded) =
                <InvalidUtilization as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUtilization(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CostModelJumpRateErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReferencePoint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for CostModelJumpRateErrors {
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
                _ if selector
                    == <InvalidUtilization as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CostModelJumpRateErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReferencePoint(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CostModelJumpRateErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidConfiguration> for CostModelJumpRateErrors {
        fn from(value: InvalidConfiguration) -> Self {
            Self::InvalidConfiguration(value)
        }
    }
    impl ::core::convert::From<InvalidReferencePoint> for CostModelJumpRateErrors {
        fn from(value: InvalidReferencePoint) -> Self {
            Self::InvalidReferencePoint(value)
        }
    }
    impl ::core::convert::From<InvalidUtilization> for CostModelJumpRateErrors {
        fn from(value: InvalidUtilization) -> Self {
            Self::InvalidUtilization(value)
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
    ///Container type for all input parameters for the `costFactorAtKinkUtilization` function with signature `costFactorAtKinkUtilization()` and selector `0xa2614e9f`
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
        name = "costFactorAtKinkUtilization",
        abi = "costFactorAtKinkUtilization()"
    )]
    pub struct CostFactorAtKinkUtilizationCall;
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
    ///Container type for all input parameters for the `kink` function with signature `kink()` and selector `0xfd2da339`
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
    #[ethcall(name = "kink", abi = "kink()")]
    pub struct KinkCall;
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
        pub from_utilization: ::ethers::core::types::U256,
        pub to_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CostModelJumpRateCalls {
        CostFactor(CostFactorCall),
        CostFactorAtFullUtilization(CostFactorAtFullUtilizationCall),
        CostFactorAtKinkUtilization(CostFactorAtKinkUtilizationCall),
        CostFactorAtZeroUtilization(CostFactorAtZeroUtilizationCall),
        Kink(KinkCall),
        RefundFactor(RefundFactorCall),
        RegisterSet(RegisterSetCall),
        Update(UpdateCall),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelJumpRateCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CostFactorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CostFactor(decoded));
            }
            if let Ok(decoded) =
                <CostFactorAtFullUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorAtFullUtilization(decoded));
            }
            if let Ok(decoded) =
                <CostFactorAtKinkUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorAtKinkUtilization(decoded));
            }
            if let Ok(decoded) =
                <CostFactorAtZeroUtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CostFactorAtZeroUtilization(decoded));
            }
            if let Ok(decoded) = <KinkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Kink(decoded));
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
    impl ::ethers::core::abi::AbiEncode for CostModelJumpRateCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CostFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CostFactorAtFullUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CostFactorAtKinkUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CostFactorAtZeroUtilization(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Kink(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundFactor(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RegisterSet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CostModelJumpRateCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CostFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorAtFullUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorAtKinkUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::CostFactorAtZeroUtilization(element) => ::core::fmt::Display::fmt(element, f),
                Self::Kink(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundFactor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RegisterSet(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CostFactorCall> for CostModelJumpRateCalls {
        fn from(value: CostFactorCall) -> Self {
            Self::CostFactor(value)
        }
    }
    impl ::core::convert::From<CostFactorAtFullUtilizationCall> for CostModelJumpRateCalls {
        fn from(value: CostFactorAtFullUtilizationCall) -> Self {
            Self::CostFactorAtFullUtilization(value)
        }
    }
    impl ::core::convert::From<CostFactorAtKinkUtilizationCall> for CostModelJumpRateCalls {
        fn from(value: CostFactorAtKinkUtilizationCall) -> Self {
            Self::CostFactorAtKinkUtilization(value)
        }
    }
    impl ::core::convert::From<CostFactorAtZeroUtilizationCall> for CostModelJumpRateCalls {
        fn from(value: CostFactorAtZeroUtilizationCall) -> Self {
            Self::CostFactorAtZeroUtilization(value)
        }
    }
    impl ::core::convert::From<KinkCall> for CostModelJumpRateCalls {
        fn from(value: KinkCall) -> Self {
            Self::Kink(value)
        }
    }
    impl ::core::convert::From<RefundFactorCall> for CostModelJumpRateCalls {
        fn from(value: RefundFactorCall) -> Self {
            Self::RefundFactor(value)
        }
    }
    impl ::core::convert::From<RegisterSetCall> for CostModelJumpRateCalls {
        fn from(value: RegisterSetCall) -> Self {
            Self::RegisterSet(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for CostModelJumpRateCalls {
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
    ///Container type for all return fields from the `costFactorAtKinkUtilization` function with signature `costFactorAtKinkUtilization()` and selector `0xa2614e9f`
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
    pub struct CostFactorAtKinkUtilizationReturn(pub ::ethers::core::types::U256);
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
    ///Container type for all return fields from the `kink` function with signature `kink()` and selector `0xfd2da339`
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
    pub struct KinkReturn(pub ::ethers::core::types::U256);
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
