pub use math_constants::*;
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
pub mod math_constants {
    #[rustfmt::skip]
    const __ABI: &str = "[]";
    ///The parsed JSON ABI of the contract.
    pub static MATHCONSTANTS_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        128,
        96,
        64,
        82,
        52,
        96,
        26,
        87,
        96,
        64,
        81,
        96,
        107,
        144,
        129,
        96,
        104,
        130,
        57,
        48,
        129,
        80,
        80,
        243,
        91,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        4,
        130,
        1,
        82,
        96,
        34,
        96,
        36,
        130,
        1,
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
        130,
        1,
        82,
        97,
        55,
        183,
        96,
        241,
        27,
        96,
        100,
        130,
        1,
        82,
        96,
        132,
        144,
        253,
        254,
        96,
        128,
        96,
        64,
        129,
        144,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        132,
        144,
        129,
        82,
        96,
        53,
        96,
        164,
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
        196,
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
        228,
        82,
        144,
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
    pub static MATHCONSTANTS_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        129,
        144,
        82,
        98,
        70,
        27,
        205,
        96,
        229,
        27,
        129,
        82,
        96,
        32,
        96,
        132,
        144,
        129,
        82,
        96,
        53,
        96,
        164,
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
        196,
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
        228,
        82,
        144,
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
    pub static MATHCONSTANTS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MathConstants<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for MathConstants<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MathConstants<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MathConstants<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MathConstants<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(MathConstants))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MathConstants<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                MATHCONSTANTS_ABI.clone(),
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
                MATHCONSTANTS_ABI.clone(),
                MATHCONSTANTS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers_contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for MathConstants<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
