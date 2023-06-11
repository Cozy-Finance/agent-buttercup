use std::collections::HashMap;

use eyre::Result;
use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB, RefDBWrapper},
    primitives::{AccountInfo, Address, ExecutionResult, TxEnv},
    Database, EVM,
};
use thiserror::Error;

use crate::agent::agent_channel::AgentSimUpdate;
use crate::agent::agent_channel::AgentUpdateResults;
use crate::state::update::{SimUpdate, SimUpdateResult, Update};
use crate::state::world_state::WorldState;
use crate::time_policy::TimeEnv;
use crate::utils::*;
use crate::EvmAddress;

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
    pub update_results: HashMap<EvmAddress, HashMap<u64, SimUpdateResult>>,
}

impl<U: Update> Default for SimState<U> {
    fn default() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.database(db);
        SimState {
            evm,
            world: None,
            update_results: HashMap::new(),
        }
    }
}

impl<U: Update> SimState<U> {
    pub fn new(world: Option<Box<dyn WorldState<WorldStateUpdate = U>>>) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000000000); // This is a large contract size limit, beware!
        evm.database(db);
        SimState {
            evm,
            world,
            update_results: HashMap::new(),
        }
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
        let raw_db = self.evm.db.as_ref().ok_or(SimStateError::EvmDbError)?;
        let db = RefDBWrapper::new(&raw_db).db;
        Ok(db
            .basic(address)
            .map_err(|_| SimStateError::EvmDbError)?
            .ok_or(SimStateError::EvmDbError)?)
    }

    pub fn get_results(&self, address: &EvmAddress) -> AgentUpdateResults {
        AgentUpdateResults::new(self.update_results.get(address))
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_raw_evm_tx(&mut self, tx: &TxEnv) -> Result<ExecutionResult> {
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
    pub fn simulate_raw_evm_tx(&mut self, tx: &TxEnv) -> Result<ExecutionResult> {
        self.evm.env.tx = tx.clone();
        Ok(self
            .evm
            .transact()
            .map_err(|_| SimStateError::EvmDbError)?
            .result)
    }

    pub fn execute_raw_world_update(&mut self, update: &U) {
        if let Some(ref mut world) = self.world {
            world.execute(update);
        }
    }

    pub fn insert_into_update_results(
        &mut self,
        tag: u64,
        address: EvmAddress,
        result: SimUpdateResult,
    ) {
        if let Some(agent_update_results) = self.update_results.get_mut(&address) {
            agent_update_results.insert(tag, result);
        } else {
            self.update_results
                .insert(address, HashMap::from([(tag, result)]));
        }
    }

    pub fn execute(&mut self, agent_update: &AgentSimUpdate<U>) -> Result<()> {
        match &agent_update.update {
            SimUpdate::Evm(tx) => {
                let result = self.execute_raw_evm_tx(tx)?;
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address.clone(),
                        SimUpdateResult::Evm(result),
                    );
                }
            }
            SimUpdate::World(update) => {
                self.execute_raw_world_update(update);
            }
            SimUpdate::Bundle(tx, update) => {
                let bundle_success = is_execution_success(self.simulate_raw_evm_tx(tx)?);
                if bundle_success {
                    self.execute_raw_evm_tx(tx)?;
                    self.execute_raw_world_update(update);
                }
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address.clone(),
                        SimUpdateResult::Bundle(bundle_success),
                    );
                }
            }

            SimUpdate::MultiBundle(txs, updates) => {
                let mut bundle_success = true;
                for tx in txs {
                    if let Ok(tx) = self.simulate_raw_evm_tx(tx) {
                        if !is_execution_success(tx) {
                            bundle_success = false;
                        }
                    } else {
                        bundle_success = false;
                    }
                }
                if bundle_success {
                    txs.iter()
                        .map(|tx| self.execute_raw_evm_tx(tx))
                        .collect::<Result<Vec<_>>>()?;
                    updates
                        .iter()
                        .map(|update| self.execute_raw_world_update(update))
                        .for_each(drop);
                }
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address.clone(),
                        SimUpdateResult::MultiBundle(bundle_success),
                    );
                }
            }
        }

        Ok(())
    }
}
