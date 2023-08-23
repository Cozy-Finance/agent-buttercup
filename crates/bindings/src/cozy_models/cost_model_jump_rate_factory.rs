pub use cost_model_jump_rate_factory::*;
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
pub mod cost_model_jump_rate_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deployModel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deployModel"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_kink"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtZeroUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtKinkUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtFullUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_model"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract CostModelJumpRate",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getModel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getModel"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_kink"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtZeroUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtKinkUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "_costFactorAtFullUtilization",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("isDeployed"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isDeployed"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("DeployedCostModelJumpRate"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("DeployedCostModelJumpRate",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costModel"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("kink"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorAtZeroUtilization",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorAtKinkUtilization",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorAtFullUtilization",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static COSTMODELJUMPRATEFACTORY_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x14[\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xC3W`\x005`\xE0\x1C\x80c\x90\x18K\x02\x14a\x01JW\x80c\xA4\x82B\xC5\x14a\x01\x82W\x80c\xBFP\xA5\x8D\x14a\x01\xBAW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ma\x01X6`\x04a\x05#V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x95a\x01\x906`\x04a\x05cV[a\x01\xCDV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01yV[a\x01\x95a\x01\xC86`\x04a\x05cV[a\x02\xB7V[`@\x80Q` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x02s`@Q\x80` \x01a\x02\x1A\x90a\x04\x91V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x830\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11ma\x03\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16a\x02\xAAW`\0a\x02\xACV[\x80[\x97\x96PPPPPPPV[`\0\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m\x85\x85\x85\x85`@Qa\x02\xEA\x90a\x04\x91V[\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x03\"W=`\0\x80>=`\0\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x81Q\x89\x81R\x90\x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R\x91\x92P\x90\x7F\x954\t\xED\xB2:\"\x96Smj\xDF\xB2\x90\xA9\xC7\x96U\xC0\xF9\xC7\xB8O\xFA\xA7\xDE\xE9\x88\xF0XbB\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`\0\x80\x85\x85`@Q` \x01a\x03\xDA\x92\x91\x90a\x05\xC8V[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x85\x01R``\x97\x90\x97\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16`!\x84\x01R`5\x83\x01\x95\x90\x95R`U\x80\x83\x01\x96\x90\x96R\x80Q\x80\x83\x03\x90\x96\x01\x86R`u\x90\x91\x01\x90RPP\x81Q\x91\x01 \x92\x91PPV[a\x0E@\x80a\x05\xE6\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x058Wa\x058a\x04\x9EV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\\W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05|Wa\x05|a\x04\x9EV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x81Q`\0[\x81\x81\x10\x15a\x05\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x05\x9FV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x05\xDDa\x05\xD7\x83\x86a\x05\x98V[\x84a\x05\x98V[\x94\x93PPPPV\xFEa\x01\0`@R4\x80\x15a\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x0E@8\x03\x80a\x0E@\x839\x81\x01`@\x81\x90Ra\0}\x91a\x018V[g\r\xE0\xB6\xB3\xA7d\0\0\x84\x11\x15a\0\xA6W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\0\xCFW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x01!W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xE0\x93\x90\x93R`\x80\x91\x90\x91R`\xA0R`\xC0Ra\x01\xB9V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x0B\x91a\x02\xAF`\09`\0\x81\x81a\x02W\x01R\x81\x81a\x04J\x01R\x81\x81a\x04\x9A\x01R\x81\x81a\x04\xC4\x01R\x81\x81a\x04\xF6\x01R\x81\x81a\x05\xDD\x01R\x81\x81a\x06\x04\x01R\x81\x81a\x06,\x01R\x81\x81a\x06S\x01R\x81\x81a\x06\xBB\x01R\x81\x81a\x06\xE2\x01R\x81\x81a\x07\n\x01R\x81\x81a\x071\x01R\x81\x81a\x07Y\x01R\x81\x81a\x07\xDC\x01R\x81\x81a\x08\x0B\x01Ra\x08\x87\x01R`\0\x81\x81a\x02\n\x01R\x81\x81a\x04$\x01Ra\x08\xDA\x01R`\0\x81\x81a\x01\xAA\x01R\x81\x81a\x04r\x01R\x81\x81a\x05\x1D\x01R\x81\x81a\x07z\x01R\x81\x81a\x08U\x01Ra\x08\xB9\x01R`\0\x81\x81a\x01\xE3\x01R\x81\x81a\x03\xEE\x01R\x81\x81a\x05C\x01R\x81\x81a\x06}\x01Ra\x084\x01Ra\x0B\x91`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\xCF\xD0\xFA\xC9\x11a\0\xDDW\x80c\xCF\xD0\xFA\xC9\x14a\x02\x05W\x80c\xD7\xC8V\xB3\x14a\x02,W\x80c\xE05\xCB\xCA\x14a\x02?W\x80c\xFD-\xA39\x14a\x02RWa\x01\nV[\x80c/\xB5e\xE8\x14a\x01\x91W\x80c;\x1Be \x14a\x01\xA3W\x80c\xA2aN\x9F\x14a\x01\xA5W\x80c\xB9*b\x0F\x14a\x01\xDEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xA3a\x01\x9F6`\x04a\n\x12V[PPV[\0[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCCa\x02:6`\x04a\n\x12V[a\x02yV[a\x01\xCCa\x02M6`\x04a\n\x12V[a\x03PV[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x82\x10\x15a\x02\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x02\xF7W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x0EWa\x03\x07\x82a\x03\xE4V[\x90Pa\x03JV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x03#\x85\x85a\n\xE3V[a\x03-\x91\x90a\n\xF6V[\x90Pa\x03F`\x01\x82a\x03?\x87\x87a\x05\x90V[\x91\x90a\x07\xAAV[\x91PP[\x92\x91PPV[`\0\x81\x83\x10\x15a\x03\x8CW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x9BWP`\0a\x03JV[`\0a\x03\xA7\x83\x85a\x05\x90V[\x90P`\0a\x03\xB6`\0\x86a\x05\x90V[\x90P`\0a\x03\xCCg\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xF6V[\x90P\x81a\x03\xD9\x81\x83a\x0B\rV[\x97\x96PPPPPPPV[`\0\x81a\x04\x12WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x04HWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x04\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10a\x04\xEEWa\x04\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\n\xE3V[a\x04\xF0V[\x82[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x05AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P`\0a\x05p\x85a\x07\xD8V[\x90P\x81a\x05}\x84\x83a\x08\xFEV[a\x05\x87\x91\x90a\x0BHV[\x95\x94PPPPPV[`\0\x82\x82\x10\x15a\x05\xCCW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xA1a\x05\xDB`\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06(W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06*V[\x85[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06yV[\x85[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90P`\0a\x07\x9Ea\x06\xB9g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x08V[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07UW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07WV[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90Pa\x05\x87\x81\x83a\x0BHV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x07\xC2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\x08\x7FWa\x03Ja\x08/`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[\x90a\t\xE3V[a\x03Ja\x08\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[`\0a\t\x13\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xAAV[\x93\x92PPPV[`\0\x82\x85\x10\x15a\tVW`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\tb\x86\x86a\n\xE3V[\x90P`\0`\x02a\tr\x83\x8Aa\n\xF6V[a\t|\x90\x84a\n\xF6V[a\t\x86\x91\x90a\x0B\rV[\x90P`\0\x88a\t\x95\x87\x8Aa\n\xE3V[a\t\x9F\x91\x90a\n\xF6V[a\t\xB1\x86g\r\xE0\xB6\xB3\xA7d\0\0a\n\xF6V[a\t\xBB\x91\x90a\x0BHV[\x90P`\0a\t\xC9\x84\x83a\n\xF6V[\x90Pa\t\xD5\x81\x84a\x0BHV[\x9A\x99PPPPPPPPPPV[`\0a\t\x13\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\x0BW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03JWa\x03Ja\n\xB4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03JWa\x03Ja\n\xB4V[`\0\x82a\x0BCW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03JWa\x03Ja\n\xB4V\xFE\xA2dipfsX\"\x12 \ty\xC9O:Gg\xCF\x0E\xFB\x8A\x10g\x16;\xBF\xAFX\x94\xD2\x91\t\xEC\xD5\xF5\xD4\xDD\x9B\x1E\xFA\xB2\xEBdsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 \xE1\x89}%NmP\x03p\xB0\x11\"\xF6\x06\x12\xCD\xB3I\xF6\xB7\xFC\x88Hy\xBFZ.\x8A\x1C2sQdsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static COSTMODELJUMPRATEFACTORY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xC3W`\x005`\xE0\x1C\x80c\x90\x18K\x02\x14a\x01JW\x80c\xA4\x82B\xC5\x14a\x01\x82W\x80c\xBFP\xA5\x8D\x14a\x01\xBAW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ma\x01X6`\x04a\x05#V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x95a\x01\x906`\x04a\x05cV[a\x01\xCDV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01yV[a\x01\x95a\x01\xC86`\x04a\x05cV[a\x02\xB7V[`@\x80Q` \x81\x01\x86\x90R\x90\x81\x01\x84\x90R``\x81\x01\x83\x90R`\x80\x81\x01\x82\x90R`\0\x90\x81\x90`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x02s`@Q\x80` \x01a\x02\x1A\x90a\x04\x91V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x830\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11ma\x03\xC4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16a\x02\xAAW`\0a\x02\xACV[\x80[\x97\x96PPPPPPPV[`\0\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m\x85\x85\x85\x85`@Qa\x02\xEA\x90a\x04\x91V[\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x03\"W=`\0\x80>=`\0\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90U\x81Q\x89\x81R\x90\x81\x01\x88\x90R\x90\x81\x01\x86\x90R``\x81\x01\x85\x90R\x91\x92P\x90\x7F\x954\t\xED\xB2:\"\x96Smj\xDF\xB2\x90\xA9\xC7\x96U\xC0\xF9\xC7\xB8O\xFA\xA7\xDE\xE9\x88\xF0XbB\x90`\x80\x01`@Q\x80\x91\x03\x90\xA2\x94\x93PPPPV[`\0\x80\x85\x85`@Q` \x01a\x03\xDA\x92\x91\x90a\x05\xC8V[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x85\x01R``\x97\x90\x97\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16`!\x84\x01R`5\x83\x01\x95\x90\x95R`U\x80\x83\x01\x96\x90\x96R\x80Q\x80\x83\x03\x90\x96\x01\x86R`u\x90\x91\x01\x90RPP\x81Q\x91\x01 \x92\x91PPV[a\x0E@\x80a\x05\xE6\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x058Wa\x058a\x04\x9EV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05\\W`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x05|Wa\x05|a\x04\x9EV[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x81Q`\0[\x81\x81\x10\x15a\x05\xB9W` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x05\x9FV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x05\xDDa\x05\xD7\x83\x86a\x05\x98V[\x84a\x05\x98V[\x94\x93PPPPV\xFEa\x01\0`@R4\x80\x15a\0^W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x0E@8\x03\x80a\x0E@\x839\x81\x01`@\x81\x90Ra\0}\x91a\x018V[g\r\xE0\xB6\xB3\xA7d\0\0\x84\x11\x15a\0\xA6W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15a\0\xCFW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x11\x15a\x01!W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\xE0\x93\x90\x93R`\x80\x91\x90\x91R`\xA0R`\xC0Ra\x01\xB9V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01\x99W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x0B\x91a\x02\xAF`\09`\0\x81\x81a\x02W\x01R\x81\x81a\x04J\x01R\x81\x81a\x04\x9A\x01R\x81\x81a\x04\xC4\x01R\x81\x81a\x04\xF6\x01R\x81\x81a\x05\xDD\x01R\x81\x81a\x06\x04\x01R\x81\x81a\x06,\x01R\x81\x81a\x06S\x01R\x81\x81a\x06\xBB\x01R\x81\x81a\x06\xE2\x01R\x81\x81a\x07\n\x01R\x81\x81a\x071\x01R\x81\x81a\x07Y\x01R\x81\x81a\x07\xDC\x01R\x81\x81a\x08\x0B\x01Ra\x08\x87\x01R`\0\x81\x81a\x02\n\x01R\x81\x81a\x04$\x01Ra\x08\xDA\x01R`\0\x81\x81a\x01\xAA\x01R\x81\x81a\x04r\x01R\x81\x81a\x05\x1D\x01R\x81\x81a\x07z\x01R\x81\x81a\x08U\x01Ra\x08\xB9\x01R`\0\x81\x81a\x01\xE3\x01R\x81\x81a\x03\xEE\x01R\x81\x81a\x05C\x01R\x81\x81a\x06}\x01Ra\x084\x01Ra\x0B\x91`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01\nW`\x005`\xE0\x1C\x80c\xCF\xD0\xFA\xC9\x11a\0\xDDW\x80c\xCF\xD0\xFA\xC9\x14a\x02\x05W\x80c\xD7\xC8V\xB3\x14a\x02,W\x80c\xE05\xCB\xCA\x14a\x02?W\x80c\xFD-\xA39\x14a\x02RWa\x01\nV[\x80c/\xB5e\xE8\x14a\x01\x91W\x80c;\x1Be \x14a\x01\xA3W\x80c\xA2aN\x9F\x14a\x01\xA5W\x80c\xB9*b\x0F\x14a\x01\xDEW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01\xA3a\x01\x9F6`\x04a\n\x12V[PPV[\0[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01\xCCa\x02:6`\x04a\n\x12V[a\x02yV[a\x01\xCCa\x02M6`\x04a\n\x12V[a\x03PV[a\x01\xCC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x82\x82\x10\x15a\x02\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x02\xF7W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x0EWa\x03\x07\x82a\x03\xE4V[\x90Pa\x03JV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x03#\x85\x85a\n\xE3V[a\x03-\x91\x90a\n\xF6V[\x90Pa\x03F`\x01\x82a\x03?\x87\x87a\x05\x90V[\x91\x90a\x07\xAAV[\x91PP[\x92\x91PPV[`\0\x81\x83\x10\x15a\x03\x8CW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x03\x9BWP`\0a\x03JV[`\0a\x03\xA7\x83\x85a\x05\x90V[\x90P`\0a\x03\xB6`\0\x86a\x05\x90V[\x90P`\0a\x03\xCCg\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xF6V[\x90P\x81a\x03\xD9\x81\x83a\x0B\rV[\x97\x96PPPPPPPV[`\0\x81a\x04\x12WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x03a\x04HWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a\x04\x96WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90PV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x10a\x04\xEEWa\x04\xE9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a\n\xE3V[a\x04\xF0V[\x82[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x05AW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[\x90P`\0a\x05p\x85a\x07\xD8V[\x90P\x81a\x05}\x84\x83a\x08\xFEV[a\x05\x87\x91\x90a\x0BHV[\x95\x94PPPPPV[`\0\x82\x82\x10\x15a\x05\xCCW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xA1a\x05\xDB`\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06(W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06*V[\x85[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x06wW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x06yV[\x85[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90P`\0a\x07\x9Ea\x06\xB9g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xD8V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07\x06W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07\x08V[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\x07UW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x07WV[\x86[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\x1AV[\x90Pa\x05\x87\x81\x83a\x0BHV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x07\xC2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\x08\x7FWa\x03Ja\x08/`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[\x90a\t\xE3V[a\x03Ja\x08\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3V[a\x08y\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\n\xE3V[`\0a\t\x13\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\xAAV[\x93\x92PPPV[`\0\x82\x85\x10\x15a\tVW`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\tb\x86\x86a\n\xE3V[\x90P`\0`\x02a\tr\x83\x8Aa\n\xF6V[a\t|\x90\x84a\n\xF6V[a\t\x86\x91\x90a\x0B\rV[\x90P`\0\x88a\t\x95\x87\x8Aa\n\xE3V[a\t\x9F\x91\x90a\n\xF6V[a\t\xB1\x86g\r\xE0\xB6\xB3\xA7d\0\0a\n\xF6V[a\t\xBB\x91\x90a\x0BHV[\x90P`\0a\t\xC9\x84\x83a\n\xF6V[\x90Pa\t\xD5\x81\x84a\x0BHV[\x9A\x99PPPPPPPPPPV[`\0a\t\x13\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\x0BW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\n\xA5W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03JWa\x03Ja\n\xB4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03JWa\x03Ja\n\xB4V[`\0\x82a\x0BCW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x03JWa\x03Ja\n\xB4V\xFE\xA2dipfsX\"\x12 \ty\xC9O:Gg\xCF\x0E\xFB\x8A\x10g\x16;\xBF\xAFX\x94\xD2\x91\t\xEC\xD5\xF5\xD4\xDD\x9B\x1E\xFA\xB2\xEBdsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 \xE1\x89}%NmP\x03p\xB0\x11\"\xF6\x06\x12\xCD\xB3I\xF6\xB7\xFC\x88Hy\xBFZ.\x8A\x1C2sQdsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static COSTMODELJUMPRATEFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CostModelJumpRateFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CostModelJumpRateFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CostModelJumpRateFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CostModelJumpRateFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CostModelJumpRateFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CostModelJumpRateFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CostModelJumpRateFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COSTMODELJUMPRATEFACTORY_ABI.clone(),
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
                COSTMODELJUMPRATEFACTORY_ABI.clone(),
                COSTMODELJUMPRATEFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deployModel` (0xbf50a58d) function
        pub fn deploy_model(
            &self,
            kink: ::ethers::core::types::U256,
            cost_factor_at_zero_utilization: ::ethers::core::types::U256,
            cost_factor_at_kink_utilization: ::ethers::core::types::U256,
            cost_factor_at_full_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [191, 80, 165, 141],
                    (
                        kink,
                        cost_factor_at_zero_utilization,
                        cost_factor_at_kink_utilization,
                        cost_factor_at_full_utilization,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModel` (0xa48242c5) function
        pub fn get_model(
            &self,
            kink: ::ethers::core::types::U256,
            cost_factor_at_zero_utilization: ::ethers::core::types::U256,
            cost_factor_at_kink_utilization: ::ethers::core::types::U256,
            cost_factor_at_full_utilization: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [164, 130, 66, 197],
                    (
                        kink,
                        cost_factor_at_zero_utilization,
                        cost_factor_at_kink_utilization,
                        cost_factor_at_full_utilization,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isDeployed` (0x90184b02) function
        pub fn is_deployed(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([144, 24, 75, 2], p0)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DeployedCostModelJumpRate` event
        pub fn deployed_cost_model_jump_rate_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedCostModelJumpRateFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedCostModelJumpRateFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for CostModelJumpRateFactory<M>
    {
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
    #[ethevent(
        name = "DeployedCostModelJumpRate",
        abi = "DeployedCostModelJumpRate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeployedCostModelJumpRateFilter {
        #[ethevent(indexed)]
        pub cost_model: ::ethers::core::types::Address,
        pub kink: ::ethers::core::types::U256,
        pub cost_factor_at_zero_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_kink_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_full_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployModel` function with signature `deployModel(uint256,uint256,uint256,uint256)` and selector `0xbf50a58d`
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
        name = "deployModel",
        abi = "deployModel(uint256,uint256,uint256,uint256)"
    )]
    pub struct DeployModelCall {
        pub kink: ::ethers::core::types::U256,
        pub cost_factor_at_zero_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_kink_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_full_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getModel` function with signature `getModel(uint256,uint256,uint256,uint256)` and selector `0xa48242c5`
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
    #[ethcall(name = "getModel", abi = "getModel(uint256,uint256,uint256,uint256)")]
    pub struct GetModelCall {
        pub kink: ::ethers::core::types::U256,
        pub cost_factor_at_zero_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_kink_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_full_utilization: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isDeployed` function with signature `isDeployed(address)` and selector `0x90184b02`
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
    #[ethcall(name = "isDeployed", abi = "isDeployed(address)")]
    pub struct IsDeployedCall(pub ::ethers::core::types::Address);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CostModelJumpRateFactoryCalls {
        DeployModel(DeployModelCall),
        GetModel(GetModelCall),
        IsDeployed(IsDeployedCall),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelJumpRateFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DeployModelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeployModel(decoded));
            }
            if let Ok(decoded) = <GetModelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetModel(decoded));
            }
            if let Ok(decoded) = <IsDeployedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDeployed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CostModelJumpRateFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeployModel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetModel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDeployed(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CostModelJumpRateFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeployModel(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetModel(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDeployed(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeployModelCall> for CostModelJumpRateFactoryCalls {
        fn from(value: DeployModelCall) -> Self {
            Self::DeployModel(value)
        }
    }
    impl ::core::convert::From<GetModelCall> for CostModelJumpRateFactoryCalls {
        fn from(value: GetModelCall) -> Self {
            Self::GetModel(value)
        }
    }
    impl ::core::convert::From<IsDeployedCall> for CostModelJumpRateFactoryCalls {
        fn from(value: IsDeployedCall) -> Self {
            Self::IsDeployed(value)
        }
    }
    ///Container type for all return fields from the `deployModel` function with signature `deployModel(uint256,uint256,uint256,uint256)` and selector `0xbf50a58d`
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
    pub struct DeployModelReturn {
        pub model: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getModel` function with signature `getModel(uint256,uint256,uint256,uint256)` and selector `0xa48242c5`
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
    pub struct GetModelReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isDeployed` function with signature `isDeployed(address)` and selector `0x90184b02`
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
    pub struct IsDeployedReturn(pub bool);
}
