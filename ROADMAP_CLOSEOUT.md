# TheNest Roadmap Closeout

Date: 2026-06-05

## Completed Phases

| Phase | Name | Status |
|---|---|---|
| P0 | Core Rust Primitives | COMPLETE |
| P1 | Scenario Workbench + Benchmark Baseline | COMPLETE |
| P2 | Regression + Benchmark Hardening | COMPLETE |
| P3 | Local Workbench Command Surface | COMPLETE |
| P4 | Scenario Authoring + Scenario Library | COMPLETE |
| P5 | Receipt Review + Evidence Reports | COMPLETE |
| P6 | Regression + Baseline Management | COMPLETE |
| P7 | Packaging + Quickstart Polish | COMPLETE |
| P8 | Final Audit + Roadmap Closeout | COMPLETE |

---

## What This Tool Can Now Do

- Run named and file-based scenarios: `hive_workbench run <name>`, `run-file <path>`
- List available scenarios: `hive_workbench list`
- Run all scenarios as a suite with a results table: `hive_workbench suite`
- Emit results in human, JSON, or quiet mode: `--output human|json|quiet`
- Validate scenario metadata (expected_outcome check): `hive_workbench validate-scenarios`
- Auto-generate receipts for terminated workers: `receipts/out/<name>_receipt.json`
- Summarize receipt files: `hive_workbench summarize receipts/out`
- Generate a Markdown evidence report: `hive_workbench report scenarios`
- Preview golden receipts without modifying files: `hive_workbench golden-preview <name>`
- Run regression checks (golden + scenario validation): `hive_workbench regression`
- Run performance benchmarks: `cargo bench`
- Package a clean handoff zip: `.\scripts\package_handoff.ps1`

---

## What This Tool Intentionally Does Not Do

- No HTTP server, no REST API
- No database or embedded storage
- No scheduler or task queue
- No async runtime (no Tokio)
- No networking
- No model-serving or LLM inference
- No distributed swarm
- No enterprise governance

---

## How to Restart Development

1. Read `ops/CURRENT_CONTEXT_CARD.md` for orientation
2. Read `docs/QUICKSTART.md` for command reference
3. Read `docs/FUTURE_ROADMAP_PARKING_LOT.md` for allowed next work
4. Run `.\scripts\validate.ps1` to confirm baseline is clean
5. Create a new task in `agent/tasks/` if extending the roadmap

---

## How to Run Validation

```powershell
.\scripts\validate.ps1
.\scripts\validate.ps1 -Demo
.\scripts\validate.ps1 -Stress
```

---

## How to Package Handoff

```powershell
.\scripts\package_handoff.ps1
```

Output: `C:\Users\cheez\Downloads\TheNest_handoff.zip`
