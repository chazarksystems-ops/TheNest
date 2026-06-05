# RT-002 — Implement append-only JSONL receipt ledger

## Prerequisite

RT-001 must be complete and passing.

## Task scope

Implement the `ledger` crate for append-only JSONL lifecycle receipts.

## Expected files to create or edit

- Root `Cargo.toml` if adding workspace member/dependencies.
- `ledger/Cargo.toml`
- `ledger/src/lib.rs`
- `ledger/src/jsonl.rs`

## Forbidden work

- No binary ledger yet.
- No blake3/hash chain yet unless explicitly approved.
- No model runtime.
- No training.
- No network calls.

## Required behavior

Implement:

- `JsonlReceiptLedger`
- constructor taking a path
- `append_receipt(&self, receipt: &WorkerExitReceipt) -> Result<()>`
- append mode only
- newline-delimited JSON
- no overwriting existing file contents

## Tests required

- Append one receipt.
- Append two receipts and verify two lines.
- Verify each line parses back to `WorkerExitReceipt`.
- Use temp directory/temp file.

## Validation commands

Run:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## Success criteria

Report:

```text
RT-002 status: PASS/FAIL/BLOCKED
files created:
files modified:
commands run:
validation result:
notes:
```
