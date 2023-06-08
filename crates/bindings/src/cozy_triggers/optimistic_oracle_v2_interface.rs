pub use optimistic_oracle_v2_interface::*;
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
pub mod optimistic_oracle_v2_interface {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DisputePrice\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"expirationTimestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProposePrice\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"currency\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"finalFee\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"RequestPrice\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[],\"indexed\":false},{\"internalType\":\"int256\",\"name\":\"price\",\"type\":\"int256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"payout\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Settle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ancillaryBytesLimit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"defaultLiveness\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"disputePriceFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"finder\",\"outputs\":[{\"internalType\":\"contract FinderInterface\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentTime\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getRequest\",\"outputs\":[{\"internalType\":\"struct OptimisticOracleV2Interface.Request\",\"name\":\"\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"settled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct OptimisticOracleV2Interface.RequestSettings\",\"name\":\"requestSettings\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"eventBased\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"refundOnDispute\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceProposed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceDisputed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceSettled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"customLiveness\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"resolvedPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expirationTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"finalFee\",\"type\":\"uint256\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getState\",\"outputs\":[{\"internalType\":\"enum OptimisticOracleV2Interface.State\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"hasPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proposePriceFor\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"requestPrice\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"requests\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"proposer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"disputer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"currency\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"settled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"struct OptimisticOracleV2Interface.RequestSettings\",\"name\":\"requestSettings\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"bool\",\"name\":\"eventBased\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"refundOnDispute\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceProposed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceDisputed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceSettled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"customLiveness\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"int256\",\"name\":\"proposedPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"int256\",\"name\":\"resolvedPrice\",\"type\":\"int256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"expirationTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"reward\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"finalFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"bond\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setBond\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"totalBond\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceProposed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceDisputed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"callbackOnPriceSettled\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCallbacks\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"customLiveness\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setCustomLiveness\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setEventBased\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setRefundOnDispute\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settle\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"payout\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"identifier\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"timestamp\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"settleAndGetPrice\",\"outputs\":[{\"internalType\":\"int256\",\"name\":\"\",\"type\":\"int256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"ancillaryData\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"requester\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"stampAncillaryData\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static OPTIMISTICORACLEV2INTERFACE_ABI: ::ethers_contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers_contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct OptimisticOracleV2Interface<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for OptimisticOracleV2Interface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for OptimisticOracleV2Interface<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for OptimisticOracleV2Interface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for OptimisticOracleV2Interface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(OptimisticOracleV2Interface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> OptimisticOracleV2Interface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    OPTIMISTICORACLEV2INTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `ancillaryBytesLimit` (0xc371dda7) function
        pub fn ancillary_bytes_limit(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([195, 113, 221, 167], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `defaultLiveness` (0xfe4e1983) function
        pub fn default_liveness(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 78, 25, 131], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disputePrice` (0xfba7f1e3) function
        pub fn dispute_price(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [251, 167, 241, 227],
                    (requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `disputePriceFor` (0x76c7823f) function
        pub fn dispute_price_for(
            &self,
            disputer: ::ethers::core::types::Address,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [118, 199, 130, 63],
                    (disputer, requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finder` (0xb9a3c84c) function
        pub fn finder(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([185, 163, 200, 76], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCurrentTime` (0x29cb924d) function
        pub fn get_current_time(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([41, 203, 146, 77], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRequest` (0xa9904f9b) function
        pub fn get_request(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, Request> {
            self.0
                .method_hash(
                    [169, 144, 79, 155],
                    (requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getState` (0xba4b930c) function
        pub fn get_state(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash(
                    [186, 75, 147, 12],
                    (requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hasPrice` (0xbc58ccaa) function
        pub fn has_price(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [188, 88, 204, 170],
                    (requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposePrice` (0xb8b4f908) function
        pub fn propose_price(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            proposed_price: ::ethers::core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [184, 180, 249, 8],
                    (requester, identifier, timestamp, ancillary_data, proposed_price),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposePriceFor` (0x7c82288f) function
        pub fn propose_price_for(
            &self,
            proposer: ::ethers::core::types::Address,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            proposed_price: ::ethers::core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [124, 130, 40, 143],
                    (
                        proposer,
                        requester,
                        identifier,
                        timestamp,
                        ancillary_data,
                        proposed_price,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requestPrice` (0x11df92f1) function
        pub fn request_price(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            currency: ::ethers::core::types::Address,
            reward: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [17, 223, 146, 241],
                    (identifier, timestamp, ancillary_data, currency, reward),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `requests` (0x9d866985) function
        pub fn requests(
            &self,
            p0: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                bool,
                RequestSettings,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([157, 134, 105, 133], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setBond` (0xad5a755a) function
        pub fn set_bond(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            bond: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [173, 90, 117, 90],
                    (identifier, timestamp, ancillary_data, bond),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCallbacks` (0xf327b075) function
        pub fn set_callbacks(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            callback_on_price_proposed: bool,
            callback_on_price_disputed: bool,
            callback_on_price_settled: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [243, 39, 176, 117],
                    (
                        identifier,
                        timestamp,
                        ancillary_data,
                        callback_on_price_proposed,
                        callback_on_price_disputed,
                        callback_on_price_settled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setCustomLiveness` (0x473c45fe) function
        pub fn set_custom_liveness(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
            custom_liveness: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [71, 60, 69, 254],
                    (identifier, timestamp, ancillary_data, custom_liveness),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEventBased` (0x120698af) function
        pub fn set_event_based(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 6, 152, 175], (identifier, timestamp, ancillary_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setRefundOnDispute` (0x91f58dcb) function
        pub fn set_refund_on_dispute(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [145, 245, 141, 203],
                    (identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settle` (0x5e9a79a9) function
        pub fn settle(
            &self,
            requester: ::ethers::core::types::Address,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [94, 154, 121, 169],
                    (requester, identifier, timestamp, ancillary_data),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `settleAndGetPrice` (0x53b59239) function
        pub fn settle_and_get_price(
            &self,
            identifier: [u8; 32],
            timestamp: ::ethers::core::types::U256,
            ancillary_data: ::ethers::core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([83, 181, 146, 57], (identifier, timestamp, ancillary_data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stampAncillaryData` (0xaf5d2f39) function
        pub fn stamp_ancillary_data(
            &self,
            ancillary_data: ::ethers::core::types::Bytes,
            requester: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([175, 93, 47, 57], (ancillary_data, requester))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `DisputePrice` event
        pub fn dispute_price_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DisputePriceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ProposePrice` event
        pub fn propose_price_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProposePriceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `RequestPrice` event
        pub fn request_price_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            RequestPriceFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Settle` event
        pub fn settle_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, SettleFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OptimisticOracleV2InterfaceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>>
    for OptimisticOracleV2Interface<M> {
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
        Hash
    )]
    #[ethevent(
        name = "DisputePrice",
        abi = "DisputePrice(address,address,address,bytes32,uint256,bytes,int256)"
    )]
    pub struct DisputePriceFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proposer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub disputer: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub proposed_price: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "ProposePrice",
        abi = "ProposePrice(address,address,bytes32,uint256,bytes,int256,uint256,address)"
    )]
    pub struct ProposePriceFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proposer: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub proposed_price: ::ethers::core::types::I256,
        pub expiration_timestamp: ::ethers::core::types::U256,
        pub currency: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "RequestPrice",
        abi = "RequestPrice(address,bytes32,uint256,bytes,address,uint256,uint256)"
    )]
    pub struct RequestPriceFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub currency: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
        pub final_fee: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers_contract::EthEvent,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "Settle",
        abi = "Settle(address,address,address,bytes32,uint256,bytes,int256,uint256)"
    )]
    pub struct SettleFilter {
        #[ethevent(indexed)]
        pub requester: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub proposer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub disputer: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub price: ::ethers::core::types::I256,
        pub payout: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimisticOracleV2InterfaceEvents {
        DisputePriceFilter(DisputePriceFilter),
        ProposePriceFilter(ProposePriceFilter),
        RequestPriceFilter(RequestPriceFilter),
        SettleFilter(SettleFilter),
    }
    impl ::ethers_contract::EthLogDecode for OptimisticOracleV2InterfaceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = DisputePriceFilter::decode_log(log) {
                return Ok(
                    OptimisticOracleV2InterfaceEvents::DisputePriceFilter(decoded),
                );
            }
            if let Ok(decoded) = ProposePriceFilter::decode_log(log) {
                return Ok(
                    OptimisticOracleV2InterfaceEvents::ProposePriceFilter(decoded),
                );
            }
            if let Ok(decoded) = RequestPriceFilter::decode_log(log) {
                return Ok(
                    OptimisticOracleV2InterfaceEvents::RequestPriceFilter(decoded),
                );
            }
            if let Ok(decoded) = SettleFilter::decode_log(log) {
                return Ok(OptimisticOracleV2InterfaceEvents::SettleFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for OptimisticOracleV2InterfaceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DisputePriceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProposePriceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RequestPriceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SettleFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DisputePriceFilter>
    for OptimisticOracleV2InterfaceEvents {
        fn from(value: DisputePriceFilter) -> Self {
            Self::DisputePriceFilter(value)
        }
    }
    impl ::core::convert::From<ProposePriceFilter>
    for OptimisticOracleV2InterfaceEvents {
        fn from(value: ProposePriceFilter) -> Self {
            Self::ProposePriceFilter(value)
        }
    }
    impl ::core::convert::From<RequestPriceFilter>
    for OptimisticOracleV2InterfaceEvents {
        fn from(value: RequestPriceFilter) -> Self {
            Self::RequestPriceFilter(value)
        }
    }
    impl ::core::convert::From<SettleFilter> for OptimisticOracleV2InterfaceEvents {
        fn from(value: SettleFilter) -> Self {
            Self::SettleFilter(value)
        }
    }
    ///Container type for all input parameters for the `ancillaryBytesLimit` function with signature `ancillaryBytesLimit()` and selector `0xc371dda7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ancillaryBytesLimit", abi = "ancillaryBytesLimit()")]
    pub struct AncillaryBytesLimitCall;
    ///Container type for all input parameters for the `defaultLiveness` function with signature `defaultLiveness()` and selector `0xfe4e1983`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "defaultLiveness", abi = "defaultLiveness()")]
    pub struct DefaultLivenessCall;
    ///Container type for all input parameters for the `disputePrice` function with signature `disputePrice(address,bytes32,uint256,bytes)` and selector `0xfba7f1e3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "disputePrice",
        abi = "disputePrice(address,bytes32,uint256,bytes)"
    )]
    pub struct DisputePriceCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `disputePriceFor` function with signature `disputePriceFor(address,address,bytes32,uint256,bytes)` and selector `0x76c7823f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "disputePriceFor",
        abi = "disputePriceFor(address,address,bytes32,uint256,bytes)"
    )]
    pub struct DisputePriceForCall {
        pub disputer: ::ethers::core::types::Address,
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `finder` function with signature `finder()` and selector `0xb9a3c84c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "finder", abi = "finder()")]
    pub struct FinderCall;
    ///Container type for all input parameters for the `getCurrentTime` function with signature `getCurrentTime()` and selector `0x29cb924d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCurrentTime", abi = "getCurrentTime()")]
    pub struct GetCurrentTimeCall;
    ///Container type for all input parameters for the `getRequest` function with signature `getRequest(address,bytes32,uint256,bytes)` and selector `0xa9904f9b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRequest", abi = "getRequest(address,bytes32,uint256,bytes)")]
    pub struct GetRequestCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getState` function with signature `getState(address,bytes32,uint256,bytes)` and selector `0xba4b930c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getState", abi = "getState(address,bytes32,uint256,bytes)")]
    pub struct GetStateCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `hasPrice` function with signature `hasPrice(address,bytes32,uint256,bytes)` and selector `0xbc58ccaa`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hasPrice", abi = "hasPrice(address,bytes32,uint256,bytes)")]
    pub struct HasPriceCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `proposePrice` function with signature `proposePrice(address,bytes32,uint256,bytes,int256)` and selector `0xb8b4f908`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "proposePrice",
        abi = "proposePrice(address,bytes32,uint256,bytes,int256)"
    )]
    pub struct ProposePriceCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub proposed_price: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `proposePriceFor` function with signature `proposePriceFor(address,address,bytes32,uint256,bytes,int256)` and selector `0x7c82288f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "proposePriceFor",
        abi = "proposePriceFor(address,address,bytes32,uint256,bytes,int256)"
    )]
    pub struct ProposePriceForCall {
        pub proposer: ::ethers::core::types::Address,
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub proposed_price: ::ethers::core::types::I256,
    }
    ///Container type for all input parameters for the `requestPrice` function with signature `requestPrice(bytes32,uint256,bytes,address,uint256)` and selector `0x11df92f1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "requestPrice",
        abi = "requestPrice(bytes32,uint256,bytes,address,uint256)"
    )]
    pub struct RequestPriceCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub currency: ::ethers::core::types::Address,
        pub reward: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "requests", abi = "requests(bytes32)")]
    pub struct RequestsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `setBond` function with signature `setBond(bytes32,uint256,bytes,uint256)` and selector `0xad5a755a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setBond", abi = "setBond(bytes32,uint256,bytes,uint256)")]
    pub struct SetBondCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub bond: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setCallbacks` function with signature `setCallbacks(bytes32,uint256,bytes,bool,bool,bool)` and selector `0xf327b075`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCallbacks",
        abi = "setCallbacks(bytes32,uint256,bytes,bool,bool,bool)"
    )]
    pub struct SetCallbacksCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub callback_on_price_proposed: bool,
        pub callback_on_price_disputed: bool,
        pub callback_on_price_settled: bool,
    }
    ///Container type for all input parameters for the `setCustomLiveness` function with signature `setCustomLiveness(bytes32,uint256,bytes,uint256)` and selector `0x473c45fe`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setCustomLiveness",
        abi = "setCustomLiveness(bytes32,uint256,bytes,uint256)"
    )]
    pub struct SetCustomLivenessCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub custom_liveness: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setEventBased` function with signature `setEventBased(bytes32,uint256,bytes)` and selector `0x120698af`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setEventBased", abi = "setEventBased(bytes32,uint256,bytes)")]
    pub struct SetEventBasedCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setRefundOnDispute` function with signature `setRefundOnDispute(bytes32,uint256,bytes)` and selector `0x91f58dcb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "setRefundOnDispute",
        abi = "setRefundOnDispute(bytes32,uint256,bytes)"
    )]
    pub struct SetRefundOnDisputeCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `settle` function with signature `settle(address,bytes32,uint256,bytes)` and selector `0x5e9a79a9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "settle", abi = "settle(address,bytes32,uint256,bytes)")]
    pub struct SettleCall {
        pub requester: ::ethers::core::types::Address,
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `settleAndGetPrice` function with signature `settleAndGetPrice(bytes32,uint256,bytes)` and selector `0x53b59239`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "settleAndGetPrice",
        abi = "settleAndGetPrice(bytes32,uint256,bytes)"
    )]
    pub struct SettleAndGetPriceCall {
        pub identifier: [u8; 32],
        pub timestamp: ::ethers::core::types::U256,
        pub ancillary_data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `stampAncillaryData` function with signature `stampAncillaryData(bytes,address)` and selector `0xaf5d2f39`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stampAncillaryData", abi = "stampAncillaryData(bytes,address)")]
    pub struct StampAncillaryDataCall {
        pub ancillary_data: ::ethers::core::types::Bytes,
        pub requester: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum OptimisticOracleV2InterfaceCalls {
        AncillaryBytesLimit(AncillaryBytesLimitCall),
        DefaultLiveness(DefaultLivenessCall),
        DisputePrice(DisputePriceCall),
        DisputePriceFor(DisputePriceForCall),
        Finder(FinderCall),
        GetCurrentTime(GetCurrentTimeCall),
        GetRequest(GetRequestCall),
        GetState(GetStateCall),
        HasPrice(HasPriceCall),
        ProposePrice(ProposePriceCall),
        ProposePriceFor(ProposePriceForCall),
        RequestPrice(RequestPriceCall),
        Requests(RequestsCall),
        SetBond(SetBondCall),
        SetCallbacks(SetCallbacksCall),
        SetCustomLiveness(SetCustomLivenessCall),
        SetEventBased(SetEventBasedCall),
        SetRefundOnDispute(SetRefundOnDisputeCall),
        Settle(SettleCall),
        SettleAndGetPrice(SettleAndGetPriceCall),
        StampAncillaryData(StampAncillaryDataCall),
    }
    impl ::ethers::core::abi::AbiDecode for OptimisticOracleV2InterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AncillaryBytesLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::AncillaryBytesLimit(decoded));
            }
            if let Ok(decoded)
                = <DefaultLivenessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DefaultLiveness(decoded));
            }
            if let Ok(decoded)
                = <DisputePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisputePrice(decoded));
            }
            if let Ok(decoded)
                = <DisputePriceForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DisputePriceFor(decoded));
            }
            if let Ok(decoded)
                = <FinderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Finder(decoded));
            }
            if let Ok(decoded)
                = <GetCurrentTimeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCurrentTime(decoded));
            }
            if let Ok(decoded)
                = <GetRequestCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetRequest(decoded));
            }
            if let Ok(decoded)
                = <GetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetState(decoded));
            }
            if let Ok(decoded)
                = <HasPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HasPrice(decoded));
            }
            if let Ok(decoded)
                = <ProposePriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposePrice(decoded));
            }
            if let Ok(decoded)
                = <ProposePriceForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProposePriceFor(decoded));
            }
            if let Ok(decoded)
                = <RequestPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RequestPrice(decoded));
            }
            if let Ok(decoded)
                = <RequestsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Requests(decoded));
            }
            if let Ok(decoded)
                = <SetBondCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetBond(decoded));
            }
            if let Ok(decoded)
                = <SetCallbacksCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetCallbacks(decoded));
            }
            if let Ok(decoded)
                = <SetCustomLivenessCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetCustomLiveness(decoded));
            }
            if let Ok(decoded)
                = <SetEventBasedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetEventBased(decoded));
            }
            if let Ok(decoded)
                = <SetRefundOnDisputeCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SetRefundOnDispute(decoded));
            }
            if let Ok(decoded)
                = <SettleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settle(decoded));
            }
            if let Ok(decoded)
                = <SettleAndGetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SettleAndGetPrice(decoded));
            }
            if let Ok(decoded)
                = <StampAncillaryDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::StampAncillaryData(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for OptimisticOracleV2InterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AncillaryBytesLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DefaultLiveness(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisputePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DisputePriceFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Finder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetCurrentTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetState(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HasPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposePriceFor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RequestPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Requests(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetBond(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetCallbacks(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetCustomLiveness(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetEventBased(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRefundOnDispute(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Settle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SettleAndGetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::StampAncillaryData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for OptimisticOracleV2InterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AncillaryBytesLimit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DefaultLiveness(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisputePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::DisputePriceFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::Finder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCurrentTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::HasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposePriceFor(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Requests(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetBond(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCallbacks(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetCustomLiveness(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEventBased(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRefundOnDispute(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Settle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SettleAndGetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::StampAncillaryData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AncillaryBytesLimitCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: AncillaryBytesLimitCall) -> Self {
            Self::AncillaryBytesLimit(value)
        }
    }
    impl ::core::convert::From<DefaultLivenessCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: DefaultLivenessCall) -> Self {
            Self::DefaultLiveness(value)
        }
    }
    impl ::core::convert::From<DisputePriceCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: DisputePriceCall) -> Self {
            Self::DisputePrice(value)
        }
    }
    impl ::core::convert::From<DisputePriceForCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: DisputePriceForCall) -> Self {
            Self::DisputePriceFor(value)
        }
    }
    impl ::core::convert::From<FinderCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: FinderCall) -> Self {
            Self::Finder(value)
        }
    }
    impl ::core::convert::From<GetCurrentTimeCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: GetCurrentTimeCall) -> Self {
            Self::GetCurrentTime(value)
        }
    }
    impl ::core::convert::From<GetRequestCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: GetRequestCall) -> Self {
            Self::GetRequest(value)
        }
    }
    impl ::core::convert::From<GetStateCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: GetStateCall) -> Self {
            Self::GetState(value)
        }
    }
    impl ::core::convert::From<HasPriceCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: HasPriceCall) -> Self {
            Self::HasPrice(value)
        }
    }
    impl ::core::convert::From<ProposePriceCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: ProposePriceCall) -> Self {
            Self::ProposePrice(value)
        }
    }
    impl ::core::convert::From<ProposePriceForCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: ProposePriceForCall) -> Self {
            Self::ProposePriceFor(value)
        }
    }
    impl ::core::convert::From<RequestPriceCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: RequestPriceCall) -> Self {
            Self::RequestPrice(value)
        }
    }
    impl ::core::convert::From<RequestsCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: RequestsCall) -> Self {
            Self::Requests(value)
        }
    }
    impl ::core::convert::From<SetBondCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: SetBondCall) -> Self {
            Self::SetBond(value)
        }
    }
    impl ::core::convert::From<SetCallbacksCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: SetCallbacksCall) -> Self {
            Self::SetCallbacks(value)
        }
    }
    impl ::core::convert::From<SetCustomLivenessCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: SetCustomLivenessCall) -> Self {
            Self::SetCustomLiveness(value)
        }
    }
    impl ::core::convert::From<SetEventBasedCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: SetEventBasedCall) -> Self {
            Self::SetEventBased(value)
        }
    }
    impl ::core::convert::From<SetRefundOnDisputeCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: SetRefundOnDisputeCall) -> Self {
            Self::SetRefundOnDispute(value)
        }
    }
    impl ::core::convert::From<SettleCall> for OptimisticOracleV2InterfaceCalls {
        fn from(value: SettleCall) -> Self {
            Self::Settle(value)
        }
    }
    impl ::core::convert::From<SettleAndGetPriceCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: SettleAndGetPriceCall) -> Self {
            Self::SettleAndGetPrice(value)
        }
    }
    impl ::core::convert::From<StampAncillaryDataCall>
    for OptimisticOracleV2InterfaceCalls {
        fn from(value: StampAncillaryDataCall) -> Self {
            Self::StampAncillaryData(value)
        }
    }
    ///Container type for all return fields from the `ancillaryBytesLimit` function with signature `ancillaryBytesLimit()` and selector `0xc371dda7`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AncillaryBytesLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `defaultLiveness` function with signature `defaultLiveness()` and selector `0xfe4e1983`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DefaultLivenessReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `disputePrice` function with signature `disputePrice(address,bytes32,uint256,bytes)` and selector `0xfba7f1e3`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DisputePriceReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `disputePriceFor` function with signature `disputePriceFor(address,address,bytes32,uint256,bytes)` and selector `0x76c7823f`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DisputePriceForReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `finder` function with signature `finder()` and selector `0xb9a3c84c`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FinderReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getCurrentTime` function with signature `getCurrentTime()` and selector `0x29cb924d`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetCurrentTimeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getRequest` function with signature `getRequest(address,bytes32,uint256,bytes)` and selector `0xa9904f9b`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRequestReturn(pub Request);
    ///Container type for all return fields from the `getState` function with signature `getState(address,bytes32,uint256,bytes)` and selector `0xba4b930c`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetStateReturn(pub u8);
    ///Container type for all return fields from the `hasPrice` function with signature `hasPrice(address,bytes32,uint256,bytes)` and selector `0xbc58ccaa`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HasPriceReturn(pub bool);
    ///Container type for all return fields from the `proposePrice` function with signature `proposePrice(address,bytes32,uint256,bytes,int256)` and selector `0xb8b4f908`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposePriceReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `proposePriceFor` function with signature `proposePriceFor(address,address,bytes32,uint256,bytes,int256)` and selector `0x7c82288f`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProposePriceForReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `requestPrice` function with signature `requestPrice(bytes32,uint256,bytes,address,uint256)` and selector `0x11df92f1`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RequestPriceReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `requests` function with signature `requests(bytes32)` and selector `0x9d866985`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RequestsReturn {
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub settled: bool,
        pub request_settings: RequestSettings,
        pub proposed_price: ::ethers::core::types::I256,
        pub resolved_price: ::ethers::core::types::I256,
        pub expiration_time: ::ethers::core::types::U256,
        pub reward: ::ethers::core::types::U256,
        pub final_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `setBond` function with signature `setBond(bytes32,uint256,bytes,uint256)` and selector `0xad5a755a`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SetBondReturn {
        pub total_bond: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `settle` function with signature `settle(address,bytes32,uint256,bytes)` and selector `0x5e9a79a9`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SettleReturn {
        pub payout: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `settleAndGetPrice` function with signature `settleAndGetPrice(bytes32,uint256,bytes)` and selector `0x53b59239`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SettleAndGetPriceReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `stampAncillaryData` function with signature `stampAncillaryData(bytes,address)` and selector `0xaf5d2f39`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StampAncillaryDataReturn(pub ::ethers::core::types::Bytes);
    ///`Request(address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Request {
        pub proposer: ::ethers::core::types::Address,
        pub disputer: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub settled: bool,
        pub request_settings: RequestSettings,
        pub proposed_price: ::ethers::core::types::I256,
        pub resolved_price: ::ethers::core::types::I256,
        pub expiration_time: ::ethers::core::types::U256,
        pub reward: ::ethers::core::types::U256,
        pub final_fee: ::ethers::core::types::U256,
    }
    ///`RequestSettings(bool,bool,bool,bool,bool,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        ::ethers_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RequestSettings {
        pub event_based: bool,
        pub refund_on_dispute: bool,
        pub callback_on_price_proposed: bool,
        pub callback_on_price_disputed: bool,
        pub callback_on_price_settled: bool,
        pub bond: ::ethers::core::types::U256,
        pub custom_liveness: ::ethers::core::types::U256,
    }
}
