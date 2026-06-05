# MICRO_P4_02_02: INVALID_SCENARIO_HANDLING

## Parent task
[TASK_P4_02](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P4/TASK_P4_02_VALIDATE_SCENARIOS_COMMAND.md)

## Workstream
03_scenario_library

## Objective
Implement invalid scenario handling as part of task VALIDATE_SCENARIOS_COMMAND.

## Minimal context files
- docs/workstreams/03_scenario_library.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P4_*.md

## Allowed edits
swarm_core/src/bin/hive_workbench.rs

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
