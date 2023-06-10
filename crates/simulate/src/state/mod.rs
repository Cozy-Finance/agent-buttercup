use eyre::Result;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, Address, ExecutionResult, TxEnv},
    EVM,
};
use thiserror::Error;

use crate::state::update::{SimUpdate, Update};
use crate::state::world_state::WorldState;
use crate::time_policy::BlockTimeEnv;

pub mod update;
pub mod world_state;

#[derive(Error, Debug, Clone)]
pub enum SimStateError {
    #[error("Evm db error")]
    EvmDbError,
}

pub struct SimState<U: Update> {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub world: Option<Box<dyn WorldState<WorldStateUpdate = U> + Send + Sync>>,
}

impl<U: Update> SimState<U> {
    pub fn new(world: Option<Box<dyn WorldState<WorldStateUpdate = U> + Send + Sync>>) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000000000); // This is a large contract size limit, beware!
        evm.database(db);
        SimState { evm, world }
    }

    /// Update the block time env.
    /// # Arguments
    /// * `block_time_env` - The block time env.
    pub fn update_block_time_env(&mut self, block_time_env: BlockTimeEnv) {
        self.evm.env.block.number = block_time_env.number;
        self.evm.env.block.timestamp = block_time_env.timestamp;
    }

    // Add an account to evm.
    pub fn add_account_info(&mut self, address: Address, account_info: AccountInfo) -> Result<()> {
        self.evm
            .db()
            .ok_or(SimStateError::EvmDbError)?
            .insert_account_info(address, account_info);

        Ok(())
    }

    pub fn update_account_info(
        &mut self,
        address: Address,
        account_info: AccountInfo,
    ) -> Result<()> {
        self.evm
            .db()
            .ok_or(SimStateError::EvmDbError)?
            .insert_account_info(address, account_info);

        Ok(())
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_evm_tx(&mut self, tx: TxEnv) -> Result<ExecutionResult> {
        self.evm.env.tx = tx;
        Ok(self
            .evm
            .transact_commit()
            .map_err(|_| SimStateError::EvmDbError)?)
    }

    pub fn execute_world_update(&mut self, update: U) {
        self.world.as_mut().unwrap().execute(update)
    }

    pub fn execute(&mut self, update: SimUpdate<U>) -> () {
        match update {
            SimUpdate::Evm(tx) => {
                self.execute_evm_tx(tx);
            }
            SimUpdate::World(update) => {
                self.execute_world_update(*update);
            }
            SimUpdate::Bundle(tx, update) => (),
            SimUpdate::MultiBundle(tx, update) => (),
        }
    }
}
