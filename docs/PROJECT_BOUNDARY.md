# Project Boundary - Kimi/HIVE P0

This project, `kimi_hive_p0`, is a P0 personal-tool lifecycle primitive.

## What This Project Is

- A single Rust workspace containing the `swarm_core` crate.
- A deterministic suffering scoring primitive using the formula:
  `score = alpha * context_bloat + beta * error_rate + gamma * coordination_debt`
- An ownership-consuming self-termination sequence (`CattleWorker::tick(self)` -> `WorkerOutcome::Terminated(EpigeneticPayload)`).
- A serializable termination receipt (`EpigeneticPayload`).

## What This Project Is NOT

It is NOT a:
- Runtime
- Scheduler
- Queue system
- Database system
- HTTP server
- Model-serving engine
- Router
- Swarm coordinator
- Enterprise-grade framework
