use crate::payload::EpigeneticPayload;
use crate::reason::TerminationReason;

pub trait Apoptosis {
    fn trigger_apoptosis(
        self,
        reason: TerminationReason,
        fault_signature: String,
    ) -> EpigeneticPayload;
}
