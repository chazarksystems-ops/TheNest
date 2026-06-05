# Workstream: 06_docs_packaging

## Purpose
Normalizing main readme documentation, quickstarts, and handoff script packaging.

## Owned areas
README.md, docs/QUICKSTART.md, scripts/package_handoff.ps1, reports/handoff/.

## Not owned areas
Rust binary source code, scenario logic.

## Current status
P7 planned.

## Remaining roadmap phases
P7 Quickstart and Packaging Polish.

## Task groups
Handoff script logic, manifest file compiling, readme mapping.

## Microtask examples
MICRO_P7_03_01_UPDATE_PACKAGE_HANDOFF_MANIFEST, MICRO_P7_02_01_REWRITE_README_STRUCTURE

## Files usually allowed
README.md, docs/QUICKSTART.md, scripts/package_handoff.ps1, reports/handoff/*

## Files usually read-only
swarm_core/src/

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
scripts/package_handoff.ps1, unzip and check content constraints.

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
