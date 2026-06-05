# drift_sentinel

## WHEN TO USE

Use after a slice or before accepting a change to catch unwanted project expansion.

## INPUTS

- changed file list
- `Cargo.toml` diffs if any
- source diffs if any
- README/docs diffs
- `docs/03_DRIFT_BOUNDARIES.md`

## ALLOWED WORK

- Inspect diffs and changed-file names.
- Report forbidden additions or scope warnings.

## FORBIDDEN WORK

- Do not fix code unless explicitly assigned.
- Do not approve runtime/orchestration expansion.
- Do not create heavy review gates.

Check especially for:

```text
Tokio
async runtime
HTTP server
database
scheduler
queue
swarm runtime
model serving
LLM calls
PyO3
GPU logic
networking
formal policy engine
heavy governance docs
```

## OUTPUT FORMAT

```text
DRIFT_SENTINEL:
STATUS: PASS / BLOCKED
FORBIDDEN_ADDITIONS_FOUND:
SCOPE_WARNINGS:
RECOMMENDED_ACTION:
```

## STOP CONDITION

Stop after the drift report.
