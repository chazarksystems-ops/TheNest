use std::env;
use std::fs;
use std::path::Path;
use swarm_core::{write_payload_json_pretty, EpigeneticPayload, Scenario, WorkerOutcome};
use uuid::Uuid;

// ─── Output mode ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
enum OutputMode {
    Human,
    Json,
    Quiet,
}

impl OutputMode {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "human" => Some(OutputMode::Human),
            "json" => Some(OutputMode::Json),
            "quiet" => Some(OutputMode::Quiet),
            _ => None,
        }
    }
}

// ─── CLI argument parsing ────────────────────────────────────────────────────

struct ParsedArgs {
    command: Option<String>,
    positional: Vec<String>,
    output: OutputMode,
}

fn parse_args(raw: &[String]) -> ParsedArgs {
    let mut command: Option<String> = None;
    let mut positional: Vec<String> = Vec::new();
    let mut output = OutputMode::Human;
    let mut skip_next = false;

    for (i, arg) in raw.iter().enumerate().skip(1) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--output" {
            if let Some(val) = raw.get(i + 1) {
                if let Some(mode) = OutputMode::from_str(val.as_str()) {
                    output = mode;
                } else {
                    eprintln!(
                        "Error: unknown output mode '{}'. Valid: human, json, quiet",
                        val
                    );
                    std::process::exit(1);
                }
                skip_next = true;
            } else {
                eprintln!("Error: --output requires a value (human, json, quiet)");
                std::process::exit(1);
            }
        } else if arg.starts_with("--output=") {
            let val = &arg["--output=".len()..];
            if let Some(mode) = OutputMode::from_str(val) {
                output = mode;
            } else {
                eprintln!(
                    "Error: unknown output mode '{}'. Valid: human, json, quiet",
                    val
                );
                std::process::exit(1);
            }
        } else if command.is_none() {
            command = Some(arg.clone());
        } else {
            positional.push(arg.clone());
        }
    }

    ParsedArgs {
        command,
        positional,
        output,
    }
}

// ─── Scenario shortcut resolution ───────────────────────────────────────────

fn resolve_scenario_path(name: &str) -> Option<String> {
    let shortcut = match name {
        "healthy" => Some("scenarios/worker_survives.json".to_string()),
        "below" => Some("scenarios/worker_just_below_threshold.json".to_string()),
        "exact" => Some("scenarios/worker_exact_threshold.json".to_string()),
        "breach" => Some("scenarios/worker_threshold_breach.json".to_string()),
        _ => None,
    };
    if let Some(p) = shortcut {
        return Some(p);
    }
    // Try direct path
    if Path::new(name).exists() {
        return Some(name.to_string());
    }
    // Try scenarios/<name>.json
    let local = format!("scenarios/{}.json", name);
    if Path::new(&local).exists() {
        return Some(local);
    }
    None
}

// ─── Usage ───────────────────────────────────────────────────────────────────

fn print_usage() {
    eprintln!("hive_workbench — local scenario workbench");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  hive_workbench <command> [args] [--output human|json|quiet]");
    eprintln!();
    eprintln!("COMMANDS:");
    eprintln!("  list                         List available scenarios");
    eprintln!("  run <scenario>               Run a named scenario (shortcuts: healthy, below, exact, breach)");
    eprintln!("  run-file <path>              Run a scenario from file path");
    eprintln!("  suite                        Run all scenarios and print result table");
    eprintln!("  validate-scenarios           Validate all scenarios (parse + outcome check)");
    eprintln!("  summarize <receipts-path>    Summarize receipt files in directory");
    eprintln!("  report scenarios             Generate evidence report markdown");
    eprintln!("  golden-preview <scenario>    Print deterministic golden receipt (no write)");
    eprintln!("  regression                   Run scenario validation and golden checks");
    eprintln!();
    eprintln!("OUTPUT MODES: human (default), json, quiet");
}

// ─── Write receipt for terminated scenario ────────────────────────────────────

fn write_receipt(scenario_name: &str, payload: &EpigeneticPayload) {
    let out_dir = "receipts/out";
    if let Err(e) = fs::create_dir_all(out_dir) {
        eprintln!("Warning: could not create receipts/out: {}", e);
        return;
    }
    let out_file = format!("{}/{}_receipt.json", out_dir, scenario_name);
    if let Err(e) = write_payload_json_pretty(&out_file, payload) {
        eprintln!("Warning: could not write receipt to {}: {}", out_file, e);
    }
}

// ─── CMD: list ────────────────────────────────────────────────────────────────

fn cmd_list(output: OutputMode) {
    let entries = collect_scenario_files();

    match output {
        OutputMode::Human => {
            println!("Available scenarios in scenarios/:");
            println!("{:<45} {:<12} {}", "File", "Expected", "Description");
            println!("{}", "-".repeat(80));
            for e in &entries {
                println!(
                    "{:<45} {:<12} {}",
                    e.file,
                    e.expected.as_deref().unwrap_or("-"),
                    e.description.as_deref().unwrap_or("-")
                );
            }
            println!("\nTotal: {} scenario(s)", entries.len());
        }
        OutputMode::Json => {
            let json_entries: Vec<serde_json::Value> = entries
                .iter()
                .map(|e| {
                    serde_json::json!({
                        "file": e.file,
                        "scenario_name": e.name,
                        "expected_outcome": e.expected,
                        "description": e.description,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json_entries).unwrap());
        }
        OutputMode::Quiet => {
            println!("Found {} scenario(s).", entries.len());
        }
    }
}

struct ScenarioEntry {
    file: String,
    name: String,
    expected: Option<String>,
    description: Option<String>,
}

fn collect_scenario_files() -> Vec<ScenarioEntry> {
    let mut entries = Vec::new();
    let dir = Path::new("scenarios");
    if !dir.exists() {
        return entries;
    }
    let mut paths: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .map(|e| e.path())
        .collect();
    paths.sort();

    for path in paths {
        let file = path.to_string_lossy().replace('\\', "/");
        // Try to load metadata
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                let name = val["scenario_name"]
                    .as_str()
                    .unwrap_or("unknown")
                    .to_string();
                let expected = val["expected_outcome"].as_str().map(|s| s.to_string());
                let description = val["description"].as_str().map(|s| s.to_string());
                entries.push(ScenarioEntry {
                    file,
                    name,
                    expected,
                    description,
                });
                continue;
            }
        }
        entries.push(ScenarioEntry {
            file,
            name: "unknown".to_string(),
            expected: None,
            description: None,
        });
    }
    entries
}

// ─── CMD: run ────────────────────────────────────────────────────────────────

fn cmd_run(name: &str, output: OutputMode) {
    let path = match resolve_scenario_path(name) {
        Some(p) => p,
        None => {
            eprintln!("Error: scenario '{}' not found", name);
            std::process::exit(2);
        }
    };
    cmd_run_file(&path, output);
}

// ─── CMD: run-file ────────────────────────────────────────────────────────────

fn cmd_run_file(path: &str, output: OutputMode) {
    if !Path::new(path).exists() {
        eprintln!("Error: file not found: {}", path);
        std::process::exit(2);
    }
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: failed to read '{}': {}", path, e);
            std::process::exit(2);
        }
    };
    let scenario = match Scenario::from_json(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: parse failure in '{}': {}", path, e);
            std::process::exit(3);
        }
    };

    let scenario_name = scenario.scenario_name.clone();
    let threshold = scenario.config.threshold;
    let alpha = scenario.config.alpha;
    let beta = scenario.config.beta;
    let gamma = scenario.config.gamma;
    let bloat = scenario.metrics.context_bloat;
    let error = scenario.metrics.error_rate;
    let debt = scenario.metrics.coordination_debt;

    let id = Uuid::new_v4();
    match scenario.run(id) {
        WorkerOutcome::Survived(w) => {
            let score = w.nociceptor.calculate_suffering();
            match output {
                OutputMode::Human => {
                    println!("=== Scenario Workbench Runner ===");
                    println!("Loading scenario from: {}", path);
                    println!("Scenario Name: {}", scenario_name);
                    println!("Config threshold: {}", threshold);
                    println!("Weights: alpha={}, beta={}, gamma={}", alpha, beta, gamma);
                    println!(
                        "Metrics: bloat={}, error={}, coordination={}",
                        bloat, error, debt
                    );
                    println!("Outcome: SURVIVED");
                    println!("Suffering score: {}", score);
                }
                OutputMode::Json => {
                    let out = serde_json::json!({
                        "scenario_name": scenario_name,
                        "outcome": "survived",
                        "score": score,
                        "threshold": threshold,
                    });
                    println!("{}", serde_json::to_string_pretty(&out).unwrap());
                }
                OutputMode::Quiet => {
                    println!("SURVIVED");
                }
            }
        }
        WorkerOutcome::Terminated(receipt) => {
            let score = receipt.final_suffering_score;
            let receipt_json = serde_json::to_string_pretty(&receipt).unwrap();
            match output {
                OutputMode::Human => {
                    println!("=== Scenario Workbench Runner ===");
                    println!("Loading scenario from: {}", path);
                    println!("Scenario Name: {}", scenario_name);
                    println!("Config threshold: {}", threshold);
                    println!("Weights: alpha={}, beta={}, gamma={}", alpha, beta, gamma);
                    println!(
                        "Metrics: bloat={}, error={}, coordination={}",
                        bloat, error, debt
                    );
                    println!("Outcome: TERMINATED (Apoptosis triggered)");
                    println!("Suffering score: {}", score);
                    println!("Receipt JSON:\n{}", receipt_json);

                    let out_dir = "receipts/out";
                    let out_file = format!("{}/{}_receipt.json", out_dir, scenario_name);
                    println!("Writing receipt to: {}", out_file);
                    let _ = fs::create_dir_all(out_dir);
                    if let Err(e) = write_payload_json_pretty(&out_file, &receipt) {
                        eprintln!("Error writing receipt: {}", e);
                    } else {
                        println!("Receipt written successfully.");
                    }
                }
                OutputMode::Json => {
                    println!("{}", receipt_json);
                }
                OutputMode::Quiet => {
                    println!("TERMINATED");
                }
            }
        }
    }
}

// ─── CMD: suite ───────────────────────────────────────────────────────────────

struct SuiteRow {
    scenario: String,
    expected: String,
    actual: String,
    score: f32,
    threshold: f32,
    status: String,
}

fn cmd_suite(output: OutputMode) {
    let entries = collect_scenario_files();
    let mut rows: Vec<SuiteRow> = Vec::new();
    let mut skipped = 0usize;

    for entry in &entries {
        let content = match fs::read_to_string(&entry.file) {
            Ok(c) => c,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };

        // Check if expected_outcome is "invalid" — don't run these
        let expected_outcome = entry.expected.clone().unwrap_or_default();
        if expected_outcome == "invalid" {
            let valid = Scenario::from_json(&content).is_err();
            rows.push(SuiteRow {
                scenario: entry.name.clone(),
                expected: "invalid".to_string(),
                actual: if valid {
                    "invalid".to_string()
                } else {
                    "valid".to_string()
                },
                score: 0.0,
                threshold: 0.0,
                status: if valid {
                    "PASS".to_string()
                } else {
                    "FAIL".to_string()
                },
            });
            continue;
        }

        let scenario = match Scenario::from_json(&content) {
            Ok(s) => s,
            Err(_) => {
                skipped += 1;
                continue;
            }
        };

        let name = scenario.scenario_name.clone();
        let threshold = scenario.config.threshold;
        let id = Uuid::new_v4();
        let (actual, score) = match scenario.run(id) {
            WorkerOutcome::Survived(w) => {
                ("survived".to_string(), w.nociceptor.calculate_suffering())
            }
            WorkerOutcome::Terminated(p) => ("terminated".to_string(), p.final_suffering_score),
        };

        let expected = if expected_outcome.is_empty() {
            "-".to_string()
        } else {
            expected_outcome.clone()
        };

        let status = if expected == "-" {
            "OK".to_string()
        } else if expected == actual {
            "PASS".to_string()
        } else {
            "FAIL".to_string()
        };

        rows.push(SuiteRow {
            scenario: name,
            expected,
            actual,
            score,
            threshold,
            status,
        });
    }

    let passed = rows
        .iter()
        .filter(|r| r.status == "PASS" || r.status == "OK")
        .count();
    let failed = rows.iter().filter(|r| r.status == "FAIL").count();
    let total = rows.len();

    match output {
        OutputMode::Human => {
            println!(
                "{:<40} {:<12} {:<12} {:<8} {:<10} {}",
                "scenario", "expected", "actual", "score", "threshold", "status"
            );
            println!("{}", "-".repeat(100));
            for r in &rows {
                println!(
                    "{:<40} {:<12} {:<12} {:<8.2} {:<10.2} {}",
                    r.scenario, r.expected, r.actual, r.score, r.threshold, r.status
                );
            }
            println!();
            println!(
                "Suite: {} run, {} passed/ok, {} failed, {} skipped",
                total, passed, failed, skipped
            );
        }
        OutputMode::Json => {
            let json_rows: Vec<serde_json::Value> = rows
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "scenario": r.scenario,
                        "expected": r.expected,
                        "actual": r.actual,
                        "score": r.score,
                        "threshold": r.threshold,
                        "status": r.status,
                    })
                })
                .collect();
            let summary = serde_json::json!({
                "rows": json_rows,
                "total": total,
                "passed": passed,
                "failed": failed,
                "skipped": skipped,
            });
            println!("{}", serde_json::to_string_pretty(&summary).unwrap());
        }
        OutputMode::Quiet => {
            println!(
                "Suite execution completed: {} run, {} passed, {} failed.",
                total, passed, failed
            );
        }
    }
}

// ─── CMD: validate-scenarios ─────────────────────────────────────────────────

fn run_validate_scenarios(output: OutputMode) -> bool {
    let entries = collect_scenario_files();
    let mut all_pass = true;
    let mut results: Vec<(String, String, String)> = Vec::new(); // (name, expected, status+message)

    for entry in &entries {
        let content = match fs::read_to_string(&entry.file) {
            Ok(c) => c,
            Err(e) => {
                results.push((
                    entry.name.clone(),
                    "?".to_string(),
                    format!("READ_ERROR: {}", e),
                ));
                all_pass = false;
                continue;
            }
        };

        let expected = entry.expected.clone().unwrap_or_default();

        if expected == "invalid" {
            // Expect parse/validation to fail
            match Scenario::from_json(&content) {
                Ok(_) => {
                    results.push((
                        entry.name.clone(),
                        "invalid".to_string(),
                        "FAIL (expected invalid, but parsed OK)".to_string(),
                    ));
                    all_pass = false;
                }
                Err(_) => {
                    results.push((
                        entry.name.clone(),
                        "invalid".to_string(),
                        "PASS (correctly rejected)".to_string(),
                    ));
                }
            }
            continue;
        }

        let scenario = match Scenario::from_json(&content) {
            Ok(s) => s,
            Err(e) => {
                results.push((
                    entry.name.clone(),
                    expected.clone(),
                    format!("PARSE_FAIL: {}", e),
                ));
                all_pass = false;
                continue;
            }
        };

        let threshold = scenario.config.threshold;
        let _ = threshold;
        let id = Uuid::new_v4();
        let actual = match scenario.run(id) {
            WorkerOutcome::Survived(_) => "survived".to_string(),
            WorkerOutcome::Terminated(_) => "terminated".to_string(),
        };

        let status = if expected.is_empty() || expected == "-" {
            "OK".to_string()
        } else if expected == actual {
            "PASS".to_string()
        } else {
            all_pass = false;
            format!("FAIL (expected={}, actual={})", expected, actual)
        };

        results.push((entry.name.clone(), expected, status));
    }

    match output {
        OutputMode::Human => {
            println!("{:<40} {:<12} {}", "Scenario", "Expected", "Status");
            println!("{}", "-".repeat(80));
            for (name, expected, status) in &results {
                println!("{:<40} {:<12} {}", name, expected, status);
            }
            if all_pass {
                println!("\nAll scenarios validated successfully.");
            } else {
                println!("\nValidation FAILED for one or more scenarios.");
            }
        }
        OutputMode::Json => {
            let rows: Vec<serde_json::Value> = results
                .iter()
                .map(|(n, e, s)| serde_json::json!({"scenario": n, "expected": e, "status": s}))
                .collect();
            let out = serde_json::json!({"results": rows, "all_pass": all_pass});
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        OutputMode::Quiet => {
            if all_pass {
                println!("validate-scenarios: PASS");
            } else {
                println!("validate-scenarios: FAIL");
            }
        }
    }

    all_pass
}

fn cmd_validate_scenarios(output: OutputMode) {
    let pass = run_validate_scenarios(output);
    if !pass {
        std::process::exit(1);
    }
}

// ─── CMD: summarize ───────────────────────────────────────────────────────────

fn cmd_summarize(receipts_path: &str, output: OutputMode) {
    let dir = Path::new(receipts_path);
    if !dir.exists() {
        match output {
            OutputMode::Json => {
                println!(
                    "{}",
                    serde_json::json!({"error": "directory not found", "path": receipts_path})
                );
            }
            _ => {
                println!("No receipts directory found at: {}", receipts_path);
            }
        }
        return;
    }

    let mut payloads: Vec<EpigeneticPayload> = Vec::new();
    let json_files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .filter_map(|r| r.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();

    for entry in &json_files {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            if let Ok(payload) = serde_json::from_str::<EpigeneticPayload>(&content) {
                payloads.push(payload);
            }
        }
    }

    let total = payloads.len();
    let terminated = total; // all receipts are termination receipts
    let scores: Vec<f32> = payloads.iter().map(|p| p.final_suffering_score).collect();
    let avg_score = if scores.is_empty() {
        0.0f32
    } else {
        scores.iter().sum::<f32>() / scores.len() as f32
    };
    let min_score = scores.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_score = scores.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let mut reason_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for p in &payloads {
        let reason = format!("{:?}", p.termination_reason);
        *reason_counts.entry(reason).or_insert(0) += 1;
    }

    let scenario_ids: Vec<String> = payloads.iter().map(|p| p.worker_id.to_string()).collect();

    match output {
        OutputMode::Human => {
            println!("=== Receipt Summary: {} ===", receipts_path);
            if total == 0 {
                println!("No valid receipts found.");
                return;
            }
            println!("Total receipts:    {}", total);
            println!("Terminated count:  {}", terminated);
            println!("Average score:     {:.4}", avg_score);
            println!(
                "Min score:         {:.4}",
                if min_score.is_infinite() {
                    0.0
                } else {
                    min_score
                }
            );
            println!(
                "Max score:         {:.4}",
                if max_score.is_infinite() {
                    0.0
                } else {
                    max_score
                }
            );
            println!("Termination reasons:");
            let mut reasons: Vec<_> = reason_counts.iter().collect();
            reasons.sort_by_key(|(k, _)| (*k).clone());
            for (r, c) in &reasons {
                println!("  {}: {}", r, c);
            }
            println!("Worker IDs:");
            for id in &scenario_ids {
                println!("  {}", id);
            }
        }
        OutputMode::Json => {
            let reasons_json: serde_json::Value = reason_counts
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::json!(v)))
                .collect();
            let out = serde_json::json!({
                "receipts_path": receipts_path,
                "total": total,
                "terminated_count": terminated,
                "average_score": avg_score,
                "min_score": if min_score.is_infinite() { 0.0 } else { min_score },
                "max_score": if max_score.is_infinite() { 0.0 } else { max_score },
                "termination_reasons": reasons_json,
                "worker_ids": scenario_ids,
            });
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        OutputMode::Quiet => {
            println!(
                "Receipts: {}, Terminated: {}, Avg score: {:.2}",
                total, terminated, avg_score
            );
        }
    }
}

// ─── CMD: report scenarios ────────────────────────────────────────────────────

fn cmd_report_scenarios(output: OutputMode) {
    // Run all scenarios, collect results
    let entries = collect_scenario_files();
    let mut rows: Vec<SuiteRow> = Vec::new();

    for entry in &entries {
        let content = match fs::read_to_string(&entry.file) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let expected_outcome = entry.expected.clone().unwrap_or_default();

        if expected_outcome == "invalid" {
            let valid = Scenario::from_json(&content).is_err();
            rows.push(SuiteRow {
                scenario: entry.name.clone(),
                expected: "invalid".to_string(),
                actual: if valid {
                    "invalid".to_string()
                } else {
                    "valid".to_string()
                },
                score: 0.0,
                threshold: 0.0,
                status: if valid {
                    "PASS".to_string()
                } else {
                    "FAIL".to_string()
                },
            });
            continue;
        }

        let scenario = match Scenario::from_json(&content) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let scenario_name = scenario.scenario_name.clone();
        let threshold = scenario.config.threshold;
        let id = Uuid::new_v4();

        let (actual, score, maybe_payload): (String, f32, Option<EpigeneticPayload>) =
            match scenario.run(id) {
                WorkerOutcome::Survived(w) => (
                    "survived".to_string(),
                    w.nociceptor.calculate_suffering(),
                    None,
                ),
                WorkerOutcome::Terminated(p) => {
                    let s = p.final_suffering_score;
                    ("terminated".to_string(), s, Some(p))
                }
            };

        // Write receipt if terminated
        if let Some(ref payload) = maybe_payload {
            write_receipt(&scenario_name, payload);
        }

        let expected = if expected_outcome.is_empty() {
            "-".to_string()
        } else {
            expected_outcome
        };
        let status = if expected == "-" {
            "OK".to_string()
        } else if expected == actual {
            "PASS".to_string()
        } else {
            "FAIL".to_string()
        };

        rows.push(SuiteRow {
            scenario: scenario_name,
            expected,
            actual,
            score,
            threshold,
            status,
        });
    }

    // Generate markdown report
    let report_dir = "reports/evidence";
    if let Err(e) = fs::create_dir_all(report_dir) {
        eprintln!("Warning: could not create {}: {}", report_dir, e);
    }

    let report_path = format!("{}/SCENARIO_EVIDENCE_REPORT.md", report_dir);
    let now = "2026-06-05";
    let mut md = String::new();
    md.push_str("# Scenario Evidence Report\n\n");
    md.push_str(&format!("Generated: {}\n\n", now));
    md.push_str("| Scenario | Expected | Actual | Score | Threshold | Status |\n");
    md.push_str("|---|---|---|---|---|---|\n");
    for r in &rows {
        md.push_str(&format!(
            "| {} | {} | {} | {:.2} | {:.2} | {} |\n",
            r.scenario, r.expected, r.actual, r.score, r.threshold, r.status
        ));
    }
    md.push_str("\n## Receipt Output Path\n\n`receipts/out/`\n");

    if let Err(e) = fs::write(&report_path, &md) {
        eprintln!("Error: could not write report to {}: {}", report_path, e);
    }

    match output {
        OutputMode::Human => {
            println!("Evidence report generated: {}", report_path);
            println!("{} scenarios processed.", rows.len());
        }
        OutputMode::Json => {
            let out = serde_json::json!({
                "report_path": report_path,
                "scenarios_processed": rows.len(),
            });
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        OutputMode::Quiet => {
            println!("Report: {}", report_path);
        }
    }
}

// ─── CMD: golden-preview ─────────────────────────────────────────────────────

fn cmd_golden_preview(name: &str, output: OutputMode) {
    let path = match resolve_scenario_path(name) {
        Some(p) => p,
        None => {
            eprintln!("Error: scenario '{}' not found", name);
            std::process::exit(2);
        }
    };

    if !Path::new(&path).exists() {
        eprintln!("Error: file not found: {}", path);
        std::process::exit(2);
    }

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: failed to read '{}': {}", path, e);
            std::process::exit(2);
        }
    };

    let scenario = match Scenario::from_json(&content) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error: parse failure: {}", e);
            std::process::exit(3);
        }
    };

    let fixed_id = Uuid::nil();
    match scenario.run(fixed_id) {
        WorkerOutcome::Survived(w) => {
            let score = w.nociceptor.calculate_suffering();
            let preview = serde_json::json!({
                "preview_note": "Worker survived — no receipt would be generated",
                "outcome": "survived",
                "score": score,
            });
            match output {
                OutputMode::Json => println!("{}", serde_json::to_string_pretty(&preview).unwrap()),
                OutputMode::Quiet => println!("SURVIVED"),
                OutputMode::Human => {
                    println!("Golden preview for '{}': SURVIVED (score: {})", name, score);
                    println!("No receipt generated for survived scenarios.");
                }
            }
        }
        WorkerOutcome::Terminated(payload) => {
            let json_str = serde_json::to_string_pretty(&payload).unwrap();
            match output {
                OutputMode::Json | OutputMode::Human => {
                    println!("{}", json_str);
                }
                OutputMode::Quiet => {
                    println!("TERMINATED");
                }
            }
            if output == OutputMode::Human {
                eprintln!("[golden-preview] This is a preview only — golden file not modified.");
            }
        }
    }
}

// ─── CMD: regression ─────────────────────────────────────────────────────────

fn cmd_regression(output: OutputMode) {
    // Run validate-scenarios internally
    let pass = run_validate_scenarios(OutputMode::Quiet);

    // Check golden files haven't changed by verifying the golden test would pass
    let golden_ok = check_golden_files();

    let all_ok = pass && golden_ok;

    match output {
        OutputMode::Human => {
            println!("=== Regression Check ===");
            println!("validate-scenarios: {}", if pass { "PASS" } else { "FAIL" });
            println!(
                "golden files:       {}",
                if golden_ok { "PASS" } else { "FAIL" }
            );
            println!();
            if all_ok {
                println!("Regression: CLEAN");
            } else {
                println!("Regression: FAILURES DETECTED");
            }
        }
        OutputMode::Json => {
            let out = serde_json::json!({
                "validate_scenarios": pass,
                "golden_files": golden_ok,
                "all_clean": all_ok,
            });
            println!("{}", serde_json::to_string_pretty(&out).unwrap());
        }
        OutputMode::Quiet => {
            println!("{}", if all_ok { "CLEAN" } else { "FAIL" });
        }
    }

    if !all_ok {
        std::process::exit(1);
    }
}

fn check_golden_files() -> bool {
    // Re-run the exact_threshold scenario with nil UUID and compare to golden
    let golden_path = "swarm_core/tests/golden/golden_receipt_exact_threshold.json";
    let scenario_path = "scenarios/worker_exact_threshold.json";

    let golden_content = match fs::read_to_string(golden_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let scenario_content = match fs::read_to_string(scenario_path) {
        Ok(c) => c,
        Err(_) => return false,
    };

    let scenario = match Scenario::from_json(&scenario_content) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let fixed_id = Uuid::nil();
    match scenario.run(fixed_id) {
        WorkerOutcome::Survived(_) => false,
        WorkerOutcome::Terminated(payload) => {
            let serialized = serde_json::to_string_pretty(&payload).unwrap_or_default();
            let val_serialized: Result<serde_json::Value, _> = serde_json::from_str(&serialized);
            let val_golden: Result<serde_json::Value, _> = serde_json::from_str(&golden_content);
            match (val_serialized, val_golden) {
                (Ok(a), Ok(b)) => a == b,
                _ => false,
            }
        }
    }
}

// ─── main ─────────────────────────────────────────────────────────────────────

fn main() {
    let raw: Vec<String> = env::args().collect();
    let args = parse_args(&raw);

    let command = match args.command {
        Some(ref c) => c.as_str().to_string(),
        None => {
            print_usage();
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "list" => {
            cmd_list(args.output);
        }
        "run" => {
            if args.positional.is_empty() {
                eprintln!("Error: 'run' requires a scenario name");
                eprintln!("Usage: hive_workbench run <scenario-name>");
                std::process::exit(1);
            }
            cmd_run(&args.positional[0], args.output);
        }
        "run-file" => {
            if args.positional.is_empty() {
                eprintln!("Error: 'run-file' requires a file path");
                eprintln!("Usage: hive_workbench run-file <path>");
                std::process::exit(1);
            }
            cmd_run_file(&args.positional[0], args.output);
        }
        "suite" => {
            cmd_suite(args.output);
        }
        "validate-scenarios" => {
            cmd_validate_scenarios(args.output);
        }
        "summarize" => {
            let path = args
                .positional
                .first()
                .map(|s| s.as_str())
                .unwrap_or("receipts/out");
            cmd_summarize(path, args.output);
        }
        "report" => {
            let sub = args.positional.first().map(|s| s.as_str()).unwrap_or("");
            if sub == "scenarios" {
                cmd_report_scenarios(args.output);
            } else {
                eprintln!(
                    "Error: unknown report subcommand '{}'. Try: report scenarios",
                    sub
                );
                std::process::exit(1);
            }
        }
        "golden-preview" => {
            if args.positional.is_empty() {
                eprintln!("Error: 'golden-preview' requires a scenario name");
                eprintln!("Usage: hive_workbench golden-preview <scenario-name>");
                std::process::exit(1);
            }
            cmd_golden_preview(&args.positional[0], args.output);
        }
        "regression" => {
            cmd_regression(args.output);
        }
        other => {
            eprintln!("Error: unknown command '{}'", other);
            eprintln!();
            print_usage();
            std::process::exit(1);
        }
    }
}
