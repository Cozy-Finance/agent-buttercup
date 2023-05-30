use std::error::Error;

use bindings::cozy_router;
use crossbeam_channel::Receiver;
use ethers::{
    prelude::U256,
    types::{Sign, I256},
};
use eyre::Result;
use revm::primitives::{ruint::Uint, Address, B160};
use simulate::{
    agent::{
        cozy_passive_buyer::CozyPassiveBuyer, simple_arbitrageur::SimpleArbitrageur, Agent,
        AgentType, SimulationEventFilter,
    },
    environment::contract::{IsDeployed, SimulationContract},
    manager::SimulationManager,
    utils::unpack_execution,
};

pub(crate) fn create_cozy_passive_buyer<S: Into<String>>(
    manager: &mut SimulationManager,
    set: &SimulationContract<IsDeployed>,
    name: S,
    address_iter: &u64,
) {
    let address = B160::from_low_u64_be(address_iter.clone());
    let event_filters = vec![];
    let passive_cozy_buyer = CozyPassiveBuyer::new(name, event_filters);
    manager
        .activate_agent(AgentType::CozyPassiveBuyer(passive_cozy_buyer), address)
        .unwrap();
    println!("Created a passive Cozy buyer at address: {}.", address);
}

pub(crate) fn purchase_protection(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    protection: U256,
    receiver: Address,
    max_cost: U256,
) -> Result<(U256, U256), Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let purchase_args = cozy_router::PurchaseCall {
        set: set_address.into(),
        market_id,
        protection,
        receiver: receiver.into(),
        max_cost,
    };
    let purchase_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("purchase", purchase_args)?,
        Uint::from(0),
    );
    let unpacked_purchase_result = unpack_execution(purchase_result)?;
    let purchase_return_results: (U256, U256) =
        cozy_router.decode_output("purchase", unpacked_purchase_result)?;
    println!(
        "Cozy passive buyer \"{}\" purchased protection.",
        buyer.inner().name(),
    );
    Ok(purchase_return_results)
}

pub(crate) fn purchase_protection_without_transfer(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    protection: U256,
    receiver: Address,
    max_cost: U256,
) -> Result<(U256, U256), Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let purchase_args = cozy_router::PurchaseWithoutTransferCall {
        set: set_address.into(),
        market_id,
        protection,
        receiver: receiver.into(),
    };
    let purchase_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("purchaseWithoutTransfer", purchase_args)?,
        Uint::from(0),
    );
    let unpacked_purchase_result = unpack_execution(purchase_result)?;
    let purchase_return_results: (U256, U256) =
        cozy_router.decode_output("purchaseWithoutTransfer", unpacked_purchase_result)?;
    println!(
        "Cozy passive buyer \"{}\" purchased protection.",
        buyer.inner().name(),
    );
    Ok(purchase_return_results)
}

pub(crate) fn cancel_protection(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    protection: U256,
    receiver: Address,
    min_refund: U256,
) -> Result<(U256, U256), Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let cancel_args = cozy_router::CancelCall {
        set: set_address.into(),
        market_id,
        protection,
        receiver: receiver.into(),
        min_refund,
    };
    let cancel_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("cancel", cancel_args)?,
        Uint::from(0),
    );
    let cancel_result = unpack_execution(cancel_result)?;
    let cancel_return_results: (U256, U256) = cozy_router.decode_output("cancel", cancel_result)?;
    println!(
        "Cozy passive buyer \"{}\" cancelled protection.",
        buyer.inner().name(),
    );
    Ok(cancel_return_results)
}

pub(crate) fn sell_protection(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    ptokens: U256,
    receiver: Address,
    min_refund: U256,
) -> Result<(U256, U256), Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let sell_args = cozy_router::SellCall {
        set: set_address.into(),
        market_id,
        ptokens,
        receiver: receiver.into(),
        min_refund,
    };
    let sell_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("sell", sell_args)?,
        Uint::from(0),
    );
    let sell_result = unpack_execution(sell_result)?;
    let sell_return_results: (U256, U256) = cozy_router.decode_output("sell", sell_result)?;
    println!(
        "Cozy passive buyer \"{}\" sold protection.",
        buyer.inner().name(),
    );
    Ok(sell_return_results)
}

pub(crate) fn claim_ptokens(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    ptokens: U256,
    receiver: Address,
) -> Result<U256, Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let claim_args = cozy_router::ClaimCall {
        set: set_address.into(),
        market_id,
        ptokens,
        receiver: receiver.into(),
    };
    let claim_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("claim", claim_args)?,
        Uint::from(0),
    );
    let claim_result = unpack_execution(claim_result)?;
    let claim_return_results: U256 = cozy_router.decode_output("claim", claim_result)?;
    println!(
        "Cozy passive buyer \"{}\" claimed ptokens.",
        buyer.inner().name(),
    );
    Ok(claim_return_results)
}

pub(crate) fn payout_protection(
    manager: &mut SimulationManager,
    set_address: Address,
    cozy_router: &SimulationContract<IsDeployed>,
    market_id: u16,
    protection: U256,
    receiver: Address,
) -> Result<(U256, U256), Box<dyn Error>> {
    let buyer = manager.agents.get("buyer").unwrap();
    let payout_args = cozy_router::PayoutCall {
        set: set_address.into(),
        market_id,
        protection,
        receiver: receiver.into(),
    };
    let payout_result = buyer.call_contract(
        &mut manager.environment,
        cozy_router,
        cozy_router.encode_function("payout", payout_args)?,
        Uint::from(0),
    );
    let payout_result = unpack_execution(payout_result)?;
    let payout_return_results: (U256, U256) = cozy_router.decode_output("payout", payout_result)?;
    println!(
        "Cozy passive buyer \"{}\" was paid out protection.",
        buyer.inner().name(),
    );
    Ok(payout_return_results)
}
