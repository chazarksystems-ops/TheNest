# context_card_updater

## WHEN TO USE

Use after a slice report has been accepted or compressed to keep future context small.

## INPUTS

- latest compact checkpoint or final report
- `ops/CURRENT_CONTEXT_CARD.md`
- `ops/COMPLETED_SLICES.md`
- `ops/NEXT_SLICE_QUEUE.md`
- `ops/BLOCKERS.md`

## ALLOWED WORK

- Update the current context card.
- Append completed slice status to the ledger.
- Update known blockers and next three slices.

## FORBIDDEN WORK

- Do not rewrite project history in detail.
- Do not add long transcripts.
- Do not create heavy governance docs.

## OUTPUT FORMAT

Update `ops/CURRENT_CONTEXT_CARD.md` with:

```text
PROJECT:
CURRENT STATUS:
LAST COMPLETED SLICE:
COMPLETED SLICES:
CURRENT CODE SHAPE:
CURRENT RECEIPT SHAPE:
KNOWN BLOCKERS:
NEXT 3 SLICES:
DO NOT ADD:
VALIDATION BASELINE:
```

## STOP CONDITION

Stop after the context card and ledgers are updated.
