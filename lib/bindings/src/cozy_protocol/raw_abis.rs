pub use raw_abis::*;
pub mod raw_abis {
    pub static ICOSTMODEL_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ICostModel.sol/ICostModel.json"
    );
    pub static IOWNABLE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IOwnable.sol/IOwnable.json"
    );
    pub static ICONFIGURATOREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IConfiguratorEvents.sol/IConfiguratorEvents.json"
    );
    pub static ISET_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ISet.sol/ISet.json"
    );
    pub static STATECHANGER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/StateChanger.sol/StateChanger.json"
    );
    pub static IERC20_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IERC20.sol/IERC20.json"
    );
    pub static IPROTECTIONCLAIMERERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IProtectionClaimerErrors.sol/IProtectionClaimerErrors.json"
    );
    pub static ADDRESSSETLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/AddressSet.sol/AddressSetLib.json"
    );
    pub static SETINSPECTOR_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetInspector.sol/SetInspector.json"
    );
    pub static IPTOKENFACTORYEVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IPTokenFactoryEvents.sol/IPTokenFactoryEvents.json"
    );
    pub static IPTOKENFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IPTokenFactory.sol/IPTokenFactory.json"
    );
    pub static ITRIGGER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ITrigger.sol/ITrigger.json"
    );
    pub static IASSETREDEEMERERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IAssetRedeemerErrors.sol/IAssetRedeemerErrors.json"
    );
    pub static STATEENUMS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/StateEnums.sol/StateEnums.json"
    );
    pub static PURCHASE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Purchase.sol/Purchase.json"
    );
    pub static DEPOSIT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Deposit.sol/Deposit.json"
    );
    pub static ERC20_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ERC20.sol/ERC20.json"
    );
    pub static FEEDRIPPER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/FeeDripper.sol/FeeDripper.json"
    );
    pub static INACTIVITYDATA_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/InactivityData.sol/InactivityData.json"
    );
    pub static IPROTECTIONPURCHASERERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IProtectionPurchaserErrors.sol/IProtectionPurchaserErrors.json"
    );
    pub static IDRIPDECAYMODEL_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IDripDecayModel.sol/IDripDecayModel.json"
    );
    pub static SETBASESTORAGE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetBaseStorage.sol/SetBaseStorage.json"
    );
    pub static DEMANDSIDELIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/DemandSideLib.sol/DemandSideLib.json"
    );
    pub static ICOMMONERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ICommonErrors.sol/ICommonErrors.json"
    );
    pub static ISTATECHANGEEVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IStateChangeEvents.sol/IStateChangeEvents.json"
    );
    pub static SETCOMMON_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetCommon.sol/SetCommon.json"
    );
    pub static ICONNECTOR_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IConnector.sol/IConnector.json"
    );
    pub static IPROTECTIONSELLEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IProtectionSellerEvents.sol/IProtectionSellerEvents.json"
    );
    pub static IBACKSTOP_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IBackstop.sol/IBackstop.json"
    );
    pub static SALE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Sale.sol/Sale.json"
    );
    pub static MATHCONSTANTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/MathConstants.sol/MathConstants.json"
    );
    pub static IPROTECTIONPURCHASEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IProtectionPurchaserEvents.sol/IProtectionPurchaserEvents.json"
    );
    pub static ISTETH_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IStETH.sol/IStETH.json"
    );
    pub static OWNABLE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Ownable.sol/Ownable.json"
    );
    pub static CONFIGURATORLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ConfiguratorLib.sol/ConfiguratorLib.json"
    );
    pub static STDINVARIANT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/StdInvariant.sol/StdInvariant.json"
    );
    pub static CLONES_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Clones.sol/Clones.json"
    );
    pub static STATETRANSITIONSLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/StateTransitionsLib.sol/StateTransitionsLib.json"
    );
    pub static PROTECTIONPURCHASER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ProtectionPurchaser.sol/ProtectionPurchaser.json"
    );
    pub static ASSETSTORAGE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/AssetStorage.sol/AssetStorage.json"
    );
    pub static DELAYLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/DelayLib.sol/DelayLib.json"
    );
    pub static ICOMMONEVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ICommonEvents.sol/ICommonEvents.json"
    );
    pub static SUPPLYSIDELIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SupplySideLib.sol/SupplySideLib.json"
    );
    pub static IPROTECTIONCLAIMEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IProtectionClaimerEvents.sol/IProtectionClaimerEvents.json"
    );
    pub static LFT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/LFT.sol/LFT.json"
    );
    pub static ASSETDEPOSITER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/AssetDepositer.sol/AssetDepositer.json"
    );
    pub static ICONFIGURATORERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IConfiguratorErrors.sol/IConfiguratorErrors.json"
    );
    pub static MARKETCALCULATIONSLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/MarketCalculationsLib.sol/MarketCalculationsLib.json"
    );
    pub static IASSETREDEEMEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IAssetRedeemerEvents.sol/IAssetRedeemerEvents.json"
    );
    pub static IPTOKEN_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IPToken.sol/IPToken.json"
    );
    pub static GOVERNABLE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Governable.sol/Governable.json"
    );
    pub static IBACKSTOPEVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IBackstopEvents.sol/IBackstopEvents.json"
    );
    pub static FIXEDPOINTMATHLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/FixedPointMathLib.sol/FixedPointMathLib.json"
    );
    pub static PTOKENFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/PTokenFactory.sol/PTokenFactory.json"
    );
    pub static COSTMODELAREACALCULATIONSLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/CostModelAreaCalculationsLib.sol/CostModelAreaCalculationsLib.json"
    );
    pub static IWETH_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IWeth.sol/IWeth.json"
    );
    pub static DRIPDECAYMODELCONSTANT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/DripDecayModelConstant.sol/DripDecayModelConstant.json"
    );
    pub static CONFIGS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Configs.sol/Configs.json"
    );
    pub static IWSTETH_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IWstETH.sol/IWstETH.json"
    );
    pub static PTOKEN_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/PToken.sol/PToken.json"
    );
    pub static MANAGER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Manager.sol/Manager.json"
    );
    pub static SETCONFIG_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetConfig.sol/SetConfig.json"
    );
    pub static COSTMODELJUMPRATE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/CostModelJumpRate.sol/CostModelJumpRate.json"
    );
    pub static COZYROUTER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/CozyRouter.sol/CozyRouter.json"
    );
    pub static IASSETDEPOSITERERRORS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IAssetDepositerErrors.sol/IAssetDepositerErrors.json"
    );
    pub static DEBUGLOG_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/DebugLog.sol/DebugLog.json"
    );
    pub static MANAGER_1_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/structs/Manager.sol/Manager.json"
    );
    pub static BACKSTOP_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/structs/Backstop.sol/Backstop.json"
    );
    pub static CONFIGURATOR_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/structs/Configurator.sol/Configurator.json"
    );
    pub static PACKEDSTRINGLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/PackedStringLib.sol/PackedStringLib.json"
    );
    pub static SET_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Set.sol/Set.json"
    );
    pub static EXPONENTIALDECAYLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ExponentialDecayLib.sol/ExponentialDecayLib.json"
    );
    pub static SAFECASTLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SafeCastLib.sol/SafeCastLib.json"
    );
    pub static FREEZINGANDTRIGGERINGSETHANDLER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/FreezingAndTriggeringSetHandler.sol/FreezingAndTriggeringSetHandler.json"
    );
    pub static IGOVERNABLE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IGovernable.sol/IGovernable.json"
    );
    pub static REDEMPTION_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Redemption.sol/Redemption.json"
    );
    pub static PROTECTIONCLAIMER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ProtectionClaimer.sol/ProtectionClaimer.json"
    );
    pub static SETCALCULATIONSLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetCalculationsLib.sol/SetCalculationsLib.json"
    );
    pub static ISETFACTORYEVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ISetFactoryEvents.sol/ISetFactoryEvents.json"
    );
    pub static SAFETRANSFERLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SafeTransferLib.sol/SafeTransferLib.json"
    );
    pub static PROTECTIONDECAYER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ProtectionDecayer.sol/ProtectionDecayer.json"
    );
    pub static SETUTILS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetUtils.sol/SetUtils.json"
    );
    pub static IMANAGER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IManager.sol/IManager.json"
    );
    pub static ILFT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ILFT.sol/ILFT.json"
    );
    pub static REDEMPTIONLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/RedemptionLib.sol/RedemptionLib.json"
    );
    pub static BACKSTOP_1_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Backstop.sol/Backstop.json"
    );
    pub static IMANAGEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IManagerEvents.sol/IManagerEvents.json"
    );
    pub static SLOTENCODER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SlotEncoder.sol/SlotEncoder.json"
    );
    pub static CONFIGURATOR_1_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Configurator.sol/Configurator.json"
    );
    pub static SETFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetFactory.sol/SetFactory.json"
    );
    pub static PAUSINGSETHANDLER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/PausingSetHandler.sol/PausingSetHandler.json"
    );
    pub static IERC20EVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IERC20Events.sol/IERC20Events.json"
    );
    pub static INVARIANTBASEDEPLOY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/InvariantTestBase.sol/InvariantBaseDeploy.json"
    );
    pub static COZYMATH_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/CozyMath.sol/CozyMath.json"
    );
    pub static CHECKUSERMETADATA_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/CheckUserMetadata.sol/CheckUserMetadata.json"
    );
    pub static SETHANDLER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/SetHandler.sol/SetHandler.json"
    );
    pub static STUB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Stub.sol/Stub.json"
    );
    pub static IASSETDEPOSITEREVENTS_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/IAssetDepositerEvents.sol/IAssetDepositerEvents.json"
    );
    pub static MARKET_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/Market.sol/Market.json"
    );
    pub static MINTDATA_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/MintData.sol/MintData.json"
    );
    pub static ASSETREDEEMER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/AssetRedeemer.sol/AssetRedeemer.json"
    );
    pub static PROTECTIONSELLER_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-protocol-v2/out/ProtectionSeller.sol/ProtectionSeller.json"
    );
}
