use crate::config::NociceptorConfig;
use crate::metrics::WorkerHealthMetrics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Nociceptor {
    pub config: NociceptorConfig,
    pub metrics: WorkerHealthMetrics,
}

impl Nociceptor {
    pub fn new(config: NociceptorConfig, metrics: WorkerHealthMetrics) -> Self {
        Self { config, metrics }
    }

    pub fn calculate_suffering(&self) -> f32 {
        (self.config.alpha * self.metrics.context_bloat)
            + (self.config.beta * self.metrics.error_rate)
            + (self.config.gamma * self.metrics.coordination_debt)
    }

    pub fn is_terminal(&self) -> bool {
        self.calculate_suffering() >= self.config.threshold
    }
}
