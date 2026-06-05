# Top-Level Development Categories

Recommended development split:

```text
kimi_hive_p0_test/
  00_boundary_and_doctrine/
  01_core_lifecycle/
  02_health_scoring/
  03_payload_and_receipts/
  04_config_and_profiles/
  05_validation_and_tests/
  06_cli_or_harness/
  07_runtime_future_not_now/
  08_docs_and_handoff/
  09_packaging_cleanup/
```

## Category 00 — Boundary and doctrine

Purpose: state what the personal tool is and what it is not.

Slices:

- Add project boundary document.
- Add do-not-drift document.
- Add current-state summary.
- Add personal-tool doctrine.

## Category 01 — Core lifecycle

Purpose: preserve and harden the ownership-consuming worker lifecycle.

Slices:

- Preserve `tick(self)` as the lifecycle boundary.
- Add explicit lifecycle state if useful.
- Formalize worker identity if useful.
- Optionally add compile-fail ownership test later.

## Category 02 — Health scoring

Purpose: make the nociceptor model clearer and harder to misuse.

Slices:

- Split config/weights from live metrics.
- Rename misleading payload fields.
- Add practical validation for invalid numeric inputs.
- Add scoring breakdown if it helps receipts.

## Category 03 — Payload and receipts

Purpose: turn the Rust-only payload into a usable audit receipt.

Slices:

- Add structured termination reason.
- Add raw metrics to payload.
- Add lightweight metadata if needed.
- Add serde support.
- Add JSON serialization test.
- Add JSONL writer later only if needed.

## Category 04 — Config and profiles

Purpose: avoid hardcoded magic values everywhere.

Slices:

- Add `NociceptorConfig`.
- Add named presets.
- Add config validation tests.
- Defer TOML loading unless it is immediately useful.

## Category 05 — Validation and tests

Purpose: keep basic confidence without heavy process.

Slices:

- Expand scoring tests.
- Expand lifecycle tests.
- Add simple validation scripts.
- Add lightweight validation notes only if helpful.

## Category 06 — CLI or harness

Purpose: create a tiny human-testable executable after core receipts are hardened.

Slices:

- Add a tiny demo binary.
- Add simple CLI arguments later.
- Add optional receipt output later.

## Category 07 — Runtime future, not now

Purpose: park future runtime ideas without implementing them.

Slices:

- Write future runtime notes.
- Sketch future traits only if useful.
- Do not implement scheduler, queue, swarm runtime, router, or model serving.

## Category 08 — Docs and handoff

Purpose: make the project easy for future agents to understand.

Slices:

- Improve README.
- Add simple architecture diagram.
- Add agent handoff prompt.

## Category 09 — Packaging cleanup

Purpose: keep source handoffs clean.

Slices:

- Exclude `target/`.
- Add `.gitignore`.
- Create clean source package.

## Low-context operations layer

This repaired packet also includes a lightweight operations layer:

- `skills/` for small task-router instructions.
- `ops/` for current context, queue, blockers, completed slices, and dispatch rules.
- `patch_notes/` for cleanup decisions.

These support low-token development and should not become a formal agent framework.
