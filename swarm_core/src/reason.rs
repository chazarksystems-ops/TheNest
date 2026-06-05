use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TerminationReason {
    ThresholdBreach,
    ManualTrigger,
    Timeout,
}
