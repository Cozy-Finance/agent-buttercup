use eyre::Result;
use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    primitives::{AccountInfo, Address, ExecutionResult, TxEnv},
    Database, EVM,
};
use thiserror::Error;

use crate::state::update::{SimUpdate, Update};
use crate::state::world_state::WorldState;
use crate::time_policy::TimeEnv;
use crate::utils::*;

pub mod update;
pub mod world_state;

#[derive(Error, Debug)]
pub enum SimStateError {
    #[error("Evm db error")]
    EvmDbError,
}

#[derive(Clone)]
pub struct SimState<U: Update> {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub world: Option<Box<dyn WorldState<WorldStateUpdate = U>>>,
}

impl<U: Update> Default for SimState<U> {
    fn default() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        SimState { evm, world: None }
    }
}

impl<U: Update> SimState<U> {
    pub fn new(world: Option<Box<dyn WorldState<WorldStateUpdate = U>>>) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000000000); // This is a large contract size limit, beware!
        evm.database(db);
        SimState { evm, world }
    }

    /// Update the time env.
    /// # Arguments
    /// * `time_env` - The time env.
    pub fn update_time_env(&mut self, time_env: TimeEnv) {
        self.evm.env.block.number = time_env.number;
        self.evm.env.block.timestamp = time_env.timestamp;
    }

    // Add an account to evm.
    pub fn insert_account_info(
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

    pub fn get_account_info(&self, address: Address) -> Result<AccountInfo> {
        Ok(self
            .evm
            .db
            .as_ref()
            .ok_or(SimStateError::EvmDbError)?
            .db
            .basic(address)?
            .ok_or(SimStateError::EvmDbError)?)
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_evm_tx(&mut self, tx: &TxEnv) -> Result<ExecutionResult> {
        self.evm.env.tx = tx.clone();
        Ok(self
            .evm
            .transact_commit()
            .map_err(|_| SimStateError::EvmDbError)?)
    }

    /// Execute a transaction in the execution environment without writing to DB.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn simulate_evm_tx(&mut self, tx: &TxEnv) -> Result<ExecutionResult> {
        self.evm.env.tx = tx.clone();
        Ok(self
            .evm
            .transact()
            .map_err(|_| SimStateError::EvmDbError)?
            .result)
    }

    pub fn execute_world_update(&mut self, update: &U) {
        if let Some(ref mut world) = self.world {
            world.execute(update);
        }
    }

    pub fn execute(&mut self, update: &SimUpdate<U>) -> Result<()> {
        match update {
            SimUpdate::Evm(tx) => {
                self.execute_evm_tx(tx)?;
            }
            SimUpdate::World(update) => {
                self.execute_world_update(update);
            }
            SimUpdate::Bundle(tx, update) => {
                if is_execution_success(self.simulate_evm_tx(tx)?) {
                    self.execute_evm_tx(tx)?;
                    self.execute_world_update(update);
                }
            }
            SimUpdate::MultiBundle(txs, updates) => {
                for tx in txs {
                    if let Ok(tx) = self.simulate_evm_tx(tx) {
                        if !is_execution_success(tx) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                txs.iter()
                    .map(|tx| self.execute_evm_tx(tx))
                    .collect::<Result<Vec<_>>>()?;
                updates
                    .iter()
                    .map(|update| self.execute_world_update(update))
                    .for_each(drop);
            }
        }

        Ok(())
    }
}
