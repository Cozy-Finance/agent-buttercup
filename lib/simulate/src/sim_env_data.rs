use std::collections::HashMap;

use crate::contract::sim_contract::{IsDeployed, SimContract};

pub struct SimEnvData {
    pub contract_registry: HashMap<String, SimContract<IsDeployed>>,
}

impl SimEnvData {
    pub fn new() -> Self {
        SimEnvData {
            contract_registry: HashMap::new(),
        }
    }
}
