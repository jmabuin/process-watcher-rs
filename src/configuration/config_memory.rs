use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigMemory {
    pub measure_memory: bool,
    pub memory_units: String
}

impl ConfigMemory {
    pub fn print(&self) {
        println!("Measure memory: {}", self.measure_memory);
        println!("Memory units: {}", self.memory_units);
    }
}