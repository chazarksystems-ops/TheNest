# 00 — Packet Status

## Status

**DRAFT IMPLEMENTATION PACKET / REVIEWABLE HANDOFF**

This packet is intended to guide a bounded implementation agent. It is not proof that the architecture is correct, compiled, benchmarked, or DGX-validated.

## Authorization state

Allowed in P0:

- Create a new local Rust workspace scaffold.
- Implement compile-tested core lifecycle primitives.
- Implement append-only local JSONL receipts for lifecycle events.
- Add unit tests for worker health threshold breaches and receipt emission.
- Run Rust-only validation commands.

Not allowed in P0:

- No vLLM launch.
- No Axolotl training.
- No LoRA merge.
- No Hugging Face model download.
- No GPU memory tests.
- No long-running synthetic generation.
- No destructive cleanup, delete, quarantine, or archive operations.
- No claims of production readiness.

## Required language for outputs

Agent outputs must use evidence labels:

- `COMPILED`: cargo command completed successfully.
- `TESTED`: tests were run and passed.
- `DRAFT`: design not compiled or tested.
- `BLOCKED`: valid task could not proceed due to environment or dependency issue.
- `INVALID`: task instruction/input is malformed or outside P0 scope.

## Source handling

The original Grok and Gemini files are source material only. Do not paste their code directly into implementation files without adapting, compiling, and testing.
