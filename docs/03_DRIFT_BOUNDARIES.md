# Drift Boundaries

This project is a personal-tool P0 primitive. It should stay small until the basic lifecycle and receipt shape are clean.

## Do not add yet

Do not add:

- Tokio or async runtime
- HTTP server
- database layer
- worker scheduler
- queue system
- swarm runtime
- model serving
- LLM calls
- vLLM
- Axolotl
- PyO3
- GPU logic
- networking
- distributed coordination
- formal policy engine
- enterprise-grade governance docs
- heavy approval gates

## What is allowed

Allowed near-term work:

- Rust source cleanup
- simple data structs
- serde derives
- JSON serialization tests
- basic validation scripts
- concise docs
- tiny demo harness after core is stable

## Review boundary

Do not ask for review for routine local choices.

Ask for review only if the requested work requires:

- editing outside the slice scope
- breaking existing behavior
- adding a major dependency
- choosing between incompatible architectures
- deleting user-authored source material
- turning the primitive into a runtime
