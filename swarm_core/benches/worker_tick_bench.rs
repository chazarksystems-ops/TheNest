use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swarm_core::{CattleWorker, NociceptorConfig, WorkerHealthMetrics};
use uuid::Uuid;

fn bench_worker_tick(c: &mut Criterion) {
    let config = NociceptorConfig::default_p0();
    let survived_metrics = WorkerHealthMetrics::new(1.0, 1.0, 1.0).unwrap();
    let terminated_metrics = WorkerHealthMetrics::new(5.0, 5.0, 5.0).unwrap();
    let id = Uuid::new_v4();

    c.bench_function("worker_tick_survived", |b| {
        b.iter_batched(
            || CattleWorker::new(id, config.clone(), survived_metrics.clone()),
            |worker| black_box(worker).tick(),
            criterion::BatchSize::SmallInput,
        )
    });

    c.bench_function("worker_tick_terminated", |b| {
        b.iter_batched(
            || CattleWorker::new(id, config.clone(), terminated_metrics.clone()),
            |worker| black_box(worker).tick(),
            criterion::BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, bench_worker_tick);
criterion_main!(benches);
