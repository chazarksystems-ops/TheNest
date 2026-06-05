# RT-001 — Implement `swarm_core`

## Task scope

Create a new Rust workspace for Hive P0 and implement the `swarm_core` crate only.

## Expected files to create or edit

- Root `Cargo.toml`
- `swarm_core/Cargo.toml`
- `swarm_core/src/lib.rs`
- `swarm_core/src/health.rs`
- `swarm_core/src/receipt.rs`
- `swarm_core/src/lifecycle.rs`

## Forbidden files / work

- Do not implement vLLM, Axolotl, PyO3, training, routing, model downloads, or dashboards.
- Do not paste raw code from the source hive notes without adapting and compiling it.
- Do not create long-running scripts.
- Do not modify any existing repo outside the new workspace.

## Required behavior

Implement:

- `HealthWeights`
- `HealthMetrics`
- `HealthSnapshot`
- deterministic score calculation: `context_bloat_weight * context_bloat + error_weight * error_rate + latency_weight * coordination_debt`
- threshold check
- `TerminationReason`
- `WorkerExitReceipt`
- lifecycle trait that consumes `self` to emit a receipt, e.g. `fn terminate(self, reason: TerminationReason, fault_signature: impl Into<String>) -> WorkerExitReceipt`

## Tests required

- Score calculation test.
- Non-terminal threshold test.
- Terminal threshold test.
- Receipt construction test.
- Compile test showing a worker can return `WorkerOutcome::Survived(Self)` or `WorkerOutcome::Terminated(WorkerExitReceipt)`.

## Validation commands

Run:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Success criteria

Report:

```text
RT-001 status: PASS/FAIL/BLOCKED
files created:
files modified:
commands run:
validation result:
notes:
```
