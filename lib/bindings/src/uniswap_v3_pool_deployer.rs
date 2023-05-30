pub use uniswap_v3_pool_deployer::*;
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
pub mod uniswap_v3_pool_deployer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"parameters\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"factory\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token0\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token1\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint24\",\"name\":\"fee\",\"type\":\"uint24\",\"components\":[]},{\"internalType\":\"int24\",\"name\":\"tickSpacing\",\"type\":\"int24\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static UNISWAPV3POOLDEPLOYER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        86,
        87,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        34,
        96,
        36,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        80,
        97,
        1,
        133,
        128,
        97,
        0,
        102,
        96,
        0,
        57,
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
        97,
        0,
        86,
        87,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        34,
        96,
        36,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        113,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        137,
        3,
        87,
        48,
        20,
        97,
        0,
        207,
        87,
        91,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        97,
        0,
        215,
        97,
        1,
        25,
        86,
        91,
        96,
        64,
        128,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        150,
        135,
        22,
        129,
        82,
        148,
        134,
        22,
        96,
        32,
        134,
        1,
        82,
        146,
        144,
        148,
        22,
        131,
        131,
        1,
        82,
        98,
        255,
        255,
        255,
        22,
        96,
        96,
        131,
        1,
        82,
        96,
        2,
        146,
        144,
        146,
        11,
        96,
        128,
        130,
        1,
        82,
        144,
        81,
        144,
        129,
        144,
        3,
        96,
        160,
        1,
        144,
        243,
        91,
        96,
        0,
        84,
        96,
        1,
        84,
        96,
        2,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        147,
        132,
        22,
        147,
        146,
        131,
        22,
        146,
        129,
        22,
        145,
        98,
        255,
        255,
        255,
        96,
        1,
        96,
        160,
        27,
        131,
        4,
        22,
        145,
        96,
        1,
        96,
        184,
        27,
        144,
        4,
        144,
        11,
        133,
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
        116,
        160,
        72,
        161,
        87,
        208,
        136,
        184,
        86,
        254,
        0,
        92,
        89,
        242,
        244,
        119,
        255,
        41,
        166,
        192,
        56,
        108,
        86,
        30,
        172,
        68,
        4,
        183,
        44,
        71,
        140,
        29,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static UNISWAPV3POOLDEPLOYER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
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
        97,
        0,
        86,
        87,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        34,
        96,
        36,
        82,
        127,
        69,
        116,
        104,
        101,
        114,
        32,
        115,
        101,
        110,
        116,
        32,
        116,
        111,
        32,
        110,
        111,
        110,
        45,
        112,
        97,
        121,
        97,
        98,
        108,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        96,
        68,
        144,
        129,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        80,
        96,
        4,
        54,
        16,
        97,
        0,
        113,
        87,
        96,
        0,
        53,
        96,
        224,
        28,
        128,
        99,
        137,
        3,
        87,
        48,
        20,
        97,
        0,
        207,
        87,
        91,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        96,
        0,
        144,
        129,
        82,
        96,
        32,
        96,
        4,
        82,
        96,
        53,
        96,
        36,
        82,
        127,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        32,
        100,
        111,
        101,
        115,
        32,
        110,
        111,
        116,
        32,
        104,
        97,
        118,
        101,
        32,
        102,
        97,
        108,
        108,
        98,
        97,
        99,
        107,
        32,
        96,
        68,
        144,
        129,
        82,
        116,
        110,
        111,
        114,
        32,
        114,
        101,
        99,
        101,
        105,
        118,
        101,
        32,
        102,
        117,
        110,
        99,
        116,
        105,
        111,
        110,
        115,
        96,
        88,
        27,
        96,
        100,
        82,
        144,
        96,
        132,
        144,
        253,
        91,
        97,
        0,
        215,
        97,
        1,
        25,
        86,
        91,
        96,
        64,
        128,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        150,
        135,
        22,
        129,
        82,
        148,
        134,
        22,
        96,
        32,
        134,
        1,
        82,
        146,
        144,
        148,
        22,
        131,
        131,
        1,
        82,
        98,
        255,
        255,
        255,
        22,
        96,
        96,
        131,
        1,
        82,
        96,
        2,
        146,
        144,
        146,
        11,
        96,
        128,
        130,
        1,
        82,
        144,
        81,
        144,
        129,
        144,
        3,
        96,
        160,
        1,
        144,
        243,
        91,
        96,
        0,
        84,
        96,
        1,
        84,
        96,
        2,
        128,
        84,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        147,
        132,
        22,
        147,
        146,
        131,
        22,
        146,
        129,
        22,
        145,
        98,
        255,
        255,
        255,
        96,
        1,
        96,
        160,
        27,
        131,
        4,
        22,
        145,
        96,
        1,
        96,
        184,
        27,
        144,
        4,
        144,
        11,
        133,
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
        116,
        160,
        72,
        161,
        87,
        208,
        136,
        184,
        86,
        254,
        0,
        92,
        89,
        242,
        244,
        119,
        255,
        41,
        166,
        192,
        56,
        108,
        86,
        30,
        172,
        68,
        4,
        183,
        44,
        71,
        140,
        29,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        7,
        6,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static UNISWAPV3POOLDEPLOYER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct UniswapV3PoolDeployer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for UniswapV3PoolDeployer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for UniswapV3PoolDeployer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for UniswapV3PoolDeployer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for UniswapV3PoolDeployer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(UniswapV3PoolDeployer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> UniswapV3PoolDeployer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    UNISWAPV3POOLDEPLOYER_ABI.clone(),
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
                UNISWAPV3POOLDEPLOYER_ABI.clone(),
                UNISWAPV3POOLDEPLOYER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `parameters` (0x89035730) function
        pub fn parameters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u32,
                i32,
            ),
        > {
            self.0
                .method_hash([137, 3, 87, 48], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for UniswapV3PoolDeployer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `parameters` function with signature `parameters()` and selector `0x89035730`
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
    #[ethcall(name = "parameters", abi = "parameters()")]
    pub struct ParametersCall;
    ///Container type for all return fields from the `parameters` function with signature `parameters()` and selector `0x89035730`
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
    pub struct ParametersReturn {
        pub factory: ::ethers::core::types::Address,
        pub token_0: ::ethers::core::types::Address,
        pub token_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_spacing: i32,
    }
}
