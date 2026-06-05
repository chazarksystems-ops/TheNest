# TASK_P3_02: WORKBENCH_BINARY

## Phase
P3

## Workstream
02_workbench_cli

## Purpose
Executes planned milestones for workbench binary in P3.

## Context packet
- docs/workstreams/02_workbench_cli.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P3_*.md

## Allowed files
swarm_core/src/bin/*, Cargo.toml

## Read-only files
swarm_core/src/*.rs (except bin)

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P3_02_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_01_DECIDE_DEMO_VS_HIVE_WORKBENCH.md) - decide demo vs hive workbench
- [MICRO_P3_02_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_02_UPDATE_CARGO_BINARY_TARGET.md) - update cargo binary target
- [MICRO_P3_02_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_02_03_UPDATE_VALIDATE_SCRIPT_BINARY_REFERENCES.md) - update validate script binary references

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
