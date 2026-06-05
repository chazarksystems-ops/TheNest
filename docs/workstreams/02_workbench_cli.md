# Workstream: 02_workbench_cli

## Purpose
Implements the CLI commands, manual CLI argument parser, and user-facing workbench binary.

## Owned areas
swarm_core/src/bin/hive_workbench.rs (evolved from demo.rs), Cargo.toml binary definitions.

## Not owned areas
Core mathematical formula modules, packaging scripts, test files.

## Current status
Ready to start P3.

## Remaining roadmap phases
P3, P5 (summarize/report commands), P6 (golden-preview/regression commands).

## Task groups
CLI Parser, Command Suite Implementation, Output Mode Formatters.

## Microtask examples
MICRO_P3_03_01_PARSE_COMMAND_NAME, MICRO_P3_04_04_RUN_SCENARIO_SUITE

## Files usually allowed
swarm_core/src/bin/hive_workbench.rs, Cargo.toml, scripts/validate.*

## Files usually read-only
Core modules under swarm_core/src/, docs/, scenarios/.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
cargo run --bin hive_workbench -- <command>, cargo test --workspace, scripts/validate.ps1

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
