# TASK_P7_01: QUICKSTART_GUIDE

## Phase
P7

## Workstream
06_docs_packaging

## Purpose
Executes planned milestones for quickstart guide in P7.

## Context packet
- docs/workstreams/06_docs_packaging.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P7_*.md

## Allowed files
docs/QUICKSTART.md

## Read-only files
swarm_core/src/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P7_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_01_01_WRITE_QUICKSTART_VALIDATE_AND_RUN.md) - write quickstart validate and run
- [MICRO_P7_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_01_02_WRITE_QUICKSTART_RECEIPTS_REPORTS_BENCHMARKS.md) - write quickstart receipts reports benchmarks

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
