use criterion::{black_box, criterion_group, criterion_main, Criterion};
use swarm_core::{EpigeneticPayload, TerminationReason};
use uuid::Uuid;

fn bench_receipt_json(c: &mut Criterion) {
    let payload = EpigeneticPayload {
        worker_id: Uuid::new_v4(),
        final_suffering_score: 15.0,
        context_bloat: 5.0,
        error_rate: 5.0,
        coordination_debt: 5.0,
        threshold: 10.0,
        bloat_weight: 1.0,
        error_weight: 1.0,
        coordination_debt_weight: 1.0,
        termination_reason: TerminationReason::ThresholdBreach,
        fault_signature: "Terminal suffering breach".to_string(),
    };

    c.bench_function("receipt_json_serialization", |b| {
        b.iter(|| serde_json::to_string(black_box(&payload)).unwrap())
    });

    let json_str = serde_json::to_string(&payload).unwrap();
    c.bench_function("receipt_json_deserialization", |b| {
        b.iter(|| {
            let parsed: EpigeneticPayload = serde_json::from_str(black_box(&json_str)).unwrap();
            black_box(parsed);
        })
    });
}

criterion_group!(benches, bench_receipt_json);
criterion_main!(benches);
