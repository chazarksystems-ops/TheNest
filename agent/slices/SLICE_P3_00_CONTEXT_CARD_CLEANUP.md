# SLICE_P3_00_CONTEXT_CARD_CLEANUP

## Purpose

Remove forbidden runtime/queue/scheduler next-slice references from operational cards and replace them with P3 local workbench command surface tasks. Create a robust command-line argument specification document.

## Allowed Files

- `ops/CURRENT_CONTEXT_CARD.md`
- `ops/NEXT_SLICE_QUEUE.md`
- `ops/COMPLETED_SLICES.md`
- `docs/roadmap/P3_CLI_ARG_SPEC.md`
- `agent/slices/SLICE_P3_00_CONTEXT_CARD_CLEANUP.md`

## Forbidden Work

- Rust implementation changes.
- Adding runtime, scheduler, queue, server, database, or networking layers.

## Validation

- Markdown syntax review.
- Workspace builds successfully via validation script.

## Stop Condition

Context cards, completed slices, next slice queue, and CLI specifications exist, align, and point strictly to CLI workbench tasks.
