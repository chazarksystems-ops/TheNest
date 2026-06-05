# Current Context Card

## PROJECT

`TheNest`

## CURRENT STATUS

P2: Evidence + Regression Harness fully completed. Scenario regression integration tests, deterministic golden receipt tests, CLI output marker validation tests, and property checks have been implemented. All validation modes are active and verified. P3 planning cleanup complete.

## LAST COMPLETED SLICE

`SLICE_P3_00_CONTEXT_CARD_CLEANUP`

## COMPLETED SLICES

- `SLICE_P3_00_CONTEXT_CARD_CLEANUP` (docs: align roadmap queue with P3 workbench plan)
- `SLICE_P2_01_GOLDEN_SNAPSHOTS` (Deterministic golden output tests in tests/golden)
- `SLICE_P2_02_SCENARIO_REGRESSION` (Automated outcome regression checks in regression_tests.rs)
- `SLICE_P2_03_CLI_STABILITY` (CLI marker output and JSON payload parser tests)
- `SLICE_P2_04_PROPERTY_CHECKS` (Deterministic parameter grids for monotonicity and validation)
- `SLICE_P2_05_BENCH_REGRESSION_DOCS` (Created P2 manual regression documentation and comparison template)
- `SLICE_P1_01_BENCHMARK_BASELINE` (Criterion benchmarks for calculations, serialized, and tick)
- `SLICE_P1_02_SCENARIO_FIXTURES` (Deterministic scenario loader and JSON schemas)
- `SLICE_P1_03_RECEIPT_FILE_SINK` (Local receipt writer to receipts/out)
- `SLICE_P1_04_SCENARIO_RUNNER_CLI` (demo binary CLI with named scenarios)
- `SLICE_P1_05_BATCH_STRESS_TEST` (batch_stress harness up to 100k workers)
- `SLICE_P1_06_VALIDATION_SCRIPT_OPT` (optional flag modes in validate scripts)
- `SLICE_P1_07_DOCS_CONTEXT_UPDATE` (updated documentation and context cards)
- `SLICE_09_01_PACKAGING_CLEANUP`
- `SLICE_00_01_PERSONAL_TOOL_BOUNDARY`
- `SLICE_00_02_DO_NOT_DRIFT`
- `SLICE_00_03_CURRENT_STATE_AUDIT`
- `SLICE_02_02_RENAME_COORDINATION_WEIGHT`
- `SLICE_02_01_SPLIT_CONFIG_AND_METRICS`
- `SLICE_02_03_INVALID_NUMERIC_INPUT_VALIDATION`
- `SLICE_03_02_ADD_RAW_METRICS_TO_PAYLOAD`
- `SLICE_03_01_ADD_TERMINATION_REASON`
- `SLICE_03_04_ADD_SERDE_RECEIPTS`
- `SLICE_03_05_JSON_SERIALIZATION_TEST`
- `SLICE_05_03_VALIDATE_SCRIPTS`
- `SLICE_00_04_ADD_OPS_CONTEXT_FILES`

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
- `lib`: containing `Scenario` struct and loading helper `Scenario::from_json`.

Binary utility tools:
- `demo`: CLI tool executing single named scenario from file and writing receipts.
- `batch_stress`: harness executing 100k ticks with/without serialization under load.

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

## KNOWN BLOCKERS

None.

## NEXT 3 SLICES

1. `SLICE_P3_01_CLI_ARG_SPEC` (CLI argument specification document)
2. `SLICE_P3_02_WORKBENCH_BINARY_DECISION` (Evolve/rename demo or create hive_workbench)
3. `SLICE_P3_03_MANUAL_CLI_PARSER` (Implement manual command parser)

## DO NOT ADD

Do not add Tokio, HTTP server, async runtime, databases, persistent queues, routers, vLLM/llama.cpp inference, python bindings/PyO3, GPU logic, or CI/CD systems.

## VALIDATION BASELINE

- `cargo fmt --all --check`
- `cargo check --workspace`
- `cargo test --workspace`
Optional tests:
- `.\scripts\validate.ps1 -Bench` (benchmarks)
- `.\scripts\validate.ps1 -Demo` (single runs)
- `.\scripts\validate.ps1 -Stress` (stress test)
