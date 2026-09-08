use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetricFrame {
    pub timestamp_ms: u64,
    pub allocated_bytes: usize,
    pub retained_bytes: usize,
}

pub fn allocated_bytes() -> usize {
    0
}

pub fn retained_bytes() -> usize {
    0
}

pub fn available_memory_bytes() -> Option<u64> {
    None
}
