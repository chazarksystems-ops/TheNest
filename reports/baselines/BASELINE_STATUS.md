# Baseline Status Report

Generated: 2026-06-05

## Test Suite Status

- Unit tests: PASS (9 tests)
- Regression tests: PASS (23 tests)
- Compile tests: PASS (1 test)

## Benchmark Reports

- Location: `reports/benchmarks/`
- Run: `cargo bench`
- Status: Advisory (not a gate)

## Stress Tests

- Location: `reports/stress/`
- Run: `.\scripts\validate.ps1 -Stress`
- Status: PASS (100K worker run)

## Golden Receipt Status

- Location: `swarm_core/tests/golden/`
- Files: `golden_receipt_exact_threshold.json`
- Status: VERIFIED — matches current nociceptor output for exact threshold scenario

## CLI Stability Status

- Binary: `hive_workbench`
- Commands: `list`, `run`, `run-file`, `suite`, `validate-scenarios`, `summarize`, `report`, `golden-preview`, `regression`
- Output modes: `human`, `json`, `quiet`
- Status: STABLE — all P3–P6 regression tests pass
