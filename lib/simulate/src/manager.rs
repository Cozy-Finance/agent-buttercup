#![warn(missing_docs)]
#![warn(unsafe_code)]
//! Simulation managers are used to manage the environments for a simulation.
//! Managers are responsible for adding agents, running agents, deploying contracts, calling contracts, and reading logs.

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

use bindings::arbiter_math;
use bytes::Bytes;
use crossbeam_channel::unbounded;
use revm::primitives::{AccountInfo, Address, Log, B160, U256};

use crate::{
    agent::{
        chronos::Chronos, cozy_set_admin::CozySetAdmin, simple_arbitrageur::SimpleArbitrageur,
        user::User, AgentType, IsActive, NotActive, TransactSettings,
    },
    environment::{
        contract::{IsDeployed, SimulationContract},
        sim_environment::SimulationEnvironment,
    },
};

#[derive(Debug)]
/// Error type for the simulation manager.
/// # Fields
/// * `message` - Error message.
/// * `output` - Byte output of the error.
pub struct ManagerError {
    /// Error message.
    pub message: String,
    /// Byte output of the error.
    pub output: Option<Bytes>,
}

impl Error for ManagerError {}

impl Display for ManagerError {
    /// Display the error message.
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message)
    }
}

/// Manages simulations.
/// # Fields
/// * `environment` - The simulation environment that the manager controls.
/// * `agents` - The agents that are currently running in the simulation environment.
pub struct SimulationManager {
    /// [`SimulationEnvironment`] that the simulation manager controls.
    pub environment: SimulationEnvironment,
    /// The agents that are currently running in the [`SimulationEnvironment`].
    pub agents: HashMap<String, AgentType<IsActive>>,
    /// The collection of different [`SimulationContract`] that are currently deployed in the [`SimulationEnvironment`].
    pub autodeployed_contracts: HashMap<String, SimulationContract<IsDeployed>>,
}

impl Default for SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationManager {
    /// Constructor function to instantiate a manager that has a default admin user and a simulation environment.
    /// The admin will always be given the 0x0...1 address.
    pub fn new() -> Self {
        let mut simulation_manager = Self {
            environment: SimulationEnvironment::new(),
            agents: HashMap::new(),
            autodeployed_contracts: HashMap::new(),
        };
        let admin = AgentType::User(User::new("admin", None));
        simulation_manager
            .activate_agent(admin, B160::from_low_u64_be(1))
            .unwrap(); // This unwrap should never fail.
        simulation_manager.auto_deploy();

        simulation_manager
    }

    /// Deploy all contracts that are needed for any simulation.
    fn auto_deploy(&mut self) {
        // Deploy `ArbiterMath`.
        let arbiter_math = SimulationContract::new(
            arbiter_math::ARBITERMATH_ABI.clone(),
            arbiter_math::ARBITERMATH_BYTECODE.clone(),
        );
        let arbiter_math =
            arbiter_math.deploy(&mut self.environment, self.agents.get("admin").unwrap(), ());
        self.autodeployed_contracts
            .insert("arbiter_math".to_string(), arbiter_math);
    }

    /// Stop the current simulation.
    pub fn shut_down(&mut self) {
        self.environment.event_senders.clear();
    }

    /// Run all agents concurrently in the current simulation environment.
    pub fn run_agents() {
        todo!()
    }

    /// Adds and activates an agent to be put in the collection of agents under the manager's control.
    /// # Arguments
    /// * `new_agent` - The agent to be added to the collection of agents.
    /// * `new_agent_address` - The address that the agent will be given.
    pub fn activate_agent(
        &mut self,
        new_agent: AgentType<NotActive>,
        new_agent_address: Address,
    ) -> Result<(), ManagerError> {
        // Check to make sure we are not creating an agent with an address or name that already exists.
        if self
            .agents
            .values()
            .any(|agent_in_db| agent_in_db.inner().address() == new_agent_address)
        {
            return Err(ManagerError {
                message: "Agent with that address already exists in the simulation environment."
                    .to_string(),
                output: None,
            });
        };
        if self
            .agents
            .keys()
            .any(|name_in_db| *name_in_db == new_agent.inner().name())
        {
            return Err(ManagerError {
                message: "Agent with that name already exists in the simulation environment."
                    .to_string(),
                output: None,
            });
        };

        // Create the agent and add it to the simulation environment so long as we don't throw an error above.
        let account_info = AccountInfo::default();
        self.environment
            .evm
            .db()
            .unwrap()
            .insert_account_info(new_agent_address, account_info.clone());
        let (event_sender, event_receiver) = unbounded::<Vec<Log>>();
        match new_agent {
            AgentType::User(user) => {
                let new_user = User::<IsActive> {
                    name: user.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    event_filters: user.event_filters,
                };
                self.agents
                    .insert(new_user.name.clone(), AgentType::User(new_user));
            }
            AgentType::SimpleArbitrageur(simple_arbitrageur) => {
                let new_simple_arbitrageur = SimpleArbitrageur::<IsActive> {
                    name: simple_arbitrageur.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    event_filters: simple_arbitrageur.event_filters,
                    prices: simple_arbitrageur.prices,
                };
                self.agents.insert(
                    new_simple_arbitrageur.name.clone(),
                    AgentType::SimpleArbitrageur(new_simple_arbitrageur),
                );
            }
            AgentType::CozySetAdmin(cozy_set_admin) => {
                let new_cozy_set_admin = CozySetAdmin::<IsActive> {
                    name: cozy_set_admin.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    event_filters: cozy_set_admin.event_filters,
                };
                self.agents.insert(
                    new_cozy_set_admin.name.clone(),
                    AgentType::CozySetAdmin(new_cozy_set_admin),
                );
            }
            AgentType::Chronos(chronos) => {
                let new_chronos = Chronos::<IsActive> {
                    name: chronos.name,
                    address: new_agent_address,
                    account_info,
                    transact_settings: TransactSettings {
                        gas_limit: u64::MAX,   // TODO: Users should have a gas limit.
                        gas_price: U256::ZERO, // TODO: Users should have an associated gas price.
                    },
                    event_receiver,
                    event_filters: chronos.event_filters,
                };
                self.agents
                    .insert(new_chronos.name.clone(), AgentType::Chronos(new_chronos));
            }
            _ => {}
        };
        self.environment.add_sender(event_sender);
        Ok(())
    }
}

#[test]
fn agent_address_collision() {
    let mut manager = SimulationManager::default();
    let alice = User::new("alice", None);
    let result = manager.activate_agent(AgentType::User(alice), B160::from_low_u64_be(1));
    assert!(result.is_err());
}
