# SLICE_02_02 — Rename Latency Weight To Coordination Debt Weight

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

Rename the misleading `latency_weight` payload field to `coordination_debt_weight` because gamma is applied to `coordination_debt`, not latency.

Success condition:

```text
No `latency_weight` field remains, tests pass, and payload terminology matches the scoring metric.
```

## Relevant current files

```text
swarm_core/src/payload.rs
swarm_core/src/worker.rs
swarm_core/src/lib.rs
swarm_core/src/nociceptor.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/src/payload.rs
swarm_core/src/worker.rs
swarm_core/src/lib.rs
swarm_core/src/nociceptor.rs
README.md
receipts/KIMI_P0_RECEIPT.md
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
- change threshold behavior
- rename gamma
- add serde
- add config objects

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

1. Rename payload field `latency_weight` to `coordination_debt_weight`.
2. Update constructor and usage sites.
3. Update tests that reference the old field.
4. Update docs/receipt wording if the old name appears.
5. Do not change the formula.

## Tests

1. terminated payload exposes `coordination_debt_weight`
2. payload still preserves alpha, beta, and gamma values correctly
3. existing lifecycle tests still pass

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
SLICE_02_02:
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
