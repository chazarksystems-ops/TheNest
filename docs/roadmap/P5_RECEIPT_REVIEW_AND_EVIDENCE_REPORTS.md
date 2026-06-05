# P5 — Receipt Review + Evidence Reports

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

Make generated receipts easy to inspect, summarize, and turn into evidence reports without creating a ledger, database, event store, queue, or runtime.

## Hard Boundary

Receipt review must stay file-only.

Allowed:

```text
read JSON files from receipts/out/
write Markdown reports under reports/evidence/
summarize receipt fields
```

Forbidden:

```text
ledger crate
database
event store
queue
scheduler
server
runtime
network service
```

## Deliverables

### 1. Improve receipt output organization

Generated receipts should go under:

```text
receipts/out/
```

Use predictable filenames when possible:

```text
receipts/out/worker_exact_threshold_terminated.json
receipts/out/worker_threshold_breach_terminated.json
```

Avoid random filenames unless necessary.

Generated receipt outputs should remain ignored by Git/package scripts, except for placeholder files like `.gitkeep` if needed.

### 2. Add receipt summary command

Workbench command:

```powershell
cargo run --bin hive_workbench -- summarize receipts/out
```

Summary should show:

```text
receipt count
terminated count
average score
min score
max score
termination reasons
scenario IDs if available
```

### 3. Add evidence report command

Workbench command:

```powershell
cargo run --bin hive_workbench -- report scenarios
```

Generate:

```text
reports/evidence/SCENARIO_EVIDENCE_REPORT.md
```

The report should include:

```text
scenario table
expected vs actual outcome
scores and thresholds
receipt output path
validation status
```

### 4. Add receipt evidence docs

Create:

```text
docs/P5_RECEIPT_EVIDENCE.md
```

Explain:

```text
receipt fields
stable machine-readable fields
human-readable fields
how to compare receipts
where receipts are written
why this is not a ledger/database/event store
```

## Tests

Add or update tests for:

```text
receipt filenames are predictable
receipt summary handles empty directory
receipt summary handles generated receipts
evidence report command creates expected Markdown
generated report contains expected scenario rows
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

Clean generated receipts after validation before packaging:

```powershell
Remove-Item -Path "receipts\out\*" -Force -ErrorAction SilentlyContinue
```

## Documentation Updates

Update:

```text
README.md
docs/P5_RECEIPT_EVIDENCE.md
ops/CURRENT_CONTEXT_CARD.md
ops/COMPLETED_SLICES.md
```

## Success Criteria

```text
receipt writing still works
receipt summary command works
evidence report command works
reports/evidence/ generated cleanly
receipts/out/ remains excluded from handoff
no ledger/database/runtime/queue was added
```

## Final Report Section

Report:

```text
P5 STATUS:
COMMANDS ADDED:
RECEIPT OUTPUT SHAPE:
REPORTS ADDED:
FILES CREATED:
FILES CHANGED:
TESTS:
VALIDATION:
DRIFT CHECK:
NOTES:
```
