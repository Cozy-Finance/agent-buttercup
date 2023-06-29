use std::{borrow::Cow, collections::HashMap};

use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    primitives::{AccountInfo, Env, ExecutionResult, TxEnv, U256 as EvmU256},
    EVM,
};
use thiserror::Error;

use crate::{
    address::Address,
    agent::agent_channel::AgentSimUpdate,
    state::{
        update::{SimUpdate, SimUpdateResult, UpdateData},
        world::World,
    },
    time_policy::TimeEnv,
    utils::*,
};

pub mod update;
pub mod world;

#[derive(Error, Debug)]
pub enum SimStateError {
    #[error("Evm db error")]
    EvmDbError,
}

#[derive(Clone, Default)]
pub struct SimState<U: UpdateData, W: World<WorldUpdateData = U>> {
    pub evm: EVM<CacheDB<EmptyDB>>,
    pub world: W,
    pub update_results: HashMap<Address, HashMap<Cow<'static, str>, SimUpdateResult<U>>>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> SimState<U, W> {
    pub fn new(world: W) -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x10000000000); // This is a large contract size limit, beware!
        evm.database(db);
        SimState {
            evm,
            world,
            update_results: HashMap::new(),
        }
    }

    pub fn get_read_db(&self) -> &CacheDB<EmptyDB> {
        self.evm.db.as_ref().expect("Db not initalized.")
    }

    pub fn read_account_info(&self, address: Address) -> AccountInfo {
        let account_info = self
            .get_read_db()
            .basic(address.into())
            .expect("Db not initialized")
            .expect("Account not found");
        log::debug!("{:?}", account_info);
        account_info
    }

    pub fn read_timestamp(&self) -> EvmU256 {
        self.evm.env.block.timestamp
    }

    pub fn read_block_number(&self) -> EvmU256 {
        self.evm.env.block.number
    }

    pub fn simulate_evm_tx_ref(&self, tx: &TxEnv, env: Option<Env>) -> ExecutionResult {
        // Create a sim_evm with no db and cloned and/or passed env (fairly cheap).
        let env = env.unwrap_or(self.evm.env.clone());
        let mut sim_evm = EVM::with_env(env);
        // Set sim_evm's db to a ref of the actual evm's db.
        sim_evm.database(self.get_read_db());
        // Update env to new tx.
        sim_evm.env.tx = tx.clone();
        // We can now use the sim_evm to simulate the tx without writing to db.
        match sim_evm.transact_ref() {
            Ok(result_and_state) => result_and_state.result,
            Err(e) => panic!("Raw evm tx execution failed: {:?}.", e),
        }
    }

    /// Update the time env.
    /// # Arguments
    /// * `time_env` - The time env.
    pub fn update_time_env(&mut self, time_env: &TimeEnv) {
        self.evm.env.block.number = time_env.number;
        self.evm.env.block.timestamp = time_env.timestamp;
    }

    // Add an account to evm.
    pub fn insert_account_info(&mut self, address: &Address, account_info: &AccountInfo) {
        self.evm
            .db()
            .expect("Db not initialized")
            .insert_account_info((*address).into(), account_info.clone());
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_evm_tx(&mut self, tx: &TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx.clone();
        match self.evm.transact_commit() {
            Ok(result) => result,
            Err(e) => panic!("Raw evm tx execution failed: {:?}.", e),
        }
    }

    /// Execute a transaction in the execution environment without writing to DB.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn simulate_evm_tx(&mut self, tx: &TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx.clone();
        match self.evm.transact() {
            Ok(result_and_state) => result_and_state.result,
            Err(e) => panic!("Raw evm tx simulation failed: {:?}.", e),
        }
    }

    pub fn execute_world_update(&mut self, update: &U) -> Option<U> {
        self.world.execute(update)
    }

    pub fn insert_into_update_results(
        &mut self,
        tag: Cow<'static, str>,
        address: Address,
        result: SimUpdateResult<U>,
    ) {
        if let Some(agent_update_results) = self.update_results.get_mut(&address) {
            agent_update_results.insert(tag, result);
        } else {
            self.update_results
                .insert(address, HashMap::from([(tag, result)]));
        }
    }

    pub fn clear_all_results(&mut self) {
        for results in self.update_results.drain() {
            let address = format!("{:2X}", results.0);
            for result in results.1.iter() {
                log::debug!("{}({}): {:?}", result.0, address, result.1);
            }
        }
    }

    pub fn execute(&mut self, agent_update: &AgentSimUpdate<U>) {
        match &agent_update.update {
            SimUpdate::Evm(tx) => {
                let result = self.execute_evm_tx(tx);
                if let Some(tag) = &agent_update.tag {
                    self.insert_into_update_results(
                        tag.clone(),
                        agent_update.address,
                        SimUpdateResult::Evm(result),
                    );
                }
            }
            SimUpdate::World(update) => {
                let result = self.execute_world_update(&update);
                if let Some(tag) = &agent_update.tag {
                    self.insert_into_update_results(
                        tag.clone(),
                        agent_update.address,
                        SimUpdateResult::World(result),
                    );
                }
            }
            SimUpdate::Bundle(tx, update) => {
                let sim_evm_result = self.simulate_evm_tx(&tx);
                let bundle_success = is_execution_success(&sim_evm_result);
                if bundle_success {
                    let evm_result = self.execute_evm_tx(&tx);
                    let world_result = self.execute_world_update(&update);
                    if let Some(tag) = &agent_update.tag {
                        self.insert_into_update_results(
                            tag.clone(),
                            agent_update.address,
                            SimUpdateResult::Bundle(true, evm_result, world_result),
                        );
                    }
                } else if let Some(tag) = &agent_update.tag {
                    self.insert_into_update_results(
                        tag.clone(),
                        agent_update.address,
                        SimUpdateResult::Bundle(false, sim_evm_result, None),
                    );
                }
            }
            SimUpdate::MultiBundle(txs, updates) => {
                let sim_evm_results = txs
                    .iter()
                    .map(|t| self.simulate_evm_tx(t))
                    .collect::<Vec<_>>();
                let bundle_success = sim_evm_results
                    .iter()
                    .map(|result| is_execution_success(result))
                    .all(|x| x);
                if bundle_success {
                    let evm_results = txs
                        .iter()
                        .map(|tx| self.execute_evm_tx(tx))
                        .collect::<Vec<_>>();
                    let world_results = updates
                        .iter()
                        .map(|update| self.execute_world_update(update))
                        .collect::<Vec<_>>();
                    if let Some(tag) = &agent_update.tag {
                        self.insert_into_update_results(
                            tag.clone(),
                            agent_update.address,
                            SimUpdateResult::MultiBundle(true, evm_results, world_results),
                        );
                    }
                } else if let Some(tag) = &agent_update.tag {
                    self.insert_into_update_results(
                        tag.clone(),
                        agent_update.address,
                        SimUpdateResult::MultiBundle(false, sim_evm_results, vec![]),
                    );
                }
            }
        }
    }
}
