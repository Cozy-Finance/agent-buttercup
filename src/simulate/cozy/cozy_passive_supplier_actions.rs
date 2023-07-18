use std::error::Error;

use bindings::cozy_router::{self, DepositWithoutTransferReturn};
use crossbeam_channel::Receiver;
use ethers::{
    prelude::U256,
    types::{Sign, I256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, Address, B160};
use simulate::{
    agent::{
        cozy_passive_supplier::CozyPassiveSupplier, simple_arbitrageur::SimpleArbitrageur, Agent,
        AgentType, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::unpack_execution,
};

pub(crate) fn create_cozy_passive_supplier<S: Into<String>>(
    manager: &mut SimulationManager,
    set: &SimulationContract<IsDeployed>,
    name: S,
    address_iter: &u64,
) {
    let address = B160::from_low_u64_be(address_iter.clone());
    let event_filters = vec![];
    let cozy_passive_supplier = CozyPassiveSupplier::new(name, event_filters);
    manager
        .activate_agent(
            AgentType::CozyPassiveSupplier(cozy_passive_supplier),
            address,
        )
        .unwrap();
    println!("Created a passive Cozy supplier at address: {}.", address);
}

pub(crate) fn deposit_assets(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    assets: U256,
    receiver: Address,
    min_shares_received: U256,
) -> Result<cozy_router::DepositReturn, Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let deposit_args = cozy_router::DepositCall {
        set: set_address.into(),
        assets,
        receiver: receiver.into(),
        min_shares_received,
    };
    let deposit_result = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("deposit", deposit_args)?,
        Uint::from(0),
    );
    let unpacked_deposit_result = unpack_execution(deposit_result)?;
    let deposit_return_results: cozy_router::DepositReturn =
        cozy_router.decode_output("deposit", unpacked_deposit_result)?;
    println!(
        "Cozy passive supplier \"{}\" deposited assets.",
        supplier.inner().name(),
    );
    Ok(deposit_return_results)
}

pub(crate) fn deposit_assets_without_transfer(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    assets: U256,
    receiver: Address,
    min_shares_received: U256,
) -> Result<cozy_router::DepositWithoutTransferReturn, Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let deposit_args = cozy_router::DepositWithoutTransferCall {
        set: set_address.into(),
        assets,
        receiver: receiver.into(),
        min_shares_received,
    };
    let deposit_result = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("depositWithoutTransfer", deposit_args)?,
        Uint::from(0),
    );
    let unpacked_deposit_result = unpack_execution(deposit_result)?;
    let deposit_return_results: cozy_router::DepositWithoutTransferReturn =
        cozy_router.decode_output("depositWithoutTransfer", unpacked_deposit_result)?;
    println!(
        "Cozy passive supplier \"{}\" deposited assets.",
        supplier.inner().name(),
    );
    Ok(deposit_return_results)
}

pub(crate) fn withdraw_assets(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    assets: U256,
    receiver: Address,
    max_shares_burned: U256,
) -> Result<cozy_router::WithdrawReturn, Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let withdraw_args = cozy_router::WithdrawCall {
        set: set_address.into(),
        assets,
        receiver: receiver.into(),
        max_shares_burned,
    };
    let withdraw_result = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("withdraw", withdraw_args)?,
        Uint::from(0),
    );
    let unpacked_withdraw_result = unpack_execution(withdraw_result)?;
    let withraw_return_results: cozy_router::WithdrawReturn =
        cozy_router.decode_output("withdraw", unpacked_withdraw_result)?;
    println!(
        "Cozy passive supplier \"{}\" withdrew assets.",
        supplier.inner().name(),
    );
    Ok(withraw_return_results)
}

pub(crate) fn redeem_shares(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    shares: U256,
    receiver: Address,
    min_assets_received: U256,
) -> Result<cozy_router::RedeemReturn, Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let redeem_args = cozy_router::RedeemCall {
        set: set_address.into(),
        shares,
        receiver: receiver.into(),
        min_assets_received,
    };
    let redeem_result = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("redeem", redeem_args)?,
        Uint::from(0),
    );
    let unpacked_redeem_result = unpack_execution(redeem_result)?;
    let redeem_return_results: cozy_router::RedeemReturn =
        cozy_router.decode_output("withdraw", unpacked_redeem_result)?;
    println!(
        "Cozy passive supplier \"{}\" redeemed shares.",
        supplier.inner().name(),
    );
    Ok(redeem_return_results)
}

pub(crate) fn complete_withdraw_assets(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    id: u64,
) -> Result<(), Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let complete_withdraw_args = cozy_router::CompleteWithdrawCall {
        set: set_address.into(),
        id,
    };
    let complete_withdraw_result = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("completeWithdraw", complete_withdraw_args)?,
        Uint::from(0),
    );
    let unpacked_complete_withraw_result = unpack_execution(complete_withdraw_result)?;
    let complete_withraw_return_results: () =
        cozy_router.decode_output("completeWithdraw", unpacked_complete_withraw_result)?;
    println!(
        "Cozy passive supplier \"{}\" completed withdrawal.",
        supplier.inner().name(),
    );
    Ok(complete_withraw_return_results)
}

pub(crate) fn complete_redeem_shares(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    id: u64,
) -> Result<(), Box<dyn Error>> {
    let supplier = manager.agents.get("supplier").unwrap();
    let complete_redeem_args = cozy_router::CompleteRedeemCall {
        set: set_address.into(),
        id,
    };
    let complete_redeem_results = supplier.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("completeRedeem", complete_redeem_args)?,
        Uint::from(0),
    );
    let unpacked_complete_redeem_result = unpack_execution(complete_redeem_results)?;
    let complete_redeem_return_results: () =
        cozy_router.decode_output("completeRedeem", unpacked_complete_redeem_result)?;
    println!(
        "Cozy passive supplier \"{}\" completed redemption.",
        supplier.inner().name(),
    );
    Ok(complete_redeem_return_results)
}
