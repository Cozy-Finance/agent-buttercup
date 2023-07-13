use std::{collections::HashMap, thread};

use crossbeam_channel::unbounded;

use crate::{
    agent::{
        agent_channel::{AgentChannel, AgentSimUpdate},
        types::AgentId,
        Agent,
    },
    state::{errors::SimStateError, update::UpdateData, world::World, SimState},
    summarizer::{Summarizer, SummaryGenerator},
    time_policy::TimePolicy,
};

type SimManagerResult<T> = Result<T, SimManagerError>;

#[derive(thiserror::Error, Debug)]
pub enum SimManagerError {
    #[error("Address collision for agent id: {0:?}.")]
    AddressCollision(AgentId),
    #[error("Error interacting with state.")]
    SimStateError(#[from] SimStateError),
}

pub struct SimManager<U: UpdateData, W: World<WorldUpdateData = U>> {
    pub time_policy: Box<dyn TimePolicy>,
    pub agents: HashMap<AgentId, Box<dyn Agent<U, W>>>,
    pub state: SimState<U, W>,
    pub summarizer: Summarizer<U, W>,
}

impl<U: UpdateData, W: World<WorldUpdateData = U>> SimManager<U, W> {
    pub fn new(
        state: SimState<U, W>,
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

    /// Run the time policy and agents to update the simulation environment.
    pub fn run_sim(&mut self) -> SimManagerResult<()> {
        // Initiate block time policy.
        self.state
            .update_time_env(self.time_policy.current_time_env());

        while self.time_policy.is_active() {
            let (sender, receiver) = unbounded::<AgentSimUpdate<U>>();

            // Concurrently:
            //      1) Spawn agents to access immutable state via a read handle and queue updates.
            //      2) Append queued updates via the write handle.
            rayon::scope(|s| {
                for (agent_id, agent) in &mut self.agents {
                    let channel: AgentChannel<U> = AgentChannel::new(&sender, agent_id);
                    s.spawn(|_| agent.step(&self.state, channel));
                }
                drop(sender);
            });

            for update in receiver.iter() {
                self.state
                    .execute(update)
                    .map_err(SimManagerError::SimStateError)?;
            }

            // Let agents resolve the step.
            thread::scope(|t| {
                for agent in self.agents.values_mut() {
                    t.spawn(|| agent.resolve_step(&self.state));
                }
            });

            // Run summarizers.
            self.summarizer.output_summaries(&self.state);

            // Clear all results.
            self.state.clear_all_results();

            // Update time policy.
            self.state.update_time_env(self.time_policy.step());
        }

        Ok(())
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    pub fn activate_agent(&mut self, mut new_agent: Box<dyn Agent<U, W>>) -> SimManagerResult<()> {
        // Register agent account info.
        let id = new_agent.id();
        if self.agents.contains_key(&id) {
            return Err(SimManagerError::AddressCollision(id));
        }

        self.state
            .insert_account_info(id.address, new_agent.account_info())
            .map_err(SimManagerError::SimStateError)?;

        // Runs the agent's activation step and queue updates.
        let (sender, receiver) = unbounded::<AgentSimUpdate<U>>();
        let channel = AgentChannel::new(&sender, &id);
        new_agent.activation_step(&self.state, channel);

        // Execute queued updates.
        drop(sender);
        for update in receiver.iter() {
            self.state
                .execute(update)
                .map_err(SimManagerError::SimStateError)?;
        }

        // Resolve activation step.
        new_agent.resolve_activation_step(&self.state);

        // Clear all results.
        self.state.clear_all_results();

        // Adds agent to local data.
        self.agents.insert(id, new_agent);

        Ok(())
    }

    pub fn get_state(&self) -> &SimState<U, W> {
        &self.state
    }

    pub fn register_summary_generator<T: serde::Serialize>(
        &mut self,
        generator: Box<dyn SummaryGenerator<U, W>>,
    ) {
        self.summarizer.register_summary_generator(generator)
    }
}
