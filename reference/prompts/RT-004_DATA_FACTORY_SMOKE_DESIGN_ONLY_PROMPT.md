# RT-004 — Data Factory Smoke Test Design Only

## Status

Design-only prompt. Do not implement unless Chaz explicitly authorizes P1.

## Task scope

Design a P1 smoke-test plan for the Rust/PyO3/data-factory direction harvested from the Grok source.

## Must include

- Input seed JSONL schema.
- Accepted output JSONL schema.
- Rejected output JSONL schema.
- Run receipt schema.
- Mock endpoint strategy.
- Validation commands.
- No external model dependency.
- Five-row test fixture plan.

## Forbidden work

- No code edits.
- No downloads.
- No model launch.
- No training.
- No large dataset generation.

## Output file requested

Create or update only:

- `P1_DATA_FACTORY_SMOKE_PLAN.md`

## Success criteria

The plan must be specific enough that a later agent can implement the smoke test without inventing architecture.
