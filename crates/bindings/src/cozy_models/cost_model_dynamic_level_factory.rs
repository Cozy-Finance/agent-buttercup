pub use cost_model_dynamic_level_factory::*;
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
pub mod cost_model_dynamic_level_factory {
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
                                name: ::std::borrow::ToOwned::to_owned("uLow_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("uHigh_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "costFactorAtZeroUtilization_",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned(
                                    "costFactorAtFullUtilization_",
                                ),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone_",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("optimalZoneRate_"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("model_"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract CostModelDynamicLevel",),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                ::std::borrow::ToOwned::to_owned("DeployedCostModelDynamicLevel"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("DeployedCostModelDynamicLevel",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costModel"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("uLow"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("uHigh"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorAtZeroUtilization",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorAtFullUtilization",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costFactorInOptimalZone",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("optimalZoneRate"),
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
    pub static COSTMODELDYNAMICLEVELFACTORY_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\x17\xC5\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xB8W`\x005`\xE0\x1C\x80c\x90\x18K\x02\x14a\x01?W\x80c\xE3\xDB\xFC2\x14a\x01wW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ba\x01M6`\x04a\x03\x12V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Aa\x01\x856`\x04a\x03RV[a\x01\xAFV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01nV[`\0\x86\x86\x86\x86\x86\x86`@Qa\x01\xC3\x90a\x02\x80V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x02\x04W=`\0\x80>=`\0\xFD[P`@\x80Q\x89\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xE2\xB7Q\xF3\xFD\xD7 \xD5\xCB\xDD1\xABln\xD7\x91y\x8B:\xA6\x07Q\xDA\xA6\xBF\x8C\xCA\xFAu+\xDD5\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA2\x96\x95PPPPPPV[a\x13\xF7\x80a\x03\x99\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03'Wa\x03'a\x02\x8DV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03KW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x03nWa\x03na\x02\x8DV[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV\xFEa\x01@`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x13\xF78\x03\x80b\0\x13\xF7\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x01\xA2V[g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15b\0\0\xACW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x86\x11\x15b\0\0\xCEW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15b\0\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x10\x15b\0\x01\x1AW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x86\x90R`\xA0\x85\x90Rb\0\x01M`\x01`\x02b\0\x018\x88\x8Ab\0\x028V[b\0\x01s` \x1Bb\0\x07\x1F\x17\x90\x92\x91\x90` \x1CV[`\xC0R`\xE0\x93\x90\x93Ra\x01\0\x91\x90\x91Ra\x01 \x91\x90\x91R`\0UPPB`\x01Ub\0\x02`V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16b\0\x01\x8CW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x02\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15b\0\x02ZWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x10Yb\0\x03\x9E`\09`\0\x81\x81a\x02\x02\x01Ra\x07\xB5\x01R`\0\x81\x81a\x03B\x01R\x81\x81a\x07\xE6\x01Ra\r\xD8\x01R`\0\x81\x81a\x02\xEB\x01R\x81\x81a\x08A\x01R\x81\x81a\x08o\x01R\x81\x81a\tJ\x01R\x81\x81a\nC\x01Ra\r\x07\x01R`\0\x81\x81a\x02\x89\x01R\x81\x81a\x07S\x01R\x81\x81a\x07\x84\x01Ra\x08\x16\x01R`\0\x81\x81a\x03\x12\x01R\x81\x81a\x08\xA5\x01R\x81\x81a\x08\xDE\x01R\x81\x81a\n\x9E\x01R\x81\x81a\x0B\x1A\x01R\x81\x81a\x0BA\x01R\x81\x81a\x0B\x9C\x01R\x81\x81a\x0B\xD0\x01R\x81\x81a\x0B\xF7\x01R\x81\x81a\x0C \x01R\x81\x81a\rA\x01R\x81\x81a\ry\x01Ra\r\xA5\x01R`\0\x81\x81a\x02<\x01R\x81\x81a\t\x1C\x01R\x81\x81a\t\xBD\x01R\x81\x81a\t\xF2\x01R\x81\x81a\n\x1F\x01R\x81\x81a\nu\x01R\x81\x81a\n\xCB\x01R\x81\x81a\n\xF2\x01R\x81\x81a\x0Bi\x01R\x81\x81a\x0C\x88\x01R\x81\x81a\x0C\xB0\x01Ra\x0C\xDE\x01Ra\x10Y`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01vW`\x005`\xE0\x1C\x80c\xAC\x02;u\x11a\x01\x19W\x80c\xCF\xD0\xFA\xC9\x11a\0\xE8W\x80c\xCF\xD0\xFA\xC9\x14a\x03=W\x80c\xD7\xC8V\xB3\x14a\x03dW\x80c\xE05\xCB\xCA\x14a\x03wW\x80c\xE6V\x92u\x14a\x03\x8AWa\x01vV[\x80c\xAC\x02;u\x14a\x02\xD3W\x80c\xB9*b\x0F\x14a\x02\xE6W\x80c\xBC\xC6\xF0\x12\x14a\x03\rW\x80c\xC8\xF3<\x91\x14a\x034Wa\x01vV[\x80c;\x1Be \x11a\x01UW\x80c;\x1Be \x14a\x02sW\x80cu\xDF(&\x14a\x02{W\x80c\x80\xA3\xAF6\x14a\x02\x84W\x80c\x86O\xD1\x99\x14a\x02\xABWa\x01vV[\x80b=\xFE`\x14a\x01\xFDW\x80c\x04\xAB6\xC9\x14a\x027W\x80c/\xB5e\xE8\x14a\x02^W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02qa\x02l6`\x04a\x0E\xDAV[a\x03\xCFV[\0[a\x02qa\x04wV[a\x02$`\0T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBEa\x02\xB96`\x04a\x0E\xDAV[a\x05\x19V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02.V[a\x02$a\x02\xE16`\x04a\x0E\xDAV[a\x05vV[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$`\x01T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$a\x03r6`\x04a\x0E\xDAV[a\x05\x91V[a\x02$a\x03\x856`\x04a\x0E\xDAV[a\x06yV[`\x02Ta\x03\xAA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02.V[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04 W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*B\x82a\x05\x19V[`\x01\x81\x90U`\0\x82\x90U`@Q\x7F\xB7C\x86\xE08\xAC\xE5\x1C\xE2\x8E\x1A;\x1D|\xB7\xB2\x03\x8B\xC9\x19\x8A|z\x1C}\xBD\x9C\xCB\x88\x9El\x1D\x92a\x04k\x92\x90\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x80\x15\x90a\x04\xB5WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14\x15[\x15a\x04\xECW`@Q\x7F\x88i\xBE,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UV[`\x01T`\0\x90\x81\x90\x80\x85\x10\x15a\x05[W`@Q\x7Fo~\xAC&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05n\x84a\x05i\x83\x88a\x0F\xABV[a\x07MV[\x95\x93PPPPV[`\0\x81\x83\x10\x15a\x05\x86W\x82a\x05\x88V[\x81[\x90P[\x92\x91PPV[`\0\x82\x82\x10\x15a\x05\xCDW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x06\x0FW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x1BB\x85a\x05\x19V[P\x90P\x82\x84\x03a\x067Wa\x06/\x81\x84a\x08\xA1V[\x91PPa\x05\x8BV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x06L\x86\x86a\x0F\xABV[a\x06V\x91\x90a\x0F\xBEV[\x90Pa\x06p`\x01\x82a\x06i\x88\x88\x87a\t}V[\x91\x90a\x07\x1FV[\x92PPPa\x05\x8BV[`\0\x81\x83\x10\x15a\x06\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x06\xC4WP`\0a\x05\x8BV[`\0a\x06\xD0B\x85a\x05\x19V[P\x90P`\0a\x06\xE0\x84\x86\x84a\t}V[\x90P`\0a\x06\xF0`\0\x87\x85a\t}V[\x90P`\0a\x07\x06g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0F\xBEV[\x90P\x81a\x07\x13\x81\x83a\x0F\xD5V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x077W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x08\nWa\x06/a\x07\xDA\x84a\x07\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x0F\xABV[a\x07\xB3\x91\x90a\x0F\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x0CoV[a\x07\xE4\x90\x83a\x10\x10V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05vV[`\0a\x08:\x84a\x07\xA9\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[\x90Pa\x08f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0F\xABV[\x81\x11\x15a\x08\x97W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92PPPa\x05\x8BV[a\x06p\x81\x83a\x0F\xABV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\t\x1AW\x82a\t\ta\x08\xD9\x85\x85a\x0C\x84V[a\t\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0CoV[a\t\x13\x91\x90a\x10\x10V[\x90Pa\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\tHWP\x81a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\ta\tv\x85\x85a\x0C\x84V[\x84\x90a\x0CoV[`\0\x83\x83\x10\x15a\t\xB9W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x11a\nlWa\nga\t\xEF\x84\x87a\x0C\x84V[\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\n\x1DW\x86a\n?V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xFCV[a\noV[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x10\x80a\n\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x11[a\x0B\x93Wa\x0B\x8E`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x11a\x0B\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x18V[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x10a\x0BeW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0BgV[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\r\xFCV[a\x0B\x96V[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x0CJWa\x0CEa\x0B\xCE\x86\x88a\x0C\x84V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x11a\x0C\x1BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\x1DV[\x88[\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89a\r\xFCV[a\x0CMV[`\0[\x90P\x80a\x0CZ\x83\x85a\x10\x10V[a\x0Cd\x91\x90a\x10\x10V[\x97\x96PPPPPPPV[`\0a\x05\x88\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x1FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10\x15a\r?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r7Wa\r2a\r\x02`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[a\r,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0E\xC5V[a\t\x13V[P`\0a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\rnWP`\0a\x05\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\r7Wa\r2a\r\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xABV[a\r,\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[`\0\x82\x85\x10\x15a\x0E8W`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0ED\x86\x86a\x0F\xABV[\x90P`\0`\x02a\x0ET\x83\x8Aa\x0F\xBEV[a\x0E^\x90\x84a\x0F\xBEV[a\x0Eh\x91\x90a\x0F\xD5V[\x90P`\0\x88a\x0Ew\x87\x8Aa\x0F\xABV[a\x0E\x81\x91\x90a\x0F\xBEV[a\x0E\x93\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xBEV[a\x0E\x9D\x91\x90a\x10\x10V[\x90P`\0a\x0E\xAB\x84\x83a\x0F\xBEV[\x90Pa\x0E\xB7\x81\x84a\x10\x10V[\x9A\x99PPPPPPPPPPV[`\0a\x05\x88\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8BWa\x05\x8Ba\x0F|V[`\0\x82a\x10\x0BW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V\xFE\xA2dipfsX\"\x12 \x96\xE4\xC7\xB9\xD6j\xB7[<_M\xD49\x88\xEBY\xD8\xB3\xFCX\xB6|B<u\xC7\xC5x[d\r9dsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 c\xFC^\x88 i\xB3w\xB4\xF5\x08\xE8L\x01c\xE6M(5\x1Ca\x84\xE2\xA8\xB4\x80\xC5\xC1e\xE1r\x81dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static COSTMODELDYNAMICLEVELFACTORY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xB8W`\x005`\xE0\x1C\x80c\x90\x18K\x02\x14a\x01?W\x80c\xE3\xDB\xFC2\x14a\x01wW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ba\x01M6`\x04a\x03\x12V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Aa\x01\x856`\x04a\x03RV[a\x01\xAFV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01nV[`\0\x86\x86\x86\x86\x86\x86`@Qa\x01\xC3\x90a\x02\x80V[\x95\x86R` \x86\x01\x94\x90\x94R`@\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15a\x02\x04W=`\0\x80>=`\0\xFD[P`@\x80Q\x89\x81R` \x81\x01\x89\x90R\x90\x81\x01\x87\x90R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\xA0\x81\x01\x84\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90\x7F\xE2\xB7Q\xF3\xFD\xD7 \xD5\xCB\xDD1\xABln\xD7\x91y\x8B:\xA6\x07Q\xDA\xA6\xBF\x8C\xCA\xFAu+\xDD5\x90`\xC0\x01`@Q\x80\x91\x03\x90\xA2\x96\x95PPPPPPV[a\x13\xF7\x80a\x03\x99\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x03'Wa\x03'a\x02\x8DV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03KW`\0\x80\xFD[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x03nWa\x03na\x02\x8DV[PP\x845\x96` \x86\x015\x96P`@\x86\x015\x95``\x81\x015\x95P`\x80\x81\x015\x94P`\xA0\x015\x92P\x90PV\xFEa\x01@`@R4\x80\x15b\0\0_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qb\0\x13\xF78\x03\x80b\0\x13\xF7\x839\x81\x01`@\x81\x90Rb\0\0\x82\x91b\0\x01\xA2V[g\r\xE0\xB6\xB3\xA7d\0\0\x85\x11\x15b\0\0\xACW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84\x86\x11\x15b\0\0\xCEW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x11\x15b\0\0\xF8W`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x83\x10\x15b\0\x01\x1AW`@Qc\xC5*\x9B\xD3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x86\x90R`\xA0\x85\x90Rb\0\x01M`\x01`\x02b\0\x018\x88\x8Ab\0\x028V[b\0\x01s` \x1Bb\0\x07\x1F\x17\x90\x92\x91\x90` \x1CV[`\xC0R`\xE0\x93\x90\x93Ra\x01\0\x91\x90\x91Ra\x01 \x91\x90\x91R`\0UPPB`\x01Ub\0\x02`V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16b\0\x01\x8CW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15b\0\x02\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[\x86Q\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[\x80\x82\x01\x80\x82\x11\x15b\0\x02ZWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x10Yb\0\x03\x9E`\09`\0\x81\x81a\x02\x02\x01Ra\x07\xB5\x01R`\0\x81\x81a\x03B\x01R\x81\x81a\x07\xE6\x01Ra\r\xD8\x01R`\0\x81\x81a\x02\xEB\x01R\x81\x81a\x08A\x01R\x81\x81a\x08o\x01R\x81\x81a\tJ\x01R\x81\x81a\nC\x01Ra\r\x07\x01R`\0\x81\x81a\x02\x89\x01R\x81\x81a\x07S\x01R\x81\x81a\x07\x84\x01Ra\x08\x16\x01R`\0\x81\x81a\x03\x12\x01R\x81\x81a\x08\xA5\x01R\x81\x81a\x08\xDE\x01R\x81\x81a\n\x9E\x01R\x81\x81a\x0B\x1A\x01R\x81\x81a\x0BA\x01R\x81\x81a\x0B\x9C\x01R\x81\x81a\x0B\xD0\x01R\x81\x81a\x0B\xF7\x01R\x81\x81a\x0C \x01R\x81\x81a\rA\x01R\x81\x81a\ry\x01Ra\r\xA5\x01R`\0\x81\x81a\x02<\x01R\x81\x81a\t\x1C\x01R\x81\x81a\t\xBD\x01R\x81\x81a\t\xF2\x01R\x81\x81a\n\x1F\x01R\x81\x81a\nu\x01R\x81\x81a\n\xCB\x01R\x81\x81a\n\xF2\x01R\x81\x81a\x0Bi\x01R\x81\x81a\x0C\x88\x01R\x81\x81a\x0C\xB0\x01Ra\x0C\xDE\x01Ra\x10Y`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\x01vW`\x005`\xE0\x1C\x80c\xAC\x02;u\x11a\x01\x19W\x80c\xCF\xD0\xFA\xC9\x11a\0\xE8W\x80c\xCF\xD0\xFA\xC9\x14a\x03=W\x80c\xD7\xC8V\xB3\x14a\x03dW\x80c\xE05\xCB\xCA\x14a\x03wW\x80c\xE6V\x92u\x14a\x03\x8AWa\x01vV[\x80c\xAC\x02;u\x14a\x02\xD3W\x80c\xB9*b\x0F\x14a\x02\xE6W\x80c\xBC\xC6\xF0\x12\x14a\x03\rW\x80c\xC8\xF3<\x91\x14a\x034Wa\x01vV[\x80c;\x1Be \x11a\x01UW\x80c;\x1Be \x14a\x02sW\x80cu\xDF(&\x14a\x02{W\x80c\x80\xA3\xAF6\x14a\x02\x84W\x80c\x86O\xD1\x99\x14a\x02\xABWa\x01vV[\x80b=\xFE`\x14a\x01\xFDW\x80c\x04\xAB6\xC9\x14a\x027W\x80c/\xB5e\xE8\x14a\x02^W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02qa\x02l6`\x04a\x0E\xDAV[a\x03\xCFV[\0[a\x02qa\x04wV[a\x02$`\0T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xBEa\x02\xB96`\x04a\x0E\xDAV[a\x05\x19V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x02.V[a\x02$a\x02\xE16`\x04a\x0E\xDAV[a\x05vV[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$`\x01T\x81V[a\x02$\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02$a\x03r6`\x04a\x0E\xDAV[a\x05\x91V[a\x02$a\x03\x856`\x04a\x0E\xDAV[a\x06yV[`\x02Ta\x03\xAA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02.V[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x04 W`@Q\x7F\x82\xB4)\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*B\x82a\x05\x19V[`\x01\x81\x90U`\0\x82\x90U`@Q\x7F\xB7C\x86\xE08\xAC\xE5\x1C\xE2\x8E\x1A;\x1D|\xB7\xB2\x03\x8B\xC9\x19\x8A|z\x1C}\xBD\x9C\xCB\x88\x9El\x1D\x92a\x04k\x92\x90\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xA1PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x15\x80\x15\x90a\x04\xB5WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x163\x14\x15[\x15a\x04\xECW`@Q\x7F\x88i\xBE,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x17\x90UV[`\x01T`\0\x90\x81\x90\x80\x85\x10\x15a\x05[W`@Q\x7Fo~\xAC&\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05n\x84a\x05i\x83\x88a\x0F\xABV[a\x07MV[\x95\x93PPPPV[`\0\x81\x83\x10\x15a\x05\x86W\x82a\x05\x88V[\x81[\x90P[\x92\x91PPV[`\0\x82\x82\x10\x15a\x05\xCDW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x06\x0FW`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x06\x1BB\x85a\x05\x19V[P\x90P\x82\x84\x03a\x067Wa\x06/\x81\x84a\x08\xA1V[\x91PPa\x05\x8BV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x06L\x86\x86a\x0F\xABV[a\x06V\x91\x90a\x0F\xBEV[\x90Pa\x06p`\x01\x82a\x06i\x88\x88\x87a\t}V[\x91\x90a\x07\x1FV[\x92PPPa\x05\x8BV[`\0\x81\x83\x10\x15a\x06\xB5W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x83\x03a\x06\xC4WP`\0a\x05\x8BV[`\0a\x06\xD0B\x85a\x05\x19V[P\x90P`\0a\x06\xE0\x84\x86\x84a\t}V[\x90P`\0a\x06\xF0`\0\x87\x85a\t}V[\x90P`\0a\x07\x06g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0F\xBEV[\x90P\x81a\x07\x13\x81\x83a\x0F\xD5V[\x98\x97PPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x077W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80T\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x10a\x08\nWa\x06/a\x07\xDA\x84a\x07\xA9\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\x0F\xABV[a\x07\xB3\x91\x90a\x0F\xBEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a\x0CoV[a\x07\xE4\x90\x83a\x10\x10V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x05vV[`\0a\x08:\x84a\x07\xA9\x87\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[\x90Pa\x08f\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x0F\xABV[\x81\x11\x15a\x08\x97W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x92PPPa\x05\x8BV[a\x06p\x81\x83a\x0F\xABV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\t\x1AW\x82a\t\ta\x08\xD9\x85\x85a\x0C\x84V[a\t\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0CoV[a\t\x13\x91\x90a\x10\x10V[\x90Pa\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\tHWP\x81a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\t\ta\tv\x85\x85a\x0C\x84V[\x84\x90a\x0CoV[`\0\x83\x83\x10\x15a\t\xB9W`@Q\x7F%\x06.%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x11a\nlWa\nga\t\xEF\x84\x87a\x0C\x84V[\x86\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x11a\n\x1DW\x86a\n?V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\r\xFCV[a\noV[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x10\x80a\n\xC0WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x11[a\x0B\x93Wa\x0B\x8E`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x11a\x0B\x16W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0B\x18V[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x10a\x0BeW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0BgV[\x87[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88a\r\xFCV[a\x0B\x96V[`\0[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x10a\x0CJWa\x0CEa\x0B\xCE\x86\x88a\x0C\x84V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89\x11a\x0C\x1BW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0C\x1DV[\x88[\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89a\r\xFCV[a\x0CMV[`\0[\x90P\x80a\x0CZ\x83\x85a\x10\x10V[a\x0Cd\x91\x90a\x10\x10V[\x97\x96PPPPPPPV[`\0a\x05\x88\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x07\x1FV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10\x15a\r?W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x15a\r7Wa\r2a\r\x02`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[a\r,\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a\x0F\xABV[\x90a\x0E\xC5V[a\t\x13V[P`\0a\x05\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11a\rnWP`\0a\x05\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x14a\r7Wa\r2a\r\xD2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xABV[a\r,\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x0F\xABV[`\0\x82\x85\x10\x15a\x0E8W`@Q\x7F\x03\x0B@\x11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0ED\x86\x86a\x0F\xABV[\x90P`\0`\x02a\x0ET\x83\x8Aa\x0F\xBEV[a\x0E^\x90\x84a\x0F\xBEV[a\x0Eh\x91\x90a\x0F\xD5V[\x90P`\0\x88a\x0Ew\x87\x8Aa\x0F\xABV[a\x0E\x81\x91\x90a\x0F\xBEV[a\x0E\x93\x86g\r\xE0\xB6\xB3\xA7d\0\0a\x0F\xBEV[a\x0E\x9D\x91\x90a\x10\x10V[\x90P`\0a\x0E\xAB\x84\x83a\x0F\xBEV[\x90Pa\x0E\xB7\x81\x84a\x10\x10V[\x9A\x99PPPPPPPPPPV[`\0a\x05\x88\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x07\x1FV[`\0\x80`@\x83\x85\x03\x12\x15a\x0FmW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x05\x8BWa\x05\x8Ba\x0F|V[`\0\x82a\x10\x0BW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[\x80\x82\x01\x80\x82\x11\x15a\x05\x8BWa\x05\x8Ba\x0F|V\xFE\xA2dipfsX\"\x12 \x96\xE4\xC7\xB9\xD6j\xB7[<_M\xD49\x88\xEBY\xD8\xB3\xFCX\xB6|B<u\xC7\xC5x[d\r9dsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 c\xFC^\x88 i\xB3w\xB4\xF5\x08\xE8L\x01c\xE6M(5\x1Ca\x84\xE2\xA8\xB4\x80\xC5\xC1e\xE1r\x81dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static COSTMODELDYNAMICLEVELFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct CostModelDynamicLevelFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for CostModelDynamicLevelFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CostModelDynamicLevelFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CostModelDynamicLevelFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CostModelDynamicLevelFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(CostModelDynamicLevelFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CostModelDynamicLevelFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                COSTMODELDYNAMICLEVELFACTORY_ABI.clone(),
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
                COSTMODELDYNAMICLEVELFACTORY_ABI.clone(),
                COSTMODELDYNAMICLEVELFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deployModel` (0xe3dbfc32) function
        pub fn deploy_model(
            &self,
            u_low: ::ethers::core::types::U256,
            u_high: ::ethers::core::types::U256,
            cost_factor_at_zero_utilization: ::ethers::core::types::U256,
            cost_factor_at_full_utilization: ::ethers::core::types::U256,
            cost_factor_in_optimal_zone: ::ethers::core::types::U256,
            optimal_zone_rate: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash(
                    [227, 219, 252, 50],
                    (
                        u_low,
                        u_high,
                        cost_factor_at_zero_utilization,
                        cost_factor_at_full_utilization,
                        cost_factor_in_optimal_zone,
                        optimal_zone_rate,
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
        ///Gets the contract's `DeployedCostModelDynamicLevel` event
        pub fn deployed_cost_model_dynamic_level_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedCostModelDynamicLevelFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedCostModelDynamicLevelFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for CostModelDynamicLevelFactory<M>
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
        name = "DeployedCostModelDynamicLevel",
        abi = "DeployedCostModelDynamicLevel(address,uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeployedCostModelDynamicLevelFilter {
        #[ethevent(indexed)]
        pub cost_model: ::ethers::core::types::Address,
        pub u_low: ::ethers::core::types::U256,
        pub u_high: ::ethers::core::types::U256,
        pub cost_factor_at_zero_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_full_utilization: ::ethers::core::types::U256,
        pub cost_factor_in_optimal_zone: ::ethers::core::types::U256,
        pub optimal_zone_rate: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployModel` function with signature `deployModel(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe3dbfc32`
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
        abi = "deployModel(uint256,uint256,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeployModelCall {
        pub u_low: ::ethers::core::types::U256,
        pub u_high: ::ethers::core::types::U256,
        pub cost_factor_at_zero_utilization: ::ethers::core::types::U256,
        pub cost_factor_at_full_utilization: ::ethers::core::types::U256,
        pub cost_factor_in_optimal_zone: ::ethers::core::types::U256,
        pub optimal_zone_rate: ::ethers::core::types::U256,
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
    pub enum CostModelDynamicLevelFactoryCalls {
        DeployModel(DeployModelCall),
        IsDeployed(IsDeployedCall),
    }
    impl ::ethers::core::abi::AbiDecode for CostModelDynamicLevelFactoryCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DeployModelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeployModel(decoded));
            }
            if let Ok(decoded) = <IsDeployedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsDeployed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CostModelDynamicLevelFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeployModel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDeployed(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for CostModelDynamicLevelFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeployModel(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDeployed(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeployModelCall> for CostModelDynamicLevelFactoryCalls {
        fn from(value: DeployModelCall) -> Self {
            Self::DeployModel(value)
        }
    }
    impl ::core::convert::From<IsDeployedCall> for CostModelDynamicLevelFactoryCalls {
        fn from(value: IsDeployedCall) -> Self {
            Self::IsDeployed(value)
        }
    }
    ///Container type for all return fields from the `deployModel` function with signature `deployModel(uint256,uint256,uint256,uint256,uint256,uint256)` and selector `0xe3dbfc32`
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
