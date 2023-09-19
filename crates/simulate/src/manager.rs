use std::collections::HashMap;

use crate::{
    address::Address,
    agent::{
        agent_channel::{AgentChannelReceiver, AgentChannelSender},
        Agent,
    },
    state::{
        update::{EvmStateUpdate, EvmStateUpdateOutput, UpdateData, WorldStateUpdate},
        world::World,
        State, StateError,
    },
    summarizer::{Summarizer, SummaryGenerator},
    time_policy::TimePolicy,
};

type SimManagerResult<T> = Result<T, SimManagerError>;

#[derive(thiserror::Error, Debug)]
pub enum SimManagerError {
    #[error("Address collision inserting new agent: {0:?}.")]
    AddressCollision(Address),
    #[error("Error interacting with state.")]
    StateError(#[from] StateError),
}

pub struct SimManager<U, W>
where
    U: UpdateData,
    W: World<WorldUpdateData = U>,
{
    pub time_policy: Box<dyn TimePolicy>,
    pub agents: HashMap<Address, Box<dyn Agent<U, W>>>,
    pub state: State<U, W>,
    pub summarizer: Summarizer<U, W>,
}

impl<U, W> SimManager<U, W>
where
    U: UpdateData,
    W: World<WorldUpdateData = U>,
{
    pub fn new(
        state: State<U, W>,
        time_policy: Box<dyn TimePolicy>,
        summarizer: Summarizer<U, W>,
    ) -> Self {
        Self {
            time_policy,
            agents: HashMap::new(),
            state,
            summarizer,
        }
    }

    pub fn get_state_mut(&mut self) -> &mut State<U, W> {
        &mut self.state
    }

    pub fn get_state(&self) -> &State<U, W> {
        &self.state
    }

    /// Run the time policy and agents to update the simulation environment.
    pub fn run_sim(&mut self) -> SimManagerResult<()> {
        // Initiate block time policy.
        self.state.update_time(self.time_policy.current_time_env());

        while self.time_policy.is_active() {
            let (evm_update_sender, evm_update_receiver) =
                crossbeam_channel::unbounded::<EvmStateUpdate>();
            let (world_update_sender, world_update_receiver) =
                crossbeam_channel::unbounded::<WorldStateUpdate<U>>();

            let mut agent_channel_receivers = HashMap::new();
            let mut agent_channel_senders = HashMap::new();
            for (addr, _) in self.agents.iter_mut() {
                let (evm_result_sender, evm_result_receiver) =
                    crossbeam_channel::unbounded::<EvmStateUpdateOutput>();
                let (world_result_sender, world_result_receiver) =
                    crossbeam_channel::unbounded::<U>();

                let channel_sender = AgentChannelSender::new(
                    addr.clone(),
                    evm_update_sender.clone(),
                    evm_result_sender,
                    world_update_sender.clone(),
                    world_result_sender,
                );
                let channel_receiver =
                    AgentChannelReceiver::new(evm_result_receiver, world_result_receiver);
                agent_channel_senders.insert(addr.clone(), channel_sender);
                agent_channel_receivers.insert(addr.clone(), channel_receiver);
            }

            rayon::scope(|s| {
                for (agent_id, agent) in self.agents.iter_mut() {
                    s.spawn(|_| {
                        agent.step(
                            &self.state,
                            agent_channel_senders
                                .get(agent_id)
                                .expect("Agent id must be in map."),
                        );
                    });
                }
            });

            while let Ok(update) = evm_update_receiver.try_recv() {
                self.state
                    .execute_evm_tx_state_update(update)
                    .map_err(SimManagerError::StateError)?;
            }

            while let Ok(update) = world_update_receiver.try_recv() {
                self.state
                    .execute_world_state_update(update)
                    .map_err(SimManagerError::StateError)?;
            }

            rayon::scope(|s| {
                for (addr, agent) in self.agents.iter_mut() {
                    s.spawn(|_| {
                        agent.resolve_step(
                            &self.state,
                            agent_channel_receivers
                                .get(addr)
                                .expect("Agent id must be in map."),
                        );
                    })
                }
            });

            // Run summarizers.
            self.summarizer.output_summaries(&self.state);

            // Update time policy.
            self.state.update_time(self.time_policy.step());
        }

        Ok(())
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    pub fn activate_agent(&mut self, mut new_agent: Box<dyn Agent<U, W>>) -> SimManagerResult<()> {
        // Register agent account info.
        let addr = new_agent.address();
        if self.agents.contains_key(&addr) {
            return Err(SimManagerError::AddressCollision(addr));
        }
        self.state
            .add_account(addr, new_agent.account_info())
            .map_err(SimManagerError::StateError)?;

        // Run agent's activation step.
        new_agent.activation_step(&mut self.state);

        // Adds agent to local map.
        self.agents.insert(addr, new_agent);

        Ok(())
    }

    pub fn register_summary_generator<T: serde::Serialize>(
        &mut self,
        generator: Box<dyn SummaryGenerator<U, W>>,
    ) {
        self.summarizer.register_summary_generator(generator)
    }
}
