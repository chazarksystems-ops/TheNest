# TASK_P4_02: VALIDATE_SCENARIOS_COMMAND

## Phase
P4

## Workstream
03_scenario_library

## Purpose
Executes planned milestones for validate scenarios command in P4.

## Context packet
- docs/workstreams/03_scenario_library.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P4_*.md

## Allowed files
swarm_core/src/*, scenarios/*

## Read-only files
swarm_core/tests/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P4_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_02_01_VALIDATE_SCENARIOS_COMMAND.md) - validate scenarios command
- [MICRO_P4_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_02_02_INVALID_SCENARIO_HANDLING.md) - invalid scenario handling

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
