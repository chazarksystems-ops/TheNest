# TheNest Quickstart

## Prerequisites

Rust toolchain (stable) — install via [rustup](https://rustup.rs/).

---

## Validate the project

```powershell
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
.\scripts\validate.ps1
```

---

## List available scenarios

```powershell
cargo run --bin hive_workbench -- list
```

---

## Run a named scenario

```powershell
cargo run --bin hive_workbench -- run healthy
cargo run --bin hive_workbench -- run breach
cargo run --bin hive_workbench -- run exact
cargo run --bin hive_workbench -- run below
```

---

## Run a scenario file

```powershell
cargo run --bin hive_workbench -- run-file scenarios/worker_survives.json
```

---

## Run all scenarios as a suite

```powershell
cargo run --bin hive_workbench -- suite
cargo run --bin hive_workbench -- suite --output json
```

---

## Output modes

```powershell
cargo run --bin hive_workbench -- run healthy --output human
cargo run --bin hive_workbench -- run healthy --output json
cargo run --bin hive_workbench -- run healthy --output quiet
```

---

## Write receipts (auto-generated on termination)

Receipts are written automatically when a worker terminates. No manual step required.

```
receipts/out/<scenario_name>_receipt.json
```

---

## Summarize receipts

```powershell
cargo run --bin hive_workbench -- summarize receipts/out
```

---

## Generate scenario evidence report

```powershell
cargo run --bin hive_workbench -- report scenarios
```

Report written to: `reports/evidence/SCENARIO_EVIDENCE_REPORT.md`

---

## Validate all scenarios (parse + expected_outcome check)

```powershell
cargo run --bin hive_workbench -- validate-scenarios
```

---

## Golden preview (show deterministic receipt, does not write golden file)

```powershell
cargo run --bin hive_workbench -- golden-preview exact
cargo run --bin hive_workbench -- golden-preview exact --output json
```

---

## Run regression checks

```powershell
cargo run --bin hive_workbench -- regression
```

---

## Run benchmarks (advisory, not a gate)

```powershell
cargo bench
# Or via:
.\scripts\validate.ps1 -Bench
```

---

## Run scenario demo

```powershell
.\scripts\validate.ps1 -Demo
```

---

## Run batch stress test

```powershell
.\scripts\validate.ps1 -Stress
```

---

## Package handoff

```powershell
.\scripts\package_handoff.ps1
```

Output: `C:\Users\cheez\Downloads\TheNest_handoff.zip`
