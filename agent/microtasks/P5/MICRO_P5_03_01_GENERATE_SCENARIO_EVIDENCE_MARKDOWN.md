# MICRO_P5_03_01: GENERATE_SCENARIO_EVIDENCE_MARKDOWN

## Parent task
[TASK_P5_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P5/TASK_P5_03_SCENARIO_EVIDENCE_REPORT.md)

## Workstream
04_receipts_evidence

## Objective
Implement generate scenario evidence markdown as part of task SCENARIO_EVIDENCE_REPORT.

## Minimal context files
- docs/workstreams/04_receipts_evidence.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P5_*.md

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
