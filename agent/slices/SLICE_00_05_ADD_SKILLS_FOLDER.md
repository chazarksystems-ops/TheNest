# SLICE_00_05 — Add Skills Folder

## Mission

You are working on one bounded slice of the `kimi_hive_p0_test` personal-tool development packet.

Project context:

```text
P0 Rust lifecycle primitive:
worker health metrics -> threshold breach -> ownership-consuming worker termination -> receipt payload emitted
```

## Personal tool doctrine

This is for quick development of a personal tool, not an engineer-grade production build. Keep changes small and practical. Do not ask for routine reviews.

## Objective

Add the small `skills/` task-router files for low-context subagent work.

## Relevant current files

```text
docs/00_PERSONAL_TOOL_DOCTRINE.md
docs/03_DRIFT_BOUNDARIES.md
templates/COMPACT_SLICE_TEMPLATE.md
README.md
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
README.md
```

## Allowed new files

```text
skills/README.md
skills/*/SKILL.md
```

## Forbidden work

Do not:

- redesign the project
- turn this into a runtime
- add networking, model serving, async orchestration, database, scheduler, queue, or GPU logic
- add enterprise governance or unnecessary review gates
- edit `target/` or generated build artifacts
- continue into adjacent slices


## Required steps

1. Create compact `SKILL.md` files for the selected skills.
2. Create `skills/README.md` as a short index.
3. Keep each skill short and practical.
4. Add a short README note for `skills/`.

## Tests / checks

1. Confirm `skills/README.md` exists.
2. Confirm each required `skills/*/SKILL.md` exists.
3. Confirm no Rust source was edited.

## Validation

For docs/spec-only work, Rust validation may be reported as:

```text
NOT RUN - specs-only markdown cleanup; no Rust source changed.
```

For Rust implementation work, run from workspace root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Final report format

```text
SLICE_00_05:
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
