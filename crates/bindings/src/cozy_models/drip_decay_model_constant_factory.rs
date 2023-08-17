pub use drip_decay_model_constant_factory::*;
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
pub mod drip_decay_model_constant_factory {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("deployModel"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deployModel"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_ratePerSecond"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_model"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract DripDecayModelConstant",),
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
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_ratePerSecond"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                ::std::borrow::ToOwned::to_owned("DeployedDripDecayModelConstant"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("DeployedDripDecayModelConstant",),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("costModel"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("ratePerSecond"),
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
    pub static DRIPDECAYMODELCONSTANTFACTORY_ABI: ::ethers_contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers_contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[Pa\tE\x80a\0m`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xC3W`\x005`\xE0\x1C\x80cD$\x96o\x14a\x01JW\x80cm6\x16\x94\x14a\x01\x87W\x80c\x90\x18K\x02\x14a\x01\x9AW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01]a\x01X6`\x04a\x04\xEAV[a\x01\xCDV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01]a\x01\x956`\x04a\x04\xEAV[a\x02\xB5V[a\x01\xBDa\x01\xA86`\x04a\x05\x06V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01~V[`\0\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m\x82`@Qa\x01\xFD\x90a\x04XV[\x90\x81R` \x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x02#W=`\0\x80>=`\0\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R` \x81\x90R`@\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x91\x92P\x90\x7F\x0E\x86:V\xB8W\xE0\xB32\x81\x02\xE9\xBB\x04'\xEDe\x92\xC3W8\x86\xB8\x9F\xC2p\xD66\xAE\xD1\xEA\x8F\x90a\x02\xA8\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x80\x82`@Q` \x01a\x02\xCB\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x03J`@Q\x80` \x01a\x02\xF1\x90a\x04XV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x830\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11ma\x03\x8BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16a\x03\x81W`\0a\x03\x83V[\x80[\x94\x93PPPPV[`\0\x80\x85\x85`@Q` \x01a\x03\xA1\x92\x91\x90a\x05vV[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x85\x01R``\x97\x90\x97\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16`!\x84\x01R`5\x83\x01\x95\x90\x95R`U\x80\x83\x01\x96\x90\x96R\x80Q\x80\x83\x03\x90\x96\x01\x86R`u\x90\x91\x01\x90RPP\x81Q\x91\x01 \x92\x91PPV[a\x03\x84\x80a\x05\x8C\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x04\xFFWa\x04\xFFa\x04eV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x1BWa\x05\x1Ba\x04eV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05?W`\0\x80\xFD[\x93\x92PPPV[`\0\x81Q`\0[\x81\x81\x10\x15a\x05gW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x05MV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x03\x83a\x05\x85\x83\x86a\x05FV[\x84a\x05FV\xFE`\xA0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x03\x848\x03\x80a\x03\x84\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x84V[`\x80Ra\0\xE8V[`\0` \x82\x84\x03\x12\x15a\0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[`\x80Qa\x02za\x01\n`\09`\0\x81\x81a\x01P\x01Ra\x01\x89\x01Ra\x02z`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xB8W`\x005`\xE0\x1C\x80c@kf'\x14a\x01?W\x80c\x8E\xFF\x1A\x98\x14a\x01\x84W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ra\x01M6`\x04a\x01\xABV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0` \x82\x84\x03\x12\x15a\x02=W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x82\xEE\xC7U\xC8\x8D\"E\xB9\xD8\xEB\xE8\xC8\x13-N\xB0\x10\x0B`\xD4\x91G\xEFg\xC1+;\x1FL\x97\x99dsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 \xF6\x92\xDD\x96\xC2\xA3\x8FC\x0F\x91\x8DA8k\xEB\x16\x9E\xA9\xAD\xB5\xC4\x13\xBA\xB3\x9A\xC2\xA2\x96Q\x8Fw\x86dsolcC\0\x08\x12\x003";
    /// The bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANTFACTORY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xC3W`\x005`\xE0\x1C\x80cD$\x96o\x14a\x01JW\x80cm6\x16\x94\x14a\x01\x87W\x80c\x90\x18K\x02\x14a\x01\x9AW[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01]a\x01X6`\x04a\x04\xEAV[a\x01\xCDV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01]a\x01\x956`\x04a\x04\xEAV[a\x02\xB5V[a\x01\xBDa\x01\xA86`\x04a\x05\x06V[`\0` \x81\x90R\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01~V[`\0\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11m\x82`@Qa\x01\xFD\x90a\x04XV[\x90\x81R` \x01\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x02#W=`\0\x80>=`\0\xFD[Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x81\x81R` \x81\x90R`@\x90\x81\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16`\x01\x17\x90UQ\x91\x92P\x90\x7F\x0E\x86:V\xB8W\xE0\xB32\x81\x02\xE9\xBB\x04'\xEDe\x92\xC3W8\x86\xB8\x9F\xC2p\xD66\xAE\xD1\xEA\x8F\x90a\x02\xA8\x90\x85\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2\x91\x90PV[`\0\x80\x82`@Q` \x01a\x02\xCB\x91\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0a\x03J`@Q\x80` \x01a\x02\xF1\x90a\x04XV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@R\x830\x7F\x04HR\xB2\xA6p\xAD\xE5@~x\xFB(c\xC5\x1D\xE9\xFC\xB9eB\xA0q\x86\xFE:\xED\xA6\xBB\x8A\x11ma\x03\x8BV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90\x91P`\xFF\x16a\x03\x81W`\0a\x03\x83V[\x80[\x94\x93PPPPV[`\0\x80\x85\x85`@Q` \x01a\x03\xA1\x92\x91\x90a\x05vV[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x85\x01R``\x97\x90\x97\x1B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\x16`!\x84\x01R`5\x83\x01\x95\x90\x95R`U\x80\x83\x01\x96\x90\x96R\x80Q\x80\x83\x03\x90\x96\x01\x86R`u\x90\x91\x01\x90RPP\x81Q\x91\x01 \x92\x91PPV[a\x03\x84\x80a\x05\x8C\x839\x01\x90V[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[`\0` \x82\x84\x03\x12\x15a\x04\xFFWa\x04\xFFa\x04eV[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x05\x1BWa\x05\x1Ba\x04eV[\x815s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x05?W`\0\x80\xFD[\x93\x92PPPV[`\0\x81Q`\0[\x81\x81\x10\x15a\x05gW` \x81\x85\x01\x81\x01Q\x86\x83\x01R\x01a\x05MV[P`\0\x93\x01\x92\x83RP\x90\x91\x90PV[`\0a\x03\x83a\x05\x85\x83\x86a\x05FV[\x84a\x05FV\xFE`\xA0`@R4\x80\x15a\0]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81Ra7\xB7`\xF1\x1B`d\x83\x01R`\x84\x82\xFD[P`@Qa\x03\x848\x03\x80a\x03\x84\x839\x81\x01`@\x81\x90Ra\0|\x91a\0\x84V[`\x80Ra\0\xE8V[`\0` \x82\x84\x03\x12\x15a\0\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x81\xFD[PQ\x91\x90PV[`\x80Qa\x02za\x01\n`\09`\0\x81\x81a\x01P\x01Ra\x01\x89\x01Ra\x02z`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x92W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01\x90\x81R\x7Fon\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[P`\x046\x10a\0\xB8W`\x005`\xE0\x1C\x80c@kf'\x14a\x01?W\x80c\x8E\xFF\x1A\x98\x14a\x01\x84W[`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01\x90\x81R\x7Fnor receive functions\0\0\0\0\0\0\0\0\0\0\0`d\x83\x01R`\x84\x82\xFD[a\x01ra\x01M6`\x04a\x01\xABV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x01r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0` \x82\x84\x03\x12\x15a\x02=W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01R\x7Frt\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x81\xFD[P5\x91\x90PV\xFE\xA2dipfsX\"\x12 \x82\xEE\xC7U\xC8\x8D\"E\xB9\xD8\xEB\xE8\xC8\x13-N\xB0\x10\x0B`\xD4\x91G\xEFg\xC1+;\x1FL\x97\x99dsolcC\0\x08\x12\x003\xA2dipfsX\"\x12 \xF6\x92\xDD\x96\xC2\xA3\x8FC\x0F\x91\x8DA8k\xEB\x16\x9E\xA9\xAD\xB5\xC4\x13\xBA\xB3\x9A\xC2\xA2\x96Q\x8Fw\x86dsolcC\0\x08\x12\x003";
    /// The deployed bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANTFACTORY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DripDecayModelConstantFactory<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for DripDecayModelConstantFactory<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DripDecayModelConstantFactory<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DripDecayModelConstantFactory<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DripDecayModelConstantFactory<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DripDecayModelConstantFactory))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DripDecayModelConstantFactory<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                DRIPDECAYMODELCONSTANTFACTORY_ABI.clone(),
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
                DRIPDECAYMODELCONSTANTFACTORY_ABI.clone(),
                DRIPDECAYMODELCONSTANTFACTORY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `deployModel` (0x4424966f) function
        pub fn deploy_model(
            &self,
            rate_per_second: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([68, 36, 150, 111], rate_per_second)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getModel` (0x6d361694) function
        pub fn get_model(
            &self,
            rate_per_second: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([109, 54, 22, 148], rate_per_second)
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
        ///Gets the contract's `DeployedDripDecayModelConstant` event
        pub fn deployed_drip_decay_model_constant_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedDripDecayModelConstantFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeployedDripDecayModelConstantFilter,
        > {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for DripDecayModelConstantFactory<M>
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
        name = "DeployedDripDecayModelConstant",
        abi = "DeployedDripDecayModelConstant(address,uint256)"
    )]
    pub struct DeployedDripDecayModelConstantFilter {
        #[ethevent(indexed)]
        pub cost_model: ::ethers::core::types::Address,
        pub rate_per_second: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deployModel` function with signature `deployModel(uint256)` and selector `0x4424966f`
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
    #[ethcall(name = "deployModel", abi = "deployModel(uint256)")]
    pub struct DeployModelCall {
        pub rate_per_second: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getModel` function with signature `getModel(uint256)` and selector `0x6d361694`
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
    #[ethcall(name = "getModel", abi = "getModel(uint256)")]
    pub struct GetModelCall {
        pub rate_per_second: ::ethers::core::types::U256,
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
    pub enum DripDecayModelConstantFactoryCalls {
        DeployModel(DeployModelCall),
        GetModel(GetModelCall),
        IsDeployed(IsDeployedCall),
    }
    impl ::ethers::core::abi::AbiDecode for DripDecayModelConstantFactoryCalls {
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
    impl ::ethers::core::abi::AbiEncode for DripDecayModelConstantFactoryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DeployModel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetModel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsDeployed(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DripDecayModelConstantFactoryCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeployModel(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetModel(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsDeployed(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DeployModelCall> for DripDecayModelConstantFactoryCalls {
        fn from(value: DeployModelCall) -> Self {
            Self::DeployModel(value)
        }
    }
    impl ::core::convert::From<GetModelCall> for DripDecayModelConstantFactoryCalls {
        fn from(value: GetModelCall) -> Self {
            Self::GetModel(value)
        }
    }
    impl ::core::convert::From<IsDeployedCall> for DripDecayModelConstantFactoryCalls {
        fn from(value: IsDeployedCall) -> Self {
            Self::IsDeployed(value)
        }
    }
    ///Container type for all return fields from the `deployModel` function with signature `deployModel(uint256)` and selector `0x4424966f`
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
    ///Container type for all return fields from the `getModel` function with signature `getModel(uint256)` and selector `0x6d361694`
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
