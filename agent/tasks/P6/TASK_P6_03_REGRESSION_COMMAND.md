# TASK_P6_03: REGRESSION_COMMAND

## Phase
P6

## Workstream
05_regression_baselines

## Purpose
Executes planned milestones for regression command in P6.

## Context packet
- docs/workstreams/05_regression_baselines.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P6_*.md

## Allowed files
swarm_core/src/*, reports/baselines/*

## Read-only files
scenarios/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P6_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_03_01_IMPLEMENT_REGRESSION_COMMAND.md) - implement regression command
- [MICRO_P6_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_03_02_REGRESSION_COMMAND_TEST.md) - regression command test

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
