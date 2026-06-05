# TASK_P3_01: CONTEXT_AND_CLI_SPEC

## Phase
P3

## Workstream
07_ops_agent_dispatch

## Purpose
Executes planned milestones for context and cli spec in P3.

## Context packet
- docs/workstreams/07_ops_agent_dispatch.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P3_*.md

## Allowed files
ops/*, docs/roadmap/*

## Read-only files
swarm_core/src/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P3_01_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_01_01_REMOVE_RUNTIME_QUEUE_REFERENCES.md) - remove runtime queue references
- [MICRO_P3_01_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P3/MICRO_P3_01_02_CREATE_CLI_ARG_SPEC.md) - create cli arg spec

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
