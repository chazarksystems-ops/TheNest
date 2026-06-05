# TASK_P5_03: SCENARIO_EVIDENCE_REPORT

## Phase
P5

## Workstream
04_receipts_evidence

## Purpose
Executes planned milestones for scenario evidence report in P5.

## Context packet
- docs/workstreams/04_receipts_evidence.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P5_*.md

## Allowed files
swarm_core/src/*, reports/evidence/*

## Read-only files
scenarios/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P5_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_03_01_GENERATE_SCENARIO_EVIDENCE_MARKDOWN.md) - generate scenario evidence markdown
- [MICRO_P5_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_03_02_EVIDENCE_REPORT_TEST.md) - evidence report test

## Acceptance criteria
- All required microtasks are implemented and pass validation.
- CLI outcomes match specification.

## Validation
- cargo fmt --all --check
- cargo check --workspace
- cargo test --workspace
- .\scripts\validate.ps1

## Expected final report
A brief task-level summary of implemented changes, edited files, and test results.

## Stop condition
All acceptance criteria are met and validation script passes.
