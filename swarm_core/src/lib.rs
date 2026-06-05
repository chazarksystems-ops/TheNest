pub mod apoptosis;
pub mod config;
pub mod metrics;
pub mod nociceptor;
pub mod payload;
pub mod reason;
pub mod receipt_sink;
pub mod worker;

pub use apoptosis::Apoptosis;
pub use config::NociceptorConfig;
pub use metrics::WorkerHealthMetrics;
pub use nociceptor::Nociceptor;
pub use payload::EpigeneticPayload;
pub use reason::TerminationReason;
pub use receipt_sink::{write_payload_json, write_payload_json_pretty};
pub use worker::{CattleWorker, WorkerOutcome};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Scenario {
    pub scenario_name: String,
    pub config: NociceptorConfig,
    pub metrics: WorkerHealthMetrics,
    // Optional metadata fields (P4) — use serde(default) so old files still load
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub expected_outcome: Option<String>, // "survived", "terminated", "invalid"
    #[serde(default)]
    pub expected_score: Option<f32>,
}

impl Scenario {
    pub fn validate(&self) -> Result<(), String> {
        NociceptorConfig::new(
            self.config.alpha,
            self.config.beta,
            self.config.gamma,
            self.config.threshold,
        )?;
        WorkerHealthMetrics::new(
            self.metrics.context_bloat,
            self.metrics.error_rate,
            self.metrics.coordination_debt,
        )?;
        Ok(())
    }

    pub fn from_json(json_str: &str) -> Result<Self, String> {
        let scenario: Self = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
        scenario.validate()?;
        Ok(scenario)
    }

    pub fn run(self, id: uuid::Uuid) -> WorkerOutcome {
        let worker = CattleWorker::new(id, self.config, self.metrics);
        worker.tick()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_health_scoring_non_terminal() {
        let config = NociceptorConfig::new(1.0, 2.0, 3.0, 10.0).unwrap();
        let metrics = WorkerHealthMetrics::new(2.0, 1.5, 1.0).unwrap();
        let nociceptor = Nociceptor::new(config, metrics);

        let score = nociceptor.calculate_suffering();
        assert_eq!(score, 8.0);
        assert!(!nociceptor.is_terminal());
    }

    #[test]
    fn test_health_scoring_terminal() {
        let config = NociceptorConfig::new(1.0, 2.0, 3.0, 10.0).unwrap();
        let metrics = WorkerHealthMetrics::new(4.0, 2.0, 1.0).unwrap();
        let nociceptor = Nociceptor::new(config, metrics);

        let score = nociceptor.calculate_suffering();
        assert_eq!(score, 11.0);
        assert!(nociceptor.is_terminal());
    }

    #[test]
    fn test_worker_survives_tick() {
        let config = NociceptorConfig::new(1.0, 1.0, 1.0, 10.0).unwrap();
        let metrics = WorkerHealthMetrics::new(1.0, 1.0, 1.0).unwrap();
        let worker = CattleWorker::new(Uuid::new_v4(), config, metrics);

        match worker.tick() {
            WorkerOutcome::Survived(w) => {
                assert_eq!(w.nociceptor.calculate_suffering(), 3.0);
            }
            WorkerOutcome::Terminated(_) => panic!("Worker should have survived"),
        }
    }

    #[test]
    fn test_worker_terminates_tick() {
        let config = NociceptorConfig::new(1.0, 1.0, 1.0, 10.0).unwrap();
        let metrics = WorkerHealthMetrics::new(5.0, 5.0, 5.0).unwrap();
        let worker = CattleWorker::new(Uuid::new_v4(), config, metrics);

        match worker.tick() {
            WorkerOutcome::Survived(_) => panic!("Worker should have terminated"),
            WorkerOutcome::Terminated(payload) => {
                assert_eq!(payload.final_suffering_score, 15.0);
                assert_eq!(
                    payload.termination_reason,
                    TerminationReason::ThresholdBreach
                );
            }
        }
    }

    #[test]
    fn test_json_serialization() {
        let config = NociceptorConfig::new(1.2, 2.3, 3.4, 10.0).unwrap();
        let metrics = WorkerHealthMetrics::new(1.0, 2.0, 3.0).unwrap();
        let worker = CattleWorker::new(Uuid::new_v4(), config, metrics);

        match worker.tick() {
            WorkerOutcome::Survived(_) => panic!("Worker should have terminated"),
            WorkerOutcome::Terminated(payload) => {
                let json_str = serde_json::to_string(&payload).unwrap();
                println!("Serialized JSON: {}", json_str);

                let deserialized: EpigeneticPayload = serde_json::from_str(&json_str).unwrap();
                assert_eq!(payload, deserialized);
            }
        }
    }

    #[test]
    fn test_nociceptor_presets() {
        let default_cfg = NociceptorConfig::default_p0();
        assert_eq!(default_cfg.threshold, 10.0);

        let strict_cfg = NociceptorConfig::strict();
        assert_eq!(strict_cfg.threshold, 5.0);

        let lenient_cfg = NociceptorConfig::lenient();
        assert_eq!(lenient_cfg.threshold, 20.0);
    }

    #[test]
    fn test_config_metrics_validation() {
        assert!(NociceptorConfig::new(1.0, 1.0, 1.0, -1.0).is_err());
        assert!(NociceptorConfig::new(f32::NAN, 1.0, 1.0, 10.0).is_err());
        assert!(NociceptorConfig::new(f32::INFINITY, 1.0, 1.0, 10.0).is_err());

        assert!(WorkerHealthMetrics::new(-1.0, 1.0, 1.0).is_err());
        assert!(WorkerHealthMetrics::new(f32::NAN, 1.0, 1.0).is_err());
        assert!(WorkerHealthMetrics::new(f32::INFINITY, 1.0, 1.0).is_err());
    }

    #[test]
    fn test_scenario_loading() {
        let survives_json = std::fs::read_to_string("../scenarios/worker_survives.json").unwrap();
        let scenario = Scenario::from_json(&survives_json).unwrap();
        assert_eq!(scenario.scenario_name, "worker_survives");

        match scenario.run(Uuid::new_v4()) {
            WorkerOutcome::Survived(w) => {
                assert_eq!(w.nociceptor.calculate_suffering(), 4.0);
            }
            WorkerOutcome::Terminated(_) => panic!("Should have survived"),
        }

        let breach_json =
            std::fs::read_to_string("../scenarios/worker_threshold_breach.json").unwrap();
        let scenario_breach = Scenario::from_json(&breach_json).unwrap();
        assert_eq!(scenario_breach.scenario_name, "worker_threshold_breach");

        match scenario_breach.run(Uuid::new_v4()) {
            WorkerOutcome::Survived(_) => panic!("Should have terminated"),
            WorkerOutcome::Terminated(payload) => {
                assert_eq!(payload.final_suffering_score, 12.0);
            }
        }

        let exact_json =
            std::fs::read_to_string("../scenarios/worker_exact_threshold.json").unwrap();
        let scenario_exact = Scenario::from_json(&exact_json).unwrap();
        assert_eq!(scenario_exact.scenario_name, "worker_exact_threshold");

        match scenario_exact.run(Uuid::new_v4()) {
            WorkerOutcome::Survived(_) => panic!("Should have terminated at exact threshold"),
            WorkerOutcome::Terminated(payload) => {
                assert_eq!(payload.final_suffering_score, 10.0);
            }
        }

        let below_json =
            std::fs::read_to_string("../scenarios/worker_just_below_threshold.json").unwrap();
        let scenario_below = Scenario::from_json(&below_json).unwrap();
        assert_eq!(scenario_below.scenario_name, "worker_just_below_threshold");

        match scenario_below.run(Uuid::new_v4()) {
            WorkerOutcome::Survived(w) => {
                assert_eq!(w.nociceptor.calculate_suffering(), 9.0);
            }
            WorkerOutcome::Terminated(_) => panic!("Should have survived just below threshold"),
        }

        let negative_json =
            std::fs::read_to_string("../scenarios/worker_invalid_negative_metric.json").unwrap();
        assert!(Scenario::from_json(&negative_json).is_err());

        let nan_json =
            std::fs::read_to_string("../scenarios/worker_invalid_nan_metric.json").unwrap();
        assert!(Scenario::from_json(&nan_json).is_err());
    }

    #[test]
    fn test_receipt_sink_writing() {
        let payload = EpigeneticPayload {
            worker_id: Uuid::new_v4(),
            final_suffering_score: 12.0,
            context_bloat: 6.0,
            error_rate: 4.0,
            coordination_debt: 2.0,
            threshold: 10.0,
            bloat_weight: 1.0,
            error_weight: 1.0,
            coordination_debt_weight: 1.0,
            termination_reason: TerminationReason::ThresholdBreach,
            fault_signature: "test".to_string(),
        };

        let temp_dir = std::env::temp_dir();
        let test_path = temp_dir.join("test_receipt.json");
        let test_path_pretty = temp_dir.join("test_receipt_pretty.json");

        assert!(write_payload_json(&test_path, &payload).is_ok());
        assert!(write_payload_json_pretty(&test_path_pretty, &payload).is_ok());

        assert!(test_path.exists());
        assert!(test_path_pretty.exists());

        let _ = std::fs::remove_file(test_path);
        let _ = std::fs::remove_file(test_path_pretty);
    }
}
