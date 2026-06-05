# TASK_P4_01: METADATA_SCHEMA

## Phase
P4

## Workstream
03_scenario_library

## Purpose
Executes planned milestones for metadata schema in P4.

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
- [MICRO_P4_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_01_ADD_OPTIONAL_METADATA_FIELDS.md) - add optional metadata fields
- [MICRO_P4_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_02_BACKWARD_COMPATIBILITY_TESTS.md) - backward compatibility tests
- [MICRO_P4_01_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P4/MICRO_P4_01_03_EXPECTED_OUTCOME_MODEL.md) - expected outcome model

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
