# 06 — Validation Matrix

| ID | Validation | Command / Method | Required result | Blocks if fail? |
|---|---|---|---|---|
| VAL-001 | Formatting | `cargo fmt --all --check` | PASS | Yes |
| VAL-002 | Compile | `cargo check --workspace` | PASS | Yes |
| VAL-003 | Tests | `cargo test --workspace` | PASS | Yes |
| VAL-004 | Health formula | Unit test | Expected score exactly or within explicit float tolerance | Yes |
| VAL-005 | Non-terminal state | Unit test | `is_terminal() == false` | Yes |
| VAL-006 | Terminal state | Unit test | `is_terminal() == true` | Yes |
| VAL-007 | Consuming lifecycle | Compile-tested pattern | Worker cannot be reused after termination path | Yes |
| VAL-008 | Receipt serialization | Unit test | Receipt serializes to JSON | Yes |
| VAL-009 | JSONL append | Temp-file test | Two receipts produce two lines | Yes |
| VAL-010 | No unauthorized deps | Cargo.toml review | No vLLM/Axolotl/PyO3/Docker/model deps in P0 | Yes |

## Evidence required from implementation agent

The agent must report:

```text
validation_status: PASS/FAIL
commands_run:
  - cargo fmt --all --check
  - cargo check --workspace
  - cargo test --workspace
files_created:
files_modified:
blocked_items:
notes:
```

## Failure handling

- If compile fails, stop and report the first compiler error plus attempted fix.
- If scope conflict appears, report `INVALID` rather than broadening the task.
- If environment lacks Rust/Cargo, report `BLOCKED` with no code invention beyond static drafts.
