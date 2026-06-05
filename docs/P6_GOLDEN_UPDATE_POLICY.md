# P6 — Golden File Update Policy

Golden files record the expected deterministic receipt output for specific scenarios. They are used in regression tests to detect unintended changes to score calculation or termination logic.

---

## Where golden files live

```
swarm_core/tests/golden/
```

Current golden file:

```
swarm_core/tests/golden/golden_receipt_exact_threshold.json
```

---

## What golden files contain

A golden file is a receipt JSON produced from a scenario with a **nil UUID** (`00000000-0000-0000-0000-000000000000`). Using a nil UUID makes the output fully deterministic — no random UUID is generated at test time, so the file can be compared byte-for-byte across runs.

---

## How tests use golden files

Tests **compare** the current nociceptor output against the golden file. They do **not** rewrite the golden file automatically. If the output does not match the golden file, the test fails.

This is intentional: a failing golden test means something in core score logic has changed, and that change must be reviewed deliberately before the golden file is updated.

---

## Previewing what the golden receipt would look like

Use `golden-preview` to see what the current receipt would look like for a deterministic scenario — without writing any file:

```powershell
cargo run --bin hive_workbench -- golden-preview exact
cargo run --bin hive_workbench -- golden-preview exact --output json
```

This is safe to run at any time. It produces no side effects.

---

## How to update a golden file

Golden files are updated **only intentionally and by hand**. There is no automated golden-update command.

Steps:
1. Run `golden-preview` and review the output carefully.
2. Confirm the change is expected and correct.
3. Copy the output manually into the golden file.

```powershell
cargo run --bin hive_workbench -- golden-preview exact --output json
# Review output, then manually update:
# swarm_core/tests/golden/golden_receipt_exact_threshold.json
```

4. Run the full test suite to confirm the updated golden file passes:
   ```powershell
   cargo test --workspace
   ```

> [!CAUTION]
> Never automate golden file overwrites. An automated overwrite would silently accept any regression, defeating the purpose of the golden test.
