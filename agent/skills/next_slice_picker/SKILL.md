# next_slice_picker

## WHEN TO USE

Use when choosing the next practical slice without rereading the entire packet.
Use dependency order, not filename or alphabetical order.

## INPUTS

- `docs/02_DEPENDENCY_ORDER.md`
- `ops/COMPLETED_SLICES.md`
- `ops/BLOCKERS.md`
- available `slices/` files

## ALLOWED WORK

- Read queue, blockers, completed-slice ledger, and dependency guidance.
- Recommend exactly one next slice.
- List the minimum files to give a subagent.

## FORBIDDEN WORK

- Do not implement a slice.
- Do not edit files.
- Do not choose by filename order when dependency guidance differs.
- Do not recommend runtime, scheduler, queue, database, networking, model-serving, or heavy governance work.

## OUTPUT FORMAT

```text
NEXT_SLICE:
WHY:
DEPENDENCIES:
FILES_TO_GIVE_SUBAGENT:
BLOCKERS_TO_AVOID:
```

## STOP CONDITION

Stop after recommending the next slice and required handoff files.
