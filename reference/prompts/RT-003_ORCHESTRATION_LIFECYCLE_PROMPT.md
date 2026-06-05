# RT-003 — Minimal orchestration lifecycle proof

## Prerequisite

RT-001 and RT-002 must be passing.

## Task scope

Implement a minimal `orchestration` crate that proves the worker lifecycle using `swarm_core` and optionally writes receipts with `ledger`.

## Expected files

- `orchestration/Cargo.toml`
- `orchestration/src/lib.rs`
- `orchestration/src/worker.rs`
- Optional tests under `orchestration/tests/`

## Forbidden work

- No actual model calls.
- No local HTTP endpoints.
- No global router.
- No vLLM/Axolotl/PyO3.
- No borrowed `RwLockReadGuard` patterns that outlive the guard.

## Required behavior

Implement a toy worker with:

- worker id
- run id
- health snapshot
- `tick(self) -> WorkerOutcome<Self>`
- terminal state emits `WorkerExitReceipt`
- surviving state returns ownership as `Self`

## Design requirement

If adding local context, use an owned snapshot or `Arc<str>`. Do not borrow from a lock guard and then drop the guard.

## Tests required

- Surviving worker returns `WorkerOutcome::Survived`.
- Terminal worker returns `WorkerOutcome::Terminated`.
- Terminated receipt includes correct final score and reason.

## Validation commands

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Success criteria

Report:

```text
RT-003 status: PASS/FAIL/BLOCKED
files created:
files modified:
commands run:
validation result:
notes:
```
