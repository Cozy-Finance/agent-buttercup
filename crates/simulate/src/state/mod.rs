use std::convert::Infallible;

use ethers::{
    abi::{Detokenize, Tokenize},
    contract::ContractCall,
    prelude::ProviderError,
    providers::{JsonRpcClient, Middleware, MiddlewareError, Provider},
    types::transaction::eip2718::TypedTransaction,
};
use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    primitives::{AccountInfo, EVMError, ExecutionResult, Halt, Output, TransactTo, TxEnv},
    EVM,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    address::Address,
    state::update::{EvmStateUpdate, EvmStateUpdateOutput, Update, WorldStateUpdate},
    time_policy::TimeEnv,
    u256::U256,
    utils::{decode_output, GasSettings},
    EthersBytes, EthersContract, EvmAddress, EvmBytes,
};

pub mod update;

pub trait World: Sync + Send {
    type WorldUpdate: Update;
    fn execute(&mut self, update: Self::WorldUpdate) -> Option<Self::WorldUpdate>;
}

pub type StateResult<T> = core::result::Result<T, StateError>;

#[derive(thiserror::Error, Debug)]
pub enum StateError {
    #[error("Evm DB not set.")]
    EvmDBNotSet,
    #[error("Evm account not found at address: {0}.")]
    EvmAccountNotFound(Address),
    #[error("EVM execution error: {0:?}.")]
    EvmExecutionError(EVMError<Infallible>),
    #[error("Missing tx data: {0:?}.")]
    EvmMissingTxData(TypedTransaction),
    #[error("Error encoding constructor: {0:?}.")]
    ConstructorEncodeError(ethers::abi::Error),
    #[error("Error decoding function output: {0:?}.")]
    FunctionOutputDecodeError(ethers::abi::AbiError),
    #[error("Missing create address.")]
    MissingCreateAddress,
    #[error("Error sending on channel.")]
    ChannelSendError,
}

pub struct State<U, W>
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub world: W,
}

impl<U, W> State<U, W>
where
    U: Update,
    W: World<WorldUpdate = U>,
{
    pub fn new(world: W) -> Self {
        let mut evm = EVM::new();
        evm.database(CacheDB::new(EmptyDB {}));
        evm.env.cfg.limit_contract_code_size = Some(0x10000000000); // This is a large contract size limit, beware!
        State { evm, world }
    }

    pub fn get_read_db(&self) -> StateResult<&CacheDB<EmptyDB>> {
        self.evm.db.as_ref().ok_or(StateError::EvmDBNotSet)
    }

    pub fn timestamp(&self) -> U256 {
        U256::from(self.evm.env.block.timestamp)
    }

    pub fn block_number(&self) -> U256 {
        U256::from(self.evm.env.block.number)
    }

    /// Update the evm's block number and timestamp.
    pub fn update_time(&mut self, time_env: TimeEnv) {
        self.evm.env.block.number = time_env.block_number.into();
        self.evm.env.block.timestamp = time_env.block_timestamp.into();
    }

    pub fn read_account_info(&self, address: Address) -> StateResult<AccountInfo> {
        let account_info = self
            .get_read_db()?
            .basic(address.into())
            .map_err(|e| StateError::EvmExecutionError(EVMError::Database(e)))?
            .ok_or(StateError::EvmAccountNotFound(address))?;
        Ok(account_info)
    }

    /// Add an account to the evm.
    pub fn add_account(&mut self, address: Address, account_info: AccountInfo) -> StateResult<()> {
        self.evm
            .db()
            .ok_or(StateError::EvmDBNotSet)?
            .insert_account_info(address.into(), account_info);

        Ok(())
    }

    pub fn call_evm_tx_raw(&self, tx: TxEnv) -> StateResult<EvmTxOutput> {
        let mut sim_evm = EVM::with_env(self.evm.env.clone());
        sim_evm.database(self.get_read_db()?);
        sim_evm.env.tx = tx;

        let output = sim_evm
            .transact_ref()
            .map_err(StateError::EvmExecutionError)?;
        unpack_evm_tx_output(output.result)
    }

    pub fn call_evm_tx<D: Detokenize>(
        &self,
        caller: Address,
        call: ContractCall<StateMiddleware, D>,
    ) -> StateResult<EvmTxOutput> {
        let tx = build_revm_tx(caller, call.tx)?;
        let mut sim_evm = EVM::with_env(self.evm.env.clone());
        sim_evm.database(self.get_read_db()?);
        sim_evm.env.tx = tx;

        let output = sim_evm
            .transact_ref()
            .map_err(StateError::EvmExecutionError)?;
        unpack_evm_tx_output(output.result)
    }

    pub fn call_evm_tx_and_decode<D: Detokenize>(
        &self,
        caller: Address,
        call: ContractCall<StateMiddleware, D>,
    ) -> StateResult<D> {
        let tx = build_revm_tx(caller, call.tx)?;
        let mut sim_evm = EVM::with_env(self.evm.env.clone());
        sim_evm.database(self.get_read_db()?);
        sim_evm.env.tx = tx;

        let output = sim_evm
            .transact_ref()
            .map_err(StateError::EvmExecutionError)?;
        let output_bytes = unpack_evm_tx_output(output.result)?.parse_success();
        decode_output(&call.function, output_bytes).map_err(StateError::FunctionOutputDecodeError)
    }

    pub fn execute_evm_tx(&mut self, tx: TxEnv) -> StateResult<EvmTxOutput> {
        self.evm.env.tx = tx;

        let output = self
            .evm
            .transact_commit()
            .map_err(StateError::EvmExecutionError)?;
        unpack_evm_tx_output(output)
    }

    pub fn execute_evm_tx_and_decode<D: Detokenize>(
        &mut self,
        caller: Address,
        call: ContractCall<StateMiddleware, D>,
    ) -> StateResult<D> {
        let tx = build_revm_tx(caller, call.tx)?;
        let output_bytes = self.execute_evm_tx(tx)?.parse_success();
        decode_output(&call.function, output_bytes).map_err(StateError::FunctionOutputDecodeError)
    }

    pub fn execute_evm_tx_state_update(
        &mut self,
        evm_tx_state_update: EvmStateUpdate,
    ) -> StateResult<()> {
        match evm_tx_state_update {
            EvmStateUpdate::Execute(caller, tx, function, result_sender) => {
                let tx = build_revm_tx(caller, tx)?;
                let output = self.execute_evm_tx(tx)?;
                result_sender
                    .send(EvmStateUpdateOutput::Execute(output, function))
                    .map_err(|_| StateError::ChannelSendError)?;
            }
            EvmStateUpdate::ExecuteRaw(_, tx, result_sender) => {
                let output = self.execute_evm_tx(tx)?;
                result_sender
                    .send(EvmStateUpdateOutput::ExecuteRaw(output))
                    .map_err(|_| StateError::ChannelSendError)?;
            }
        }
        Ok(())
    }

    pub fn execute_call_raw<D: Detokenize>(&mut self, tx: TxEnv) -> StateResult<EvmBytes> {
        self.evm.env.tx = tx;

        let output = self
            .evm
            .transact_commit()
            .map_err(StateError::EvmExecutionError)?;
        Ok(unpack_evm_tx_output(output)?.parse_success())
    }

    pub fn deploy_evm_contract<T: Tokenize>(
        &mut self,
        caller: Address,
        abi: &EthersContract,
        bytecode: &EthersBytes,
        args: T,
    ) -> StateResult<Address> {
        let tx_data = match abi.clone().constructor {
            Some(constructor) => {
                let encoded_vec = constructor
                    .encode_input(bytecode.to_vec(), &args.into_tokens())
                    .map_err(StateError::ConstructorEncodeError)?;
                EvmBytes::from(encoded_vec)
            }
            None => bytecode.0.clone(),
        };
        let tx_gas_settings = GasSettings::default();
        self.evm.env.tx = TxEnv {
            caller: caller.into(),
            gas_limit: tx_gas_settings.gas_limit,
            gas_price: tx_gas_settings.gas_price.into(),
            gas_priority_fee: tx_gas_settings.gas_priority_fee.map(|x| x.into()),
            transact_to: TransactTo::create(),
            value: U256::zero().into(),
            data: tx_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        };

        let output = self
            .evm
            .transact_commit()
            .map_err(StateError::EvmExecutionError)?;
        match unpack_evm_tx_output(output) {
            Ok(output) => match output {
                EvmTxOutput::SuccessCreate(address) => Ok(address),
                _ => unreachable!(),
            },
            Err(e) => Err(e),
        }
    }

    pub fn execute_world_state_update(
        &mut self,
        world_state_update: WorldStateUpdate<U>,
    ) -> StateResult<()> {
        let result = self.world.execute(world_state_update.update);
        if let Some(result) = result {
            world_state_update
                .result_sender
                .send(result)
                .map_err(|_| StateError::ChannelSendError)?;
        }
        Ok(())
    }
}

pub fn build_revm_tx(caller: Address, ethers_tx: TypedTransaction) -> StateResult<TxEnv> {
    let transact_to = match ethers_tx.to_addr() {
        Some(to) => TransactTo::Call(EvmAddress::from(*to)),
        None => unreachable!(),
    };
    let tx_data = EvmBytes::from(
        ethers_tx
            .data()
            .ok_or(StateError::EvmMissingTxData(ethers_tx.clone()))?
            .to_vec(),
    );
    let tx_gas_settings = GasSettings::default();
    Ok(TxEnv {
        caller: caller.into(),
        gas_limit: tx_gas_settings.gas_limit,
        gas_price: tx_gas_settings.gas_price.into(),
        gas_priority_fee: tx_gas_settings.gas_priority_fee.map(|x| x.into()),
        transact_to,
        value: U256::zero().into(),
        data: tx_data,
        chain_id: None,
        nonce: None,
        access_list: Vec::new(),
    })
}

#[derive(Debug, Clone)]
pub enum EvmTxOutput {
    Success(EvmBytes),
    SuccessCreate(Address),
    Halt(Halt),
    Revert(EvmBytes),
}

impl EvmTxOutput {
    pub fn parse_success(self) -> EvmBytes {
        match self {
            EvmTxOutput::Success(output) => output,
            _ => panic!("EvmTxOutput is not success."),
        }
    }
}

pub fn unpack_evm_tx_output(result: ExecutionResult) -> StateResult<EvmTxOutput> {
    match result {
        ExecutionResult::Halt { reason, .. } => Ok(EvmTxOutput::Halt(reason)),
        ExecutionResult::Revert { output, .. } => Ok(EvmTxOutput::Revert(output)),
        ExecutionResult::Success { output, .. } => match output {
            Output::Call(output) => Ok(EvmTxOutput::Success(output)),
            Output::Create(_, address) => Ok(EvmTxOutput::SuccessCreate(
                address.ok_or(StateError::MissingCreateAddress)?.into(),
            )),
        },
    }
}

#[derive(Debug)]
pub struct StateMiddleware();

#[async_trait::async_trait]
impl Middleware for StateMiddleware {
    type Error = StateError;
    type Provider = StateProvider;
    type Inner = Self;

    fn inner(&self) -> &Self::Inner {
        unimplemented!()
    }

    fn provider(&self) -> &Provider<Self::Provider> {
        unimplemented!()
    }
}

impl MiddlewareError for StateError {
    type Inner = Self;

    fn from_err(e: Self::Inner) -> Self {
        e
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        None
    }
}

#[derive(Debug)]
pub struct StateProvider();

#[async_trait::async_trait]
impl JsonRpcClient for StateProvider {
    type Error = ProviderError;

    async fn request<T: Serialize + Send + Sync, R: DeserializeOwned>(
        &self,
        _method: &str,
        _params: T,
    ) -> Result<R, ProviderError> {
        unimplemented!()
    }
}
