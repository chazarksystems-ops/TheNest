# 05 — P0 File Tree

```text
hive_runtime_p0/
  Cargo.toml
  README.md
  swarm_core/
    Cargo.toml
    src/
      lib.rs
      health.rs
      receipt.rs
      lifecycle.rs
  ledger/
    Cargo.toml
    src/
      lib.rs
      jsonl.rs
  orchestration/                 # optional in P0 if needed for proof tests
    Cargo.toml
    src/
      lib.rs
      worker.rs
  orchestrator_bin/              # optional placeholder only; no runtime model calls
    Cargo.toml
    src/
      main.rs
```

## Dependency direction

```text
swarm_core        # no internal project dependencies
ledger     --> swarm_core
orchestration --> swarm_core, ledger
orchestrator_bin --> swarm_core, ledger, orchestration
```

Rules:

- `swarm_core` must not depend on `ledger`, `orchestration`, or `inference`.
- `ledger` may serialize `swarm_core` receipts.
- `orchestration` may use `swarm_core` traits and ledger writers.
- `inference` is not implemented in P0.
