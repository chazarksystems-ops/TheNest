# Handoff Manifest

Generated: 2026-06-05

## Package

- Zip path: `C:\Users\cheez\Downloads\TheNest_handoff.zip`
- Script: `.\scripts\package_handoff.ps1`

## Included Root Folders

| Path | Notes |
|---|---|
| `Cargo.toml` | Workspace manifest |
| `Cargo.lock` | Dependency lock file |
| `README.md` | Project overview |
| `.gitignore` | Git exclusions |
| `ROADMAP_CLOSEOUT.md` | Phase completion record |
| `swarm_core/` | Core Rust library |
| `scenarios/` | All scenario JSON files |
| `receipts/` | Directory with `.gitkeep` only — no generated receipts |
| `docs/` | All documentation files |
| `reports/` | Status docs only — no generated benchmark or stress output |
| `scripts/` | PowerShell validation and packaging scripts |
| `ops/` | Operator context cards |
| `agent/` | Agent task files |
| `reference/` | Reference material |

## Excluded

| Excluded path | Reason |
|---|---|
| `target/` | Build artifacts — reconstructed by `cargo build` |
| `receipts/out/*` | Generated receipts — local artifacts, not committed |
| `*.zip` inside repo | Avoid packaging a zip inside a zip |
| Temporary agent artifacts | Not part of the deliverable |

## Validation Status at Packaging

| Check | Status |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo check --workspace` | PASS |
| `cargo test --workspace` | PASS |
| `scripts/validate.ps1` | PASS |
| `scripts/validate.ps1 -Demo` | PASS |
| `scripts/validate.ps1 -Stress` | PASS |
