# Project Context Summary

The archive reviewed earlier contained a Rust workspace named `kimi_hive_p0_test` with one crate, `swarm_core`.

## Existing shape

```text
kimi_hive_p0_test/
  Cargo.toml
  Cargo.lock
  README.md

  swarm_core/
    Cargo.toml
    src/
      lib.rs
      nociceptor.rs
      worker.rs
      payload.rs
      apoptosis.rs

  receipts/
    KIMI_P0_RECEIPT.md

  target/
    generated build artifacts
```

## Existing implementation

The implementation proves this lifecycle:

```text
worker health metrics
-> suffering score calculation
-> threshold breach detection
-> worker consumes itself by ownership move
-> termination receipt emitted
```

## Existing concepts

- `Nociceptor`: deterministic scoring calculator.
- `CattleWorker`: worker with UUID and nociceptor.
- `WorkerOutcome`: survived or terminated result.
- `Apoptosis`: ownership-consuming termination trait.
- `EpigeneticPayload`: termination receipt.

## Current scoring formula

```text
score = alpha * context_bloat
      + beta * error_rate
      + gamma * coordination_debt
```

## Important current limitation

The current payload uses the name `latency_weight`, but `gamma` applies to `coordination_debt`. This should be renamed to `coordination_debt_weight`.

## Correct interpretation

Good P0 lifecycle primitive.
Not yet a full HIVE runtime.

Do not expand it into orchestration, networking, model serving, or database work yet.
