use std::{borrow::Cow, collections::HashMap};

use revm::{
    db::{CacheDB, DatabaseRef, EmptyDB},
    primitives::{AccountInfo, Env, ExecutionResult, TxEnv},
    EVM,
};

use crate::{
    address::Address,
    agent::agent_channel::AgentSimUpdate,
    state::{
        errors::SimStateError,
        update::{SimUpdate, SimUpdateResult, UpdateData},
        world::World,
    },
    time_policy::TimeEnv,
    u256::U256,
    utils::*,
};

pub mod errors;
pub mod update;
pub mod world;

type SimStateResult<T> = Result<T, SimStateError>;

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

    pub fn get_read_db(&self) -> SimStateResult<&CacheDB<EmptyDB>> {
        self.evm.db.as_ref().ok_or(SimStateError::EvmDBNotSet)
    }

    pub fn read_account_info(&self, address: Address) -> SimStateResult<AccountInfo> {
        let account_info = self
            .get_read_db()?
            .basic(address.into())
            .map_err(|e| SimStateError::EvmStateError(e.into()))?
            .ok_or(SimStateError::AccountNotFound(address))?;
        Ok(account_info)
    }

    pub fn read_timestamp(&self) -> U256 {
        U256::from(self.evm.env.block.timestamp)
    }

    pub fn read_block_number(&self) -> U256 {
        U256::from(self.evm.env.block.number)
    }

    pub fn simulate_evm_tx_ref(
        &self,
        tx: &TxEnv,
        env: Option<Env>,
    ) -> SimStateResult<ExecutionResult> {
        // Create a sim_evm with no db and cloned and/or passed env (fairly cheap).
        let env = env.unwrap_or(self.evm.env.clone());
        let mut sim_evm = EVM::with_env(env);
        // Set sim_evm's db to a ref of the actual evm's db.
        sim_evm.database(self.get_read_db()?);
        // Update env to new tx.
        sim_evm.env.tx = tx.clone();
        // We can now use the sim_evm to simulate the tx without writing to db.
        match sim_evm.transact_ref() {
            Ok(result_and_state) => Ok(result_and_state.result),
            Err(e) => Err(SimStateError::EvmStateError(anyhow::format_err!(
                "Error while simulating evm tx: {:?}",
                e
            ))),
        }
    }

    /// Update the time env.
    /// # Arguments
    /// * `time_env` - The time env.
    pub fn update_time_env(&mut self, time_env: TimeEnv) {
        self.evm.env.block.number = time_env.number.into();
        self.evm.env.block.timestamp = time_env.timestamp.into();
    }

    // Add an account to evm.
    pub fn insert_account_info(
        &mut self,
        address: Address,
        account_info: AccountInfo,
    ) -> SimStateResult<()> {
        self.evm
            .db()
            .ok_or(SimStateError::EvmDBNotSet)?
            .insert_account_info(address.into(), account_info);

        Ok(())
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn execute_evm_tx(&mut self, tx: TxEnv) -> SimStateResult<ExecutionResult> {
        self.evm.env.tx = tx;
        match self.evm.transact_commit() {
            Ok(result) => Ok(result),
            Err(e) => Err(SimStateError::EvmStateError(anyhow::format_err!(
                "Error while executing evm tx: {:?}",
                e
            ))),
        }
    }

    /// Execute a transaction in the execution environment without writing to DB.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub fn simulate_evm_tx(&mut self, tx: &TxEnv) -> SimStateResult<ExecutionResult> {
        self.evm.env.tx = tx.clone();
        match self.evm.transact() {
            Ok(result_and_state) => Ok(result_and_state.result),
            Err(e) => Err(SimStateError::EvmStateError(anyhow::format_err!(
                "Error while simulating evm tx: {:?}",
                e
            ))),
        }
    }

    pub fn execute_world_update(&mut self, update: U) -> Option<U> {
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

    pub fn execute(&mut self, agent_update: AgentSimUpdate<U>) -> SimStateResult<()> {
        match agent_update.update {
            SimUpdate::Evm(tx) => {
                let result = self.execute_evm_tx(tx)?;
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::Evm(result),
                    );
                }
            }
            SimUpdate::World(update) => {
                let result = self.execute_world_update(update);
                if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::World(result),
                    );
                }
            }
            SimUpdate::Bundle(tx, update) => {
                let sim_evm_result = self.simulate_evm_tx(&tx)?;
                let bundle_success = is_execution_success(&sim_evm_result);
                if bundle_success {
                    let evm_result = self.execute_evm_tx(tx)?;
                    let world_result = self.execute_world_update(update);
                    if let Some(tag) = agent_update.tag {
                        self.insert_into_update_results(
                            tag,
                            agent_update.address,
                            SimUpdateResult::Bundle(true, evm_result, world_result),
                        );
                    }
                } else if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::Bundle(false, sim_evm_result, None),
                    );
                }
            }
            SimUpdate::MultiBundle(txs, updates) => {
                let sim_evm_results = txs
                    .iter()
                    .map(|t| self.simulate_evm_tx(t))
                    .collect::<Result<Vec<_>, SimStateError>>()?;
                let bundle_success = sim_evm_results.iter().map(is_execution_success).all(|x| x);
                if bundle_success {
                    let evm_results = txs
                        .into_iter()
                        .map(|tx| self.execute_evm_tx(tx))
                        .collect::<Result<Vec<_>, SimStateError>>()?;
                    let world_results = updates
                        .into_iter()
                        .map(|update| self.execute_world_update(update))
                        .collect::<Vec<_>>();
                    if let Some(tag) = agent_update.tag {
                        self.insert_into_update_results(
                            tag,
                            agent_update.address,
                            SimUpdateResult::MultiBundle(true, evm_results, world_results),
                        );
                    }
                } else if let Some(tag) = agent_update.tag {
                    self.insert_into_update_results(
                        tag,
                        agent_update.address,
                        SimUpdateResult::MultiBundle(false, sim_evm_results, vec![]),
                    );
                }
            }
        }

        Ok(())
    }
}
