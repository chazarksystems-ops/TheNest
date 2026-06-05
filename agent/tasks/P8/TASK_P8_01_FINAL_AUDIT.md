# TASK_P8_01: FINAL_AUDIT

## Phase
P8

## Workstream
08_final_audit_closeout

## Purpose
Executes planned milestones for final audit in P8.

## Context packet
- docs/workstreams/08_final_audit_closeout.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P8_*.md

## Allowed files
reports/audits/*

## Read-only files
swarm_core/src/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P8_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_01_RUN_FINAL_STRUCTURE_AUDIT.md) - run final structure audit
- [MICRO_P8_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_02_RUN_FINAL_VALIDATION_AUDIT.md) - run final validation audit
- [MICRO_P8_01_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P8/MICRO_P8_01_03_WRITE_P8_FINAL_AUDIT_REPORT.md) - write p8 final audit report

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
