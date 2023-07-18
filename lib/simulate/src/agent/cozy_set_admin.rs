#![warn(missing_docs)]
#![warn(unsafe_code)]

use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crossbeam_channel::Receiver;
use revm::primitives::{Address, Log, U256};

use super::{AgentStatus, Identifiable, IsActive, NotActive};
use crate::agent::{filter_events, Agent, SimulationEventFilter, TransactSettings};

pub struct CozySetAdmin<AgentState: AgentStatus> {
    /// Name of the agent.
    pub name: String,
    /// Public address of the simulation manager.
    pub address: AgentState::Address,
    /// [`revm::primitives`] account of the simulation manager.
    pub account_info: AgentState::AccountInfo,
    /// Contains the default transaction options for revm such as gas limit and gas price.
    pub transact_settings: AgentState::TransactSettings,
    /// The [`crossbeam_channel::Receiver`] for the events are sent down from [`SimulationEnvironment`]'s dispatch.
    pub event_receiver: AgentState::EventReceiver,
    /// The filter for the events that the agent is interested in.
    pub event_filters: Vec<SimulationEventFilter>,
}

impl<AgentState: AgentStatus> Identifiable for CozySetAdmin<AgentState> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Agent for CozySetAdmin<IsActive> {
    fn address(&self) -> Address {
        self.address
    }
    fn transact_settings(&self) -> &TransactSettings {
        &self.transact_settings
    }
    fn receiver(&self) -> Receiver<Vec<Log>> {
        self.event_receiver.clone()
    }
    fn event_filters(&self) -> Vec<SimulationEventFilter> {
        self.event_filters.clone()
    }
}

impl CozySetAdmin<NotActive> {
    /// Creates a new [`PassiveCozySupplier`] which requires a vector of [`SimulationEventFilter`] and automatically sets default initial stored prices.
    pub fn new<S: Into<String>>(
        name: S,
        event_filters: Vec<SimulationEventFilter>,
    ) -> CozySetAdmin<NotActive> {
        CozySetAdmin::<NotActive> {
            name: name.into(),
            address: (),
            account_info: (),
            transact_settings: (),
            event_receiver: (),
            event_filters,
        }
    }
}

impl CozySetAdmin<IsActive> {}
