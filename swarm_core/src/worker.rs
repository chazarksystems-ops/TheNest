use crate::apoptosis::Apoptosis;
use crate::config::NociceptorConfig;
use crate::metrics::WorkerHealthMetrics;
use crate::nociceptor::Nociceptor;
use crate::payload::EpigeneticPayload;
use crate::reason::TerminationReason;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkerOutcome {
    Survived(CattleWorker),
    Terminated(EpigeneticPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CattleWorker {
    pub id: Uuid,
    pub nociceptor: Nociceptor,
}

impl CattleWorker {
    pub fn new(id: Uuid, config: NociceptorConfig, metrics: WorkerHealthMetrics) -> Self {
        Self {
            id,
            nociceptor: Nociceptor::new(config, metrics),
        }
    }

    pub fn tick(self) -> WorkerOutcome {
        if self.nociceptor.is_terminal() {
            let score = self.nociceptor.calculate_suffering();
            let threshold = self.nociceptor.config.threshold;
            WorkerOutcome::Terminated(self.trigger_apoptosis(
                TerminationReason::ThresholdBreach,
                format!(
                    "Terminal suffering breach: score {} >= threshold {}",
                    score, threshold
                ),
            ))
        } else {
            WorkerOutcome::Survived(self)
        }
    }
}

impl Apoptosis for CattleWorker {
    fn trigger_apoptosis(
        self,
        reason: TerminationReason,
        fault_signature: String,
    ) -> EpigeneticPayload {
        EpigeneticPayload {
            worker_id: self.id,
            final_suffering_score: self.nociceptor.calculate_suffering(),
            context_bloat: self.nociceptor.metrics.context_bloat,
            error_rate: self.nociceptor.metrics.error_rate,
            coordination_debt: self.nociceptor.metrics.coordination_debt,
            threshold: self.nociceptor.config.threshold,
            bloat_weight: self.nociceptor.config.alpha,
            error_weight: self.nociceptor.config.beta,
            coordination_debt_weight: self.nociceptor.config.gamma,
            termination_reason: reason,
            fault_signature,
        }
    }
}
