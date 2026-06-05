# P6 — Regression + Baseline Management

## Current Baseline

The project is currently treated as P0, P1, and P2 complete.

Current project identity:

```text
Kimi/HIVE P0 Master is a local Rust personal-tool workbench centered on swarm_core.
```

Current capabilities:

```text
worker health metrics
-> suffering score calculation
-> threshold breach detection
-> ownership-consuming worker termination
-> structured receipt emitted
-> JSON serialization
-> scenario runner
-> receipt file output
-> batch stress harness
-> Criterion benchmark baseline
-> golden regression tests
-> CLI stability tests
-> property-style scoring checks
```

## Standing Boundaries

This phase must not add:

```text
runtime
scheduler
queue
database
HTTP server
Tokio runtime
networking
model-serving
GPU logic
PyO3
CI/CD ceremony
enterprise governance
large framework behavior
```

Dependency discipline:

```text
Prefer Rust std plus existing serde/serde_json.
Do not add clap, anyhow, walkdir, insta, proptest, quickcheck, or similar dependencies unless the benefit is clear, small, and documented.
Manual argument parsing is acceptable for this personal tool.
```

## Goal

Make it easy to know when behavior, receipt shape, CLI output, or performance changes over time.

## Hard Rules

Benchmarks are advisory.

```text
Benchmarks inform decisions.
Benchmarks do not fail default validation.
```

Golden files are intentional.

```text
Golden files are not silently overwritten during tests.
```

Prefer preview-first behavior:

```text
golden-preview = allowed
automatic golden-update = avoid for now
```

## Deliverables

### 1. Baseline metadata file

Create:

```text
reports/baselines/BASELINE_STATUS.md
```

Include:

```text
current baseline date
validated state / commit if available
test suite status
benchmark report path
stress report path
golden receipt status
CLI stability status
```

### 2. Baseline comparison template

Create:

```text
reports/baselines/BASELINE_COMPARISON_TEMPLATE.md
```

Use for manual comparisons after changes.

### 3. Golden update policy

Create:

```text
docs/P6_GOLDEN_UPDATE_POLICY.md
```

State:

```text
Golden files are updated intentionally, not silently.
Tests must not rewrite golden files.
A preview command may show what a new golden would look like.
```

Optional command:

```powershell
cargo run --bin hive_workbench -- golden-preview exact
```

This should print the proposed golden receipt. It should not overwrite the golden file.

### 4. Benchmark comparison docs

Create or update:

```text
docs/P6_BASELINE_COMPARISON.md
```

Include:

```text
how to run cargo bench
how to compare against P1/P2 baseline reports
what changes are concerning
why benchmark regressions are advisory
why default validation does not fail on benchmark numbers
```

### 5. Regression command

Optional workbench command:

```powershell
cargo run --bin hive_workbench -- regression
```

It should run scenario validation and print concise status.

It should not run Criterion benchmarks by default.

## Tests

Add or update tests for:

```text
golden-preview prints valid JSON
golden files are not modified by tests
regression command succeeds if implemented
baseline docs exist
```

Avoid tests that depend on machine-specific benchmark timing.

## Validation

Run:

```powershell
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
.\scripts\validate.ps1
.\scripts\validate.ps1 -Demo
```

Optional, not required for default pass:

```powershell
.\scripts\validate.ps1 -Bench
.\scripts\validate.ps1 -Stress
```

## Documentation Updates

Update:

```text
README.md
docs/P2_BENCHMARK_REGRESSION.md if needed
docs/P6_GOLDEN_UPDATE_POLICY.md
docs/P6_BASELINE_COMPARISON.md
ops/CURRENT_CONTEXT_CARD.md
ops/COMPLETED_SLICES.md
```

## Success Criteria

```text
baseline status docs exist
baseline comparison template exists
golden update policy exists
golden files are not silently overwritten
benchmark comparison remains advisory
regression command or equivalent workflow exists
default validation remains fast
```

## Final Report Section

Report:

```text
P6 STATUS:
BASELINE DOCS:
GOLDEN POLICY:
COMMANDS ADDED:
FILES CREATED:
FILES CHANGED:
TESTS:
VALIDATION:
DRIFT CHECK:
NOTES:
```
