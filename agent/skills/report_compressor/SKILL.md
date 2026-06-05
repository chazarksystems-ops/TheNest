# report_compressor

## WHEN TO USE

Use when a Codex/subagent report is too verbose and needs to become a durable checkpoint.

## INPUTS

- full Codex or subagent report
- `docs/05_FINAL_REPORT_FORMAT.md`
- `ops/CURRENT_CONTEXT_CARD.md`

## ALLOWED WORK

- Extract only the useful spine: slice, status, changed files, created files, validation, blocker, drift, and likely next slice.

## FORBIDDEN WORK

- Do not preserve long logs unless they contain a blocker.
- Do not invent validation results.
- Do not choose broad architecture work as the next slice.

## OUTPUT FORMAT

```text
CHECKPOINT:
- slice:
- status:
- changed:
- created:
- validation:
- blocker:
- drift:
- next likely slice:
```

## STOP CONDITION

Stop after creating the compact checkpoint.
