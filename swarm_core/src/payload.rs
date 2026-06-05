use crate::reason::TerminationReason;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EpigeneticPayload {
    pub worker_id: Uuid,
    pub final_suffering_score: f32,
    pub context_bloat: f32,
    pub error_rate: f32,
    pub coordination_debt: f32,
    pub threshold: f32,
    pub bloat_weight: f32,
    pub error_weight: f32,
    pub coordination_debt_weight: f32,
    pub termination_reason: TerminationReason,
    pub fault_signature: String,
}
