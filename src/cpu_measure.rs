
/// This struct represents each one of the measures for the CPU
pub struct CpuMeasure {
    pub time_seconds: f64,
    pub percentage: f64
}

impl CpuMeasure {
    pub fn new(s: f64, p: f64) -> Self {
        CpuMeasure{time_seconds: s, percentage: p}
    }
}