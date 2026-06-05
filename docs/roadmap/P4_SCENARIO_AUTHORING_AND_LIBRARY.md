# P4 — Scenario Authoring + Scenario Library

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

Make scenarios easier to create, understand, and extend without touching Rust source.

## Key Rule

Scenario metadata must be backward-compatible.

Support both:

```text
old minimal scenario JSON shape
new metadata-rich scenario JSON shape
```

Do not break existing scenario files.

## Deliverables

### 1. Add optional scenario metadata

Support optional fields like:

```json
{
  "id": "worker_exact_threshold",
  "description": "Exact threshold should terminate.",
  "expected_outcome": "terminated",
  "expected_score": 10.0,
  "config": {
    "alpha": 1.0,
    "beta": 1.0,
    "gamma": 1.0,
    "threshold": 10.0
  },
  "metrics": {
    "context_bloat": 5.0,
    "error_rate": 3.0,
    "coordination_debt": 2.0
  }
}
```

Expected outcomes:

```text
survived
terminated
invalid
```

Invalid scenarios are allowed as test fixtures. They should not be treated as executable success cases.

### 2. Add scenario validation command

Add workbench command:

```powershell
cargo run --bin hive_workbench -- validate-scenarios
```

It should check:

```text
all scenario JSON files parse
all valid scenarios execute
expected outcomes match actual outcomes
invalid scenarios fail for expected reasons
no scenario is missing required config/metric fields
```

### 3. Add scenario authoring template

Create:

```text
scenarios/TEMPLATE.scenario.json
```

It should show every supported field and explain optional fields briefly.

### 4. Add scenario authoring docs

Create:

```text
docs/P4_SCENARIO_AUTHORING.md
```

Include:

```text
scenario file shape
required fields
optional metadata
valid expected outcomes
how to add a new scenario
how to validate scenarios
how to run one scenario
how to run all scenarios
```

### 5. Expand scenario library

Add useful examples:

```text
worker_zero_metrics.json
worker_high_context_only.json
worker_high_error_only.json
worker_high_coordination_only.json
worker_strict_profile_breach.json
worker_lenient_profile_survives.json
invalid_negative_threshold.json
invalid_infinite_metric.json
```

Keep filenames predictable.

## Tests

Add or update tests for:

```text
old minimal scenario files still load
metadata-rich scenario files load
expected_outcome is honored
invalid scenarios fail for expected reasons
validate-scenarios command reports correct status
expanded examples match expected behavior
```

## Validation

Run:

```powershell
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
.\scripts\validate.ps1
.\scripts\validate.ps1 -Demo
```

## Documentation Updates

Update:

```text
README.md
docs/P1_SCENARIO_WORKBENCH.md if needed
docs/P4_SCENARIO_AUTHORING.md
ops/CURRENT_CONTEXT_CARD.md
ops/COMPLETED_SLICES.md
```

## Success Criteria

```text
scenario metadata is parsed
old scenario files still work
scenario template exists
validate-scenarios works
expanded scenario library exists
scenario regression tests pass
no new runtime/server/database/network dependency was added
```

## Final Report Section

Report:

```text
P4 STATUS:
SCENARIOS ADDED:
METADATA SUPPORT:
COMMANDS ADDED:
FILES CREATED:
FILES CHANGED:
TESTS:
VALIDATION:
DRIFT CHECK:
NOTES:
```
