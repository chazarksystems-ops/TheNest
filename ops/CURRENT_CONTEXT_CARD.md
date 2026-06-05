# Current Context Card

## PROJECT

`TheNest`

## CURRENT STATUS

**ROADMAP COMPLETE — P0 through P8 fully implemented and validated.**

All phases P3–P8 have been implemented:
- P3: `hive_workbench` CLI with list, run, run-file, suite, output modes
- P4: Scenario metadata, validate-scenarios, expanded scenario library (14 files)
- P5: Predictable receipt filenames, summarize command, evidence report command
- P6: golden-preview, regression command, baseline docs
- P7: QUICKSTART.md, README rewrite, handoff manifest
- P8: Final audit, ROADMAP_CLOSEOUT.md, future parking lot

## LAST COMPLETED SLICE

`P8_FINAL_AUDIT_AND_CLOSEOUT`

## COMPLETED SLICES

See `ops/COMPLETED_SLICES.md` for the full history including P0-P8.

## CURRENT CODE SHAPE

Single-crate Rust workspace `swarm_core` containing modules:
- `config`: `NociceptorConfig` representing static weights and limits.
- `metrics`: `WorkerHealthMetrics` representing live agent parameters.
- `nociceptor`: `Nociceptor` score calculator.
- `reason`: `TerminationReason` struct.
- `payload`: `EpigeneticPayload` termination receipt.
- `apoptosis`: `Apoptosis` move trait.
- `worker`: `CattleWorker` lifecycle agent and `WorkerOutcome` transitions.
- `receipt_sink`: module to write epigenetic receipts to disk.
- `lib`: `Scenario` struct with optional P4 metadata fields (id, description, expected_outcome, expected_score).

Binary utility tools:
- `hive_workbench`: Full CLI workbench (P3-P6 commands).
- `batch_stress`: harness executing 100k ticks with/without serialization under load.
- `demo`: Legacy demo binary (kept for historical reference, superseded by hive_workbench).

## CURRENT RECEIPT SHAPE

`EpigeneticPayload` contains:
- `worker_id: Uuid`
- `final_suffering_score: f32`
- `context_bloat: f32`
- `error_rate: f32`
- `coordination_debt: f32`
- `threshold: f32`
- `bloat_weight: f32`
- `error_weight: f32`
- `coordination_debt_weight: f32`
- `termination_reason: TerminationReason`
- `fault_signature: String`

## CLI COMMANDS AVAILABLE

```
hive_workbench list
hive_workbench run <scenario-name>
hive_workbench run-file <path>
hive_workbench suite
hive_workbench validate-scenarios
hive_workbench summarize <receipts-path>
hive_workbench report scenarios
hive_workbench golden-preview <scenario-name>
hive_workbench regression
```

Output modes: `--output human|json|quiet`

Shortcuts: `healthy`, `below`, `exact`, `breach`

## KNOWN BLOCKERS

None.

## NEXT SAFE WORK

Refer to `docs/FUTURE_ROADMAP_PARKING_LOT.md` for allowed future ideas.

See `ROADMAP_CLOSEOUT.md` for the completed roadmap summary.

## DO NOT ADD

Do not add Tokio, HTTP server, async runtime, databases, persistent queues, routers, LLM inference, python bindings/PyO3, GPU logic, or CI/CD systems.

## VALIDATION BASELINE

- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace` (9 unit tests + 1 compile test + 23 regression tests = 33 total)

Optional tests:
- `.\scripts\validate.ps1 -Bench` (benchmarks — advisory)
- `.\scripts\validate.ps1 -Demo` (scenario demo runs)
- `.\scripts\validate.ps1 -Stress` (100K worker stress test)
