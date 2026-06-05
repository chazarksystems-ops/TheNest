# Lightweight Final Report Format

Use this report format for every subagent slice.

```text
SLICE_ID:
STATUS: PASS / BLOCKED / PARTIAL

CHANGED:
- file/path
- file/path

CREATED:
- file/path
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

## Reporting rule

Keep reports short. The purpose is to confirm useful progress, not to create formal engineering paperwork.
