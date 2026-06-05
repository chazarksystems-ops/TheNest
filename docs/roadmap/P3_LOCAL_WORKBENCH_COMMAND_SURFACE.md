# P3 — Local Workbench Command Surface

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

Make the existing scenario/demo functionality feel like a small local workbench from the command line without turning it into a runtime, server, scheduler, or queue.

## Required Preflight Audit

Before editing behavior, inspect and briefly record the current P2 state:

```text
current binaries
current scenario JSON shape
current receipt output shape
current validate.ps1 flags
current reports/ directory
current README/context card claims
```

Create or update:

```text
reports/audits/P3_PREFLIGHT_AUDIT.md
```

Keep this audit concise.

## Deliverables

### 1. Normalize the workbench command surface

Preferred binary name:

```text
hive_workbench
```

Acceptable alternative:

```text
scenario_runner
```

If the current `demo` binary is already clean enough, either:

```text
evolve demo into the workbench binary
```

or:

```text
add hive_workbench and leave demo as a thin compatibility path
```

Do not maintain two overlapping CLIs unless there is a clear reason.

### 2. Required commands

Support a practical command set:

```powershell
cargo run --bin hive_workbench -- list
cargo run --bin hive_workbench -- run healthy
cargo run --bin hive_workbench -- run below
cargo run --bin hive_workbench -- run exact
cargo run --bin hive_workbench -- run breach
cargo run --bin hive_workbench -- run-file scenarios/worker_threshold_breach.json
cargo run --bin hive_workbench -- suite
```

If the binary remains `demo`, document the equivalent commands clearly.

### 3. Output modes

Support these output modes if practical:

```text
human
json
quiet
```

Example commands:

```powershell
cargo run --bin hive_workbench -- run exact --output human
cargo run --bin hive_workbench -- run exact --output json
```

Human output should be readable. JSON output should be machine-readable and stable enough for tests.

### 4. Scenario listing

The list command should show:

```text
scenario name
path
expected outcome if declared
brief description if declared
```

### 5. Suite runner

The suite command should run all valid scenarios and print a concise result table:

```text
scenario | expected | actual | score | threshold | status
```

### 6. Preserve current regression coverage

Update CLI stability tests to target the new command surface, or document why `demo` remains the tested CLI.

## Tests

Add or update tests for:

```text
list command succeeds
run exact succeeds and terminates
run below succeeds and survives
suite command succeeds
json output parses when requested
no server/runtime/network behavior exists
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

Optional if still fast:

```powershell
.\scripts\validate.ps1 -Stress
```

## Documentation Updates

Update:

```text
README.md
docs/QUICKSTART.md if already present
ops/CURRENT_CONTEXT_CARD.md
ops/COMPLETED_SLICES.md
```

## Success Criteria

```text
workbench command surface exists
scenario listing works
single-scenario execution works
suite execution works
human/json/quiet modes exist or are documented as deferred
validation passes
no forbidden runtime/server/database/network dependency was added
```

## Final Report Section

Report:

```text
P3 STATUS:
COMMANDS ADDED:
FILES CREATED:
FILES CHANGED:
TESTS ADDED/UPDATED:
VALIDATION:
DRIFT CHECK:
NOTES:
```
