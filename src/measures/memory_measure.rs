/// This struct represents each one of the measures for the Memory
pub struct MemoryMeasure {
    pub time_seconds: f64,
    pub quantity: u64
}

impl MemoryMeasure {
    pub fn new(s: f64, q: u64) -> Self {
        MemoryMeasure {time_seconds: s, quantity: q}
    }
}