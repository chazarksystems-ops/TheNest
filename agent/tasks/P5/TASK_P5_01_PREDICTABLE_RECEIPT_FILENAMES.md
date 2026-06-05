# TASK_P5_01: PREDICTABLE_RECEIPT_FILENAMES

## Phase
P5

## Workstream
04_receipts_evidence

## Purpose
Executes planned milestones for predictable receipt filenames in P5.

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
- [MICRO_P5_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_01_01_DEFINE_RECEIPT_FILENAME_RULE.md) - define receipt filename rule
- [MICRO_P5_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P5/MICRO_P5_01_02_APPLY_FILENAME_RULE_TO_RUN_COMMAND.md) - apply filename rule to run command

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
