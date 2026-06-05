# SLICE_03_02 — Add Raw Metrics To EpigeneticPayload

## Mission

You are working on one bounded slice of the `kimi_hive_p0_test` Rust workspace.

Project context:

```text
P0 Rust lifecycle primitive:
worker health metrics -> threshold breach -> ownership-consuming worker termination -> receipt payload emitted
```

## Personal tool doctrine

This is for quick development of a personal tool, not an engineer-grade production build.

Do not over-engineer.
Do not add unnecessary process.
Do not ask for routine reviews.
Proceed with this slice unless there is a true blocker, conflicting instruction, major architecture fork, or unsafe/destructive action.

## Objective

Add raw health metrics and threshold to `EpigeneticPayload` so the receipt fully explains its own final score.

Success condition:

```text
Terminated payload includes context_bloat, error_rate, coordination_debt, threshold, and final suffering score.
```

## Relevant current files

```text
swarm_core/src/payload.rs
swarm_core/src/worker.rs
swarm_core/src/nociceptor.rs
swarm_core/src/lib.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/src/payload.rs
swarm_core/src/worker.rs
swarm_core/src/lib.rs
```

## Allowed new files

```text
none
```

## Forbidden work

Do not:

- redesign the project
- turn this into a full HIVE runtime
- ask for review for routine local choices
- change scoring math
- change terminal threshold behavior
- add receipt writer
- add CLI

Global forbidden work:

- add runtime
- add networking
- add model serving
- add async orchestration
- add database
- add scheduler
- add queue
- add GPU logic
- create unnecessary review gates
- edit `target/` or generated build artifacts
- continue into adjacent slices

## Required steps

1. Add payload fields for `context_bloat`, `error_rate`, `coordination_debt`, and `threshold`.
2. Populate those fields during apoptosis.
3. Update tests and any direct constructors.
4. Keep existing final score and weight fields.

## Tests

1. terminal payload includes same raw metrics held by the worker before termination
2. terminal payload includes threshold
3. final score remains correct

## Validation

Run from workspace root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

If a validation failure is local to this slice, fix it. If the fix requires editing outside allowed files, report `BLOCKED`.

## Final report format

```text
SLICE_03_02:
STATUS: PASS / BLOCKED / PARTIAL

CHANGED:
- file/path

CREATED:
- file/path

VALIDATION:
- cargo fmt --all --check: PASS/FAIL/NOT RUN
- cargo check --workspace: PASS/FAIL/NOT RUN
- cargo test --workspace: PASS/FAIL/NOT RUN

SUMMARY:
Short explanation of what was completed.

BLOCKER:
Only include if truly blocked.

DRIFT CHECK:
Confirm no runtime, networking, model-serving, async orchestration, database, scheduler, queue, or unrelated architecture was added.
```

## Stop condition

Stop after this slice is complete.
Do not continue into neighboring slices.
