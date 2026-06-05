# receipt_shaper

## WHEN TO USE

Use for receipt/payload slices only.

## INPUTS

- relevant `slices/SLICE_03_*.md`
- `swarm_core/src/payload.rs`
- `swarm_core/src/worker.rs`
- `swarm_core/src/lib.rs`
- `swarm_core/Cargo.toml` only when the slice explicitly allows dependency edits

## ALLOWED WORK

- `EpigeneticPayload`
- `TerminationReason`
- raw metric capture
- serde derives
- JSON serialization tests

## FORBIDDEN WORK

- Do not add file writers unless explicitly assigned.
- Do not add CLI.
- Do not add runtime.
- Do not add JSONL output unless explicitly assigned.
- Do not change scoring math.
- Do not change lifecycle ownership semantics.

## OUTPUT FORMAT

Use the selected slice report format.

## STOP CONDITION

Stop after the assigned receipt/payload slice.
