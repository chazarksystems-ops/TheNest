# SLICE_03_04 — Add Serde Support For Receipts

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

Add serde serialization/deserialization derives for `EpigeneticPayload` and related receipt-facing types.

This slice is responsible for serde support only. JSON serialization tests belong to `SLICE_03_05_JSON_SERIALIZATION_TEST`.

Success condition:

```text
Receipt-facing types derive Serialize/Deserialize where needed, existing tests still pass, and this slice does not add serde_json tests or file output.
```

## Relevant current files

```text
swarm_core/Cargo.toml
swarm_core/src/payload.rs
swarm_core/src/lib.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/Cargo.toml
swarm_core/src/payload.rs
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
- add `serde_json`
- add JSON serialization tests
- add receipt file writer
- add JSONL output
- add CLI
- change lifecycle behavior
- change scoring math

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

1. Add `serde = { version = "1", features = ["derive"] }` to dependencies if not already present.
2. Derive `Serialize` and `Deserialize` for `EpigeneticPayload` and any field types that require it.
3. Preserve existing behavior.
4. Do not add `serde_json` in this slice.
5. Do not add file output.

## Tests

1. Existing payload/lifecycle tests still compile and pass.
2. No JSON serialization test is added in this slice.

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
SLICE_03_04:
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
