
pub struct CpuMeasure {
    pub time_seconds: u64,
    pub percentage: f64
}

impl CpuMeasure {
    pub fn new(s: u64, p: f64) -> Self {
        CpuMeasure{time_seconds: s, percentage: p}
    }
}