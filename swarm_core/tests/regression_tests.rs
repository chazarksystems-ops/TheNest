use std::fs;
use std::process::Command;
use swarm_core::{Nociceptor, NociceptorConfig, Scenario, WorkerHealthMetrics, WorkerOutcome};
use uuid::Uuid;

#[test]
fn test_scenario_regression_outcomes() {
    // 1. worker_survives
    let survives_content = fs::read_to_string("../scenarios/worker_survives.json")
        .expect("Failed to read scenarios/worker_survives.json");
    let scenario = Scenario::from_json(&survives_content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_survives");
    match scenario.run(Uuid::new_v4()) {
        WorkerOutcome::Survived(w) => {
            assert!(w.nociceptor.calculate_suffering() < w.nociceptor.config.threshold);
        }
        WorkerOutcome::Terminated(_) => panic!("worker_survives should have survived"),
    }

    // 2. worker_just_below_threshold
    let below_content = fs::read_to_string("../scenarios/worker_just_below_threshold.json")
        .expect("Failed to read scenarios/worker_just_below_threshold.json");
    let scenario = Scenario::from_json(&below_content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_just_below_threshold");
    match scenario.run(Uuid::new_v4()) {
        WorkerOutcome::Survived(w) => {
            assert_eq!(w.nociceptor.calculate_suffering(), 9.0);
        }
        WorkerOutcome::Terminated(_) => panic!("worker_just_below_threshold should have survived"),
    }

    // 3. worker_exact_threshold
    let exact_content = fs::read_to_string("../scenarios/worker_exact_threshold.json")
        .expect("Failed to read scenarios/worker_exact_threshold.json");
    let scenario = Scenario::from_json(&exact_content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_exact_threshold");
    match scenario.run(Uuid::new_v4()) {
        WorkerOutcome::Survived(_) => panic!("worker_exact_threshold should have terminated"),
        WorkerOutcome::Terminated(payload) => {
            assert_eq!(payload.final_suffering_score, 10.0);
        }
    }

    // 4. worker_threshold_breach
    let breach_content = fs::read_to_string("../scenarios/worker_threshold_breach.json")
        .expect("Failed to read scenarios/worker_threshold_breach.json");
    let scenario = Scenario::from_json(&breach_content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_threshold_breach");
    match scenario.run(Uuid::new_v4()) {
        WorkerOutcome::Survived(_) => panic!("worker_threshold_breach should have terminated"),
        WorkerOutcome::Terminated(payload) => {
            assert_eq!(payload.final_suffering_score, 12.0);
        }
    }

    // 5. worker_invalid_negative_metric
    let negative_content = fs::read_to_string("../scenarios/worker_invalid_negative_metric.json")
        .expect("Failed to read scenarios/worker_invalid_negative_metric.json");
    assert!(Scenario::from_json(&negative_content).is_err());

    // 6. worker_invalid_nan_metric
    let nan_content = fs::read_to_string("../scenarios/worker_invalid_nan_metric.json")
        .expect("Failed to read scenarios/worker_invalid_nan_metric.json");
    assert!(Scenario::from_json(&nan_content).is_err());
}

#[test]
fn test_golden_receipt_exact_threshold() {
    let exact_content = fs::read_to_string("../scenarios/worker_exact_threshold.json").unwrap();
    let scenario = Scenario::from_json(&exact_content).unwrap();
    let fixed_id = Uuid::nil(); // "00000000-0000-0000-0000-000000000000"

    match scenario.run(fixed_id) {
        WorkerOutcome::Survived(_) => panic!("Expected exact threshold scenario to terminate"),
        WorkerOutcome::Terminated(payload) => {
            let serialized = serde_json::to_string_pretty(&payload).unwrap();
            let golden_content =
                fs::read_to_string("tests/golden/golden_receipt_exact_threshold.json")
                    .expect("Failed to read golden receipt file");

            let val_serialized: serde_json::Value = serde_json::from_str(&serialized).unwrap();
            let val_golden: serde_json::Value = serde_json::from_str(&golden_content).unwrap();

            assert_eq!(
                val_serialized, val_golden,
                "Serialized receipt does not match golden receipt!"
            );
        }
    }
}

fn run_demo_binary(args: &[&str]) -> (String, std::process::ExitStatus) {
    let output = Command::new("cargo")
        .current_dir("..")
        .arg("run")
        .arg("--bin")
        .arg("demo")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute cargo run --bin demo");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    (format!("{}\n{}", stdout, stderr), output.status)
}

#[test]
fn test_cli_output_stability() {
    // Test demo running below scenario
    let (out_below, status_below) = run_demo_binary(&["below"]);
    assert!(
        status_below.success(),
        "CLI failed for 'below'. Output: {}",
        out_below
    );
    assert!(out_below.contains("Scenario Name: worker_just_below_threshold"));
    assert!(out_below.contains("Outcome: SURVIVED"));
    assert!(out_below.contains("Suffering score: 9"));

    // Test demo running exact scenario
    let (out_exact, status_exact) = run_demo_binary(&["exact"]);
    assert!(
        status_exact.success(),
        "CLI failed for 'exact'. Output: {}",
        out_exact
    );
    assert!(out_exact.contains("Scenario Name: worker_exact_threshold"));
    assert!(out_exact.contains("Outcome: TERMINATED (Apoptosis triggered)"));
    assert!(out_exact.contains("Suffering score: 10"));
    assert!(out_exact.contains("Receipt JSON:"));

    // Extract JSON payload from stdout and parse it
    let marker = "Receipt JSON:\n";
    if let Some(pos) = out_exact.find(marker) {
        let json_start = &out_exact[pos + marker.len()..];
        if let Some(end_pos) = json_start.find('}') {
            let json_str = &json_start[..=end_pos];
            let parsed: serde_json::Value = serde_json::from_str(json_str)
                .expect("Failed to parse JSON receipt from CLI output");
            assert_eq!(parsed["final_suffering_score"], 10.0);
            assert_eq!(parsed["termination_reason"], "ThresholdBreach");
        }
    }
}

#[test]
fn test_property_scoring_monotonicity_and_bounds() {
    let config = NociceptorConfig::new(1.0, 1.0, 1.0, 10.0).unwrap();
    let base_metrics = WorkerHealthMetrics::new(1.0, 1.0, 1.0).unwrap();
    let base_nociceptor = Nociceptor::new(config.clone(), base_metrics);
    let base_score = base_nociceptor.calculate_suffering();

    // 1. Monotonicity checks
    for i in 0..50 {
        let val = i as f32 * 0.1;

        let m1 = WorkerHealthMetrics::new(1.0 + val, 1.0, 1.0).unwrap();
        let n1 = Nociceptor::new(config.clone(), m1);
        assert!(n1.calculate_suffering() >= base_score);

        let m2 = WorkerHealthMetrics::new(1.0, 1.0 + val, 1.0).unwrap();
        let n2 = Nociceptor::new(config.clone(), m2);
        assert!(n2.calculate_suffering() >= base_score);

        let m3 = WorkerHealthMetrics::new(1.0, 1.0, 1.0 + val).unwrap();
        let n3 = Nociceptor::new(config.clone(), m3);
        assert!(n3.calculate_suffering() >= base_score);
    }

    // 2. Threshold boundary checks
    for i in 0..1000 {
        let val = i as f32 * 0.02; // 0.0 to 20.0
        let bloat = val / 3.0;
        let error = val / 3.0;
        let debt = val / 3.0;

        if let Ok(metrics) = WorkerHealthMetrics::new(bloat, error, debt) {
            let nociceptor = Nociceptor::new(config.clone(), metrics);
            let score = nociceptor.calculate_suffering();
            let is_terminal = nociceptor.is_terminal();

            if score >= 10.0 {
                assert!(
                    is_terminal,
                    "Expected terminal for score: {} >= 10.0",
                    score
                );
            } else {
                assert!(
                    !is_terminal,
                    "Expected non-terminal for score: {} < 10.0",
                    score
                );
            }
        }
    }

    // 3. Validation rejection boundary checks
    assert!(NociceptorConfig::new(1.0, 1.0, 1.0, -1.0).is_err()); // negative threshold is rejected
    assert!(NociceptorConfig::new(1.0, f32::NAN, 1.0, 10.0).is_err());
    assert!(NociceptorConfig::new(1.0, 1.0, f32::INFINITY, 10.0).is_err());

    assert!(WorkerHealthMetrics::new(-0.01, 1.0, 1.0).is_err());
    assert!(WorkerHealthMetrics::new(1.0, f32::NAN, 1.0).is_err());
    assert!(WorkerHealthMetrics::new(1.0, 1.0, f32::INFINITY).is_err());

    // 4. Panic-free check for valid ranges (no panics across 1000 generated points)
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                let bloat = i as f32 * 1.5;
                let error = j as f32 * 2.3;
                let debt = k as f32 * 0.7;
                let metrics = WorkerHealthMetrics::new(bloat, error, debt).unwrap();
                let nociceptor = Nociceptor::new(config.clone(), metrics);
                let _score = nociceptor.calculate_suffering();
                let _is_term = nociceptor.is_terminal();
            }
        }
    }
}
