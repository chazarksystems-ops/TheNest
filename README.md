# Kimi/HIVE P0 Lifecycle Primitive

This project is a P0 personal-tool Rust lifecycle primitive proving:

```text
worker health metrics
-> suffering score calculation
-> threshold breach detection
-> ownership-consuming worker termination
-> termination receipt emitted
```

## What This Proves

- **Compile-Enforced Apoptosis**: Worker instances are consumed by value using Rust's ownership system during terminal transitions. It is a compile-time impossibility to reuse or re-schedule a terminated agent.
- **Dynamic Pain Metric Evaluation**: Live pain score computation using the formula:
  `score = alpha * context_bloat + beta * error_rate + gamma * coordination_debt`
- **Hardened Epigenetic Graveyard Receipts**: Terminated outcomes serialize cleanly to stable JSON with all raw metrics, config weights, and structured reasons.

## What This Does NOT Prove (Explicitly Excluded)

- **Orchestration / Scheduling**: No async runtimes (Tokio or actor frameworks) manage worker loops.
- **Model Inference**: No LLM connections (local or remote), vLLM instances, or tokenizer logic are implemented.
- **Python / GPU Integration**: No PyO3, Python scripts, PyTorch, or CUDA bindings exist.
- **Enterprise Middleware**: No database, queue, HTTP routing, or network systems are present.

## Directory Structure

```text
TheNest/
  Cargo.toml          # Workspace manifest
  .gitignore          # Git exclusions
  README.md           # This document

  swarm_core/         # Core lifecycle primitive crate
    Cargo.toml
    src/
      lib.rs          # Export structures and baseline tests
      config.rs       # NociceptorConfig with presets and checks
      metrics.rs      # WorkerHealthMetrics for runtime metrics
      nociceptor.rs   # Suffering calculator
      reason.rs       # TerminationReason enum
      payload.rs      # EpigeneticPayload receipt data
      apoptosis.rs    # Apoptosis move trait
      worker.rs       # CattleWorker and tick transition
    tests/
      regression_tests.rs # Golden tests, CLI markers, & property scoring
      golden/
        golden_receipt_exact_threshold.json # Fixed UUID golden output

  docs/               # Personal-tool doctrines and boundaries
  ops/                # Low-context dispatch queues and context card
  agent/agent/skills/             # Small task-runner execution scripts
  agent/agent/slices/             # Slice specifications
  agent/agent/templates/          # Spec templates
  reference/          # Historical Grok/Gemini design logs & schemas
  scripts/            # Validation scripts (validate.sh, validate.ps1)
```

## How to Validate

Run the validation suite directly from the workspace root:

### Windows PowerShell
```powershell
# Run default light correctness check (check + format + tests)
.\scripts\validate.ps1

# Run with optional benchmarks
.\scripts\validate.ps1 -Bench

# Run with optional scenario workbench CLI runner demo
.\scripts\validate.ps1 -Demo

# Run with optional batch stress harness
.\scripts\validate.ps1 -Stress
```

### Linux / Unix Shell
```bash
chmod +x ./scripts/validate.sh

# Run default light correctness check
./scripts/validate.sh

# Run optional checks
./scripts/validate.sh --bench
./scripts/validate.sh --demo
./scripts/validate.sh --stress
```

---

## P1 Scenario Workbench & CLI Usage

P1 turns the core library primitives into a local developer workbench under `swarm_core`.

### 1. Run Named Scenarios
You can run deterministic scenarios using the demo runner. Surviving outcomes print live scores, while terminated outcomes output a structured `EpigeneticPayload` receipt and write it to `receipts/out/<worker-id>_receipt.json`.

```powershell
# Run a healthy scenario (worker survives)
cargo run --bin demo -- healthy

# Run a breach scenario (worker triggers apoptosis)
cargo run --bin demo -- breach

# Run a borderline threshold scenario
cargo run --bin demo -- exact

# Run a custom JSON scenario path
cargo run --bin demo -- scenarios/worker_survives.json
```

### 2. Run Batch Stress Harness
Measure worker tick and serialization performance under varying cohort scales (from 100 to 100,000 workers):
```powershell
cargo run --release --bin batch_stress
```

### 3. Run Microbenchmarks
Execute the Criterion benchmark suite to measure nociceptor calculation speeds, worker ticking paths, and JSON serialization throughput:
```powershell
cargo bench
```
Visual reports are generated at `target/criterion/report/index.html`.

---

## P2 Evidence & Regression Testing

P2 ensures code behavior, serialization schemas, and CLI commands remain stable and correct over time.

### 1. Golden Receipt Verification
Golden output snapshots are stored under `swarm_core/tests/golden/`. The test executes the exact-threshold scenario using a deterministic Nil UUID, serializes the receipt, and asserts structural equality with [golden_receipt_exact_threshold.json](file:///C:/Users/cheez/Downloads/TheNest/swarm_core/tests/golden/golden_receipt_exact_threshold.json).

### 2. CLI Stability Tests
Tests execute the scenario runner demo via process execution and confirm output status codes, text markers (`Outcome:`, `Suffering score:`), and correct JSON receipt schemas.

### 3. Scoring Property Checks
Grids of metric configurations are evaluated programmatically to verify:
- Monotonicity: Increasing pain parameters never decreases the computed score.
- Range Boundaries: Verify NaN, infinity, and negative values are strictly rejected.

### 4. Running Regression Tests
All regression tests run as part of the default validation suite:
```powershell
cargo test --workspace
```

---

## Drift Boundaries & Exclusions (Strictly Enforced)

To keep the codebase small and focused as a local personal tool, the following remain strictly **excluded** in P1/P2:
- No async runtimes (Tokio or actor frameworks) or network ports/servers.
- No databases, persistent ledgers, queues, or watchers.
- No local or remote LLM connections, GPU bindings, or Python PyO3 bindings.

