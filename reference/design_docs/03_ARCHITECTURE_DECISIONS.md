# 03 — Architecture Decisions

## AD-001: P0 is Rust lifecycle only

Decision: P0 implements lifecycle, health scoring, and receipts only.

Rationale: The source files combine many layers. The first reliable step is a small compile-tested Rust foundation.

## AD-002: JSONL receipts before binary ledgers

Decision: P0 uses append-only JSONL receipts.

Rationale: Human-readable receipts are easier to inspect during early design. Binary hash chains can be added after schemas stabilize.

## AD-003: Source metaphors are preserved but implementation names are explicit

Decision: The packet preserves terms like hive, nociception, apoptosis, and epigenetic payload as conceptual labels, but code should use explicit names where clarity matters.

Rationale: The metaphors are useful design handles, but code must be auditable.

## AD-004: Model runtime is behind future adapters

Decision: P0 defines no vLLM, llama.cpp, mistral-rs, TensorRT-LLM, or Axolotl integration.

Rationale: Model runtime work depends on installed versions, hardware verification, and separate safety gates.

## AD-005: Avoid borrowed-lock lifetime traps

Decision: Do not design P0 workers that borrow from `RwLockReadGuard` and then outlive/drop the guard. Use owned snapshots, `Arc<str>`, or scoped execution.

Rationale: The Gemini source suggests zero-copy borrowed prompt structures. That goal is valid, but the safe implementation needs careful lock lifetime design.

## AD-006: Rust ownership proves only what Rust owns

Decision: Do not claim that consuming a Rust worker frees external model memory unless that memory is owned by the worker object.

Rationale: vLLM endpoints, model servers, GPU KV cache, and external processes are outside Rust ownership unless explicitly integrated.

## AD-007: Training data requires verifier-backed records

Decision: Future synthetic records must include validation metadata and rejection reasons.

Rationale: Generated reasoning traces are not ground truth. Future data records need explicit verifier status, source/license fields, and run IDs.
