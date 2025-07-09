use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigMemory {
    pub measure_memory: bool,
    pub memory_units: String
}
