# SLICE_00_01 — Add Personal Tool Boundary Document

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

Create a concise boundary document stating that this is a quick personal-tool P0 lifecycle primitive, not an engineer-grade production build.

Success condition:

```text
docs/PROJECT_BOUNDARY.md exists, README references it, and no Rust behavior changes.
```

## Relevant current files

```text
README.md
swarm_core/src/lib.rs
swarm_core/src/worker.rs
swarm_core/src/nociceptor.rs
swarm_core/src/payload.rs
swarm_core/src/apoptosis.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
README.md
```

## Allowed new files

```text
docs/PROJECT_BOUNDARY.md
```

## Forbidden work

Do not:

- redesign the project
- turn this into a full HIVE runtime
- ask for review for routine local choices
- edit Rust source code
- add dependencies
- add tests

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

1. Create `docs/PROJECT_BOUNDARY.md`.
2. State project identity and personal-tool doctrine.
3. List implemented concepts.
4. List non-goals: runtime, scheduler, queue, model server, router, database, distributed system.
5. Update README with a short reference.

## Tests

No Rust tests required. Run default validation if possible.

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
SLICE_00_01:
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
