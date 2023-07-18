use std::{borrow::Cow, fmt::Debug, sync::Arc};

use bindings::{
    cozy_models::{
        cost_model_dynamic_level_factory, cost_model_jump_rate_factory,
        drip_decay_model_constant_factory,
    },
    cozy_protocol::{cozy_router, manager},
    set::{AccountingReturn, MarketsReturn},
};
use revm::primitives::{ExecutionResult, TxEnv};
use simulate::{
    address::Address,
    contract::sim_contract::SimContract,
    state::SimState,
    u256::U256,
    utils::{build_call_tx, unpack_execution},
};

use crate::cozy::{
    world::{CozyUpdate, CozyWorld},
    EthersAddress,
};

pub type CozyWorldContractResult<T> = Result<T, anyhow::Error>;

#[macro_export]
macro_rules! impl_basic_world_contract {
    ($struct_name:ident) => {
        #[derive(Debug, Clone)]
        pub struct $struct_name {
            pub name: Cow<'static, str>,
            pub address: Address,
            pub contract: SimContract,
        }

        impl $struct_name {
            pub fn new(
                name: Cow<'static, str>,
                address: Address,
                contract: SimContract,
            ) -> Arc<Self> {
                Arc::new($struct_name {
                    name,
                    address,
                    contract,
                })
            }
        }
    };
}

impl_basic_world_contract!(CozyRouter);

impl CozyRouter {
    pub fn build_deposit_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::DepositCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("deposit", args)?,
        ))
    }

    pub fn build_deposit_without_transfer_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::DepositWithoutTransferCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract
                .encode_function("depositWithoutTransfer", args)?,
        ))
    }

    pub fn build_withdraw_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::WithdrawCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("withdraw", args)?,
        ))
    }

    pub fn build_redeem_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::RedeemCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("redeem", args)?,
        ))
    }

    pub fn build_complete_withdraw_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::CompleteWithdrawCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("completeWithdraw", args)?,
        ))
    }

    pub fn build_complete_redeem_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::CompleteRedeemCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("completeRedeem", args)?,
        ))
    }

    pub fn build_purchase_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::PurchaseCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("purchase", args)?,
        ))
    }

    pub fn build_purchase_without_transfer_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::PurchaseWithoutTransferCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract
                .encode_function("purchaseWithoutTransfer", args)?,
        ))
    }

    pub fn build_cancel_tx(
        self,
        sender_addr: Address,
        args: cozy_router::CancelCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("cancel", args)?,
        ))
    }

    pub fn build_sell_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::SellCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("sell", args)?,
        ))
    }

    pub fn build_claim_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::ClaimCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("claim", args)?,
        ))
    }

    pub fn build_payout_tx(
        &self,
        sender_addr: Address,
        args: cozy_router::PayoutCall,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract.encode_function("payout", args)?,
        ))
    }

    pub fn read_purchase_assets_needed(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cozy_router::PurchaseCall,
    ) -> CozyWorldContractResult<U256> {
        let purchase_tx = self.build_purchase_tx(sender_addr, args)?;
        let result = match unpack_execution(state.simulate_evm_tx_ref(&purchase_tx, None)?) {
            Ok(bytes) => bytes,
            _ => return Ok(U256::MAX),
        };
        let purchase_return = self
            .contract
            .decode_output::<cozy_router::PurchaseReturn>("purchase", result)?;
        Ok(purchase_return.assets_needed)
    }

    pub fn decode_ptokens_received(
        &self,
        execution_result: &ExecutionResult,
    ) -> CozyWorldContractResult<U256> {
        let purchase_output = self.contract.decode_output::<cozy_router::PurchaseReturn>(
            "purchase",
            unpack_execution(execution_result.clone())?,
        )?;
        Ok(purchase_output.ptokens)
    }
}

impl_basic_world_contract!(CozyBaseToken);

impl CozyBaseToken {
    pub fn read_token_balance(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
    ) -> CozyWorldContractResult<U256> {
        let sender_ethers_addr: EthersAddress = sender_addr.into();
        let balance_tx = build_call_tx(
            sender_addr,
            self.address,
            self.contract
                .encode_function("balanceOf", sender_ethers_addr)?,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None)?)?;
        let balance: U256 = self.contract.decode_output("balanceOf", result)?;
        Ok(balance)
    }

    pub fn build_max_approve_router_tx(
        &self,
        sender_addr: Address,
        cozyrouter_addr: Address,
    ) -> CozyWorldContractResult<TxEnv> {
        let cozyrouter_ethers_addr: EthersAddress = cozyrouter_addr.into();
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract
                .encode_function("approve", (cozyrouter_ethers_addr, U256::MAX))?,
        ))
    }

    pub fn build_transfer_token_to_router_tx(
        &self,
        sender_addr: Address,
        cozyrouter_addr: Address,
        amount: U256,
    ) -> CozyWorldContractResult<TxEnv> {
        let cozyrouter_ethers_addr: EthersAddress = cozyrouter_addr.into();
        Ok(build_call_tx(
            sender_addr,
            self.address,
            self.contract
                .encode_function("transfer", (cozyrouter_ethers_addr, amount))?,
        ))
    }
}

impl_basic_world_contract!(CozySetLogic);

impl CozySetLogic {
    pub fn read_protection_balance(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_addr: Address,
        market_id: u16,
        ptokens: U256,
    ) -> CozyWorldContractResult<U256> {
        let balance_tx = build_call_tx(
            sender_addr,
            set_addr,
            self.contract
                .encode_function("convertToProtection", (market_id, ptokens))?,
        );
        let result = unpack_execution(state.simulate_evm_tx_ref(&balance_tx, None)?)?;
        let balance: U256 = self.contract.decode_output("convertToProtection", result)?;
        Ok(balance)
    }

    fn build_remaining_protection_tx(
        &self,
        sender_addr: Address,
        set_address: Address,
        market_id: u16,
    ) -> CozyWorldContractResult<TxEnv> {
        Ok(build_call_tx(
            sender_addr,
            set_address,
            self.contract
                .encode_function("remainingProtection", market_id)?,
        ))
    }

    pub fn read_remaining_protection(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
    ) -> CozyWorldContractResult<U256> {
        let remaining_protection_tx =
            self.build_remaining_protection_tx(sender_addr, set_address, market_id)?;
        let result = unpack_execution(state.simulate_evm_tx_ref(&remaining_protection_tx, None)?)?;
        let remaining_protection_return: U256 =
            self.contract.decode_output("remainingProtection", result)?;
        Ok(remaining_protection_return)
    }

    pub fn read_market_data(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_num: usize,
    ) -> CozyWorldContractResult<MarketsReturn> {
        let call_data = self
            .contract
            .encode_function("markets", (U256::from(market_num),))?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?)?;
        Ok(self
            .contract
            .decode_output::<MarketsReturn>("markets", result)?)
    }

    pub fn read_total_assets(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
    ) -> CozyWorldContractResult<u128> {
        let call_data = self.contract.encode_function("accounting", ())?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?)?;
        let total_assets = self
            .contract
            .decode_output::<AccountingReturn>("accounting", result)?
            .asset_balance;
        Ok(total_assets)
    }

    pub fn read_total_protection_available(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("totalCollateralAvailable", ())?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let total_protection_available = self
            .contract
            .decode_output::<U256>("totalCollateralAvailable", result)?;
        Ok(total_protection_available)
    }

    pub fn read_utilization(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self.contract.encode_function("utilization", market_id)?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let utilization = self.contract.decode_output::<U256>("utilization", result)?;
        Ok(utilization)
    }

    pub fn read_effective_active_protection(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("effectiveActiveProtection", market_id)?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let effective_active_protection = self
            .contract
            .decode_output::<U256>("effectiveActiveProtection", result)?;
        Ok(effective_active_protection)
    }

    pub fn read_ptoken_addr(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        set_address: Address,
        market_id: u16,
    ) -> CozyWorldContractResult<Address> {
        let call_data = self.contract.encode_function("markets", market_id)?;
        let query = build_call_tx(sender_addr, set_address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let ptoken_addr = self
            .contract
            .decode_output::<MarketsReturn>("markets", result)?
            .ptoken;
        Ok(ptoken_addr.into())
    }
}

impl_basic_world_contract!(CozyJumpRateFactory);

impl CozyJumpRateFactory {
    pub fn build_deploy_cost_model_jump_rate_tx(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cost_model_jump_rate_factory::DeployModelCall,
    ) -> CozyWorldContractResult<(Address, TxEnv)> {
        let call_data = self.contract.encode_function("deployModel", args)?;
        let tx = build_call_tx(sender_addr, self.address, call_data);

        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None)?)
            .expect("Error simulating cost model deployment.");
        let addr: Address = self
            .contract
            .decode_output::<EthersAddress>("deployModel", tx_result)?
            .into();

        Ok((addr, tx))
    }
}

impl_basic_world_contract!(CozyJumpRateModel);

impl CozyJumpRateModel {
    pub fn read_current_cost_factor(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        utilization: U256,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("costFactor", (utilization, utilization))?;
        let query = build_call_tx(sender_addr, self.address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let cost_factor = self.contract.decode_output::<U256>("costFactor", result)?;
        Ok(cost_factor)
    }

    pub fn read_current_refund_factor(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        utilization: U256,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("refundFactor", (utilization, utilization))?;
        let query = build_call_tx(sender_addr, self.address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let refund_factor = self
            .contract
            .decode_output::<U256>("refundFactor", result)?;
        Ok(refund_factor)
    }
}

impl_basic_world_contract!(CozyDynamicLevelFactory);

impl CozyDynamicLevelFactory {
    pub fn build_deploy_cost_model_dynamic_level_tx(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: cost_model_dynamic_level_factory::DeployModelCall,
    ) -> CozyWorldContractResult<(Address, TxEnv)> {
        let call_data = self.contract.encode_function("deployModel", args)?;
        let tx = build_call_tx(sender_addr, self.address, call_data);

        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None)?)
            .expect("Error simulating cost model deployment.");
        let addr: Address = self
            .contract
            .decode_output::<EthersAddress>("deployModel", tx_result)?
            .into();

        Ok((addr, tx))
    }
}

impl_basic_world_contract!(CozyDynamicLevelModel);

impl CozyDynamicLevelModel {
    pub fn read_current_cost_factor(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        utilization: U256,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("costFactor", (utilization, utilization))?;
        let query = build_call_tx(sender_addr, self.address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let cost_factor = self.contract.decode_output::<U256>("costFactor", result)?;
        Ok(cost_factor)
    }

    pub fn read_current_refund_factor(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        utilization: U256,
    ) -> CozyWorldContractResult<U256> {
        let call_data = self
            .contract
            .encode_function("refundFactor", (utilization, utilization))?;
        let query = build_call_tx(sender_addr, self.address, call_data);
        let result = unpack_execution(state.simulate_evm_tx_ref(&query, None)?).unwrap();
        let refund_factor = self
            .contract
            .decode_output::<U256>("refundFactor", result)?;
        Ok(refund_factor)
    }
}

impl_basic_world_contract!(CozyManager);

impl CozyManager {
    pub fn build_create_set_tx(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: manager::CreateSetCall,
    ) -> CozyWorldContractResult<(Address, TxEnv)> {
        let call_data = self.contract.encode_function("createSet", args)?;
        let tx = build_call_tx(sender_addr, self.address, call_data);
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None)?)
            .expect("Error simulating cost model deployment.");
        let addr: EthersAddress = self.contract.decode_output("createSet", tx_result)?;

        Ok((addr.into(), tx))
    }
}

impl_basic_world_contract!(CozyDripDecayConstantFactory);

impl CozyDripDecayConstantFactory {
    pub fn build_deploy_drip_decay_model_constant_tx(
        &self,
        sender_addr: Address,
        state: &SimState<CozyUpdate, CozyWorld>,
        args: drip_decay_model_constant_factory::DeployModelCall,
    ) -> CozyWorldContractResult<(Address, TxEnv)> {
        let call_data = self.contract.encode_function("deployModel", args)?;
        let tx = build_call_tx(sender_addr, self.address, call_data);
        let tx_result = unpack_execution(state.simulate_evm_tx_ref(&tx, None)?)
            .expect("Error simulating drip decay model deployment.");
        let addr: Address = self
            .contract
            .decode_output::<EthersAddress>("deployModel", tx_result)?
            .into();

        Ok((addr, tx))
    }
}

impl_basic_world_contract!(CozyPtokenLogic);

impl CozyPtokenLogic {
    pub fn build_max_approve_router_tx(
        &self,
        sender_addr: Address,
        ptoken_addr: Address,
        cozyrouter_addr: Address,
    ) -> CozyWorldContractResult<TxEnv> {
        let cozyrouter_ethers_addr: EthersAddress = cozyrouter_addr.into();
        Ok(build_call_tx(
            sender_addr,
            ptoken_addr,
            self.contract
                .encode_function("approve", (cozyrouter_ethers_addr, U256::MAX))?,
        ))
    }
}

impl_basic_world_contract!(CozyUmaTriggerFactory);
impl_basic_world_contract!(CozyChainlinkTriggerFactory);
impl_basic_world_contract!(CozyBackstop);
impl_basic_world_contract!(CozySetFactory);
impl_basic_world_contract!(CozyPtokenFactory);
impl_basic_world_contract!(Weth);
