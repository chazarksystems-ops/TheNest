# RT-005 — Router Design Only

## Status

Design-only prompt. Do not implement unless Chaz explicitly authorizes router work.

## Task scope

Create a design spec for a route-cost router based on health metrics, latency, failure count, load, and task affinity.

## Must include

- Route input struct.
- Specialist registry struct.
- Health metrics struct.
- Cost formula.
- Deterministic tie-break rule.
- Persistence strategy.
- Test matrix.
- Future embedding-based affinity slot.

## Forbidden work

- No vLLM endpoint calls.
- No model serving.
- No external embeddings.
- No Prometheus/Grafana.
- No code implementation.

## Output file requested

Create or update only:

- `P2_ROUTER_DESIGN_SPEC.md`

## Source concept to preserve

The Grok source’s router idea is useful: route to the lowest-cost healthy specialist using semantic/affinity fit, historical failure pressure, and current load. Convert this into deterministic Rust-friendly data structures and tests.
