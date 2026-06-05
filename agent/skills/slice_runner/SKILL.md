# slice_runner

## WHEN TO USE

Use when running exactly one bounded `slices/SLICE_*.md` task with minimal context.

## INPUTS

- one `slices/SLICE_*.md` file
- `docs/00_PERSONAL_TOOL_DOCTRINE.md`
- `docs/03_DRIFT_BOUNDARIES.md`
- `docs/04_VALIDATION_POLICY.md`
- `docs/05_FINAL_REPORT_FORMAT.md`
- `ops/CURRENT_CONTEXT_CARD.md`

## ALLOWED WORK

- Read the assigned slice.
- Edit only files listed under allowed edits or allowed new files.
- Run listed validation if implementation occurs.
- Return a compact final report.

## FORBIDDEN WORK

- Do not widen scope silently.
- Do not edit relevant/context files unless they are also listed under allowed edits.
- Do not continue into neighboring slices.
- Do not add runtime, networking, model-serving, async orchestration, database, scheduler, queue, or unrelated architecture.

## OUTPUT FORMAT

Use `docs/05_FINAL_REPORT_FORMAT.md` or the report block embedded in the slice.
If a needed file is outside allowed scope, report `BLOCKED` instead of editing it.

## STOP CONDITION

Stop after the assigned slice is complete or blocked.
