pub use raw_abis::*;
pub mod raw_abis {
    pub static ICOSTMODEL_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/ICostModel.sol/ICostModel.json"
    );
    pub static COSTMODELJUMPRATEFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/CostModelJumpRateFactory.sol/CostModelJumpRateFactory.json"
    );
    pub static IDRIPDECAYMODEL_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/IDripDecayModel.sol/IDripDecayModel.json"
    );
    pub static EXPONENTIALDECAY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/ExponentialDecay.sol/ExponentialDecay.json"
    );
    pub static COSTMODELDYNAMICLEVEL_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/CostModelDynamicLevel.sol/CostModelDynamicLevel.json"
    );
    pub static FIXEDPOINTMATHLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/FixedPointMathLib.sol/FixedPointMathLib.json"
    );
    pub static COSTMODELAREACALCULATIONSLIB_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/CostModelAreaCalculationsLib.sol/CostModelAreaCalculationsLib.json"
    );
    pub static DRIPDECAYMODELCONSTANT_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/DripDecayModelConstant.sol/DripDecayModelConstant.json"
    );
    pub static COSTMODELJUMPRATE_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/CostModelJumpRate.sol/CostModelJumpRate.json"
    );
    pub static BASEMODELFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/BaseModelFactory.sol/BaseModelFactory.json"
    );
    pub static CREATE2_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/Create2.sol/Create2.json"
    );
    pub static COSTMODELDYNAMICLEVELFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/CostModelDynamicLevelFactory.sol/CostModelDynamicLevelFactory.json"
    );
    pub static DRIPDECAYMODELCONSTANTFACTORY_RAW_ABI: &str = include_str!(
        "/Users/raghavbansal/Documents/arbiter-cozy/contracts/cozy-models-v2-simulation/out/DripDecayModelConstantFactory.sol/DripDecayModelConstantFactory.json"
    );
}
