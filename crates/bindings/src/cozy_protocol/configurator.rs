pub use configurator::*;
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
pub mod configurator {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientBalance\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidConfiguration\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidStateTransition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RoundsToZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Unauthorized\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateTime_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateDeadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesQueued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"reserveFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"backstopFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"setOwnerFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesAccrued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"ptokenAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPauser_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PauserUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accounting\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"assetBalance\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedSetOwnerFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyReserveFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyBackstopFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalPurchasesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalSalesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"assetsPendingRedemption\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"asset\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"backstop\",\"outputs\":[{\"internalType\":\"contract IBackstop\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfMatured\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cumulativeMinted\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dripSupplierFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeUpdateConfigs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMints\",\"outputs\":[{\"internalType\":\"struct MintData[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint216\",\"name\":\"amount\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"time\",\"type\":\"uint40\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastConfigUpdate\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"queuedConfigUpdateHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"configUpdateTime\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"configUpdateDeadline\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"contract IManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"contract IPToken\",\"name\":\"ptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct MarketConfigStorage\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"enum MarketState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"activeProtection\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDecayRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDripRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"purchasesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"salesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastDecayTime\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mints\",\"outputs\":[{\"internalType\":\"uint216\",\"name\":\"amount\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"time\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauser\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ptokenFactory\",\"outputs\":[{\"internalType\":\"contract IPTokenFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setState\",\"outputs\":[{\"internalType\":\"enum SetState\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ITrigger\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerLookups\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"marketExists\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"marketId\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateConfigs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newPauser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePauser\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CONFIGURATOR_ABI: ::ethers_contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers_contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct Configurator<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for Configurator<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Configurator<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Configurator<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Configurator<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Configurator))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Configurator<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers_contract::Contract::new(
                address.into(),
                CONFIGURATOR_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accounting` (0x9624e83e) function
        pub fn accounting(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, (u128, u128, u128, u128, u128, u128, u128)>
        {
            self.0
                .method_hash([150, 36, 232, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `backstop` (0x7dea1817) function
        pub fn backstop(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([125, 234, 24, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfMatured` (0x3489b7a4) function
        pub fn balance_of_matured(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 137, 183, 164], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeMinted` (0xf6eab327) function
        pub fn cumulative_minted(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([246, 234, 179, 39], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dripSupplierFees` (0x1891661c) function
        pub fn drip_supplier_fees(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 145, 102, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeUpdateConfigs` (0x3ba6f8e3) function
        pub fn finalize_update_configs(
            &self,
            set_config: SetConfig,
            market_configs: ::std::vec::Vec<MarketConfig>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 166, 248, 227], (set_config, market_configs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMints` (0x74ce5671) function
        pub fn get_mints(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::vec::Vec<MintData>> {
            self.0
                .method_hash([116, 206, 86, 113], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastConfigUpdate` (0x57a1ad84) function
        pub fn last_config_update(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ([u8; 32], u64, u64)> {
            self.0
                .method_hash([87, 161, 173, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markets` (0xb1283e77) function
        pub fn markets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                MarketConfigStorage,
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                u128,
                u128,
                u64,
            ),
        > {
            self.0
                .method_hash([177, 40, 62, 119], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mints` (0x0bedd3a7) function
        pub fn mints(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, (::ethers::core::types::U256, u64)>
        {
            self.0
                .method_hash([11, 237, 211, 167], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauser` (0x9fd0506d) function
        pub fn pauser(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 208, 80, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ptokenFactory` (0x764d6892) function
        pub fn ptoken_factory(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([118, 77, 104, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x8b673e7a) function
        pub fn set_config(&self) -> ::ethers_contract::builders::ContractCall<M, (u32, u16)> {
            self.0
                .method_hash([139, 103, 62, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setState` (0x1203402f) function
        pub fn set_state(&self) -> ::ethers_contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([18, 3, 64, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers_contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerLookups` (0x58cfaac7) function
        pub fn trigger_lookups(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, (bool, u16)> {
            self.0
                .method_hash([88, 207, 170, 199], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateConfigs` (0xfa3ebe07) function
        pub fn update_configs(
            &self,
            set_config: SetConfig,
            market_configs: ::std::vec::Vec<MarketConfig>,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 62, 190, 7], (set_config, market_configs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePauser` (0x554bab3c) function
        pub fn update_pauser(
            &self,
            new_pauser: ::ethers::core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 75, 171, 60], new_pauser)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `ConfigUpdatesFinalized` event
        pub fn config_updates_finalized_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesFinalizedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConfigUpdatesQueued` event
        pub fn config_updates_queued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesQueuedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeesAccrued` event
        pub fn fees_accrued_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, FeesAccruedFilter> {
            self.0.event()
        }
        ///Gets the contract's `MarketCreated` event
        pub fn market_created_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, MarketCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PauserUpdated` event
        pub fn pauser_updated_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, PauserUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers_contract::builders::Event<::std::sync::Arc<M>, M, ConfiguratorEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers_contract::Contract<M>> for Configurator<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientBalance", abi = "InsufficientBalance()")]
    pub struct InsufficientBalance;
    ///Custom Error type `InvalidAddress` with signature `InvalidAddress()` and selector `0xe6c4247b`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidAddress", abi = "InvalidAddress()")]
    pub struct InvalidAddress;
    ///Custom Error type `InvalidConfiguration` with signature `InvalidConfiguration()` and selector `0xc52a9bd3`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidConfiguration", abi = "InvalidConfiguration()")]
    pub struct InvalidConfiguration;
    ///Custom Error type `InvalidState` with signature `InvalidState()` and selector `0xbaf3f0f7`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidState", abi = "InvalidState()")]
    pub struct InvalidState;
    ///Custom Error type `InvalidStateTransition` with signature `InvalidStateTransition()` and selector `0x8f9a780c`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidStateTransition", abi = "InvalidStateTransition()")]
    pub struct InvalidStateTransition;
    ///Custom Error type `RoundsToZero` with signature `RoundsToZero()` and selector `0xc440e0aa`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RoundsToZero", abi = "RoundsToZero()")]
    pub struct RoundsToZero;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers_contract::EthError,
        ::ethers_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConfiguratorErrors {
        InsufficientBalance(InsufficientBalance),
        InvalidAddress(InvalidAddress),
        InvalidConfiguration(InvalidConfiguration),
        InvalidState(InvalidState),
        InvalidStateTransition(InvalidStateTransition),
        RoundsToZero(RoundsToZero),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConfiguratorErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalance(decoded));
            }
            if let Ok(decoded) = <InvalidAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidAddress(decoded));
            }
            if let Ok(decoded) =
                <InvalidConfiguration as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidConfiguration(decoded));
            }
            if let Ok(decoded) = <InvalidState as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidState(decoded));
            }
            if let Ok(decoded) =
                <InvalidStateTransition as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidStateTransition(decoded));
            }
            if let Ok(decoded) = <RoundsToZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundsToZero(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfiguratorErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidStateTransition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundsToZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers_contract::ContractRevert for ConfiguratorErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InsufficientBalance as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidAddress as ::ethers_contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidConfiguration as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidState as ::ethers_contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidStateTransition as ::ethers_contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <RoundsToZero as ::ethers_contract::EthError>::selector() => true,
                _ if selector == <Unauthorized as ::ethers_contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConfiguratorErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidState(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStateTransition(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundsToZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ConfiguratorErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for ConfiguratorErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for ConfiguratorErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<InvalidConfiguration> for ConfiguratorErrors {
        fn from(value: InvalidConfiguration) -> Self {
            Self::InvalidConfiguration(value)
        }
    }
    impl ::core::convert::From<InvalidState> for ConfiguratorErrors {
        fn from(value: InvalidState) -> Self {
            Self::InvalidState(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for ConfiguratorErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<RoundsToZero> for ConfiguratorErrors {
        fn from(value: RoundsToZero) -> Self {
            Self::RoundsToZero(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for ConfiguratorErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
        name = "ConfigUpdatesFinalized",
        abi = "ConfigUpdatesFinalized((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct ConfigUpdatesFinalizedFilter {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
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
        name = "ConfigUpdatesQueued",
        abi = "ConfigUpdatesQueued((uint32,uint16),(address,address,address,uint16,uint16,uint16)[],uint256,uint256)"
    )]
    pub struct ConfigUpdatesQueuedFilter {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
        pub update_time: ::ethers::core::types::U256,
        pub update_deadline: ::ethers::core::types::U256,
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
    #[ethevent(name = "FeesAccrued", abi = "FeesAccrued(uint128,uint128,uint128)")]
    pub struct FeesAccruedFilter {
        pub reserve_fees: u128,
        pub backstop_fees: u128,
        pub set_owner_fees: u128,
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
    #[ethevent(name = "MarketCreated", abi = "MarketCreated(uint16,address)")]
    pub struct MarketCreatedFilter {
        #[ethevent(indexed)]
        pub market_id: u16,
        pub ptoken_address: ::ethers::core::types::Address,
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
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address,address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "PauserUpdated", abi = "PauserUpdated(address)")]
    pub struct PauserUpdatedFilter {
        #[ethevent(indexed)]
        pub new_pauser: ::ethers::core::types::Address,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConfiguratorEvents {
        ApprovalFilter(ApprovalFilter),
        ConfigUpdatesFinalizedFilter(ConfigUpdatesFinalizedFilter),
        ConfigUpdatesQueuedFilter(ConfigUpdatesQueuedFilter),
        FeesAccruedFilter(FeesAccruedFilter),
        MarketCreatedFilter(MarketCreatedFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PauserUpdatedFilter(PauserUpdatedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers_contract::EthLogDecode for ConfiguratorEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ConfigUpdatesFinalizedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::ConfigUpdatesFinalizedFilter(decoded));
            }
            if let Ok(decoded) = ConfigUpdatesQueuedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::ConfigUpdatesQueuedFilter(decoded));
            }
            if let Ok(decoded) = FeesAccruedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::FeesAccruedFilter(decoded));
            }
            if let Ok(decoded) = MarketCreatedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::MarketCreatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PauserUpdatedFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::PauserUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(ConfiguratorEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ConfiguratorEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigUpdatesFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigUpdatesQueuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesAccruedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for ConfiguratorEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ConfigUpdatesFinalizedFilter> for ConfiguratorEvents {
        fn from(value: ConfigUpdatesFinalizedFilter) -> Self {
            Self::ConfigUpdatesFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<ConfigUpdatesQueuedFilter> for ConfiguratorEvents {
        fn from(value: ConfigUpdatesQueuedFilter) -> Self {
            Self::ConfigUpdatesQueuedFilter(value)
        }
    }
    impl ::core::convert::From<FeesAccruedFilter> for ConfiguratorEvents {
        fn from(value: FeesAccruedFilter) -> Self {
            Self::FeesAccruedFilter(value)
        }
    }
    impl ::core::convert::From<MarketCreatedFilter> for ConfiguratorEvents {
        fn from(value: MarketCreatedFilter) -> Self {
            Self::MarketCreatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for ConfiguratorEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for ConfiguratorEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PauserUpdatedFilter> for ConfiguratorEvents {
        fn from(value: PauserUpdatedFilter) -> Self {
            Self::PauserUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for ConfiguratorEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
    #[ethcall(name = "accounting", abi = "accounting()")]
    pub struct AccountingCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `backstop` function with signature `backstop()` and selector `0x7dea1817`
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
    #[ethcall(name = "backstop", abi = "backstop()")]
    pub struct BackstopCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `balanceOfMatured` function with signature `balanceOfMatured(address)` and selector `0x3489b7a4`
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
    #[ethcall(name = "balanceOfMatured", abi = "balanceOfMatured(address)")]
    pub struct BalanceOfMaturedCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `cumulativeMinted` function with signature `cumulativeMinted(address,uint256)` and selector `0xf6eab327`
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
    #[ethcall(name = "cumulativeMinted", abi = "cumulativeMinted(address,uint256)")]
    pub struct CumulativeMintedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `dripSupplierFees` function with signature `dripSupplierFees()` and selector `0x1891661c`
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
    #[ethcall(name = "dripSupplierFees", abi = "dripSupplierFees()")]
    pub struct DripSupplierFeesCall;
    ///Container type for all input parameters for the `finalizeUpdateConfigs` function with signature `finalizeUpdateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])` and selector `0x3ba6f8e3`
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
        name = "finalizeUpdateConfigs",
        abi = "finalizeUpdateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct FinalizeUpdateConfigsCall {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
    }
    ///Container type for all input parameters for the `getMints` function with signature `getMints(address)` and selector `0x74ce5671`
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
    #[ethcall(name = "getMints", abi = "getMints(address)")]
    pub struct GetMintsCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastConfigUpdate` function with signature `lastConfigUpdate()` and selector `0x57a1ad84`
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
    #[ethcall(name = "lastConfigUpdate", abi = "lastConfigUpdate()")]
    pub struct LastConfigUpdateCall;
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    #[ethcall(name = "markets", abi = "markets(uint256)")]
    pub struct MarketsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `mints` function with signature `mints(address,uint256)` and selector `0x0bedd3a7`
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
    #[ethcall(name = "mints", abi = "mints(address,uint256)")]
    pub struct MintsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    #[ethcall(name = "pauser", abi = "pauser()")]
    pub struct PauserCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    ///Container type for all input parameters for the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    #[ethcall(name = "ptokenFactory", abi = "ptokenFactory()")]
    pub struct PtokenFactoryCall;
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    #[ethcall(name = "setConfig", abi = "setConfig()")]
    pub struct SetConfigCall;
    ///Container type for all input parameters for the `setState` function with signature `setState()` and selector `0x1203402f`
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
    #[ethcall(name = "setState", abi = "setState()")]
    pub struct SetStateCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    #[ethcall(name = "triggerLookups", abi = "triggerLookups(address)")]
    pub struct TriggerLookupsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `updateConfigs` function with signature `updateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])` and selector `0xfa3ebe07`
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
        name = "updateConfigs",
        abi = "updateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct UpdateConfigsCall {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
    }
    ///Container type for all input parameters for the `updatePauser` function with signature `updatePauser(address)` and selector `0x554bab3c`
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
    #[ethcall(name = "updatePauser", abi = "updatePauser(address)")]
    pub struct UpdatePauserCall {
        pub new_pauser: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ConfiguratorCalls {
        DomainSeparator(DomainSeparatorCall),
        AcceptOwnership(AcceptOwnershipCall),
        Accounting(AccountingCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        Backstop(BackstopCall),
        BalanceOf(BalanceOfCall),
        BalanceOfMatured(BalanceOfMaturedCall),
        CumulativeMinted(CumulativeMintedCall),
        Decimals(DecimalsCall),
        DripSupplierFees(DripSupplierFeesCall),
        FinalizeUpdateConfigs(FinalizeUpdateConfigsCall),
        GetMints(GetMintsCall),
        LastConfigUpdate(LastConfigUpdateCall),
        Manager(ManagerCall),
        Markets(MarketsCall),
        Mints(MintsCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Pauser(PauserCall),
        PendingOwner(PendingOwnerCall),
        Permit(PermitCall),
        PtokenFactory(PtokenFactoryCall),
        SetConfig(SetConfigCall),
        SetState(SetStateCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TriggerLookups(TriggerLookupsCall),
        UpdateConfigs(UpdateConfigsCall),
        UpdatePauser(UpdatePauserCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConfiguratorCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) =
                <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <AccountingCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Accounting(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <BackstopCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Backstop(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfMaturedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOfMatured(decoded));
            }
            if let Ok(decoded) =
                <CumulativeMintedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeMinted(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) =
                <DripSupplierFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DripSupplierFees(decoded));
            }
            if let Ok(decoded) =
                <FinalizeUpdateConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FinalizeUpdateConfigs(decoded));
            }
            if let Ok(decoded) = <GetMintsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMints(decoded));
            }
            if let Ok(decoded) =
                <LastConfigUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastConfigUpdate(decoded));
            }
            if let Ok(decoded) = <ManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Manager(decoded));
            }
            if let Ok(decoded) = <MarketsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Markets(decoded));
            }
            if let Ok(decoded) = <MintsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mints(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauserCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pauser(decoded));
            }
            if let Ok(decoded) = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PendingOwner(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <PtokenFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PtokenFactory(decoded));
            }
            if let Ok(decoded) = <SetConfigCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetConfig(decoded));
            }
            if let Ok(decoded) = <SetStateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetState(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TriggerLookupsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TriggerLookups(decoded));
            }
            if let Ok(decoded) = <UpdateConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateConfigs(decoded));
            }
            if let Ok(decoded) = <UpdatePauserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdatePauser(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConfiguratorCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AcceptOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Accounting(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Backstop(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOfMatured(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeMinted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DripSupplierFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FinalizeUpdateConfigs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastConfigUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Markets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PtokenFactory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TriggerLookups(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateConfigs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ConfiguratorCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Accounting(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Backstop(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfMatured(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeMinted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::DripSupplierFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeUpdateConfigs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMints(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastConfigUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Markets(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtokenFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerLookups(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateConfigs(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePauser(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for ConfiguratorCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for ConfiguratorCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AccountingCall> for ConfiguratorCalls {
        fn from(value: AccountingCall) -> Self {
            Self::Accounting(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for ConfiguratorCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for ConfiguratorCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for ConfiguratorCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BackstopCall> for ConfiguratorCalls {
        fn from(value: BackstopCall) -> Self {
            Self::Backstop(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for ConfiguratorCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfMaturedCall> for ConfiguratorCalls {
        fn from(value: BalanceOfMaturedCall) -> Self {
            Self::BalanceOfMatured(value)
        }
    }
    impl ::core::convert::From<CumulativeMintedCall> for ConfiguratorCalls {
        fn from(value: CumulativeMintedCall) -> Self {
            Self::CumulativeMinted(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for ConfiguratorCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DripSupplierFeesCall> for ConfiguratorCalls {
        fn from(value: DripSupplierFeesCall) -> Self {
            Self::DripSupplierFees(value)
        }
    }
    impl ::core::convert::From<FinalizeUpdateConfigsCall> for ConfiguratorCalls {
        fn from(value: FinalizeUpdateConfigsCall) -> Self {
            Self::FinalizeUpdateConfigs(value)
        }
    }
    impl ::core::convert::From<GetMintsCall> for ConfiguratorCalls {
        fn from(value: GetMintsCall) -> Self {
            Self::GetMints(value)
        }
    }
    impl ::core::convert::From<LastConfigUpdateCall> for ConfiguratorCalls {
        fn from(value: LastConfigUpdateCall) -> Self {
            Self::LastConfigUpdate(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for ConfiguratorCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<MarketsCall> for ConfiguratorCalls {
        fn from(value: MarketsCall) -> Self {
            Self::Markets(value)
        }
    }
    impl ::core::convert::From<MintsCall> for ConfiguratorCalls {
        fn from(value: MintsCall) -> Self {
            Self::Mints(value)
        }
    }
    impl ::core::convert::From<NameCall> for ConfiguratorCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for ConfiguratorCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for ConfiguratorCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauserCall> for ConfiguratorCalls {
        fn from(value: PauserCall) -> Self {
            Self::Pauser(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for ConfiguratorCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for ConfiguratorCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PtokenFactoryCall> for ConfiguratorCalls {
        fn from(value: PtokenFactoryCall) -> Self {
            Self::PtokenFactory(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for ConfiguratorCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for ConfiguratorCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for ConfiguratorCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for ConfiguratorCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for ConfiguratorCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for ConfiguratorCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for ConfiguratorCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TriggerLookupsCall> for ConfiguratorCalls {
        fn from(value: TriggerLookupsCall) -> Self {
            Self::TriggerLookups(value)
        }
    }
    impl ::core::convert::From<UpdateConfigsCall> for ConfiguratorCalls {
        fn from(value: UpdateConfigsCall) -> Self {
            Self::UpdateConfigs(value)
        }
    }
    impl ::core::convert::From<UpdatePauserCall> for ConfiguratorCalls {
        fn from(value: UpdatePauserCall) -> Self {
            Self::UpdatePauser(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
    pub struct AccountingReturn {
        pub asset_balance: u128,
        pub accrued_set_owner_fees: u128,
        pub accrued_cozy_reserve_fees: u128,
        pub accrued_cozy_backstop_fees: u128,
        pub total_purchases_fees: u128,
        pub total_sales_fees: u128,
        pub assets_pending_redemption: u128,
    }
    ///Container type for all return fields from the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    ///Container type for all return fields from the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `backstop` function with signature `backstop()` and selector `0x7dea1817`
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
    pub struct BackstopReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfMatured` function with signature `balanceOfMatured(address)` and selector `0x3489b7a4`
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
    pub struct BalanceOfMaturedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `cumulativeMinted` function with signature `cumulativeMinted(address,uint256)` and selector `0xf6eab327`
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
    pub struct CumulativeMintedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `getMints` function with signature `getMints(address)` and selector `0x74ce5671`
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
    pub struct GetMintsReturn(pub ::std::vec::Vec<MintData>);
    ///Container type for all return fields from the `lastConfigUpdate` function with signature `lastConfigUpdate()` and selector `0x57a1ad84`
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
    pub struct LastConfigUpdateReturn {
        pub queued_config_update_hash: [u8; 32],
        pub config_update_time: u64,
        pub config_update_deadline: u64,
    }
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    pub struct MarketsReturn {
        pub ptoken: ::ethers::core::types::Address,
        pub trigger: ::ethers::core::types::Address,
        pub config: MarketConfigStorage,
        pub state: u8,
        pub active_protection: ::ethers::core::types::U256,
        pub last_decay_rate: ::ethers::core::types::U256,
        pub last_drip_rate: ::ethers::core::types::U256,
        pub purchases_fee_pool: u128,
        pub sales_fee_pool: u128,
        pub last_decay_time: u64,
    }
    ///Container type for all return fields from the `mints` function with signature `mints(address,uint256)` and selector `0x0bedd3a7`
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
    pub struct MintsReturn {
        pub amount: ::ethers::core::types::U256,
        pub time: u64,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    pub struct PauserReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    pub struct PtokenFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    pub struct SetConfigReturn {
        pub leverage_factor: u32,
        pub deposit_fee: u16,
    }
    ///Container type for all return fields from the `setState` function with signature `setState()` and selector `0x1203402f`
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
    pub struct SetStateReturn(pub u8);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    pub struct TriggerLookupsReturn {
        pub market_exists: bool,
        pub market_id: u16,
    }
}
