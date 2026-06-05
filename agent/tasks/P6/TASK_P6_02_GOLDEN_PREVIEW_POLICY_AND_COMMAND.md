# TASK_P6_02: GOLDEN_PREVIEW_POLICY_AND_COMMAND

## Phase
P6

## Workstream
05_regression_baselines

## Purpose
Executes planned milestones for golden preview policy and command in P6.

## Context packet
- docs/workstreams/05_regression_baselines.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P6_*.md

## Allowed files
swarm_core/src/*, reports/baselines/*, docs/*

## Read-only files
scenarios/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P6_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_01_WRITE_GOLDEN_UPDATE_POLICY.md) - write golden update policy
- [MICRO_P6_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_02_IMPLEMENT_GOLDEN_PREVIEW_NO_OVERWRITE.md) - implement golden preview no overwrite
- [MICRO_P6_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P6/MICRO_P6_02_03_TEST_GOLDEN_FILES_NOT_MUTATED.md) - test golden files not mutated

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
