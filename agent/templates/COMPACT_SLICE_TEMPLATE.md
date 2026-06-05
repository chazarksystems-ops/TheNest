# SLICE_ID — TITLE

## Mission

Work only this bounded slice in `kimi_hive_p0_test`.

Project context:

```text
P0 Rust lifecycle primitive:
worker health metrics -> threshold breach -> ownership-consuming worker termination -> receipt payload emitted
```

Personal-tool doctrine:

```text
This is for quick personal-tool development, not an engineer-grade production build.
Do not over-engineer.
Do not ask for routine reviews.
Proceed unless truly blocked.
```

## Objective

[One exact objective.]

## Relevant current files

```text
[file list]
```

## Allowed edits

```text
[file list]
```

## Allowed new files

```text
[file list or none]
```

## Forbidden

Do not add runtime, networking, model serving, async orchestration, database, GPU logic, scheduler, queue, router, unnecessary review gates, or unrelated architecture.

## Steps

1. [Exact step]
2. [Exact step]
3. [Exact step]

## Tests

Add/update tests for:

1. [claim]
2. [claim]

## Validation

Run:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Final report format

```text
SLICE_ID:
STATUS:
CHANGED:
CREATED:
VALIDATION:
SUMMARY:
BLOCKER:
DRIFT CHECK:
```

## Stop

Stop after this slice. Do not continue into neighboring work.

## Relevant-file rule

Relevant files are context only. They are read-only unless also listed under `Allowed edits` or `Allowed new files`.
