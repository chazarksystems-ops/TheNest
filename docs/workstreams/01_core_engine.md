# Workstream: 01_core_engine

## Purpose
Protects and maintains swarm_core core types, configurations, worker logic, metrics scoring, and apoptosis transitions.

## Owned areas
swarm_core/src/lib.rs, config.rs, metrics.rs, nociceptor.rs, apoptosis.rs, worker.rs, reason.rs, payload.rs

## Not owned areas
CLI binaries, validation scripts, scenario JSON files, documentation.

## Current status
P0-P2 completed. Active maintenance and stability protection.

## Remaining roadmap phases
Mainly complete, acts as import dependency for other phases.

## Task groups
Core engine data models, pain metrics math safety.

## Microtask examples
MICRO_P4_01_01_ADD_OPTIONAL_METADATA_FIELDS (adjusts Scenario structs in core)

## Files usually allowed
swarm_core/src/*.rs (except src/bin/*)

## Files usually read-only
All other directories.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
cargo check --workspace, cargo test --workspace, cargo fmt

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
