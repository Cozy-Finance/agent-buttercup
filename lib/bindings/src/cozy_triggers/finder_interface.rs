pub use finder_interface::*;
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
pub mod finder_interface {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"interfaceName\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"implementationAddress\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeImplementationAddress\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"interfaceName\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getImplementationAddress\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static FINDERINTERFACE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct FinderInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FinderInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FinderInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FinderInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FinderInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(FinderInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FinderInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                FINDERINTERFACE_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `changeImplementationAddress` (0x31f9665e) function
        pub fn change_implementation_address(
            &self,
            interface_name: [u8; 32],
            implementation_address: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 249, 102, 94], (interface_name, implementation_address))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getImplementationAddress` (0xaafd5e40) function
        pub fn get_implementation_address(
            &self,
            interface_name: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([170, 253, 94, 64], interface_name)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for FinderInterface<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `changeImplementationAddress` function with signature `changeImplementationAddress(bytes32,address)` and selector `0x31f9665e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "changeImplementationAddress",
        abi = "changeImplementationAddress(bytes32,address)"
    )]
    pub struct ChangeImplementationAddressCall {
        pub interface_name: [u8; 32],
        pub implementation_address: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getImplementationAddress` function with signature `getImplementationAddress(bytes32)` and selector `0xaafd5e40`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getImplementationAddress",
        abi = "getImplementationAddress(bytes32)"
    )]
    pub struct GetImplementationAddressCall {
        pub interface_name: [u8; 32],
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FinderInterfaceCalls {
        ChangeImplementationAddress(ChangeImplementationAddressCall),
        GetImplementationAddress(GetImplementationAddressCall),
    }
    impl ::ethers::core::abi::AbiDecode for FinderInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ChangeImplementationAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeImplementationAddress(decoded));
            }
            if let Ok(decoded) =
                <GetImplementationAddressCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetImplementationAddress(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FinderInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ChangeImplementationAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetImplementationAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for FinderInterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ChangeImplementationAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetImplementationAddress(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ChangeImplementationAddressCall> for FinderInterfaceCalls {
        fn from(value: ChangeImplementationAddressCall) -> Self {
            Self::ChangeImplementationAddress(value)
        }
    }
    impl ::core::convert::From<GetImplementationAddressCall> for FinderInterfaceCalls {
        fn from(value: GetImplementationAddressCall) -> Self {
            Self::GetImplementationAddress(value)
        }
    }
    ///Container type for all return fields from the `getImplementationAddress` function with signature `getImplementationAddress(bytes32)` and selector `0xaafd5e40`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetImplementationAddressReturn(pub ::ethers::core::types::Address);
}
