pub use drip_decay_model_constant::*;
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
pub mod drip_decay_model_constant {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"_ratePerSecond\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"dripDecayRate\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ratePerSecond\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static DRIPDECAYMODELCONSTANT_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        160,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        16,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        64,
        81,
        97,
        1,
        91,
        56,
        3,
        128,
        97,
        1,
        91,
        131,
        57,
        129,
        1,
        96,
        64,
        129,
        144,
        82,
        97,
        0,
        47,
        145,
        97,
        0,
        55,
        86,
        91,
        96,
        128,
        82,
        97,
        0,
        80,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        0,
        73,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        81,
        145,
        144,
        80,
        86,
        91,
        96,
        128,
        81,
        96,
        237,
        97,
        0,
        110,
        96,
        0,
        57,
        96,
        0,
        129,
        129,
        96,
        69,
        1,
        82,
        96,
        125,
        1,
        82,
        96,
        237,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        64,
        107,
        102,
        39,
        20,
        96,
        55,
        87,
        128,
        99,
        142,
        255,
        26,
        152,
        20,
        96,
        121,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        103,
        96,
        66,
        54,
        96,
        4,
        96,
        159,
        86,
        91,
        80,
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
        144,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        103,
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
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        176,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        129,
        133,
        33,
        91,
        115,
        191,
        96,
        138,
        48,
        172,
        107,
        189,
        233,
        55,
        75,
        103,
        237,
        133,
        103,
        229,
        72,
        104,
        193,
        243,
        109,
        149,
        220,
        107,
        138,
        240,
        115,
        55,
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
        51,
    ];
    ///The bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANT_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        96,
        50,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        64,
        107,
        102,
        39,
        20,
        96,
        55,
        87,
        128,
        99,
        142,
        255,
        26,
        152,
        20,
        96,
        121,
        87,
        91,
        96,
        0,
        128,
        253,
        91,
        96,
        103,
        96,
        66,
        54,
        96,
        4,
        96,
        159,
        86,
        91,
        80,
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
        144,
        86,
        91,
        96,
        64,
        81,
        144,
        129,
        82,
        96,
        32,
        1,
        96,
        64,
        81,
        128,
        145,
        3,
        144,
        243,
        91,
        96,
        103,
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
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        96,
        176,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        53,
        145,
        144,
        80,
        86,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        129,
        133,
        33,
        91,
        115,
        191,
        96,
        138,
        48,
        172,
        107,
        189,
        233,
        55,
        75,
        103,
        237,
        133,
        103,
        229,
        72,
        104,
        193,
        243,
        109,
        149,
        220,
        107,
        138,
        240,
        115,
        55,
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
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static DRIPDECAYMODELCONSTANT_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DripDecayModelConstant<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for DripDecayModelConstant<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DripDecayModelConstant<M> {
        type Target = ::ethers_contract::Contract<M>;
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
            Self(::ethers_contract::Contract::new(
                address.into(),
                DRIPDECAYMODELCONSTANT_ABI.clone(),
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
                DRIPDECAYMODELCONSTANT_ABI.clone(),
                DRIPDECAYMODELCONSTANT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `dripDecayRate` (0x406b6627) function
        pub fn drip_decay_rate(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([64, 107, 102, 39], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ratePerSecond` (0x8eff1a98) function
        pub fn rate_per_second(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 255, 26, 152], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
        for DripDecayModelConstant<M>
    {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `dripDecayRate` function with signature `dripDecayRate(uint256)` and selector `0x406b6627`
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
    #[ethcall(name = "dripDecayRate", abi = "dripDecayRate(uint256)")]
    pub struct DripDecayRateCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `ratePerSecond` function with signature `ratePerSecond()` and selector `0x8eff1a98`
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
    #[ethcall(name = "ratePerSecond", abi = "ratePerSecond()")]
    pub struct RatePerSecondCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum DripDecayModelConstantCalls {
        DripDecayRate(DripDecayRateCall),
        RatePerSecond(RatePerSecondCall),
    }
    impl ::ethers::core::abi::AbiDecode for DripDecayModelConstantCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DripDecayRateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DripDecayRate(decoded));
            }
            if let Ok(decoded) = <RatePerSecondCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RatePerSecond(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DripDecayModelConstantCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DripDecayRate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RatePerSecond(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DripDecayRateReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `ratePerSecond` function with signature `ratePerSecond()` and selector `0x8eff1a98`
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
    pub struct RatePerSecondReturn(pub ::ethers::core::types::U256);
}
