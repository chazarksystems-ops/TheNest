# 01 — Source Review Summary

## Source files

- `source/grok_hive_source.txt`
- `source/gemini_hive_source.txt`

## Grok source: useful harvest

The Grok source contributes the operational DGX/data-factory track:

- Rust/PyO3 hybrid data-factory idea.
- Tokio/reqwest async pipeline idea.
- Local model endpoint boundary using localhost model servers.
- JSONL streaming/write pattern.
- Validation-first generated record handling.
- Later-stage vLLM endpoint ideas.
- Later-stage Axolotl/QLoRA/fine-tuning ideas.
- Later-stage suffering-aware router and blackboard metrics.
- Later-stage metrics/dashboard/vertical generation concepts.

## Grok source: do not directly adopt

Do not directly adopt:

- “production-ready” claims.
- Unverified DGX memory arithmetic.
- Multi-model launch scripts.
- Fine-tuning configs.
- Router code without compile audit.
- Dataset generation at scale.
- Generated thought/reasoning trace fields as training truth.

## Gemini source: useful harvest

The Gemini source contributes the Rust architecture/lifecycle spine:

- Nociceptor/health-metric concept.
- Ownership-consuming termination transition.
- Epigenetic payload / worker-exit receipt concept.
- Local vs global blackboard separation.
- Cargo workspace split: `swarm_core`, `orchestration`, `inference`, `ledger`, `orchestrator_bin`.
- Append-only ledger concept.
- AST hazard-gate concept.
- Explicit phase separation: foundations, state partitioning, model hydration, validation/hardening.

## Gemini source: do not directly adopt

Do not directly adopt:

- Claims that the compiler proves all runtime scheduling safety.
- Any pattern that borrows from a lock guard and uses the worker after the guard is dropped.
- Claims that Rust `Drop` frees external model KV cache unless Rust actually owns that resource.
- “Mathematically erased” or “ironclad” language in implementation docs.
- Direct model hydration code; P0 must not touch model runtime loading.

## Combined interpretation

The two files should be merged as two tracks:

1. **P0 Rust lifecycle foundation** — compile-tested, local, small.
2. **P1+ DGX/data-factory/model-router track** — future, gated by P0 evidence.
