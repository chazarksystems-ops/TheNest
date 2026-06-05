# Workstream: 05_regression_baselines

## Purpose
Maintaining test suites, golden snapshots, baseline comparisons, and update policies.

## Owned areas
swarm_core/tests/ integration tests, reports/baselines/, golden snapshot files.

## Not owned areas
Main application CLI runtime commands, scenario JSON authoring.

## Current status
P6 planned.

## Remaining roadmap phases
P6 Baseline and Regression management.

## Task groups
Golden update rules, golden preview printing, regression CLI checking.

## Microtask examples
MICRO_P6_02_02_IMPLEMENT_GOLDEN_PREVIEW_NO_OVERWRITE, MICRO_P6_03_01_IMPLEMENT_REGRESSION_COMMAND

## Files usually allowed
swarm_core/tests/*.rs, swarm_core/tests/golden/*, reports/baselines/*, docs/P6_GOLDEN_UPDATE_POLICY.md

## Files usually read-only
Cargo.toml dependencies, package scripts.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
cargo test --workspace, hive_workbench regression

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
