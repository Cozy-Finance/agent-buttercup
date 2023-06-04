pub use drip_decay_model_constant::*;
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
pub mod drip_decay_model_constant {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_ratePerSecond\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dripDecayRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ratePerSecond\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static DRIPDECAYMODELCONSTANT_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        160,
        52,
        97,
        0,
        94,
        87,
        96,
        31,
        97,
        1,
        51,
        56,
        129,
        144,
        3,
        145,
        130,
        1,
        96,
        31,
        25,
        22,
        131,
        1,
        145,
        96,
        1,
        96,
        1,
        96,
        64,
        27,
        3,
        131,
        17,
        132,
        132,
        16,
        23,
        97,
        0,
        99,
        87,
        128,
        132,
        146,
        96,
        32,
        148,
        96,
        64,
        82,
        131,
        57,
        129,
        1,
        3,
        18,
        97,
        0,
        94,
        87,
        81,
        96,
        128,
        82,
        96,
        64,
        81,
        96,
        185,
        144,
        129,
        97,
        0,
        122,
        130,
        57,
        96,
        128,
        81,
        129,
        129,
        129,
        96,
        73,
        1,
        82,
        96,
        133,
        1,
        82,
        243,
        91,
        96,
        0,
        128,
        253,
        91,
        99,
        78,
        72,
        123,
        113,
        96,
        224,
        27,
        96,
        0,
        82,
        96,
        65,
        96,
        4,
        82,
        96,
        36,
        96,
        0,
        253,
        254,
        96,
        128,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        21,
        96,
        18,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        144,
        129,
        53,
        96,
        224,
        28,
        144,
        129,
        99,
        64,
        107,
        102,
        39,
        20,
        96,
        111,
        87,
        80,
        99,
        142,
        255,
        26,
        152,
        20,
        96,
        52,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        96,
        108,
        87,
        128,
        96,
        3,
        25,
        54,
        1,
        18,
        96,
        108,
        87,
        96,
        32,
        96,
        64,
        81,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        96,
        168,
        87,
        96,
        32,
        54,
        96,
        3,
        25,
        1,
        18,
        96,
        168,
        87,
        96,
        32,
        144,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        243,
        91,
        80,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        10,
    ];
    ///The bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANT_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        96,
        4,
        54,
        16,
        21,
        96,
        18,
        87,
        96,
        0,
        128,
        253,
        91,
        96,
        0,
        144,
        129,
        53,
        96,
        224,
        28,
        144,
        129,
        99,
        64,
        107,
        102,
        39,
        20,
        96,
        111,
        87,
        80,
        99,
        142,
        255,
        26,
        152,
        20,
        96,
        52,
        87,
        96,
        0,
        128,
        253,
        91,
        52,
        96,
        108,
        87,
        128,
        96,
        3,
        25,
        54,
        1,
        18,
        96,
        108,
        87,
        96,
        32,
        96,
        64,
        81,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        243,
        91,
        128,
        253,
        91,
        144,
        80,
        52,
        96,
        168,
        87,
        96,
        32,
        54,
        96,
        3,
        25,
        1,
        18,
        96,
        168,
        87,
        96,
        32,
        144,
        127,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        129,
        82,
        243,
        91,
        80,
        128,
        253,
        254,
        161,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        18,
        0,
        10,
    ];
    ///The deployed bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DripDecayModelConstant<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DripDecayModelConstant<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DripDecayModelConstant<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DripDecayModelConstant<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DripDecayModelConstant<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(DripDecayModelConstant))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DripDecayModelConstant<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DRIPDECAYMODELCONSTANT_ABI.clone(),
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
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DRIPDECAYMODELCONSTANT_ABI.clone(),
                DRIPDECAYMODELCONSTANT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dripDecayRate` (0x406b6627) function
        pub fn drip_decay_rate(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 107, 102, 39], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ratePerSecond` (0x8eff1a98) function
        pub fn rate_per_second(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 255, 26, 152], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DripDecayModelConstant<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `dripDecayRate` function with signature `dripDecayRate(uint256)` and selector `0x406b6627`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "dripDecayRate", abi = "dripDecayRate(uint256)")]
    pub struct DripDecayRateCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `ratePerSecond` function with signature `ratePerSecond()` and selector `0x8eff1a98`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ratePerSecond", abi = "ratePerSecond()")]
    pub struct RatePerSecondCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DripDecayModelConstantCalls {
        DripDecayRate(DripDecayRateCall),
        RatePerSecond(RatePerSecondCall),
    }
    impl ::ethers::core::abi::AbiDecode for DripDecayModelConstantCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <DripDecayRateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DripDecayRate(decoded));
            }
            if let Ok(decoded)
                = <RatePerSecondCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RatePerSecond(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DripDecayModelConstantCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DripDecayRate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RatePerSecond(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for DripDecayModelConstantCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DripDecayRate(element) => ::core::fmt::Display::fmt(element, f),
                Self::RatePerSecond(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DripDecayRateCall> for DripDecayModelConstantCalls {
        fn from(value: DripDecayRateCall) -> Self {
            Self::DripDecayRate(value)
        }
    }
    impl ::core::convert::From<RatePerSecondCall> for DripDecayModelConstantCalls {
        fn from(value: RatePerSecondCall) -> Self {
            Self::RatePerSecond(value)
        }
    }
    ///Container type for all return fields from the `dripDecayRate` function with signature `dripDecayRate(uint256)` and selector `0x406b6627`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DripDecayRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ratePerSecond` function with signature `ratePerSecond()` and selector `0x8eff1a98`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RatePerSecondReturn(pub ::ethers::core::types::U256);
}
