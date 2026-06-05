use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swarm_core::{Nociceptor, NociceptorConfig, WorkerHealthMetrics};

fn bench_nociceptor(c: &mut Criterion) {
    let config = NociceptorConfig::default_p0();
    let metrics = WorkerHealthMetrics::new(2.0, 1.5, 1.0).unwrap();
    let nociceptor = Nociceptor::new(config, metrics);

    c.bench_function("calculate_suffering", |b| {
        b.iter(|| black_box(&nociceptor).calculate_suffering())
    });

    c.bench_function("nociceptor_is_terminal", |b| {
        b.iter(|| black_box(&nociceptor).is_terminal())
    });
}

criterion_group!(benches, bench_nociceptor);
criterion_main!(benches);
