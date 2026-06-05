# Spec Packet Cleanup Notes

This repaired packet adds a thin `skills/` + `ops/` layer so future Codex/subagent work can stay low-context.

## Cleanup decisions

- `SLICE_03_04` means serde derives/dependency only.
- `SLICE_03_05` means JSON serialization test only after serde support exists.
- `SLICE_02_02` allows `swarm_core/src/nociceptor.rs` because terminology may appear there.
- Dependency order beats filename/alphabetical order.
- Early README edits should stay tiny; reserve broad README organization for `SLICE_08_01`.
- `PROJECT_BOUNDARY.md` should define what the project is.
- `DO_NOT_DRIFT.md` should define what not to add.
- `PERSONAL_TOOL_DOCTRINE.md` should define how agents should behave.
- Files listed under “Relevant current files” are context only unless also listed under “Allowed edits” or “Allowed new files.”

## Validation note

Rust validation was not run during this specs-only cleanup because no Rust source was changed.
