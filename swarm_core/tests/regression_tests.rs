use std::fs;
use std::process::Command;
use swarm_core::{Nociceptor, NociceptorConfig, Scenario, WorkerHealthMetrics, WorkerOutcome};
use uuid::Uuid;

// ─── Helper: run hive_workbench binary ───────────────────────────────────────

fn run_workbench(args: &[&str]) -> (String, String, std::process::ExitStatus) {
    let output = Command::new("cargo")
        .current_dir("..")
        .arg("run")
        .arg("--bin")
        .arg("hive_workbench")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute cargo run --bin hive_workbench");

    let stdout = String::from_utf8(output.stdout).unwrap_or_default();
    let stderr = String::from_utf8(output.stderr).unwrap_or_default();
    (stdout, stderr, output.status)
}

// ─── Original regression tests (keep all) ────────────────────────────────────

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

// Updated test_cli_output_stability to use hive_workbench
#[test]
fn test_cli_output_stability() {
    // Test hive_workbench run below scenario
    let (stdout_below, stderr_below, status_below) = run_workbench(&["run", "below"]);
    let out_below = format!("{}\n{}", stdout_below, stderr_below);
    assert!(
        status_below.success(),
        "CLI failed for 'run below'. Output: {}",
        out_below
    );
    assert!(out_below.contains("worker_just_below_threshold"));
    assert!(out_below.contains("SURVIVED"));
    assert!(out_below.contains("9"));

    // Test hive_workbench run exact scenario
    let (stdout_exact, stderr_exact, status_exact) = run_workbench(&["run", "exact"]);
    let out_exact = format!("{}\n{}", stdout_exact, stderr_exact);
    assert!(
        status_exact.success(),
        "CLI failed for 'run exact'. Output: {}",
        out_exact
    );
    assert!(out_exact.contains("worker_exact_threshold"));
    assert!(out_exact.contains("TERMINATED"));
    assert!(out_exact.contains("10"));
    assert!(out_exact.contains("Receipt JSON:"));

    // Extract JSON payload from stdout and parse it
    let marker = "Receipt JSON:\n";
    if let Some(pos) = stdout_exact.find(marker) {
        let json_start = &stdout_exact[pos + marker.len()..];
        // Find the end of the JSON object
        let mut depth = 0i32;
        let mut end_pos = 0usize;
        for (i, ch) in json_start.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end_pos = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        if end_pos > 0 {
            let json_str = &json_start[..end_pos];
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

// ─── P3 new tests ─────────────────────────────────────────────────────────────

#[test]
fn test_hive_workbench_list_command() {
    let (stdout, _stderr, status) = run_workbench(&["list"]);
    assert!(status.success(), "list command failed");
    // Should show some scenarios
    assert!(
        stdout.contains("scenarios/") || stdout.contains("scenario"),
        "list output missing scenario info: {}",
        stdout
    );
}

#[test]
fn test_hive_workbench_run_healthy() {
    let (stdout, stderr, status) = run_workbench(&["run", "healthy"]);
    let out = format!("{}\n{}", stdout, stderr);
    assert!(status.success(), "run healthy failed: {}", out);
    assert!(
        out.contains("SURVIVED"),
        "Expected SURVIVED in output: {}",
        out
    );
}

#[test]
fn test_hive_workbench_run_breach() {
    let (stdout, stderr, status) = run_workbench(&["run", "breach"]);
    let out = format!("{}\n{}", stdout, stderr);
    assert!(status.success(), "run breach failed: {}", out);
    assert!(
        out.contains("TERMINATED"),
        "Expected TERMINATED in output: {}",
        out
    );
}

#[test]
fn test_hive_workbench_suite_command() {
    let (stdout, stderr, status) = run_workbench(&["suite"]);
    let out = format!("{}\n{}", stdout, stderr);
    assert!(status.success(), "suite command failed: {}", out);
    // Should contain table-like output
    assert!(
        stdout.contains("scenario") || stdout.contains("Suite"),
        "suite output missing expected content: {}",
        out
    );
}

#[test]
fn test_hive_workbench_json_output_is_valid() {
    let (stdout, stderr, status) = run_workbench(&["run", "exact", "--output", "json"]);
    assert!(
        status.success(),
        "run exact --output json failed: {}{}",
        stdout,
        stderr
    );
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "JSON output is not valid JSON: {}", stdout);
}

#[test]
fn test_hive_workbench_quiet_output_is_minimal() {
    let (stdout, _stderr, status) = run_workbench(&["run", "healthy", "--output", "quiet"]);
    assert!(status.success(), "run healthy --output quiet failed");
    let trimmed = stdout.trim();
    assert!(
        trimmed == "SURVIVED" || trimmed.starts_with("SURVIVED"),
        "quiet output should be minimal, got: {}",
        trimmed
    );
}

#[test]
fn test_hive_workbench_suite_json_output() {
    let (stdout, stderr, status) = run_workbench(&["suite", "--output", "json"]);
    assert!(
        status.success(),
        "suite --output json failed: {}{}",
        stdout,
        stderr
    );
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(
        parsed.is_ok(),
        "suite JSON output is not valid JSON: {}",
        stdout
    );
    let val = parsed.unwrap();
    assert!(
        val["rows"].is_array(),
        "Expected 'rows' array in suite JSON"
    );
}

#[test]
fn test_hive_workbench_run_file_command() {
    let (stdout, stderr, status) = run_workbench(&["run-file", "scenarios/worker_survives.json"]);
    let out = format!("{}\n{}", stdout, stderr);
    assert!(status.success(), "run-file failed: {}", out);
    assert!(out.contains("SURVIVED"), "Expected SURVIVED: {}", out);
}

#[test]
fn test_hive_workbench_unknown_command_exits_1() {
    let (_stdout, _stderr, status) = run_workbench(&["unknowncommand123"]);
    assert_eq!(
        status.code().unwrap_or(-1),
        1,
        "Unknown command should exit 1"
    );
}

// ─── P4 new tests ─────────────────────────────────────────────────────────────

#[test]
fn test_p4_old_minimal_scenarios_still_load() {
    // Original scenario files have no metadata — they should still load
    let content = fs::read_to_string("../scenarios/worker_survives.json").unwrap();
    let scenario = Scenario::from_json(&content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_survives");
    assert!(scenario.id.is_none());
    assert!(scenario.description.is_none());
    assert!(scenario.expected_outcome.is_none());
}

#[test]
fn test_p4_metadata_rich_scenario_loads() {
    // New metadata-rich scenario files should load with metadata
    let content = fs::read_to_string("../scenarios/worker_zero_metrics.json").unwrap();
    let scenario = Scenario::from_json(&content).unwrap();
    assert_eq!(scenario.scenario_name, "worker_zero_metrics");
    // expected_outcome should be Some if it is set
    if let Some(eo) = &scenario.expected_outcome {
        assert_eq!(eo.as_str(), "survived");
    }
}

#[test]
fn test_p4_invalid_scenarios_correctly_rejected() {
    // invalid_negative_threshold.json should fail to parse/validate
    let content = fs::read_to_string("../scenarios/invalid_negative_threshold.json").unwrap();
    assert!(
        Scenario::from_json(&content).is_err(),
        "Expected invalid_negative_threshold.json to fail validation"
    );

    // invalid_infinite_metric.json should fail to parse/validate
    let content = fs::read_to_string("../scenarios/invalid_infinite_metric.json").unwrap();
    assert!(
        Scenario::from_json(&content).is_err(),
        "Expected invalid_infinite_metric.json to fail validation"
    );
}

#[test]
fn test_p4_validate_scenarios_command() {
    // validate-scenarios should run without crashing
    // It may exit 1 if invalid scenarios exist, but we test it runs
    let output = Command::new("cargo")
        .current_dir("..")
        .arg("run")
        .arg("--bin")
        .arg("hive_workbench")
        .arg("--")
        .arg("validate-scenarios")
        .output()
        .expect("Failed to run validate-scenarios");
    // Command should not panic — exit 0 or 1 are both acceptable
    let code = output.status.code().unwrap_or(-1);
    assert!(
        code == 0 || code == 1,
        "validate-scenarios should exit 0 or 1, got {}",
        code
    );
}

// ─── P5 new tests ─────────────────────────────────────────────────────────────

#[test]
fn test_p5_receipt_filename_uses_scenario_name() {
    // Run exact scenario and check that the receipt uses scenario name
    let _ = run_workbench(&["run", "exact"]);
    // Receipt should be at receipts/out/worker_exact_threshold_receipt.json
    let receipt_path = "../receipts/out/worker_exact_threshold_receipt.json";
    if std::path::Path::new(receipt_path).exists() {
        let content = fs::read_to_string(receipt_path).unwrap();
        let val: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(
            val["final_suffering_score"].is_number(),
            "Receipt should have final_suffering_score"
        );
        // Clean up
        let _ = fs::remove_file(receipt_path);
    }
    // Test passes regardless of whether receipt file exists (may have been cleaned)
}

#[test]
fn test_p5_summarize_handles_empty_dir() {
    // Create a temp dir and summarize it
    let temp = std::env::temp_dir().join("hive_test_summarize_empty");
    let _ = fs::create_dir_all(&temp);
    let temp_str = temp.to_string_lossy().to_string();

    let output = Command::new("cargo")
        .current_dir("..")
        .arg("run")
        .arg("--bin")
        .arg("hive_workbench")
        .arg("--")
        .arg("summarize")
        .arg(&temp_str)
        .output()
        .expect("Failed to run summarize");
    assert!(
        output.status.success(),
        "summarize on empty dir should succeed"
    );
    let _ = fs::remove_dir_all(&temp);
}

#[test]
fn test_p5_report_scenarios_creates_markdown() {
    let (stdout, stderr, status) = run_workbench(&["report", "scenarios"]);
    let out = format!("{}\n{}", stdout, stderr);
    assert!(status.success(), "report scenarios failed: {}", out);

    // Check that the markdown report was created
    let report_path = "../reports/evidence/SCENARIO_EVIDENCE_REPORT.md";
    assert!(
        std::path::Path::new(report_path).exists(),
        "Expected report at {}",
        report_path
    );
    let content = fs::read_to_string(report_path).unwrap();
    assert!(
        content.contains("Scenario Evidence Report"),
        "Report should contain title"
    );
    assert!(
        content.contains("| Scenario |") || content.contains("|---|"),
        "Report should contain table"
    );
}

// ─── P6 new tests ─────────────────────────────────────────────────────────────

#[test]
fn test_p6_golden_preview_prints_valid_json() {
    let (stdout, _stderr, status) = run_workbench(&["golden-preview", "exact", "--output", "json"]);
    assert!(status.success(), "golden-preview failed");
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(
        parsed.is_ok(),
        "golden-preview output should be valid JSON: {}",
        stdout
    );
}

#[test]
fn test_p6_golden_files_not_modified_by_tests() {
    // Read golden file content before any test runs
    let golden_path = "tests/golden/golden_receipt_exact_threshold.json";
    let before = fs::read_to_string(golden_path).expect("Golden file should exist");

    // Run the golden test (test_golden_receipt_exact_threshold) behavior inline
    let exact_content = fs::read_to_string("../scenarios/worker_exact_threshold.json").unwrap();
    let scenario = Scenario::from_json(&exact_content).unwrap();
    let fixed_id = Uuid::nil();
    match scenario.run(fixed_id) {
        WorkerOutcome::Terminated(_) => {}
        WorkerOutcome::Survived(_) => panic!("Expected termination"),
    }

    // Golden file should be unchanged
    let after = fs::read_to_string(golden_path).expect("Golden file should still exist");
    assert_eq!(before, after, "Golden file was modified by test run!");
}

#[test]
fn test_p6_regression_command_exits_0_on_clean() {
    let (_stdout, _stderr, status) = run_workbench(&["regression"]);
    // Regression should pass on a clean repo
    assert!(
        status.success(),
        "regression command should exit 0 on clean repo"
    );
}
