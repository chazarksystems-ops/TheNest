# 02 — Extracted Requirements

## Metaphor-to-code translation

| Source metaphor | Implementation term | P0 handling |
|---|---|---|
| Nociception / pain | `WorkerHealth` / `HealthPenalty` | Implement deterministic scoring. |
| Suffering score | `health_score` or `termination_score` | Implement formula and threshold tests. |
| Apoptosis | `terminate(self) -> WorkerExitReceipt` | Consume `self` by value. |
| Death rattle | `WorkerExitReceipt` / `EpigeneticPayload` | Serialize to receipt ledger. |
| Cattle worker | `Worker` | Implement minimal local worker in tests or orchestration crate. |
| Shepherd | `Supervisor` / `Shepherd` | Future P0/P1 after `swarm_core`. |
| Farmer | `RootOrchestrator` | Future. |
| Blackboard | `TaskContext` / `LocalBlackboard` | Avoid unsafe borrowed-lock patterns. |
| Epigenetics | `FailureSummary` / `HealthAdjustment` | Future; P0 stores receipt only. |
| Suffering-aware router | `RouteCost` | P1/P2 design only. |

## P0 functional requirements

P0 must provide:

1. A Rust workspace that compiles.
2. A `swarm_core` crate containing:
   - health metrics struct;
   - deterministic score calculation;
   - threshold check;
   - termination reason enum;
   - worker exit receipt struct;
   - ownership-consuming lifecycle trait.
3. A `ledger` crate containing:
   - append-only JSONL writer;
   - receipt serialization;
   - unit tests using temp files.
4. Optional minimal `orchestration` crate containing:
   - a toy worker proving `self`-consuming termination;
   - no live model calls.
5. Evidence outputs:
   - command transcript or receipt file showing `cargo fmt --all --check`;
   - `cargo check --workspace`;
   - `cargo test --workspace`.

## P0 nonfunctional requirements

- Keep implementation local-first.
- Keep all outputs human-readable unless a binary format is explicitly approved later.
- Prefer JSONL receipts in P0 over bincode/postcard. Binary ledgers can come later after schema stabilization.
- Make all thresholds explicit and configurable.
- Do not use hidden global mutable state.
- Avoid holding async locks across await points unless deliberately justified.
- Avoid cloning large context strings in future code; but P0 should prefer correctness over premature zero-copy complexity.

## P1+ requirements captured but deferred

- Rust/PyO3 pipeline.
- Local vLLM endpoint client.
- Router with health metrics.
- Persistent router blackboard.
- Prometheus/Grafana/TUI observability.
- Axolotl training config.
- LoRA merge scripts.
- Multi-vertical dataset generation.
