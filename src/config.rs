use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use crate::config_cpu::ConfigCpu;
use crate::config_memory::ConfigMemory;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub measure_interval: u64,
    pub cpu: ConfigCpu,
    pub memory: ConfigMemory,
    pub command: String,
}


impl Config {
    pub fn from_json(path: String) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let c = serde_json::from_reader(reader)?;
        Ok(c)
    }

    pub fn default() -> Self {
        let c = Config {
            measure_interval: 3,
            cpu: ConfigCpu { measure_cpu: true },
            memory: ConfigMemory { measure_memory: true, memory_units: "MB".to_string() },
            command: "".to_string(),
        };

        c
    }

    pub fn print(&self) {
        println!("Measure interval: {}", self.measure_interval);
        self.cpu.print();
        self.memory.print();
        println!("Command: {}", self.command);
    }
}
