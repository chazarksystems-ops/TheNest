# Workstream: 04_receipts_evidence

## Purpose
Managing receipt filename patterns, directory rules, and scenario evidence report compilers.

## Owned areas
receipts/out/ target directory, reports/evidence/ documentation.

## Not owned areas
Core engine scoring logic, CLI parser implementations.

## Current status
P5 planned.

## Remaining roadmap phases
P5 Receipt review and evidence reporting.

## Task groups
Predictable filename logic, summarize receipts command, markdown report generator.

## Microtask examples
MICRO_P5_01_02_APPLY_FILENAME_RULE_TO_RUN_COMMAND, MICRO_P5_03_01_GENERATE_SCENARIO_EVIDENCE_MARKDOWN

## Files usually allowed
reports/evidence/*, swarm_core/src/receipt_sink.rs, receipts/.gitkeep

## Files usually read-only
scenarios/, core engine source files.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
hive_workbench report scenarios, verification of reports/evidence/ output content.

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
