use swarm_core::{CattleWorker, NociceptorConfig, WorkerHealthMetrics};
use uuid::Uuid;

fn main() {
    let config = NociceptorConfig::default_p0();
    let metrics = WorkerHealthMetrics::new(1.0, 1.0, 1.0).unwrap();
    let worker = CattleWorker::new(Uuid::new_v4(), config, metrics);

    // This tick consumes worker
    let _outcome = worker.tick();

    // Trying to access worker after it was moved into tick() should fail compilation
    let _id = worker.id;
}
