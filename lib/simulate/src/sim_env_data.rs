use revm::primitives::B160;
use std::collections::HashMap;

pub struct SimEnvData {
    pub address_registry: HashMap<String, B160>,
}

impl SimEnvData {
    pub fn new() -> Self {
        SimEnvData {
            address_registry: HashMap::new(),
        }
    }
}
