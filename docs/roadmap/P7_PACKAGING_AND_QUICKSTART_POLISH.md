# P7 — Packaging + Quickstart Polish

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

Make the project easy to package, hand off, and restart later.

## Packaging Rules

Include:

```text
source
docs
ops
skills
slices
templates
reference
reports
receipts/.gitkeep
```

Exclude:

```text
target/
receipts/out/*
temporary benchmark output caches if large/generated
```

Do not exclude important Markdown reports.

## Deliverables

### 1. Packaging script

Create:

```text
scripts/package_handoff.ps1
```

Optional:

```text
scripts/package_handoff.sh
```

The PowerShell script should:

```text
exclude target/
exclude receipts/out/*
include receipts/.gitkeep
include source/docs/ops/agent/skills/agent/slices/agent/templates/reference/reports
write zip to C:\Users\cheez\Downloads\TheNest_handoff.zip
verify target/ is not present
verify receipts/out generated files are not present
```

### 2. Handoff manifest

Create or generate:

```text
reports/handoff/HANDOFF_MANIFEST.md
```

Include:

```text
zip path
date
included root folders
excluded generated folders
validation status
packaging command used
```

### 3. Quickstart docs

Create:

```text
docs/QUICKSTART.md
```

Include commands for:

```text
validate project
list scenarios
run a scenario
run the suite
write receipts
summarize receipts
generate evidence report
run regression checks
run benchmarks
package handoff
```

### 4. README final layout

Update README into the main entrypoint.

Sections:

```text
What this is
What this is not
Quickstart
Directory map
Core Rust concepts
Scenario workbench
Receipts and reports
Regression and golden tests
Validation
Benchmarks
Packaging
Next-safe-work
```

### 5. Ops context update

Update:

```text
ops/CURRENT_CONTEXT_CARD.md
ops/COMPLETED_SLICES.md
ops/NEXT_SLICE_QUEUE.md
```

`NEXT_SLICE_QUEUE.md` should not point to risky runtime/scheduler work.

Use:

```text
Next possible roadmap: P8 audit / future planning
```

## Tests

Test the packaging script enough to confirm:

```text
zip exists
target/ excluded
receipts/out generated files excluded
reports included
docs included
source included
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

Then run:

```powershell
.\scripts\package_handoff.ps1
```

## Success Criteria

```text
package script works
handoff zip rebuilt
handoff manifest exists
quickstart exists
README is accurate
current context card is updated
no generated receipts included in zip
target/ excluded
```

## Final Report Section

Report:

```text
P7 STATUS:
PACKAGING SCRIPT:
HANDOFF ZIP:
QUICKSTART:
README:
FILES CREATED:
FILES CHANGED:
VALIDATION:
DRIFT CHECK:
NOTES:
```
