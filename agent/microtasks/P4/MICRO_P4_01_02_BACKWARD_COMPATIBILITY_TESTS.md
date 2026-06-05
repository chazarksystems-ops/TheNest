# MICRO_P4_01_02: BACKWARD_COMPATIBILITY_TESTS

## Parent task
[TASK_P4_01](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P4/TASK_P4_01_METADATA_SCHEMA.md)

## Workstream
03_scenario_library

## Objective
Implement backward compatibility tests as part of task METADATA_SCHEMA.

## Minimal context files
- docs/workstreams/03_scenario_library.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P4_*.md

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
