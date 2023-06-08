pub use i_backstop::*;
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
pub mod i_backstop {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bool\",\"name\":\"status_\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"BackstopApprovalStatusUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Claim\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ISet\",\"name\":\"set_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isApproved\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct BackstopApproval[]\",\"name\":\"approvals_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ISet\",\"name\":\"set\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"status\",\"type\":\"bool\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateApprovals\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static IBACKSTOP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct IBackstop<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for IBackstop<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for IBackstop<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for IBackstop<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for IBackstop<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(IBackstop)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> IBackstop<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    IBACKSTOP_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `claim` (0x379607f5) function
        pub fn claim(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 150, 7, 245], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApproved` (0x673448dd) function
        pub fn is_approved(
            &self,
            set: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([103, 52, 72, 221], set)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateApprovals` (0xcddfdbda) function
        pub fn update_approvals(
            &self,
            approvals: ::std::vec::Vec<BackstopApproval>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([205, 223, 219, 218], approvals)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `BackstopApprovalStatusUpdated` event
        pub fn backstop_approval_status_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            BackstopApprovalStatusUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Claim` event
        pub fn claim_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            IBackstopEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for IBackstop<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "BackstopApprovalStatusUpdated",
        abi = "BackstopApprovalStatusUpdated(address,bool)"
    )]
    pub struct BackstopApprovalStatusUpdatedFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        pub status: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Claim", abi = "Claim(address,address,uint256)")]
    pub struct ClaimFilter {
        #[ethevent(indexed)]
        pub set: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBackstopEvents {
        BackstopApprovalStatusUpdatedFilter(BackstopApprovalStatusUpdatedFilter),
        ClaimFilter(ClaimFilter),
    }
    impl ::ethers::contract::EthLogDecode for IBackstopEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = BackstopApprovalStatusUpdatedFilter::decode_log(log) {
                return Ok(IBackstopEvents::BackstopApprovalStatusUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ClaimFilter::decode_log(log) {
                return Ok(IBackstopEvents::ClaimFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for IBackstopEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BackstopApprovalStatusUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BackstopApprovalStatusUpdatedFilter> for IBackstopEvents {
        fn from(value: BackstopApprovalStatusUpdatedFilter) -> Self {
            Self::BackstopApprovalStatusUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFilter> for IBackstopEvents {
        fn from(value: ClaimFilter) -> Self {
            Self::ClaimFilter(value)
        }
    }
    ///Container type for all input parameters for the `claim` function with signature `claim(uint256)` and selector `0x379607f5`
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
    #[ethcall(name = "claim", abi = "claim(uint256)")]
    pub struct ClaimCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApproved` function with signature `isApproved(address)` and selector `0x673448dd`
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
    #[ethcall(name = "isApproved", abi = "isApproved(address)")]
    pub struct IsApprovedCall {
        pub set: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `updateApprovals` function with signature `updateApprovals((address,bool)[])` and selector `0xcddfdbda`
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
    #[ethcall(name = "updateApprovals", abi = "updateApprovals((address,bool)[])")]
    pub struct UpdateApprovalsCall {
        pub approvals: ::std::vec::Vec<BackstopApproval>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum IBackstopCalls {
        Claim(ClaimCall),
        IsApproved(IsApprovedCall),
        UpdateApprovals(UpdateApprovalsCall),
    }
    impl ::ethers::core::abi::AbiDecode for IBackstopCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded)
                = <IsApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsApproved(decoded));
            }
            if let Ok(decoded)
                = <UpdateApprovalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UpdateApprovals(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for IBackstopCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UpdateApprovals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for IBackstopCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateApprovals(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ClaimCall> for IBackstopCalls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<IsApprovedCall> for IBackstopCalls {
        fn from(value: IsApprovedCall) -> Self {
            Self::IsApproved(value)
        }
    }
    impl ::core::convert::From<UpdateApprovalsCall> for IBackstopCalls {
        fn from(value: UpdateApprovalsCall) -> Self {
            Self::UpdateApprovals(value)
        }
    }
    ///Container type for all return fields from the `isApproved` function with signature `isApproved(address)` and selector `0x673448dd`
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
    pub struct IsApprovedReturn(pub bool);
    ///`BackstopApproval(address,bool)`
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
    pub struct BackstopApproval {
        pub set: ::ethers::core::types::Address,
        pub status: bool,
    }
}
