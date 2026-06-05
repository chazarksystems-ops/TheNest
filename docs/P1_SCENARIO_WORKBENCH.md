# P1 Scenario Workbench

The **Scenario Workbench** is a lightweight local development harness for executing, inspecting, benchmarking, and stress-testing the HIVE lifecycle primitives (`swarm_core`).

It enables deterministic scenario execution and analysis without adding full runtime orchestration, databases, queues, or async frameworks.

---

## 1. Scenario Definition Format

Scenarios are defined in JSON format. They specify a test case name, the static configuration for the nociceptor, and the initial health metrics of the worker.

Example (`scenarios/worker_survives.json`):
```json
{
  "scenario_name": "worker_survives",
  "config": {
    "alpha": 1.0,
    "beta": 1.0,
    "gamma": 1.0,
    "threshold": 10.0
  },
  "metrics": {
    "context_bloat": 2.0,
    "error_rate": 1.0,
    "coordination_debt": 1.0
  }
}
```

### Pre-defined Scenarios:
- **`worker_survives`**: A healthy worker whose pain score is below the threshold.
- **`worker_exact_threshold`**: A borderline worker whose pain score is exactly equal to the threshold (triggers apoptosis / terminal path).
- **`worker_just_below_threshold`**: A borderline worker whose pain score is just below the threshold (survival path).
- **`worker_threshold_breach`**: An unhealthy worker whose pain score exceeds the threshold, triggering apoptosis.
- **`worker_invalid_negative_metric`**: An invalid scenario containing negative metric values, rejected during deserialization.
- **`worker_invalid_nan_metric`**: An invalid scenario containing `NaN` values, rejected during deserialization.

---

## 2. Scenario Runner CLI

The CLI loads scenarios, executes a single worker tick, and outputs structured termination receipts when apoptosis occurs.

### Usage:
```powershell
# Run using preset scenario names
cargo run --bin demo -- healthy
cargo run --bin demo -- breach
cargo run --bin demo -- exact
cargo run --bin demo -- below

# Run using a custom file path
cargo run --bin demo -- scenarios/worker_survives.json
```

### Output Behavior:
- **Survived Workers**: Prints details and the calculated suffering score.
- **Terminated Workers**: Prints the suffering score and details, writes the structured termination receipt to the `receipts/out/` folder in JSON format, and outputs the pretty-printed receipt JSON to stdout.

---

## 3. Batch Stress Harness

The batch stress runner evaluates the raw performance of the `swarm_core` lifecycles at scale.

### Usage:
```powershell
cargo run --release --bin batch_stress
```

### Measured Sizes:
- **100** workers (memory-only & serialized)
- **1,000** workers (memory-only & serialized)
- **10,000** workers (memory-only & serialized)
- **100,000** workers (memory-only & serialized)

For each size, the harness executes the worker ticks synchronously, serializing a configured 25% termination cohort, and prints total elapsed time and throughput statistics.
