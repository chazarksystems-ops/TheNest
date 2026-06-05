# TheNest

**A local personal Rust workbench for scenario-driven worker lifecycle simulation.**

TheNest is a single-binary, synchronous, file-based Rust tool for authoring and running "worker health" scenarios. It simulates a nociceptor-based apoptosis model: workers accumulate a suffering score from weighted health metrics, and terminate when the score crosses a configurable threshold.

---

## What This Is

- A **Rust learning and simulation workbench** for the nociceptor/apoptosis model
- A **scenario runner** for named and file-based worker health scenarios
- A **receipt generator** for terminated workers (JSON epigenetic payloads)
- A **regression and golden test system** for deterministic output verification
- A **personal local tool** — runs synchronously, no server, no scheduler, no database

---

## What This Is Not

- Not a production system
- Not a scheduler or task queue
- Not a server or REST API
- Not a database
- Not an async/Tokio runtime
- Not a model-serving or LLM inference system
- Not a distributed swarm

---

## Quickstart

```powershell
# Validate the project
.\scripts\validate.ps1

# List available scenarios
cargo run --bin hive_workbench -- list

# Run a scenario
cargo run --bin hive_workbench -- run healthy
cargo run --bin hive_workbench -- run breach

# Run all scenarios as a suite
cargo run --bin hive_workbench -- suite

# Generate evidence report
cargo run --bin hive_workbench -- report scenarios
```

See [docs/QUICKSTART.md](docs/QUICKSTART.md) for the full command reference.

---

## Directory Map

```text
TheNest/
  Cargo.toml                   # Workspace root
  swarm_core/                  # Core library + binaries
    src/
      lib.rs                   # Scenario, Nociceptor, Worker types
      config.rs                # NociceptorConfig
      metrics.rs               # WorkerHealthMetrics
      nociceptor.rs            # Suffering score calculation
      apoptosis.rs             # Apoptosis trait
      worker.rs                # CattleWorker lifecycle
      reason.rs                # TerminationReason enum
      payload.rs               # EpigeneticPayload receipt type
      receipt_sink.rs          # Receipt file writing
    src/bin/
      hive_workbench.rs        # Main CLI binary
      batch_stress.rs          # Stress test runner
    tests/
      regression_tests.rs      # 23 regression tests
      compile_tests.rs         # Trybuild compile-fail test
      golden/                  # Deterministic golden receipt files
    benches/                   # Criterion benchmarks

  scenarios/                   # Scenario JSON files
  receipts/out/                # Generated termination receipts (gitignored)
  reports/
    evidence/                  # Evidence reports (Markdown)
    baselines/                 # Baseline status docs
    benchmarks/                # Benchmark output
    audits/                    # Final audit docs
    handoff/                   # Handoff manifest

  docs/                        # Design and policy docs
  scripts/                     # validate.ps1, package_handoff.ps1
  ops/                         # Context cards and slice queue
  agent/                       # Task and microtask files
  reference/                   # Reference documents
```

---

## Core Rust Concepts

| Type | Purpose |
|---|---|
| `NociceptorConfig` | Weights (alpha, beta, gamma) and termination threshold |
| `WorkerHealthMetrics` | Live metrics: context_bloat, error_rate, coordination_debt |
| `Nociceptor` | Computes `score = alpha*bloat + beta*error + gamma*debt` |
| `CattleWorker` | Executes one tick: survive or trigger apoptosis |
| `EpigeneticPayload` | Hardened receipt emitted on termination |
| `TerminationReason` | Enum: ThresholdBreach |
| `Apoptosis` | Trait: consumes self, yields receipt |

---

## Scenario Workbench

Scenarios are JSON files in `scenarios/`. Each defines a worker configuration and metrics.

```json
{
  "scenario_name": "worker_survives",
  "description": "Optional description",
  "expected_outcome": "survived",
  "config": { "alpha": 1.0, "beta": 1.0, "gamma": 1.0, "threshold": 10.0 },
  "metrics": { "context_bloat": 2.0, "error_rate": 1.0, "coordination_debt": 1.0 }
}
```

### CLI Commands

| Command | What it does |
|---|---|
| `list` | List all available scenario files |
| `run <name>` | Run a named or shortcut scenario |
| `run-file <path>` | Run a scenario from a file path |
| `suite` | Run all scenarios, print result table |
| `validate-scenarios` | Check expected_outcome matches actual |

Shortcuts: `healthy`, `below`, `exact`, `breach`

---

## Receipts and Reports

When a worker terminates, a receipt is written to:
```
receipts/out/<scenario_name>_receipt.json
```

Receipts are not committed (listed in `.gitignore`).

```powershell
# Summarize receipts
cargo run --bin hive_workbench -- summarize receipts/out

# Generate Markdown evidence report
cargo run --bin hive_workbench -- report scenarios
```

Evidence report written to: `reports/evidence/SCENARIO_EVIDENCE_REPORT.md`

---

## Regression and Golden Tests

Golden files are in `swarm_core/tests/golden/`. They record the expected deterministic receipt output for specific scenarios (using `Uuid::nil()` for reproducibility).

Tests compare against golden files — they never overwrite them.

```powershell
# Preview what a golden receipt would look like (no write)
cargo run --bin hive_workbench -- golden-preview exact --output json

# Run full regression check
cargo run --bin hive_workbench -- regression
```

See [docs/P6_GOLDEN_UPDATE_POLICY.md](docs/P6_GOLDEN_UPDATE_POLICY.md) for the update policy.

---

## Validation

```powershell
.\scripts\validate.ps1            # Format + check + test
.\scripts\validate.ps1 -Demo     # Also run scenario demo
.\scripts\validate.ps1 -Stress   # Also run batch stress (100K workers)
.\scripts\validate.ps1 -Bench    # Also run Criterion benchmarks (advisory)
```

---

## Benchmarks

```powershell
cargo bench
```

Results are advisory — they are not a required gate for commits.

Historical benchmarks at: `reports/benchmarks/`

---

## Packaging

```powershell
.\scripts\package_handoff.ps1
```

Output: `C:\Users\cheez\Downloads\TheNest_handoff.zip`

Excludes `target/`, `receipts/out/*`, and zip files from the archive.

---

## Next Safe Work

See [docs/FUTURE_ROADMAP_PARKING_LOT.md](docs/FUTURE_ROADMAP_PARKING_LOT.md) for allowed future ideas and permanently forbidden scope.

The full roadmap (P0–P8) is complete. See [ROADMAP_CLOSEOUT.md](ROADMAP_CLOSEOUT.md).
