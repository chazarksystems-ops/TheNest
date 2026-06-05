use std::time::Instant;
use swarm_core::{CattleWorker, NociceptorConfig, WorkerHealthMetrics, WorkerOutcome};
use uuid::Uuid;

fn run_stress_test(size: usize, serialize: bool) {
    println!(
        "--- Running Stress Test: {} workers, serialization={} ---",
        size, serialize
    );
    let config = NociceptorConfig::default_p0();

    let survived_metrics = WorkerHealthMetrics::new(1.0, 1.0, 1.0).unwrap();
    let terminated_metrics = WorkerHealthMetrics::new(5.0, 5.0, 5.0).unwrap();

    let mut workers = Vec::with_capacity(size);
    for i in 0..size {
        let metrics = if i % 4 == 0 {
            terminated_metrics.clone()
        } else {
            survived_metrics.clone()
        };
        workers.push(CattleWorker::new(Uuid::new_v4(), config.clone(), metrics));
    }

    let start = Instant::now();
    let mut survived_count = 0;
    let mut terminated_count = 0;
    let mut serialized_bytes = 0;

    for worker in workers {
        match worker.tick() {
            WorkerOutcome::Survived(_) => {
                survived_count += 1;
            }
            WorkerOutcome::Terminated(payload) => {
                terminated_count += 1;
                if serialize {
                    let s = serde_json::to_string(&payload).unwrap();
                    serialized_bytes += s.len();
                }
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Survived: {}", survived_count);
    println!("Terminated (Receipts emitted): {}", terminated_count);
    if serialize {
        println!("Total serialized size: {} bytes", serialized_bytes);
    }
    println!();
}

fn main() {
    println!("=== Batch Stress Test Runner ===");
    run_stress_test(100, false);
    run_stress_test(100, true);

    run_stress_test(1000, false);
    run_stress_test(1000, true);

    run_stress_test(10000, false);
    run_stress_test(10000, true);

    run_stress_test(100000, false);
    run_stress_test(100000, true);
}
