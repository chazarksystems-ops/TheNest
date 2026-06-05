# SLICE_03_05 — Add JSON Serialization Test

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

Add a focused test proving that a terminated worker receipt can be serialized into stable JSON. This slice depends on `SLICE_03_04_ADD_SERDE_RECEIPTS` being complete.

Success condition:

```text
A test covers worker.tick() -> Terminated(payload) -> serde_json serialization.
```

## Relevant current files

```text
swarm_core/src/lib.rs
swarm_core/src/worker.rs
swarm_core/src/payload.rs
swarm_core/Cargo.toml
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/src/lib.rs
swarm_core/src/worker.rs
swarm_core/src/payload.rs
swarm_core/Cargo.toml
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
- add file writer
- add CLI
- change public behavior just to satisfy test

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

1. Confirm `SLICE_03_04_ADD_SERDE_RECEIPTS` is complete or report `BLOCKED`.
2. Ensure `serde_json` is available as a dev-dependency if needed.
3. Add a test that constructs a terminal worker.
4. Call `tick()`.
5. Serialize the payload with `serde_json`.
6. Assert key fields appear in the JSON.

## Tests

1. terminated payload serializes to JSON
2. JSON includes final_suffering_score
3. JSON includes worker_id
4. test does not write files

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
SLICE_03_05:
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
