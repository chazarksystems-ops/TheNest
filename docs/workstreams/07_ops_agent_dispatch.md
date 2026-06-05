# Workstream: 07_ops_agent_dispatch

## Purpose
Maintains dispatch queues, slice files, current context cards, and agent templates.

## Owned areas
ops/ cards, agent/ tasks and microtasks directories.

## Not owned areas
Production codebase code, reports.

## Current status
Completed P3_00 card cleanup. Active dispatch management.

## Remaining roadmap phases
All phases.

## Task groups
Current state card syncing, slice file authoring.

## Microtask examples
MICRO_P3_01_01_REMOVE_RUNTIME_QUEUE_REFERENCES

## Files usually allowed
ops/*, agent/*

## Files usually read-only
Rust sources.

## Forbidden work
Do not add runtime, scheduler, queue, database, HTTP server, Tokio runtime, networking, model-serving, GPU logic, PyO3, CI/CD ceremony, or enterprise governance.

## Validation expected
Context card syncs correctly with completed history.

## Handoff format
Clean Pull Request / Commit and updated operations context card sync.
