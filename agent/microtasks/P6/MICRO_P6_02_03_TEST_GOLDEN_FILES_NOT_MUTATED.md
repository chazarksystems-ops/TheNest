# MICRO_P6_02_03: TEST_GOLDEN_FILES_NOT_MUTATED

## Parent task
[TASK_P6_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P6/TASK_P6_02_GOLDEN_PREVIEW_POLICY_AND_COMMAND.md)

## Workstream
05_regression_baselines

## Objective
Implement test golden files not mutated as part of task GOLDEN_PREVIEW_POLICY_AND_COMMAND.

## Minimal context files
- docs/workstreams/05_regression_baselines.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P6_*.md

## Allowed edits
swarm_core/tests/regression_tests.rs

## Required checks
- Code complies with style guidelines.
- Validation script passes.

## Forbidden work
- Do not read the whole repo unless blocked.
- Do not widen scope.
- If required files are missing, report BLOCKED.
- Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Output/report format
A short 2-3 line summary of completed code changes.

## Done when
The targeted file change is made and workspace passes light compilation.
