use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigCpu {
    pub measure_cpu: bool,
}

impl ConfigCpu {
    pub fn print(&self) {
        println!("Measure CPU: {}", self.measure_cpu)
    }
}