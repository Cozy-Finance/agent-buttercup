pub use set::*;
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
pub mod set {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"contract IManager\",\"name\":\"manager_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IPTokenFactory\",\"name\":\"ptokenFactory_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IBackstop\",\"name\":\"backstop_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"DelayNotElapsed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Initialized\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientAssets\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientBalance\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidConfiguration\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidDeposit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidPurchase\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidState\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidStateTransition\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RedemptionNotFound\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"maxRedeemableShares_\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"RedemptionRequestExceedsMax\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"RoundsToZero\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"SafeCastFailed\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Unauthorized\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnpackableString\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Approval\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IPToken\",\"name\":\"ptoken_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Claim\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesFinalized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateTime_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"updateDeadline_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ConfigUpdatesQueued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Deposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"reserveFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"backstopFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint128\",\"name\":\"setOwnerFees_\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeesAccrued\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"ptokenAddress_\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"MarketCreated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[],\"indexed\":true},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"enum MarketState\",\"name\":\"updatedTo_\",\"type\":\"uint8\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"MarketStateUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferStarted\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newPauser_\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"PauserUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IPToken\",\"name\":\"ptoken_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"cost_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Purchase\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redemptionId_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"Redeem\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"redemptionId_\",\"type\":\"uint256\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"RedemptionPending\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"contract IPToken\",\"name\":\"ptoken_\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"refund_\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Sell\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"enum SetState\",\"name\":\"updatedTo_\",\"type\":\"uint8\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SetStateUpdated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Transfer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"DOMAIN_SEPARATOR\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"acceptOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"accounting\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"assetBalance\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedSetOwnerFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyReserveFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"accruedCozyBackstopFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalPurchasesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"totalSalesFees\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"assetsPendingRedemption\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"allowance\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"approve\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"asset\",\"outputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"backstop\",\"outputs\":[{\"internalType\":\"contract IBackstop\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOf\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"balanceOfMatured\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claim\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"protection_\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"backstop_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimCozyFees\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"reserveAmount_\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopAmount_\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"caller_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"claimSetFees\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"setOwnerFees_\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"redemptionId_\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"completeRedeem\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"assetsRedeemed_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToAssets\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToPTokens\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToProtection\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"convertToShares\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"cumulativeMinted\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"decimals\",\"outputs\":[{\"internalType\":\"uint8\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct DepositFeesAssets\",\"name\":\"depositFeesAssets_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"reserveFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"setOwnerFeeAssets\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"dripSupplierFees\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"effectiveActiveProtection\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"finalizeUpdateConfigs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"user_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getMints\",\"outputs\":[{\"internalType\":\"struct MintData[]\",\"name\":\"\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint216\",\"name\":\"amount\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"time\",\"type\":\"uint40\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"pauser_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20\",\"name\":\"asset_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastConfigUpdate\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"queuedConfigUpdateHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"configUpdateTime\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"configUpdateDeadline\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"manager\",\"outputs\":[{\"internalType\":\"contract IManager\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"markets\",\"outputs\":[{\"internalType\":\"contract IPToken\",\"name\":\"ptoken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct MarketConfigStorage\",\"name\":\"config\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"enum MarketState\",\"name\":\"state\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"activeProtection\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDecayRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"lastDripRate\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"purchasesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"salesFeePool\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastDecayTime\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxDeposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"maxDeposit_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"maxRedemptionRequest\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"redeemableShares_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"mints\",\"outputs\":[{\"internalType\":\"uint216\",\"name\":\"amount\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"time\",\"type\":\"uint40\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"nonces\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"pause\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pauser\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingOwner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"spender\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"deadline\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"r\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"s\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"permit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"previewClaim\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"previewDeposit\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct DepositFeesAssets\",\"name\":\"depositFeesAssets_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"reserveFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"setOwnerFeeAssets\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"previewPurchase\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"assetsNeeded_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct PurchaseFeesAssets\",\"name\":\"purchaseFeesAssets_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"totalCost\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"cost\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"reserveFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"setOwnerFeeAssets\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"redemptionId_\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"previewRedemption\",\"outputs\":[{\"internalType\":\"struct RedemptionPreview\",\"name\":\"redemptionPreview_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint40\",\"name\":\"delayRemaining\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"uint216\",\"name\":\"shares\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"assets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"marketId_\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"previewSale\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"refund_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct SaleFeesAssets\",\"name\":\"saleFeesAssets_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"reserveFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"supplierFeeAssets\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"ptokenFactory\",\"outputs\":[{\"internalType\":\"contract IPTokenFactory\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"purchase\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct PurchaseFeesAssets\",\"name\":\"purchaseFeesAssets_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint128\",\"name\":\"totalCost\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"cost\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"reserveFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"backstopFeeAssets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"setOwnerFeeAssets\",\"type\":\"uint128\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"shares_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"redeem\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"redemptionId_\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"assets_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"redemptions\",\"outputs\":[{\"internalType\":\"uint40\",\"name\":\"queueTime\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"uint216\",\"name\":\"shares\",\"type\":\"uint216\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"assets\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint40\",\"name\":\"delay\",\"type\":\"uint40\",\"components\":[]},{\"internalType\":\"uint32\",\"name\":\"queuedAccISFsLength\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"queuedAccISF\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"remainingProtection\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IERC20\",\"name\":\"token_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"rescueFunds\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"ptokens_\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"receiver_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"owner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sell\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"refund_\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protection_\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setConfig\",\"outputs\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"setState\",\"outputs\":[{\"internalType\":\"enum SetState\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"symbol\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalCollateralAvailable\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"totalSupply\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transfer\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"from_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to_\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount_\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferFrom\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner_\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ITrigger\",\"name\":\"\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"triggerLookups\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"marketExists\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"marketId\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"unpause\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct SetConfig\",\"name\":\"setConfig_\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint32\",\"name\":\"leverageFactor\",\"type\":\"uint32\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"depositFee\",\"type\":\"uint16\",\"components\":[]}]},{\"internalType\":\"struct MarketConfig[]\",\"name\":\"marketConfigs_\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"contract ITrigger\",\"name\":\"trigger\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract ICostModel\",\"name\":\"costModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IDripDecayModel\",\"name\":\"dripDecayModel\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"weight\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"purchaseFee\",\"type\":\"uint16\",\"components\":[]},{\"internalType\":\"uint16\",\"name\":\"saleFee\",\"type\":\"uint16\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateConfigs\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"enum MarketState\",\"name\":\"newMarketState_\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updateMarketState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_newPauser\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"updatePauser\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"marketId_\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"utilization\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static SET_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(|| {
            ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid")
        });
    pub struct Set<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Set<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Set<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Set<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Set<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Set))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Set<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                SET_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `accounting` (0x9624e83e) function
        pub fn accounting(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128, u128, u128, u128, u128, u128)>
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `backstop` (0x7dea1817) function
        pub fn backstop(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([125, 234, 24, 23], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfMatured` (0x3489b7a4) function
        pub fn balance_of_matured(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([52, 137, 183, 164], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claim` (0x06af14f1) function
        pub fn claim(
            &self,
            market_id: u16,
            ptokens: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([6, 175, 20, 241], (market_id, ptokens, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimCozyFees` (0xbdf4fb4e) function
        pub fn claim_cozy_fees(
            &self,
            owner: ::ethers::core::types::Address,
            backstop: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([189, 244, 251, 78], (owner, backstop))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimSetFees` (0x1f1cc4c5) function
        pub fn claim_set_fees(
            &self,
            caller: ::ethers::core::types::Address,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([31, 28, 196, 197], (caller, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `completeRedeem` (0x99f51d86) function
        pub fn complete_redeem(
            &self,
            redemption_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([153, 245, 29, 134], redemption_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToAssets` (0x07a2d13a) function
        pub fn convert_to_assets(
            &self,
            shares: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([7, 162, 209, 58], shares)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToPTokens` (0x81c1a18c) function
        pub fn convert_to_p_tokens(
            &self,
            market_id: u16,
            protection: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 193, 161, 140], (market_id, protection))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToProtection` (0x1437feaa) function
        pub fn convert_to_protection(
            &self,
            market_id: u16,
            ptokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([20, 55, 254, 170], (market_id, ptokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `convertToShares` (0xc6e6f592) function
        pub fn convert_to_shares(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([198, 230, 245, 146], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `cumulativeMinted` (0xf6eab327) function
        pub fn cumulative_minted(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([246, 234, 179, 39], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deposit` (0x6e553f65) function
        pub fn deposit(
            &self,
            assets: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, DepositFeesAssets),
        > {
            self.0
                .method_hash([110, 85, 63, 101], (assets, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dripSupplierFees` (0x1891661c) function
        pub fn drip_supplier_fees(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 145, 102, 28], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `effectiveActiveProtection` (0x17766c81) function
        pub fn effective_active_protection(
            &self,
            market_id: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([23, 118, 108, 129], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `finalizeUpdateConfigs` (0x3ba6f8e3) function
        pub fn finalize_update_configs(
            &self,
            set_config: SetConfig,
            market_configs: ::std::vec::Vec<MarketConfig>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 166, 248, 227], (set_config, market_configs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMints` (0x74ce5671) function
        pub fn get_mints(
            &self,
            user: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<MintData>> {
            self.0
                .method_hash([116, 206, 86, 113], user)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x7e0f72fd) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
            pauser: ::ethers::core::types::Address,
            asset: ::ethers::core::types::Address,
            set_config: SetConfig,
            market_configs: ::std::vec::Vec<MarketConfig>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [126, 15, 114, 253],
                    (owner, pauser, asset, set_config, market_configs),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastConfigUpdate` (0x57a1ad84) function
        pub fn last_config_update(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], u64, u64)> {
            self.0
                .method_hash([87, 161, 173, 132], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `manager` (0x481c6a75) function
        pub fn manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([72, 28, 106, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `markets` (0xb1283e77) function
        pub fn markets(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
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
        ///Calls the contract's `maxDeposit` (0x6083e59a) function
        pub fn max_deposit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([96, 131, 229, 154], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `maxRedemptionRequest` (0xd8a1892f) function
        pub fn max_redemption_request(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([216, 161, 137, 47], owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mints` (0x0bedd3a7) function
        pub fn mints(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (::ethers::core::types::U256, u64)>
        {
            self.0
                .method_hash([11, 237, 211, 167], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x8456cb59) function
        pub fn pause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([132, 86, 203, 89], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauser` (0x9fd0506d) function
        pub fn pauser(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 208, 80, 109], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewClaim` (0xddaf883f) function
        pub fn preview_claim(
            &self,
            market_id: u16,
            ptokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([221, 175, 136, 63], (market_id, ptokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewDeposit` (0xef8b30f7) function
        pub fn preview_deposit(
            &self,
            assets: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, DepositFeesAssets),
        > {
            self.0
                .method_hash([239, 139, 48, 247], assets)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewPurchase` (0xc69f5180) function
        pub fn preview_purchase(
            &self,
            market_id: u16,
            protection: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                PurchaseFeesAssets,
            ),
        > {
            self.0
                .method_hash([198, 159, 81, 128], (market_id, protection))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewRedemption` (0x06de9b0f) function
        pub fn preview_redemption(
            &self,
            redemption_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, RedemptionPreview> {
            self.0
                .method_hash([6, 222, 155, 15], redemption_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `previewSale` (0x7c6a8909) function
        pub fn preview_sale(
            &self,
            market_id: u64,
            ptokens: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                SaleFeesAssets,
            ),
        > {
            self.0
                .method_hash([124, 106, 137, 9], (market_id, ptokens))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ptokenFactory` (0x764d6892) function
        pub fn ptoken_factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([118, 77, 104, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `purchase` (0xf09b77db) function
        pub fn purchase(
            &self,
            market_id: u16,
            protection: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, PurchaseFeesAssets),
        > {
            self.0
                .method_hash([240, 155, 119, 219], (market_id, protection, receiver))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redeem` (0xba087652) function
        pub fn redeem(
            &self,
            shares: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (u64, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([186, 8, 118, 82], (shares, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `redemptions` (0xbeb65893) function
        pub fn redemptions(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                ::ethers::core::types::U256,
                u128,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                u64,
                u32,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([190, 182, 88, 147], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `remainingProtection` (0x58b61432) function
        pub fn remaining_protection(
            &self,
            market_id: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([88, 182, 20, 50], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueFunds` (0x6ccae054) function
        pub fn rescue_funds_with_token(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 202, 224, 84], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rescueFunds` (0xe53b2017) function
        pub fn rescue_funds(
            &self,
            to: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 59, 32, 23], to)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sell` (0x3ae885f5) function
        pub fn sell(
            &self,
            market_id: u16,
            ptokens: ::ethers::core::types::U256,
            receiver: ::ethers::core::types::Address,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, ::ethers::core::types::U256)>
        {
            self.0
                .method_hash([58, 232, 133, 245], (market_id, ptokens, receiver, owner))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setConfig` (0x8b673e7a) function
        pub fn set_config(&self) -> ::ethers::contract::builders::ContractCall<M, (u32, u16)> {
            self.0
                .method_hash([139, 103, 62, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setState` (0x1203402f) function
        pub fn set_state(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([18, 3, 64, 47], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalCollateralAvailable` (0xaf04be85) function
        pub fn total_collateral_available(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 4, 190, 133], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `triggerLookups` (0x58cfaac7) function
        pub fn trigger_lookups(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, u16)> {
            self.0
                .method_hash([88, 207, 170, 199], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0x3f4ba83a) function
        pub fn unpause(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 75, 168, 58], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateConfigs` (0xfa3ebe07) function
        pub fn update_configs(
            &self,
            set_config: SetConfig,
            market_configs: ::std::vec::Vec<MarketConfig>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 62, 190, 7], (set_config, market_configs))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updateMarketState` (0x7a75fdde) function
        pub fn update_market_state(
            &self,
            new_market_state: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 117, 253, 222], new_market_state)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updatePauser` (0x554bab3c) function
        pub fn update_pauser(
            &self,
            new_pauser: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 75, 171, 60], new_pauser)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `utilization` (0xb18043a2) function
        pub fn utilization(
            &self,
            market_id: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([177, 128, 67, 162], market_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        ///Gets the contract's `Claim` event
        pub fn claim_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimFilter> {
            self.0.event()
        }
        ///Gets the contract's `ConfigUpdatesFinalized` event
        pub fn config_updates_finalized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesFinalizedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConfigUpdatesQueued` event
        pub fn config_updates_queued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConfigUpdatesQueuedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `FeesAccrued` event
        pub fn fees_accrued_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeesAccruedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MarketCreated` event
        pub fn market_created_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarketCreatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `MarketStateUpdated` event
        pub fn market_state_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MarketStateUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `PauserUpdated` event
        pub fn pauser_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauserUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Purchase` event
        pub fn purchase_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PurchaseFilter> {
            self.0.event()
        }
        ///Gets the contract's `Redeem` event
        pub fn redeem_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RedeemFilter> {
            self.0.event()
        }
        ///Gets the contract's `RedemptionPending` event
        pub fn redemption_pending_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RedemptionPendingFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Sell` event
        pub fn sell_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SellFilter> {
            self.0.event()
        }
        ///Gets the contract's `SetStateUpdated` event
        pub fn set_state_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetStateUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SetEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Set<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DelayNotElapsed` with signature `DelayNotElapsed()` and selector `0x27836e1f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DelayNotElapsed", abi = "DelayNotElapsed()")]
    pub struct DelayNotElapsed;
    ///Custom Error type `Initialized` with signature `Initialized()` and selector `0x5daa87a0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Initialized", abi = "Initialized()")]
    pub struct Initialized;
    ///Custom Error type `InsufficientAssets` with signature `InsufficientAssets()` and selector `0x96d80433`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InsufficientAssets", abi = "InsufficientAssets()")]
    pub struct InsufficientAssets;
    ///Custom Error type `InsufficientBalance` with signature `InsufficientBalance()` and selector `0xf4d678b8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidConfiguration", abi = "InvalidConfiguration()")]
    pub struct InvalidConfiguration;
    ///Custom Error type `InvalidDeposit` with signature `InvalidDeposit()` and selector `0xb2e532de`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidDeposit", abi = "InvalidDeposit()")]
    pub struct InvalidDeposit;
    ///Custom Error type `InvalidPurchase` with signature `InvalidPurchase()` and selector `0x53d13992`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidPurchase", abi = "InvalidPurchase()")]
    pub struct InvalidPurchase;
    ///Custom Error type `InvalidState` with signature `InvalidState()` and selector `0xbaf3f0f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidStateTransition", abi = "InvalidStateTransition()")]
    pub struct InvalidStateTransition;
    ///Custom Error type `RedemptionNotFound` with signature `RedemptionNotFound()` and selector `0x986a1905`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RedemptionNotFound", abi = "RedemptionNotFound()")]
    pub struct RedemptionNotFound;
    ///Custom Error type `RedemptionRequestExceedsMax` with signature `RedemptionRequestExceedsMax(uint256)` and selector `0x59818bce`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "RedemptionRequestExceedsMax",
        abi = "RedemptionRequestExceedsMax(uint256)"
    )]
    pub struct RedemptionRequestExceedsMax {
        pub max_redeemable_shares: ::ethers::core::types::U256,
    }
    ///Custom Error type `RoundsToZero` with signature `RoundsToZero()` and selector `0xc440e0aa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "RoundsToZero", abi = "RoundsToZero()")]
    pub struct RoundsToZero;
    ///Custom Error type `SafeCastFailed` with signature `SafeCastFailed()` and selector `0x45eef127`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SafeCastFailed", abi = "SafeCastFailed()")]
    pub struct SafeCastFailed;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Custom Error type `UnpackableString` with signature `UnpackableString()` and selector `0x40a42ef1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnpackableString", abi = "UnpackableString()")]
    pub struct UnpackableString;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SetErrors {
        DelayNotElapsed(DelayNotElapsed),
        Initialized(Initialized),
        InsufficientAssets(InsufficientAssets),
        InsufficientBalance(InsufficientBalance),
        InvalidAddress(InvalidAddress),
        InvalidConfiguration(InvalidConfiguration),
        InvalidDeposit(InvalidDeposit),
        InvalidPurchase(InvalidPurchase),
        InvalidState(InvalidState),
        InvalidStateTransition(InvalidStateTransition),
        RedemptionNotFound(RedemptionNotFound),
        RedemptionRequestExceedsMax(RedemptionRequestExceedsMax),
        RoundsToZero(RoundsToZero),
        SafeCastFailed(SafeCastFailed),
        Unauthorized(Unauthorized),
        UnpackableString(UnpackableString),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SetErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DelayNotElapsed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DelayNotElapsed(decoded));
            }
            if let Ok(decoded) = <Initialized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialized(decoded));
            }
            if let Ok(decoded) =
                <InsufficientAssets as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientAssets(decoded));
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
            if let Ok(decoded) = <InvalidDeposit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidDeposit(decoded));
            }
            if let Ok(decoded) = <InvalidPurchase as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPurchase(decoded));
            }
            if let Ok(decoded) = <InvalidState as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidState(decoded));
            }
            if let Ok(decoded) =
                <InvalidStateTransition as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidStateTransition(decoded));
            }
            if let Ok(decoded) =
                <RedemptionNotFound as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RedemptionNotFound(decoded));
            }
            if let Ok(decoded) =
                <RedemptionRequestExceedsMax as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RedemptionRequestExceedsMax(decoded));
            }
            if let Ok(decoded) = <RoundsToZero as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RoundsToZero(decoded));
            }
            if let Ok(decoded) = <SafeCastFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SafeCastFailed(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded) = <UnpackableString as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnpackableString(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DelayNotElapsed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientAssets(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidPurchase(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidStateTransition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedemptionNotFound(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RedemptionRequestExceedsMax(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RoundsToZero(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeCastFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnpackableString(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SetErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DelayNotElapsed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Initialized as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientAssets as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidConfiguration as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidDeposit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidPurchase as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidState as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidStateTransition as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RedemptionNotFound as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <RedemptionRequestExceedsMax as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector == <RoundsToZero as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <SafeCastFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <UnpackableString as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SetErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DelayNotElapsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPurchase(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidState(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidStateTransition(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionNotFound(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionRequestExceedsMax(element) => ::core::fmt::Display::fmt(element, f),
                Self::RoundsToZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeCastFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpackableString(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SetErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DelayNotElapsed> for SetErrors {
        fn from(value: DelayNotElapsed) -> Self {
            Self::DelayNotElapsed(value)
        }
    }
    impl ::core::convert::From<Initialized> for SetErrors {
        fn from(value: Initialized) -> Self {
            Self::Initialized(value)
        }
    }
    impl ::core::convert::From<InsufficientAssets> for SetErrors {
        fn from(value: InsufficientAssets) -> Self {
            Self::InsufficientAssets(value)
        }
    }
    impl ::core::convert::From<InsufficientBalance> for SetErrors {
        fn from(value: InsufficientBalance) -> Self {
            Self::InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for SetErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<InvalidConfiguration> for SetErrors {
        fn from(value: InvalidConfiguration) -> Self {
            Self::InvalidConfiguration(value)
        }
    }
    impl ::core::convert::From<InvalidDeposit> for SetErrors {
        fn from(value: InvalidDeposit) -> Self {
            Self::InvalidDeposit(value)
        }
    }
    impl ::core::convert::From<InvalidPurchase> for SetErrors {
        fn from(value: InvalidPurchase) -> Self {
            Self::InvalidPurchase(value)
        }
    }
    impl ::core::convert::From<InvalidState> for SetErrors {
        fn from(value: InvalidState) -> Self {
            Self::InvalidState(value)
        }
    }
    impl ::core::convert::From<InvalidStateTransition> for SetErrors {
        fn from(value: InvalidStateTransition) -> Self {
            Self::InvalidStateTransition(value)
        }
    }
    impl ::core::convert::From<RedemptionNotFound> for SetErrors {
        fn from(value: RedemptionNotFound) -> Self {
            Self::RedemptionNotFound(value)
        }
    }
    impl ::core::convert::From<RedemptionRequestExceedsMax> for SetErrors {
        fn from(value: RedemptionRequestExceedsMax) -> Self {
            Self::RedemptionRequestExceedsMax(value)
        }
    }
    impl ::core::convert::From<RoundsToZero> for SetErrors {
        fn from(value: RoundsToZero) -> Self {
            Self::RoundsToZero(value)
        }
    }
    impl ::core::convert::From<SafeCastFailed> for SetErrors {
        fn from(value: SafeCastFailed) -> Self {
            Self::SafeCastFailed(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for SetErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnpackableString> for SetErrors {
        fn from(value: UnpackableString) -> Self {
            Self::UnpackableString(value)
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Claim",
        abi = "Claim(address,address,address,uint256,uint256,address)"
    )]
    pub struct ClaimFilter {
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub protection: ::ethers::core::types::U256,
        pub ptokens: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ptoken: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,address,uint256,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "MarketStateUpdated",
        abi = "MarketStateUpdated(uint16,address,uint8)"
    )]
    pub struct MarketStateUpdatedFilter {
        #[ethevent(indexed)]
        pub market_id: u16,
        #[ethevent(indexed)]
        pub trigger: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub updated_to: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Purchase",
        abi = "Purchase(address,address,uint256,uint256,address,uint256)"
    )]
    pub struct PurchaseFilter {
        #[ethevent(indexed)]
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        pub protection: ::ethers::core::types::U256,
        pub ptokens: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ptoken: ::ethers::core::types::Address,
        pub cost: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Redeem",
        abi = "Redeem(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct RedeemFilter {
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub redemption_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "RedemptionPending",
        abi = "RedemptionPending(address,address,address,uint256,uint256,uint256)"
    )]
    pub struct RedemptionPendingFilter {
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub assets: ::ethers::core::types::U256,
        pub shares: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub redemption_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Sell",
        abi = "Sell(address,address,address,uint256,uint256,address,uint256)"
    )]
    pub struct SellFilter {
        pub caller: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub receiver: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        pub protection: ::ethers::core::types::U256,
        pub ptokens: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub ptoken: ::ethers::core::types::Address,
        pub refund: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "SetStateUpdated", abi = "SetStateUpdated(uint8)")]
    pub struct SetStateUpdatedFilter {
        #[ethevent(indexed)]
        pub updated_to: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
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
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SetEvents {
        ApprovalFilter(ApprovalFilter),
        ClaimFilter(ClaimFilter),
        ConfigUpdatesFinalizedFilter(ConfigUpdatesFinalizedFilter),
        ConfigUpdatesQueuedFilter(ConfigUpdatesQueuedFilter),
        DepositFilter(DepositFilter),
        FeesAccruedFilter(FeesAccruedFilter),
        MarketCreatedFilter(MarketCreatedFilter),
        MarketStateUpdatedFilter(MarketStateUpdatedFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PauserUpdatedFilter(PauserUpdatedFilter),
        PurchaseFilter(PurchaseFilter),
        RedeemFilter(RedeemFilter),
        RedemptionPendingFilter(RedemptionPendingFilter),
        SellFilter(SellFilter),
        SetStateUpdatedFilter(SetStateUpdatedFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for SetEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(SetEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ClaimFilter::decode_log(log) {
                return Ok(SetEvents::ClaimFilter(decoded));
            }
            if let Ok(decoded) = ConfigUpdatesFinalizedFilter::decode_log(log) {
                return Ok(SetEvents::ConfigUpdatesFinalizedFilter(decoded));
            }
            if let Ok(decoded) = ConfigUpdatesQueuedFilter::decode_log(log) {
                return Ok(SetEvents::ConfigUpdatesQueuedFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(SetEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = FeesAccruedFilter::decode_log(log) {
                return Ok(SetEvents::FeesAccruedFilter(decoded));
            }
            if let Ok(decoded) = MarketCreatedFilter::decode_log(log) {
                return Ok(SetEvents::MarketCreatedFilter(decoded));
            }
            if let Ok(decoded) = MarketStateUpdatedFilter::decode_log(log) {
                return Ok(SetEvents::MarketStateUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(SetEvents::OwnershipTransferStartedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(SetEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PauserUpdatedFilter::decode_log(log) {
                return Ok(SetEvents::PauserUpdatedFilter(decoded));
            }
            if let Ok(decoded) = PurchaseFilter::decode_log(log) {
                return Ok(SetEvents::PurchaseFilter(decoded));
            }
            if let Ok(decoded) = RedeemFilter::decode_log(log) {
                return Ok(SetEvents::RedeemFilter(decoded));
            }
            if let Ok(decoded) = RedemptionPendingFilter::decode_log(log) {
                return Ok(SetEvents::RedemptionPendingFilter(decoded));
            }
            if let Ok(decoded) = SellFilter::decode_log(log) {
                return Ok(SetEvents::SellFilter(decoded));
            }
            if let Ok(decoded) = SetStateUpdatedFilter::decode_log(log) {
                return Ok(SetEvents::SetStateUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(SetEvents::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for SetEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfigUpdatesFinalizedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ConfigUpdatesQueuedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesAccruedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketCreatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MarketStateUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PurchaseFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedeemFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RedemptionPendingFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SellFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetStateUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for SetEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFilter> for SetEvents {
        fn from(value: ClaimFilter) -> Self {
            Self::ClaimFilter(value)
        }
    }
    impl ::core::convert::From<ConfigUpdatesFinalizedFilter> for SetEvents {
        fn from(value: ConfigUpdatesFinalizedFilter) -> Self {
            Self::ConfigUpdatesFinalizedFilter(value)
        }
    }
    impl ::core::convert::From<ConfigUpdatesQueuedFilter> for SetEvents {
        fn from(value: ConfigUpdatesQueuedFilter) -> Self {
            Self::ConfigUpdatesQueuedFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for SetEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<FeesAccruedFilter> for SetEvents {
        fn from(value: FeesAccruedFilter) -> Self {
            Self::FeesAccruedFilter(value)
        }
    }
    impl ::core::convert::From<MarketCreatedFilter> for SetEvents {
        fn from(value: MarketCreatedFilter) -> Self {
            Self::MarketCreatedFilter(value)
        }
    }
    impl ::core::convert::From<MarketStateUpdatedFilter> for SetEvents {
        fn from(value: MarketStateUpdatedFilter) -> Self {
            Self::MarketStateUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter> for SetEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for SetEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PauserUpdatedFilter> for SetEvents {
        fn from(value: PauserUpdatedFilter) -> Self {
            Self::PauserUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<PurchaseFilter> for SetEvents {
        fn from(value: PurchaseFilter) -> Self {
            Self::PurchaseFilter(value)
        }
    }
    impl ::core::convert::From<RedeemFilter> for SetEvents {
        fn from(value: RedeemFilter) -> Self {
            Self::RedeemFilter(value)
        }
    }
    impl ::core::convert::From<RedemptionPendingFilter> for SetEvents {
        fn from(value: RedemptionPendingFilter) -> Self {
            Self::RedemptionPendingFilter(value)
        }
    }
    impl ::core::convert::From<SellFilter> for SetEvents {
        fn from(value: SellFilter) -> Self {
            Self::SellFilter(value)
        }
    }
    impl ::core::convert::From<SetStateUpdatedFilter> for SetEvents {
        fn from(value: SetStateUpdatedFilter) -> Self {
            Self::SetStateUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for SetEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    ///Container type for all input parameters for the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
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
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
    #[ethcall(name = "accounting", abi = "accounting()")]
    pub struct AccountingCall;
    ///Container type for all input parameters for the `allowance` function with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `approve` function with signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `asset` function with signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    ///Container type for all input parameters for the `backstop` function with signature `backstop()` and selector `0x7dea1817`
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
    #[ethcall(name = "backstop", abi = "backstop()")]
    pub struct BackstopCall;
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `balanceOfMatured` function with signature `balanceOfMatured(address)` and selector `0x3489b7a4`
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
    #[ethcall(name = "balanceOfMatured", abi = "balanceOfMatured(address)")]
    pub struct BalanceOfMaturedCall {
        pub user: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `claim` function with signature `claim(uint16,uint256,address,address)` and selector `0x06af14f1`
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
    #[ethcall(name = "claim", abi = "claim(uint16,uint256,address,address)")]
    pub struct ClaimCall {
        pub market_id: u16,
        pub ptokens: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `claimCozyFees` function with signature `claimCozyFees(address,address)` and selector `0xbdf4fb4e`
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
    #[ethcall(name = "claimCozyFees", abi = "claimCozyFees(address,address)")]
    pub struct ClaimCozyFeesCall {
        pub owner: ::ethers::core::types::Address,
        pub backstop: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `claimSetFees` function with signature `claimSetFees(address,address)` and selector `0x1f1cc4c5`
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
    #[ethcall(name = "claimSetFees", abi = "claimSetFees(address,address)")]
    pub struct ClaimSetFeesCall {
        pub caller: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `completeRedeem` function with signature `completeRedeem(uint64)` and selector `0x99f51d86`
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
    #[ethcall(name = "completeRedeem", abi = "completeRedeem(uint64)")]
    pub struct CompleteRedeemCall {
        pub redemption_id: u64,
    }
    ///Container type for all input parameters for the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    #[ethcall(name = "convertToAssets", abi = "convertToAssets(uint256)")]
    pub struct ConvertToAssetsCall {
        pub shares: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToPTokens` function with signature `convertToPTokens(uint16,uint256)` and selector `0x81c1a18c`
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
    #[ethcall(name = "convertToPTokens", abi = "convertToPTokens(uint16,uint256)")]
    pub struct ConvertToPTokensCall {
        pub market_id: u16,
        pub protection: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToProtection` function with signature `convertToProtection(uint16,uint256)` and selector `0x1437feaa`
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
        name = "convertToProtection",
        abi = "convertToProtection(uint16,uint256)"
    )]
    pub struct ConvertToProtectionCall {
        pub market_id: u16,
        pub ptokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    #[ethcall(name = "convertToShares", abi = "convertToShares(uint256)")]
    pub struct ConvertToSharesCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `cumulativeMinted` function with signature `cumulativeMinted(address,uint256)` and selector `0xf6eab327`
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
    #[ethcall(name = "cumulativeMinted", abi = "cumulativeMinted(address,uint256)")]
    pub struct CumulativeMintedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    ///Container type for all input parameters for the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    #[ethcall(name = "deposit", abi = "deposit(uint256,address)")]
    pub struct DepositCall {
        pub assets: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `dripSupplierFees` function with signature `dripSupplierFees()` and selector `0x1891661c`
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
    #[ethcall(name = "dripSupplierFees", abi = "dripSupplierFees()")]
    pub struct DripSupplierFeesCall;
    ///Container type for all input parameters for the `effectiveActiveProtection` function with signature `effectiveActiveProtection(uint16)` and selector `0x17766c81`
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
        name = "effectiveActiveProtection",
        abi = "effectiveActiveProtection(uint16)"
    )]
    pub struct EffectiveActiveProtectionCall {
        pub market_id: u16,
    }
    ///Container type for all input parameters for the `finalizeUpdateConfigs` function with signature `finalizeUpdateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])` and selector `0x3ba6f8e3`
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
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
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
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,(uint32,uint16),(address,address,address,uint16,uint16,uint16)[])` and selector `0x7e0f72fd`
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
        name = "initialize",
        abi = "initialize(address,address,address,(uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
        pub pauser: ::ethers::core::types::Address,
        pub asset: ::ethers::core::types::Address,
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
    }
    ///Container type for all input parameters for the `lastConfigUpdate` function with signature `lastConfigUpdate()` and selector `0x57a1ad84`
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
    #[ethcall(name = "lastConfigUpdate", abi = "lastConfigUpdate()")]
    pub struct LastConfigUpdateCall;
    ///Container type for all input parameters for the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    #[ethcall(name = "manager", abi = "manager()")]
    pub struct ManagerCall;
    ///Container type for all input parameters for the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    #[ethcall(name = "markets", abi = "markets(uint256)")]
    pub struct MarketsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `maxDeposit` function with signature `maxDeposit()` and selector `0x6083e59a`
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
    #[ethcall(name = "maxDeposit", abi = "maxDeposit()")]
    pub struct MaxDepositCall;
    ///Container type for all input parameters for the `maxRedemptionRequest` function with signature `maxRedemptionRequest(address)` and selector `0xd8a1892f`
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
    #[ethcall(name = "maxRedemptionRequest", abi = "maxRedemptionRequest(address)")]
    pub struct MaxRedemptionRequestCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `mints` function with signature `mints(address,uint256)` and selector `0x0bedd3a7`
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
    #[ethcall(name = "mints", abi = "mints(address,uint256)")]
    pub struct MintsCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause()` and selector `0x8456cb59`
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
    #[ethcall(name = "pause", abi = "pause()")]
    pub struct PauseCall;
    ///Container type for all input parameters for the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    #[ethcall(name = "pauser", abi = "pauser()")]
    pub struct PauserCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `permit` function with signature `permit(address,address,uint256,uint256,uint8,bytes32,bytes32)` and selector `0xd505accf`
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
    ///Container type for all input parameters for the `previewClaim` function with signature `previewClaim(uint16,uint256)` and selector `0xddaf883f`
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
    #[ethcall(name = "previewClaim", abi = "previewClaim(uint16,uint256)")]
    pub struct PreviewClaimCall {
        pub market_id: u16,
        pub ptokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    #[ethcall(name = "previewDeposit", abi = "previewDeposit(uint256)")]
    pub struct PreviewDepositCall {
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewPurchase` function with signature `previewPurchase(uint16,uint256)` and selector `0xc69f5180`
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
    #[ethcall(name = "previewPurchase", abi = "previewPurchase(uint16,uint256)")]
    pub struct PreviewPurchaseCall {
        pub market_id: u16,
        pub protection: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `previewRedemption` function with signature `previewRedemption(uint64)` and selector `0x06de9b0f`
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
    #[ethcall(name = "previewRedemption", abi = "previewRedemption(uint64)")]
    pub struct PreviewRedemptionCall {
        pub redemption_id: u64,
    }
    ///Container type for all input parameters for the `previewSale` function with signature `previewSale(uint64,uint256)` and selector `0x7c6a8909`
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
    #[ethcall(name = "previewSale", abi = "previewSale(uint64,uint256)")]
    pub struct PreviewSaleCall {
        pub market_id: u64,
        pub ptokens: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    #[ethcall(name = "ptokenFactory", abi = "ptokenFactory()")]
    pub struct PtokenFactoryCall;
    ///Container type for all input parameters for the `purchase` function with signature `purchase(uint16,uint256,address)` and selector `0xf09b77db`
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
    #[ethcall(name = "purchase", abi = "purchase(uint16,uint256,address)")]
    pub struct PurchaseCall {
        pub market_id: u16,
        pub protection: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    #[ethcall(name = "redeem", abi = "redeem(uint256,address,address)")]
    pub struct RedeemCall {
        pub shares: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `redemptions` function with signature `redemptions(uint256)` and selector `0xbeb65893`
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
    #[ethcall(name = "redemptions", abi = "redemptions(uint256)")]
    pub struct RedemptionsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `remainingProtection` function with signature `remainingProtection(uint16)` and selector `0x58b61432`
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
    #[ethcall(name = "remainingProtection", abi = "remainingProtection(uint16)")]
    pub struct RemainingProtectionCall {
        pub market_id: u16,
    }
    ///Container type for all input parameters for the `rescueFunds` function with signature `rescueFunds(address,address,uint256)` and selector `0x6ccae054`
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
    #[ethcall(name = "rescueFunds", abi = "rescueFunds(address,address,uint256)")]
    pub struct RescueFundsWithTokenCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `rescueFunds` function with signature `rescueFunds(address)` and selector `0xe53b2017`
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
    #[ethcall(name = "rescueFunds", abi = "rescueFunds(address)")]
    pub struct RescueFundsCall {
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `sell` function with signature `sell(uint16,uint256,address,address)` and selector `0x3ae885f5`
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
    #[ethcall(name = "sell", abi = "sell(uint16,uint256,address,address)")]
    pub struct SellCall {
        pub market_id: u16,
        pub ptokens: ::ethers::core::types::U256,
        pub receiver: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    #[ethcall(name = "setConfig", abi = "setConfig()")]
    pub struct SetConfigCall;
    ///Container type for all input parameters for the `setState` function with signature `setState()` and selector `0x1203402f`
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
    #[ethcall(name = "setState", abi = "setState()")]
    pub struct SetStateCall;
    ///Container type for all input parameters for the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    ///Container type for all input parameters for the `totalCollateralAvailable` function with signature `totalCollateralAvailable()` and selector `0xaf04be85`
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
    #[ethcall(name = "totalCollateralAvailable", abi = "totalCollateralAvailable()")]
    pub struct TotalCollateralAvailableCall;
    ///Container type for all input parameters for the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    ///Container type for all input parameters for the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    #[ethcall(name = "triggerLookups", abi = "triggerLookups(address)")]
    pub struct TriggerLookupsCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `unpause` function with signature `unpause()` and selector `0x3f4ba83a`
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
    #[ethcall(name = "unpause", abi = "unpause()")]
    pub struct UnpauseCall;
    ///Container type for all input parameters for the `updateConfigs` function with signature `updateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])` and selector `0xfa3ebe07`
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
        name = "updateConfigs",
        abi = "updateConfigs((uint32,uint16),(address,address,address,uint16,uint16,uint16)[])"
    )]
    pub struct UpdateConfigsCall {
        pub set_config: SetConfig,
        pub market_configs: ::std::vec::Vec<MarketConfig>,
    }
    ///Container type for all input parameters for the `updateMarketState` function with signature `updateMarketState(uint8)` and selector `0x7a75fdde`
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
    #[ethcall(name = "updateMarketState", abi = "updateMarketState(uint8)")]
    pub struct UpdateMarketStateCall {
        pub new_market_state: u8,
    }
    ///Container type for all input parameters for the `updatePauser` function with signature `updatePauser(address)` and selector `0x554bab3c`
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
    #[ethcall(name = "updatePauser", abi = "updatePauser(address)")]
    pub struct UpdatePauserCall {
        pub new_pauser: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `utilization` function with signature `utilization(uint16)` and selector `0xb18043a2`
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
    #[ethcall(name = "utilization", abi = "utilization(uint16)")]
    pub struct UtilizationCall {
        pub market_id: u16,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SetCalls {
        DomainSeparator(DomainSeparatorCall),
        AcceptOwnership(AcceptOwnershipCall),
        Accounting(AccountingCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        Asset(AssetCall),
        Backstop(BackstopCall),
        BalanceOf(BalanceOfCall),
        BalanceOfMatured(BalanceOfMaturedCall),
        Claim(ClaimCall),
        ClaimCozyFees(ClaimCozyFeesCall),
        ClaimSetFees(ClaimSetFeesCall),
        CompleteRedeem(CompleteRedeemCall),
        ConvertToAssets(ConvertToAssetsCall),
        ConvertToPTokens(ConvertToPTokensCall),
        ConvertToProtection(ConvertToProtectionCall),
        ConvertToShares(ConvertToSharesCall),
        CumulativeMinted(CumulativeMintedCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        DripSupplierFees(DripSupplierFeesCall),
        EffectiveActiveProtection(EffectiveActiveProtectionCall),
        FinalizeUpdateConfigs(FinalizeUpdateConfigsCall),
        GetMints(GetMintsCall),
        Initialize(InitializeCall),
        LastConfigUpdate(LastConfigUpdateCall),
        Manager(ManagerCall),
        Markets(MarketsCall),
        MaxDeposit(MaxDepositCall),
        MaxRedemptionRequest(MaxRedemptionRequestCall),
        Mints(MintsCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        Pauser(PauserCall),
        PendingOwner(PendingOwnerCall),
        Permit(PermitCall),
        PreviewClaim(PreviewClaimCall),
        PreviewDeposit(PreviewDepositCall),
        PreviewPurchase(PreviewPurchaseCall),
        PreviewRedemption(PreviewRedemptionCall),
        PreviewSale(PreviewSaleCall),
        PtokenFactory(PtokenFactoryCall),
        Purchase(PurchaseCall),
        Redeem(RedeemCall),
        Redemptions(RedemptionsCall),
        RemainingProtection(RemainingProtectionCall),
        RescueFundsWithToken(RescueFundsWithTokenCall),
        RescueFunds(RescueFundsCall),
        Sell(SellCall),
        SetConfig(SetConfigCall),
        SetState(SetStateCall),
        Symbol(SymbolCall),
        TotalCollateralAvailable(TotalCollateralAvailableCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        TransferOwnership(TransferOwnershipCall),
        TriggerLookups(TriggerLookupsCall),
        Unpause(UnpauseCall),
        UpdateConfigs(UpdateConfigsCall),
        UpdateMarketState(UpdateMarketStateCall),
        UpdatePauser(UpdatePauserCall),
        Utilization(UtilizationCall),
    }
    impl ::ethers::core::abi::AbiDecode for SetCalls {
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
            if let Ok(decoded) = <ClaimCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Claim(decoded));
            }
            if let Ok(decoded) = <ClaimCozyFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClaimCozyFees(decoded));
            }
            if let Ok(decoded) = <ClaimSetFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ClaimSetFees(decoded));
            }
            if let Ok(decoded) =
                <CompleteRedeemCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteRedeem(decoded));
            }
            if let Ok(decoded) =
                <ConvertToAssetsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToAssets(decoded));
            }
            if let Ok(decoded) =
                <ConvertToPTokensCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToPTokens(decoded));
            }
            if let Ok(decoded) =
                <ConvertToProtectionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToProtection(decoded));
            }
            if let Ok(decoded) =
                <ConvertToSharesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConvertToShares(decoded));
            }
            if let Ok(decoded) =
                <CumulativeMintedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeMinted(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) =
                <DripSupplierFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DripSupplierFees(decoded));
            }
            if let Ok(decoded) =
                <EffectiveActiveProtectionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EffectiveActiveProtection(decoded));
            }
            if let Ok(decoded) =
                <FinalizeUpdateConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FinalizeUpdateConfigs(decoded));
            }
            if let Ok(decoded) = <GetMintsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMints(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
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
            if let Ok(decoded) = <MaxDepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MaxDeposit(decoded));
            }
            if let Ok(decoded) =
                <MaxRedemptionRequestCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxRedemptionRequest(decoded));
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
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
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
            if let Ok(decoded) = <PreviewClaimCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreviewClaim(decoded));
            }
            if let Ok(decoded) =
                <PreviewDepositCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreviewDeposit(decoded));
            }
            if let Ok(decoded) =
                <PreviewPurchaseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreviewPurchase(decoded));
            }
            if let Ok(decoded) =
                <PreviewRedemptionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreviewRedemption(decoded));
            }
            if let Ok(decoded) = <PreviewSaleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PreviewSale(decoded));
            }
            if let Ok(decoded) = <PtokenFactoryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PtokenFactory(decoded));
            }
            if let Ok(decoded) = <PurchaseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Purchase(decoded));
            }
            if let Ok(decoded) = <RedeemCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redeem(decoded));
            }
            if let Ok(decoded) = <RedemptionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Redemptions(decoded));
            }
            if let Ok(decoded) =
                <RemainingProtectionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemainingProtection(decoded));
            }
            if let Ok(decoded) =
                <RescueFundsWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RescueFundsWithToken(decoded));
            }
            if let Ok(decoded) = <RescueFundsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RescueFunds(decoded));
            }
            if let Ok(decoded) = <SellCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sell(decoded));
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
            if let Ok(decoded) =
                <TotalCollateralAvailableCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalCollateralAvailable(decoded));
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
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdateConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateConfigs(decoded));
            }
            if let Ok(decoded) =
                <UpdateMarketStateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdateMarketState(decoded));
            }
            if let Ok(decoded) = <UpdatePauserCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UpdatePauser(decoded));
            }
            if let Ok(decoded) = <UtilizationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Utilization(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SetCalls {
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
                Self::Claim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimCozyFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimSetFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CompleteRedeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConvertToAssets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConvertToPTokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConvertToProtection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConvertToShares(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeMinted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DripSupplierFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EffectiveActiveProtection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FinalizeUpdateConfigs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastConfigUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Manager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Markets(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxRedemptionRequest(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Mints(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewClaim(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewDeposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewPurchase(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewRedemption(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PreviewSale(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PtokenFactory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Purchase(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Redeem(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Redemptions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemainingProtection(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueFundsWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RescueFunds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sell(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalCollateralAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TriggerLookups(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateConfigs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdateMarketState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UpdatePauser(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Utilization(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for SetCalls {
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
                Self::Claim(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimCozyFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimSetFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteRedeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToAssets(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToPTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToProtection(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConvertToShares(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeMinted(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DripSupplierFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::EffectiveActiveProtection(element) => ::core::fmt::Display::fmt(element, f),
                Self::FinalizeUpdateConfigs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastConfigUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Manager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Markets(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxRedemptionRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mints(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewClaim(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewDeposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewPurchase(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewRedemption(element) => ::core::fmt::Display::fmt(element, f),
                Self::PreviewSale(element) => ::core::fmt::Display::fmt(element, f),
                Self::PtokenFactory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Purchase(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redeem(element) => ::core::fmt::Display::fmt(element, f),
                Self::Redemptions(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemainingProtection(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueFundsWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::RescueFunds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sell(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetState(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalCollateralAvailable(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TriggerLookups(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateConfigs(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateMarketState(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdatePauser(element) => ::core::fmt::Display::fmt(element, f),
                Self::Utilization(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for SetCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for SetCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<AccountingCall> for SetCalls {
        fn from(value: AccountingCall) -> Self {
            Self::Accounting(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for SetCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for SetCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<AssetCall> for SetCalls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<BackstopCall> for SetCalls {
        fn from(value: BackstopCall) -> Self {
            Self::Backstop(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for SetCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfMaturedCall> for SetCalls {
        fn from(value: BalanceOfMaturedCall) -> Self {
            Self::BalanceOfMatured(value)
        }
    }
    impl ::core::convert::From<ClaimCall> for SetCalls {
        fn from(value: ClaimCall) -> Self {
            Self::Claim(value)
        }
    }
    impl ::core::convert::From<ClaimCozyFeesCall> for SetCalls {
        fn from(value: ClaimCozyFeesCall) -> Self {
            Self::ClaimCozyFees(value)
        }
    }
    impl ::core::convert::From<ClaimSetFeesCall> for SetCalls {
        fn from(value: ClaimSetFeesCall) -> Self {
            Self::ClaimSetFees(value)
        }
    }
    impl ::core::convert::From<CompleteRedeemCall> for SetCalls {
        fn from(value: CompleteRedeemCall) -> Self {
            Self::CompleteRedeem(value)
        }
    }
    impl ::core::convert::From<ConvertToAssetsCall> for SetCalls {
        fn from(value: ConvertToAssetsCall) -> Self {
            Self::ConvertToAssets(value)
        }
    }
    impl ::core::convert::From<ConvertToPTokensCall> for SetCalls {
        fn from(value: ConvertToPTokensCall) -> Self {
            Self::ConvertToPTokens(value)
        }
    }
    impl ::core::convert::From<ConvertToProtectionCall> for SetCalls {
        fn from(value: ConvertToProtectionCall) -> Self {
            Self::ConvertToProtection(value)
        }
    }
    impl ::core::convert::From<ConvertToSharesCall> for SetCalls {
        fn from(value: ConvertToSharesCall) -> Self {
            Self::ConvertToShares(value)
        }
    }
    impl ::core::convert::From<CumulativeMintedCall> for SetCalls {
        fn from(value: CumulativeMintedCall) -> Self {
            Self::CumulativeMinted(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for SetCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for SetCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<DripSupplierFeesCall> for SetCalls {
        fn from(value: DripSupplierFeesCall) -> Self {
            Self::DripSupplierFees(value)
        }
    }
    impl ::core::convert::From<EffectiveActiveProtectionCall> for SetCalls {
        fn from(value: EffectiveActiveProtectionCall) -> Self {
            Self::EffectiveActiveProtection(value)
        }
    }
    impl ::core::convert::From<FinalizeUpdateConfigsCall> for SetCalls {
        fn from(value: FinalizeUpdateConfigsCall) -> Self {
            Self::FinalizeUpdateConfigs(value)
        }
    }
    impl ::core::convert::From<GetMintsCall> for SetCalls {
        fn from(value: GetMintsCall) -> Self {
            Self::GetMints(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for SetCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastConfigUpdateCall> for SetCalls {
        fn from(value: LastConfigUpdateCall) -> Self {
            Self::LastConfigUpdate(value)
        }
    }
    impl ::core::convert::From<ManagerCall> for SetCalls {
        fn from(value: ManagerCall) -> Self {
            Self::Manager(value)
        }
    }
    impl ::core::convert::From<MarketsCall> for SetCalls {
        fn from(value: MarketsCall) -> Self {
            Self::Markets(value)
        }
    }
    impl ::core::convert::From<MaxDepositCall> for SetCalls {
        fn from(value: MaxDepositCall) -> Self {
            Self::MaxDeposit(value)
        }
    }
    impl ::core::convert::From<MaxRedemptionRequestCall> for SetCalls {
        fn from(value: MaxRedemptionRequestCall) -> Self {
            Self::MaxRedemptionRequest(value)
        }
    }
    impl ::core::convert::From<MintsCall> for SetCalls {
        fn from(value: MintsCall) -> Self {
            Self::Mints(value)
        }
    }
    impl ::core::convert::From<NameCall> for SetCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for SetCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for SetCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for SetCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauserCall> for SetCalls {
        fn from(value: PauserCall) -> Self {
            Self::Pauser(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for SetCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<PermitCall> for SetCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<PreviewClaimCall> for SetCalls {
        fn from(value: PreviewClaimCall) -> Self {
            Self::PreviewClaim(value)
        }
    }
    impl ::core::convert::From<PreviewDepositCall> for SetCalls {
        fn from(value: PreviewDepositCall) -> Self {
            Self::PreviewDeposit(value)
        }
    }
    impl ::core::convert::From<PreviewPurchaseCall> for SetCalls {
        fn from(value: PreviewPurchaseCall) -> Self {
            Self::PreviewPurchase(value)
        }
    }
    impl ::core::convert::From<PreviewRedemptionCall> for SetCalls {
        fn from(value: PreviewRedemptionCall) -> Self {
            Self::PreviewRedemption(value)
        }
    }
    impl ::core::convert::From<PreviewSaleCall> for SetCalls {
        fn from(value: PreviewSaleCall) -> Self {
            Self::PreviewSale(value)
        }
    }
    impl ::core::convert::From<PtokenFactoryCall> for SetCalls {
        fn from(value: PtokenFactoryCall) -> Self {
            Self::PtokenFactory(value)
        }
    }
    impl ::core::convert::From<PurchaseCall> for SetCalls {
        fn from(value: PurchaseCall) -> Self {
            Self::Purchase(value)
        }
    }
    impl ::core::convert::From<RedeemCall> for SetCalls {
        fn from(value: RedeemCall) -> Self {
            Self::Redeem(value)
        }
    }
    impl ::core::convert::From<RedemptionsCall> for SetCalls {
        fn from(value: RedemptionsCall) -> Self {
            Self::Redemptions(value)
        }
    }
    impl ::core::convert::From<RemainingProtectionCall> for SetCalls {
        fn from(value: RemainingProtectionCall) -> Self {
            Self::RemainingProtection(value)
        }
    }
    impl ::core::convert::From<RescueFundsWithTokenCall> for SetCalls {
        fn from(value: RescueFundsWithTokenCall) -> Self {
            Self::RescueFundsWithToken(value)
        }
    }
    impl ::core::convert::From<RescueFundsCall> for SetCalls {
        fn from(value: RescueFundsCall) -> Self {
            Self::RescueFunds(value)
        }
    }
    impl ::core::convert::From<SellCall> for SetCalls {
        fn from(value: SellCall) -> Self {
            Self::Sell(value)
        }
    }
    impl ::core::convert::From<SetConfigCall> for SetCalls {
        fn from(value: SetConfigCall) -> Self {
            Self::SetConfig(value)
        }
    }
    impl ::core::convert::From<SetStateCall> for SetCalls {
        fn from(value: SetStateCall) -> Self {
            Self::SetState(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for SetCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalCollateralAvailableCall> for SetCalls {
        fn from(value: TotalCollateralAvailableCall) -> Self {
            Self::TotalCollateralAvailable(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for SetCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for SetCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for SetCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for SetCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TriggerLookupsCall> for SetCalls {
        fn from(value: TriggerLookupsCall) -> Self {
            Self::TriggerLookups(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for SetCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdateConfigsCall> for SetCalls {
        fn from(value: UpdateConfigsCall) -> Self {
            Self::UpdateConfigs(value)
        }
    }
    impl ::core::convert::From<UpdateMarketStateCall> for SetCalls {
        fn from(value: UpdateMarketStateCall) -> Self {
            Self::UpdateMarketState(value)
        }
    }
    impl ::core::convert::From<UpdatePauserCall> for SetCalls {
        fn from(value: UpdatePauserCall) -> Self {
            Self::UpdatePauser(value)
        }
    }
    impl ::core::convert::From<UtilizationCall> for SetCalls {
        fn from(value: UtilizationCall) -> Self {
            Self::Utilization(value)
        }
    }
    ///Container type for all return fields from the `DOMAIN_SEPARATOR` function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    ///Container type for all return fields from the `accounting` function with signature `accounting()` and selector `0x9624e83e`
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
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
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfMaturedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `claim` function with signature `claim(uint16,uint256,address,address)` and selector `0x06af14f1`
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
    pub struct ClaimReturn {
        pub protection: u128,
    }
    ///Container type for all return fields from the `claimCozyFees` function with signature `claimCozyFees(address,address)` and selector `0xbdf4fb4e`
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
    pub struct ClaimCozyFeesReturn {
        pub reserve_amount: u128,
        pub backstop_amount: u128,
    }
    ///Container type for all return fields from the `claimSetFees` function with signature `claimSetFees(address,address)` and selector `0x1f1cc4c5`
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
    pub struct ClaimSetFeesReturn {
        pub set_owner_fees: u128,
    }
    ///Container type for all return fields from the `completeRedeem` function with signature `completeRedeem(uint64)` and selector `0x99f51d86`
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
    pub struct CompleteRedeemReturn {
        pub assets_redeemed: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `convertToAssets` function with signature `convertToAssets(uint256)` and selector `0x07a2d13a`
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
    pub struct ConvertToAssetsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToPTokens` function with signature `convertToPTokens(uint16,uint256)` and selector `0x81c1a18c`
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
    pub struct ConvertToPTokensReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToProtection` function with signature `convertToProtection(uint16,uint256)` and selector `0x1437feaa`
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
    pub struct ConvertToProtectionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `convertToShares` function with signature `convertToShares(uint256)` and selector `0xc6e6f592`
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
    pub struct ConvertToSharesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `cumulativeMinted` function with signature `cumulativeMinted(address,uint256)` and selector `0xf6eab327`
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
    pub struct CumulativeMintedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `decimals` function with signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
    ///Container type for all return fields from the `deposit` function with signature `deposit(uint256,address)` and selector `0x6e553f65`
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
    pub struct DepositReturn {
        pub shares: ::ethers::core::types::U256,
        pub deposit_fees_assets: DepositFeesAssets,
    }
    ///Container type for all return fields from the `effectiveActiveProtection` function with signature `effectiveActiveProtection(uint16)` and selector `0x17766c81`
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
    pub struct EffectiveActiveProtectionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getMints` function with signature `getMints(address)` and selector `0x74ce5671`
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
    pub struct GetMintsReturn(pub ::std::vec::Vec<MintData>);
    ///Container type for all return fields from the `lastConfigUpdate` function with signature `lastConfigUpdate()` and selector `0x57a1ad84`
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
    pub struct LastConfigUpdateReturn {
        pub queued_config_update_hash: [u8; 32],
        pub config_update_time: u64,
        pub config_update_deadline: u64,
    }
    ///Container type for all return fields from the `manager` function with signature `manager()` and selector `0x481c6a75`
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
    pub struct ManagerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `markets` function with signature `markets(uint256)` and selector `0xb1283e77`
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
    ///Container type for all return fields from the `maxDeposit` function with signature `maxDeposit()` and selector `0x6083e59a`
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
    pub struct MaxDepositReturn {
        pub max_deposit: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `maxRedemptionRequest` function with signature `maxRedemptionRequest(address)` and selector `0xd8a1892f`
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
    pub struct MaxRedemptionRequestReturn {
        pub redeemable_shares: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `mints` function with signature `mints(address,uint256)` and selector `0x0bedd3a7`
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
    pub struct MintsReturn {
        pub amount: ::ethers::core::types::U256,
        pub time: u64,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `nonces` function with signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pauser` function with signature `pauser()` and selector `0x9fd0506d`
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
    pub struct PauserReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
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
    pub struct PendingOwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `previewClaim` function with signature `previewClaim(uint16,uint256)` and selector `0xddaf883f`
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
    pub struct PreviewClaimReturn(pub u128);
    ///Container type for all return fields from the `previewDeposit` function with signature `previewDeposit(uint256)` and selector `0xef8b30f7`
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
    pub struct PreviewDepositReturn {
        pub shares: ::ethers::core::types::U256,
        pub deposit_fees_assets: DepositFeesAssets,
    }
    ///Container type for all return fields from the `previewPurchase` function with signature `previewPurchase(uint16,uint256)` and selector `0xc69f5180`
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
    pub struct PreviewPurchaseReturn {
        pub assets_needed: ::ethers::core::types::U256,
        pub ptokens: ::ethers::core::types::U256,
        pub purchase_fees_assets: PurchaseFeesAssets,
    }
    ///Container type for all return fields from the `previewRedemption` function with signature `previewRedemption(uint64)` and selector `0x06de9b0f`
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
    pub struct PreviewRedemptionReturn {
        pub redemption_preview: RedemptionPreview,
    }
    ///Container type for all return fields from the `previewSale` function with signature `previewSale(uint64,uint256)` and selector `0x7c6a8909`
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
    pub struct PreviewSaleReturn {
        pub refund: ::ethers::core::types::U256,
        pub protection: ::ethers::core::types::U256,
        pub sale_fees_assets: SaleFeesAssets,
    }
    ///Container type for all return fields from the `ptokenFactory` function with signature `ptokenFactory()` and selector `0x764d6892`
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
    pub struct PtokenFactoryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `purchase` function with signature `purchase(uint16,uint256,address)` and selector `0xf09b77db`
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
    pub struct PurchaseReturn {
        pub ptokens: ::ethers::core::types::U256,
        pub purchase_fees_assets: PurchaseFeesAssets,
    }
    ///Container type for all return fields from the `redeem` function with signature `redeem(uint256,address,address)` and selector `0xba087652`
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
    pub struct RedeemReturn {
        pub redemption_id: u64,
        pub assets: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `redemptions` function with signature `redemptions(uint256)` and selector `0xbeb65893`
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
    pub struct RedemptionsReturn {
        pub queue_time: u64,
        pub shares: ::ethers::core::types::U256,
        pub assets: u128,
        pub owner: ::ethers::core::types::Address,
        pub receiver: ::ethers::core::types::Address,
        pub delay: u64,
        pub queued_acc_is_fs_length: u32,
        pub queued_acc_isf: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `remainingProtection` function with signature `remainingProtection(uint16)` and selector `0x58b61432`
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
    pub struct RemainingProtectionReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `sell` function with signature `sell(uint16,uint256,address,address)` and selector `0x3ae885f5`
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
    pub struct SellReturn {
        pub refund: u128,
        pub protection: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `setConfig` function with signature `setConfig()` and selector `0x8b673e7a`
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
    pub struct SetConfigReturn {
        pub leverage_factor: u32,
        pub deposit_fee: u16,
    }
    ///Container type for all return fields from the `setState` function with signature `setState()` and selector `0x1203402f`
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
    pub struct SetStateReturn(pub u8);
    ///Container type for all return fields from the `symbol` function with signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    ///Container type for all return fields from the `totalCollateralAvailable` function with signature `totalCollateralAvailable()` and selector `0xaf04be85`
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
    pub struct TotalCollateralAvailableReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalSupply` function with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `transfer` function with signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    ///Container type for all return fields from the `transferFrom` function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
    ///Container type for all return fields from the `triggerLookups` function with signature `triggerLookups(address)` and selector `0x58cfaac7`
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
    pub struct TriggerLookupsReturn {
        pub market_exists: bool,
        pub market_id: u16,
    }
    ///Container type for all return fields from the `utilization` function with signature `utilization(uint16)` and selector `0xb18043a2`
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
    pub struct UtilizationReturn(pub ::ethers::core::types::U256);
}
