# 04 — P0 Scope

## P0 goal

Create a small Rust workspace that proves the core hive lifecycle mechanics:

- health scoring;
- threshold-based termination;
- ownership-consuming worker exit;
- serializable exit receipts;
- append-only receipt ledger;
- compile/test validation.

## P0 success criteria

P0 is successful only when all of these are true:

- Workspace exists and compiles.
- `swarm_core` exposes stable core types.
- At least one test proves a non-terminal health state survives.
- At least one test proves a terminal health state emits a receipt.
- At least one test proves the receipt writer appends JSONL.
- Validation commands pass:
  - `cargo fmt --all --check`
  - `cargo check --workspace`
  - `cargo test --workspace`

## P0 file changes expected

Expected files:

- Root `Cargo.toml`
- `swarm_core/Cargo.toml`
- `swarm_core/src/lib.rs`
- `swarm_core/src/health.rs`
- `swarm_core/src/receipt.rs`
- `swarm_core/src/lifecycle.rs`
- `ledger/Cargo.toml`
- `ledger/src/lib.rs`
- `ledger/src/jsonl.rs`
- Optional `orchestration` crate only if needed for lifecycle proof.

## P0 forbidden work

- No model serving.
- No training.
- No Python/PyO3.
- No vLLM.
- No Axolotl.
- No dashboards.
- No Prometheus/Grafana.
- No Docker.
- No network downloads.
- No generated datasets.
- No direct use of source-file code without compile/test adaptation.

## Future P1/P2/P3 staging

- **P1:** data-factory smoke test with mocked or local HTTP endpoint.
- **P2:** route-cost design and router blackboard.
- **P3:** DGX local model endpoint verification.
- **P4:** fine-tune packet after data schema is validated.
