# SLICE_00_02 — Add Do Not Drift Note

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

Create a short practical guardrail document so future agents do not expand the P0 into a runtime or over-governed framework.

Success condition:

```text
docs/DO_NOT_DRIFT.md exists and clearly forbids unnecessary runtime/process expansion.
```

## Relevant current files

```text
README.md
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
README.md
```

## Allowed new files

```text
docs/DO_NOT_DRIFT.md
```

## Forbidden work

Do not:

- redesign the project
- turn this into a full HIVE runtime
- ask for review for routine local choices
- edit Rust source code
- add dependencies
- create long governance docs

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

1. Create `docs/DO_NOT_DRIFT.md`.
2. Include forbidden expansions: Tokio, HTTP server, database, model serving, scheduler, queue, GPU logic, PyO3, vLLM, Axolotl.
3. Include personal-tool reminder: proceed without unnecessary reviews.
4. Reference the doc from README if appropriate.

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
SLICE_00_02:
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
