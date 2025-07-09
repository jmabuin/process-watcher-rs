use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigCpu {
    pub measure_cpu: bool,
}
