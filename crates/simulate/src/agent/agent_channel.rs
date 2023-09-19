use crossbeam_channel::{Receiver, Sender};
use ethers::abi::Detokenize;
use revm::primitives::TxEnv;

use crate::{
    address::Address,
    state::{
        update::{EvmStateUpdate, EvmStateUpdateOutput, UpdateData, WorldStateUpdate},
        EvmTxOutput, StateError, StateMiddleware,
    },
    utils::decode_output,
};

pub type AgentChannelResult<T> = Result<T, AgentChannelError>;

#[derive(thiserror::Error, Debug)]
pub enum AgentChannelError {
    #[error("Error interacting with state.")]
    StateError(#[from] StateError),
    #[error("No more tx outputs to receive.")]
    MissingEvmTxOutput,
    #[error("No more tx outputs to receive.")]
    MissingWorldUpdateOutput,
    #[error("Tried receiving wrong tx output type.")]
    WrongEvmTxOutputType,
    #[error("Tried receiving tx output without tx function.")]
    MissingEvmTxFunction,
}

pub struct AgentChannelSender<U: UpdateData> {
    address: Address,
    pub evm_update_sender: Sender<EvmStateUpdate>,
    pub evm_result_sender: Sender<EvmStateUpdateOutput>,
    pub world_update_sender: Sender<WorldStateUpdate<U>>,
    pub world_result_sender: Sender<U>,
}

impl<U: UpdateData> AgentChannelSender<U> {
    pub fn new(
        address: Address,
        evm_update_sender: Sender<EvmStateUpdate>,
        evm_result_sender: Sender<EvmStateUpdateOutput>,
        world_update_sender: Sender<WorldStateUpdate<U>>,
        world_result_sender: Sender<U>,
    ) -> Self {
        Self {
            address,
            evm_update_sender,
            evm_result_sender,
            world_update_sender,
            world_result_sender,
        }
    }

    pub fn execute_evm_tx_raw(&self, tx: TxEnv) {
        self.evm_update_sender
            .send(EvmStateUpdate::ExecuteRaw(
                self.address,
                tx,
                self.evm_result_sender.clone(),
            ))
            .expect("Channel send error.");
    }

    pub fn execute_evm_tx<D: Detokenize>(
        &self,
        call: ethers::contract::ContractCall<StateMiddleware, D>,
    ) {
        self.evm_update_sender
            .send(EvmStateUpdate::Execute(
                self.address,
                call.tx,
                call.function,
                self.evm_result_sender.clone(),
            ))
            .expect("Channel send error.");
    }

    pub fn execute_world_update(&self, update: U) {
        self.world_update_sender
            .send(WorldStateUpdate {
                update,
                result_sender: self.world_result_sender.clone(),
            })
            .expect("Channel send error.");
    }
}

pub struct AgentChannelReceiver<U: UpdateData> {
    pub evm_result_receiver: Receiver<EvmStateUpdateOutput>,
    pub world_result_receiver: Receiver<U>,
}

impl<U: UpdateData> AgentChannelReceiver<U> {
    pub fn new(
        evm_result_receiver: Receiver<EvmStateUpdateOutput>,
        world_result_receiver: Receiver<U>,
    ) -> Self {
        Self {
            evm_result_receiver,
            world_result_receiver,
        }
    }

    pub fn receive_evm_tx_output<D: Detokenize>(&self) -> AgentChannelResult<Option<D>> {
        match self.evm_result_receiver.try_recv() {
            Ok(output) => match output {
                EvmStateUpdateOutput::Execute(evm_tx_output, function) => match evm_tx_output {
                    EvmTxOutput::Success(output_bytes) => {
                        let decoded = decode_output(&function, output_bytes)
                            .map_err(|e| StateError::FunctionOutputDecodeError(e))?;
                        return Ok(Some(decoded));
                    }
                    _ => Ok(None),
                },
                EvmStateUpdateOutput::ExecuteRaw(_) => Err(AgentChannelError::WrongEvmTxOutputType),
            },
            Err(_) => Err(AgentChannelError::MissingEvmTxOutput),
        }
    }

    pub fn receive_evm_tx_output_raw(&self) -> AgentChannelResult<EvmTxOutput> {
        match self.evm_result_receiver.try_recv() {
            Ok(output) => match output {
                EvmStateUpdateOutput::ExecuteRaw(output) => Ok(output),
                EvmStateUpdateOutput::Execute(_, _) => Err(AgentChannelError::WrongEvmTxOutputType),
            },
            Err(_) => Err(AgentChannelError::MissingEvmTxOutput),
        }
    }

    pub fn receive_world_update_output(&self) -> AgentChannelResult<U> {
        match self.world_result_receiver.try_recv() {
            Ok(output) => Ok(output),
            Err(_) => Err(AgentChannelError::MissingWorldUpdateOutput),
        }
    }
}
