#![warn(missing_docs)]

//! ## Agent
//!
//! An abstract representation of an agent on the EVM, to be used in simulations.
//! Some examples of agents are market makers or arbitrageurs.
//! All agents must implement the [`Agent`] trait.
use std::{
    sync::Arc,
    sync::{RwLockReadGuard, RwLockWriteGuard},
    thread,
};

use tokio::sync::RwLock as AsyncRwLock;
use tokio_stream::{Stream, StreamExt};

use async_trait::async_trait;
use bytes::Bytes;
use ethers::abi::Token;
use revm::primitives::{Address, ExecutionResult, Log, Output, TransactTo, TxEnv, B160, U256};

use crate::environment::{IsDeployed, NotDeployed, SimulationContract, SimulationEnvironment};

pub mod admin;
pub mod user;
/// Describes the gas settings for a transaction.
pub struct TransactSettings {
    /// Gas limit for the transaction for a simulation.
    pub gas_limit: u64,
    /// Gas limit for the transaction for a simulation.
    pub gas_price: U256,
}

/// Basic traits that every `Agent` must implement in order to properly interact with an EVM.
#[async_trait]
pub trait Agent: Send + Sync {
    /// Returns the address of the agent.
    fn address(&self) -> Address;
    /// Returns the transaction settings of the agent.
    fn transact_settings(&self) -> &TransactSettings;
    // /// Returns the writer to the simulation environment the agent is acting in.
    // fn simulation_environment_write(&self) -> RwLockWriteGuard<'_, SimulationEnvironment>;
    // /// Returns a reader to the simulation environment the agent is acting in.
    // fn simulation_environment_read(&self) -> RwLockReadGuard<'_, SimulationEnvironment>;
    // TODO: Trying this out
    // fn simulation_environment(&self) -> &SimulationEnvironment;
    fn receiver(&self) -> crossbeam_channel::Receiver<Vec<Log>>;
    /// Used to allow agents to make a generic call a specific smart contract.
    async fn call_contract(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: &SimulationContract<IsDeployed>,
        call_data: Bytes,
        value: U256,
    ) -> ExecutionResult {
        let tx = self.build_call_transaction(contract.address.unwrap(), call_data, value);
        simulation_environment.execute(tx).await
    }

    /// A constructor to build a `TxEnv` for an agent (uses agent data like `address` and `TransactSettings`).
    fn build_call_transaction(
        &self,
        receiver_address: B160,
        call_data: Bytes,
        value: U256,
    ) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::Call(receiver_address),
            value,
            data: call_data,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }

    /// Gets the most current event (which is all that is stored in the event buffer).
    async fn read_logs(&self, simulation_environment: &mut SimulationEnvironment) -> Vec<Log> {
        self.receiver().recv().unwrap()
    }

    // async fn watch(&self, simulation_environment: SimulationEnvironment) {
    //     println!("got here?");
    //     loop {
    //         let logs = self.read_logs(simulation_environment).await;
    //         for log in logs {
    //             println!("Log: {:?}", log);
    //         }
    //     }
    // }

    /// Deploy a contract to the current simulation environment.
    async fn deploy(
        &self,
        simulation_environment: &mut SimulationEnvironment,
        contract: SimulationContract<NotDeployed>,
        args: Vec<Token>,
    ) -> SimulationContract<IsDeployed> {
        // Append constructor args (if available) to generate the deploy bytecode.
        let constructor = contract.base_contract.abi().constructor();

        let bytecode = match constructor {
            Some(constructor) => Bytes::from(
                constructor
                    .encode_input(contract.bytecode.clone(), &args)
                    .unwrap(),
            ),
            None => Bytes::from(contract.bytecode.clone()),
        };

        // Take the execution result and extract the contract address.
        // Manager address will always be the sender for contract deployments.

        let deploy_transaction = self.build_deploy_transaction(bytecode);
        let execution_result = simulation_environment.execute(deploy_transaction).await;
        let output = match execution_result {
            ExecutionResult::Success { output, .. } => output,
            ExecutionResult::Revert { output, .. } => panic!("Failed due to revert: {:?}", output),
            ExecutionResult::Halt { reason, .. } => panic!("Failed due to halt: {:?}", reason),
        };
        let contract_address = match output {
            Output::Create(_, address) => address.unwrap(),
            _ => panic!("failed"),
        };

        contract.to_deployed(contract_address)
    }
    /// Helper function to build a deploy transaction.
    fn build_deploy_transaction(&self, bytecode: Bytes) -> TxEnv {
        TxEnv {
            caller: self.address(),
            gas_limit: self.transact_settings().gas_limit,
            gas_price: self.transact_settings().gas_price,
            gas_priority_fee: None,
            transact_to: TransactTo::create(),
            value: U256::ZERO,
            data: bytecode,
            chain_id: None,
            nonce: None,
            access_list: Vec::new(),
        }
    }
}
