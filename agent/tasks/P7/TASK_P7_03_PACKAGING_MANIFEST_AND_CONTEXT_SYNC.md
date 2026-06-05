# TASK_P7_03: PACKAGING_MANIFEST_AND_CONTEXT_SYNC

## Phase
P7

## Workstream
06_docs_packaging

## Purpose
Executes planned milestones for packaging manifest and context sync in P7.

## Context packet
- docs/workstreams/06_docs_packaging.md
- ops/CURRENT_CONTEXT_CARD.md
- docs/roadmap/P7_*.md

## Allowed files
scripts/package_handoff.ps1, reports/handoff/*, ops/*

## Read-only files
swarm_core/src/*

## Forbidden files/work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Required microtasks
- [MICRO_P7_03_01](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_01_UPDATE_PACKAGE_HANDOFF_MANIFEST.md) - update package handoff manifest
- [MICRO_P7_03_02](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_02_SYNC_OPS_CONTEXT_FILES.md) - sync ops context files
- [MICRO_P7_03_03](file:///C:\Users\cheez\Downloads\TheNest/agent/microtasks/P7/MICRO_P7_03_03_VERIFY_PACKAGE_EXCLUDES_GENERATED_FILES.md) - verify package excludes generated files

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
