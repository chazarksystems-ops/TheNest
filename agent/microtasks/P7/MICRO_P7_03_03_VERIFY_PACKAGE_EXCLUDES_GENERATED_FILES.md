# MICRO_P7_03_03: VERIFY_PACKAGE_EXCLUDES_GENERATED_FILES

## Parent task
[TASK_P7_03](file:///C:\Users\cheez\Downloads\TheNest/agent/tasks/P7/TASK_P7_03_PACKAGING_MANIFEST_AND_CONTEXT_SYNC.md)

## Workstream
06_docs_packaging

## Objective
Implement verify package excludes generated files as part of task PACKAGING_MANIFEST_AND_CONTEXT_SYNC.

## Minimal context files
- docs/workstreams/06_docs_packaging.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P7_*.md

## Allowed edits
scripts/package_handoff.ps1

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
