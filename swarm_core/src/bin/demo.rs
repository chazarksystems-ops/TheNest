use std::env;
use std::fs;
use std::path::Path;
use swarm_core::{write_payload_json_pretty, Scenario, WorkerOutcome};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin demo -- <scenario_name_or_file>");
        eprintln!("Named scenarios: healthy, exact, breach, below");
        std::process::exit(1);
    }

    let input = args[1].as_str();
    let file_path = match input {
        "healthy" => "scenarios/worker_survives.json".to_string(),
        "exact" => "scenarios/worker_exact_threshold.json".to_string(),
        "breach" => "scenarios/worker_threshold_breach.json".to_string(),
        "below" => "scenarios/worker_just_below_threshold.json".to_string(),
        other => {
            if Path::new(other).exists() {
                other.to_string()
            } else {
                let local_path = format!("scenarios/{}.json", other);
                if Path::new(&local_path).exists() {
                    local_path
                } else {
                    eprintln!("Error: Scenario file not found for '{}'", input);
                    std::process::exit(1);
                }
            }
        }
    };

    println!("=== Scenario Workbench Runner ===");
    println!("Loading scenario from: {}", file_path);

    let json_content = fs::read_to_string(&file_path).expect("Failed to read scenario file");

    let scenario = Scenario::from_json(&json_content).expect("Failed to parse scenario JSON");

    println!("Scenario Name: {}", scenario.scenario_name);
    println!("Config threshold: {}", scenario.config.threshold);
    println!(
        "Weights: alpha={}, beta={}, gamma={}",
        scenario.config.alpha, scenario.config.beta, scenario.config.gamma
    );
    println!(
        "Metrics: bloat={}, error={}, coordination={}",
        scenario.metrics.context_bloat,
        scenario.metrics.error_rate,
        scenario.metrics.coordination_debt
    );

    let id = Uuid::new_v4();
    match scenario.run(id) {
        WorkerOutcome::Survived(w) => {
            println!("Outcome: SURVIVED");
            println!("Suffering score: {}", w.nociceptor.calculate_suffering());
        }
        WorkerOutcome::Terminated(receipt) => {
            println!("Outcome: TERMINATED (Apoptosis triggered)");
            println!("Suffering score: {}", receipt.final_suffering_score);
            let pretty_json = serde_json::to_string_pretty(&receipt).unwrap();
            println!("Receipt JSON:\n{}", pretty_json);

            let out_dir = "receipts/out";
            let out_file = format!("{}/{}_receipt.json", out_dir, receipt.worker_id);
            println!("Writing receipt to: {}", out_file);
            if let Err(e) = write_payload_json_pretty(&out_file, &receipt) {
                eprintln!("Error writing receipt to file: {}", e);
            } else {
                println!("Receipt written successfully.");
            }
        }
    }
}
