use std::collections::HashMap;

use crate::EvmAddress;

pub struct SimEnvData {
    pub address_registry: HashMap<String, EvmAddress>,
}

impl SimEnvData {
    pub fn new() -> Self {
        SimEnvData {
            address_registry: HashMap::new(),
        }
    }
}
