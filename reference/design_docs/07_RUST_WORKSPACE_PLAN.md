# 07 — Rust Workspace Plan

## Workspace members

### `swarm_core`

Purpose: pure domain types and lifecycle traits.

Recommended modules:

- `health.rs`
  - `HealthWeights`
  - `HealthMetrics`
  - `HealthSnapshot`
  - `HealthScore`
- `receipt.rs`
  - `WorkerExitReceipt`
  - `TerminationReason`
  - `WorkerId`
- `lifecycle.rs`
  - `TerminatesWithReceipt`
  - `WorkerOutcome<T>`

### `ledger`

Purpose: append-only local receipt persistence.

Recommended modules:

- `jsonl.rs`
  - `JsonlReceiptLedger`
  - `append_receipt`
  - temp-file tests

### `orchestration` optional in P0

Purpose: minimal compile-proof worker lifecycle.

Recommended modules:

- `worker.rs`
  - `ToyWorker`
  - `tick(self) -> WorkerOutcome<Self>`

Do not implement task scheduling, model calls, blackboard concurrency, or router logic in P0 unless explicitly approved.

### `inference` deferred

Purpose: future local model endpoint adapters.

Do not implement in P0.

### `orchestrator_bin` optional placeholder

Purpose: future CLI entrypoint.

P0 may include a placeholder `main.rs` that prints package status. It must not call model endpoints.

## Suggested data model

```rust
pub struct HealthWeights {
    pub context_bloat_weight: f32,
    pub error_weight: f32,
    pub latency_weight: f32,
}

pub struct HealthMetrics {
    pub context_bloat: f32,
    pub error_rate: f32,
    pub coordination_debt: f32,
}

pub struct HealthSnapshot {
    pub weights: HealthWeights,
    pub metrics: HealthMetrics,
    pub threshold: f32,
}

pub enum TerminationReason {
    HealthThresholdBreached,
    Timeout,
    ValidationFailed,
    Cancelled,
}

pub struct WorkerExitReceipt {
    pub worker_id: String,
    pub run_id: String,
    pub final_score: f32,
    pub threshold: f32,
    pub reason: TerminationReason,
    pub fault_signature: String,
    pub timestamp_unix_ms: u64,
}
```

## Design note

Names can change during implementation, but behavior must remain stable: deterministic health score, threshold transition, owned termination receipt, append-only evidence.
