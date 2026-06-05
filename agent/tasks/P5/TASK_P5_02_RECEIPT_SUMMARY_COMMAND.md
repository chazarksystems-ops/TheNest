# TASK_P5_02: RECEIPT_SUMMARY_COMMAND

## Phase
P5

## Workstream
04_receipts_evidence

## Purpose
Executes planned milestones for receipt summary command in P5.

## Context packet
- docs/workstreams/04_receipts_evidence.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P5_*.md

## Allowed files
swarm_core/src/*, receipts/*

## Read-only files
scenarios/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P5_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_01_PARSE_RECEIPTS_OUT_DIRECTORY.md) - parse receipts out directory
- [MICRO_P5_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_02_SUMMARIZE_RECEIPT_COUNTS_AND_SCORES.md) - summarize receipt counts and scores
- [MICRO_P5_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_02_03_SUMMARY_EMPTY_DIR_BEHAVIOR.md) - summary empty dir behavior

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
