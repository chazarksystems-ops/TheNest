# SLICE_02_03 — Invalid Numeric Input Validation

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

Add practical validation for invalid nociceptor config/metric values after config/metrics are split.

## Relevant current files

```text
swarm_core/src/nociceptor.rs
swarm_core/src/config.rs
swarm_core/src/metrics.rs
swarm_core/src/lib.rs
```

Relevant files are context only. They are read-only unless also listed under Allowed edits or Allowed new files.

## Allowed edits

```text
swarm_core/src/nociceptor.rs
swarm_core/src/config.rs
swarm_core/src/metrics.rs
swarm_core/src/lib.rs
```

## Allowed new files

```text
none
```

## Forbidden work

Do not:

- redesign the project
- turn this into a runtime
- add networking, model serving, async orchestration, database, scheduler, queue, or GPU logic
- add enterprise governance or unnecessary review gates
- edit `target/` or generated build artifacts
- continue into adjacent slices

- Do not add a policy engine.
- Do not change scoring math unless required to reject invalid input.

## Required steps

1. Confirm `SLICE_02_01_SPLIT_CONFIG_AND_METRICS` is complete or report `BLOCKED`.
2. Add validation for NaN, infinity, and negative values where appropriate.
3. Keep error handling small and local.
4. Do not add a policy engine.

## Tests / checks

1. valid config accepted
2. NaN rejected
3. infinity rejected
4. inappropriate negative values rejected

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
SLICE_02_03:
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
