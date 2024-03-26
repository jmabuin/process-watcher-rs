pub struct MemoryMeasure {
    pub time_seconds: u64,
    pub quantity: u64
}

impl MemoryMeasure {
    pub fn new(s: u64, q: u64) -> Self {
        MemoryMeasure {time_seconds: s, quantity: q}
    }
}