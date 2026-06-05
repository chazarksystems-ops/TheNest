use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkerHealthMetrics {
    pub context_bloat: f32,
    pub error_rate: f32,
    pub coordination_debt: f32,
}

impl WorkerHealthMetrics {
    pub fn new(
        context_bloat: f32,
        error_rate: f32,
        coordination_debt: f32,
    ) -> Result<Self, String> {
        if context_bloat.is_nan() || error_rate.is_nan() || coordination_debt.is_nan() {
            return Err("Metrics cannot be NaN".to_string());
        }
        if context_bloat.is_infinite()
            || error_rate.is_infinite()
            || coordination_debt.is_infinite()
        {
            return Err("Metrics cannot be infinite".to_string());
        }
        if context_bloat < 0.0 || error_rate < 0.0 || coordination_debt < 0.0 {
            return Err("Metrics cannot be negative".to_string());
        }
        Ok(Self {
            context_bloat,
            error_rate,
            coordination_debt,
        })
    }
}
