# MICRO_P3_04_01: LIST_SCENARIOS

## Parent task
[TASK_P3_04](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P3/TASK_P3_04_COMMANDS_LIST_RUN_RUNFILE_SUITE.md)

## Workstream
02_workbench_cli

## Objective
Implement list scenarios as part of task COMMANDS_LIST_RUN_RUNFILE_SUITE.

## Minimal context files
- docs/workstreams/02_workbench_cli.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P3_*.md

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
