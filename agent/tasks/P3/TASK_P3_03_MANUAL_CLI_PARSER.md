# TASK_P3_03: MANUAL_CLI_PARSER

## Phase
P3

## Workstream
02_workbench_cli

## Purpose
Executes planned milestones for manual cli parser in P3.

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
- [MICRO_P3_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_01_PARSE_COMMAND_NAME.md) - parse command name
- [MICRO_P3_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_02_PARSE_OUTPUT_FLAG.md) - parse output flag
- [MICRO_P3_03_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_03_03_STABLE_USAGE_ERRORS.md) - stable usage errors

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
