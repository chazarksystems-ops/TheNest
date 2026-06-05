# Workstream: 03_scenario_library

## Purpose
Authoring scenarios, schemas, templates, and evaluation validation rules.

## Owned areas
scenarios/ library folder, Scenario struct metadata field imports.

## Not owned areas
Command-line parsing logic, packaging, receipts output sink.

## Current status
P4 planned.

## Remaining roadmap phases
P4 Scenario Library expansion and validation.

## Task groups
Scenario authoring schemas, metadata fields compatibility, validation command evaluation.

## Microtask examples
MICRO_P4_03_01_CREATE_SCENARIO_TEMPLATE, MICRO_P4_02_01_VALIDATE_SCENARIOS_COMMAND

## Files usually allowed
scenarios/*.json, swarm_core/src/lib.rs (Scenario parsing)

## Files usually read-only
bin/ files, reports/, scripts/.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
cargo test --workspace, hive_workbench validate-scenarios

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
