# SLICE_02_01 — Split Nociceptor Config And Worker Health Metrics

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

Separate static scoring weights/threshold from live worker health metrics.

Success condition:

```text
Nociceptor construction uses distinct config and metrics types while existing scoring behavior remains unchanged.
```

## Relevant current files

```text
swarm_core/src/nociceptor.rs
swarm_core/src/lib.rs
swarm_core/src/worker.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/src/nociceptor.rs
swarm_core/src/lib.rs
swarm_core/src/worker.rs
```

## Allowed new files

```text
swarm_core/src/config.rs
swarm_core/src/metrics.rs
```

## Forbidden work

Do not:

- redesign the project
- turn this into a full HIVE runtime
- ask for review for routine local choices
- change scoring math
- change lifecycle semantics
- add serialization unless already present
- add TOML loading

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

1. Add `NociceptorConfig` with alpha, beta, gamma, threshold.
2. Add `WorkerHealthMetrics` with context_bloat, error_rate, coordination_debt.
3. Update `Nociceptor` to use the split types while preserving public behavior as much as practical.
4. Re-export new types from `lib.rs`.
5. Update affected tests.

## Tests

1. same input values produce same suffering score as before
2. terminal/sub-threshold/exact-threshold behavior remains unchanged
3. worker lifecycle tests still pass

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
SLICE_02_01:
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
