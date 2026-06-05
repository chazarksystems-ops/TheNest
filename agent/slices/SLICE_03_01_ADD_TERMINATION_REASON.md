# SLICE_03_01 — Add Structured TerminationReason

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

Add a machine-readable `TerminationReason` enum while preserving the human-readable fault signature.

Success condition:

```text
Terminated payload includes `TerminationReason::ThresholdBreach` and existing tests pass.
```

## Relevant current files

```text
swarm_core/src/payload.rs
swarm_core/src/worker.rs
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
swarm_core/src/reason.rs
```

## Forbidden work

Do not:

- redesign the project
- turn this into a full HIVE runtime
- ask for review for routine local choices
- remove fault_signature unless explicitly required
- add broad policy engine
- add multiple unused reasons unless useful

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

1. Create `TerminationReason` enum with at least `ThresholdBreach`.
2. Add a reason field to `EpigeneticPayload`.
3. Set reason to `ThresholdBreach` during apoptosis.
4. Re-export the enum from `lib.rs`.
5. Update tests.

## Tests

1. terminated worker payload reason is `ThresholdBreach`
2. fault_signature remains available for human reading
3. existing lifecycle behavior remains unchanged

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
SLICE_03_01:
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
