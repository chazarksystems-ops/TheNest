# TASK_P3_04: COMMANDS_LIST_RUN_RUNFILE_SUITE

## Phase
P3

## Workstream
02_workbench_cli

## Purpose
Executes planned milestones for commands list run runfile suite in P3.

## Context packet
- docs/workstreams/02_workbench_cli.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P3_*.md

## Allowed files
swarm_core/src/bin/*

## Read-only files
swarm_core/src/*.rs (except bin)

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P3_04_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_01_LIST_SCENARIOS.md) - list scenarios
- [MICRO_P3_04_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_02_RUN_NAMED_SCENARIO.md) - run named scenario
- [MICRO_P3_04_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_03_RUN_FILE_SCENARIO.md) - run file scenario
- [MICRO_P3_04_04](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_04_04_RUN_SCENARIO_SUITE.md) - run scenario suite

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
