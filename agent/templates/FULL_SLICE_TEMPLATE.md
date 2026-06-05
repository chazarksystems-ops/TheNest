# SLICE_ID — SLICE_TITLE

## 1. Mission

You are working on a small bounded slice of the `kimi_hive_p0_test` Rust workspace.

This project is a P0 Rust lifecycle primitive proving:

```text
worker health metrics
-> threshold breach
-> ownership-consuming worker termination
-> receipt payload emitted
```

Your job is to complete only this slice. Do not redesign the project.

## 2. Personal tool doctrine

This is a quick personal-tool development project, not an engineer-grade production build.

Do not over-engineer.
Do not add unnecessary process.
Do not request reviews for routine local changes.
Proceed with the assigned slice unless there is a true blocker, conflicting instruction, major architecture fork, or unsafe/destructive action.

## 3. Current project shape

Workspace:

```text
kimi_hive_p0_test/
  Cargo.toml
  Cargo.lock
  README.md

  swarm_core/
    Cargo.toml
    src/
      lib.rs
      nociceptor.rs
      worker.rs
      payload.rs
      apoptosis.rs

  receipts/
    KIMI_P0_RECEIPT.md
```

Core concepts:

- `Nociceptor` calculates suffering score.
- `CattleWorker` owns a `Nociceptor`.
- `CattleWorker::tick(self)` consumes the worker.
- `WorkerOutcome::Survived(CattleWorker)` returns a surviving worker.
- `WorkerOutcome::Terminated(EpigeneticPayload)` emits a receipt when the worker terminates.
- `Apoptosis::trigger_apoptosis(self)` consumes the worker and creates the payload.
- `EpigeneticPayload` is the termination receipt.

## 4. Slice objective

Complete this specific change:

```text
[Describe the exact task here.]
```

This slice is successful when:

```text
[Describe the exact observable success condition.]
```

## 5. Allowed files

You may edit only these files:

```text
[LIST FILES HERE]
```

You may create only these files:

```text
[LIST NEW FILES HERE]
```

## 6. Forbidden work

Do not:

- add async runtime
- add Tokio
- add HTTP server
- add database
- add model serving
- add GPU logic
- add vLLM, Axolotl, PyO3, or Python integration
- add networking
- add worker scheduling
- add a swarm runtime
- rewrite unrelated modules
- rename public concepts unless this slice explicitly says to
- modify generated build artifacts
- modify `target/`
- create unnecessary review gates
- ask for review unless truly blocked

## 7. Required implementation details

Make the following exact changes:

1. `[Step one]`
2. `[Step two]`
3. `[Step three]`

Keep the change minimal and local.

## 8. Required tests

Add or update tests proving:

1. `[Test expectation one]`
2. `[Test expectation two]`
3. `[Test expectation three]`

Do not remove existing tests unless the slice explicitly requires replacing them.

## 9. Validation commands

Run these commands from the workspace root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

All should pass. If one fails, fix locally if the fix is inside this slice.

## 10. Expected final report

```text
SLICE_ID:
STATUS: PASS / BLOCKED / PARTIAL

CHANGED:
- file/path
- file/path

CREATED:
- file/path
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

## 11. Stop condition

Stop after this slice is complete.

Do not continue into adjacent slices.
Do not perform cleanup outside the allowed files.
Do not invent follow-up architecture.

## Relevant-file rule

Relevant files are context only. They are read-only unless also listed under `Allowed edits` or `Allowed new files`.
