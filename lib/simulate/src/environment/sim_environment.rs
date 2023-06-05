#![warn(missing_docs)]
#![warn(unsafe_code)]
//! The environment that constitutes a simulation is handled here.

use crossbeam_channel::Sender;
use revm::{
    db::{CacheDB, EmptyDB},
    primitives::{AccountInfo, Address, ExecutionResult, Log, TxEnv, U256},
    EVM,
};

use crate::block_time_policy::BlockTimeEnv;
use crate::sim_env_data::SimEnvData;

/// The simulation environment that houses the execution environment and event logs.
/// # Fields
/// * `evm` - The EVM that is used for the simulation.
/// * `event_senders` - The senders on the event channel that is used to send events to the agents and simulation manager.
pub struct SimulationEnvironment {
    /// The EVM that is used for the simulation.
    pub(crate) evm: EVM<CacheDB<EmptyDB>>,
    /// The sender on the event channel that is used to send events to the agents and simulation manager.
    pub(crate) event_senders: Vec<Sender<Vec<Log>>>,
    /// Shared sim env data that is accessed by different agents.
    pub data: SimEnvData,
}

impl SimulationEnvironment {
    pub(crate) fn new() -> Self {
        let mut evm = EVM::new();
        let db = CacheDB::new(EmptyDB {});
        evm.env.cfg.limit_contract_code_size = Some(0x100000); // This is a large contract size limit, beware!
        evm.env.block.gas_limit = U256::MAX;
        evm.database(db);
        let event_senders = vec![];
        let data = SimEnvData::new();
        Self {
            evm,
            event_senders,
            data,
        }
    }

    /// Update the block time env.
    /// # Arguments
    /// * `block_time_env` - The block time env.
    pub(crate) fn update_block_time_env(&mut self, block_time_env: BlockTimeEnv) {
        self.evm.env.block.number = block_time_env.number;
        self.evm.env.block.timestamp = block_time_env.timestamp;
    }

    // Add an account to evm.
    pub(crate) fn add_account_info(&mut self, address: Address, account_info: AccountInfo) {
        self.evm
            .db()
            .unwrap()
            .insert_account_info(address, account_info);
    }

    /// Execute a transaction in the execution environment.
    /// # Arguments
    /// * `tx` - The transaction environment that is used to execute the transaction.
    /// # Returns
    /// * `ExecutionResult` - The execution result of the transaction.
    pub(crate) fn execute(&mut self, tx: TxEnv) -> ExecutionResult {
        self.evm.env.tx = tx;

        let execution_result = match self.evm.transact_commit() {
            Ok(val) => val,
            // URGENT: change this to a custom error
            Err(_) => panic!("failed"),
        };
        self.echo_logs(execution_result.logs());

        execution_result
    }
    /// Echo the logs to the event channel.
    /// # Arguments
    /// * `logs` - The logs that are to be echoed.
    fn echo_logs(&mut self, logs: Vec<Log>) {
        for event_sender in self.event_senders.iter() {
            event_sender.send(logs.clone()).unwrap();
        }
        // self.event_sender.send(logs).unwrap();
    }
    pub(crate) fn add_sender(&mut self, sender: Sender<Vec<Log>>) {
        self.event_senders.push(sender);
    }
}
