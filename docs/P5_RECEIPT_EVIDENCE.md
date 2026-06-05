# P5 — Receipt Evidence

A **receipt** is a JSON file written to disk when a worker terminates. It records the final state of the worker at the moment of termination. Receipts are used for evidence review, regression comparison, and debugging.

---

## What an EpigeneticPayload receipt contains

| Field | Description |
|---|---|
| `worker_id` | UUID identifying the worker instance |
| `final_suffering_score` | The computed score at termination |
| `context_bloat` | Metric value used in score calculation |
| `error_rate` | Metric value used in score calculation |
| `coordination_debt` | Metric value used in score calculation |
| `threshold` | The threshold the score was compared against |
| `bloat_weight` | Weight (`alpha`) applied to context bloat |
| `error_weight` | Weight (`beta`) applied to error rate |
| `coordination_debt_weight` | Weight (`gamma`) applied to coordination debt |
| `termination_reason` | String describing why the worker was terminated |
| `fault_signature` | A structured fingerprint of the termination event |

---

## When receipts are written

Receipts are written **only when a worker terminates** (i.e., the suffering score meets or exceeds the threshold). If a worker survives a scenario run, no receipt file is produced.

---

## Receipt file location

```
receipts/out/<scenario_name>_receipt.json
```

Example: running the `breach` scenario produces:

```
receipts/out/breach_receipt.json
```

> **Note:** This is a plain JSON file on disk — not a ledger, not a database, not an append-only log. Each run overwrites the previous receipt for that scenario name.

---

## receipts/out/ is excluded from git

The `receipts/out/*` pattern is listed in `.gitignore`. Receipt files are local artifacts and are not committed to source control. The `receipts/` directory itself contains only a `.gitkeep` file in version control.

---

## Summarize receipts

To print a summary table of all receipt files in a directory:

```powershell
cargo run --bin hive_workbench -- summarize receipts/out
```

This reads every `*_receipt.json` file in the target directory and outputs a summary of scenario names, scores, thresholds, and termination reasons.

---

## Generate evidence report

To generate a Markdown evidence report covering all scenarios (including those that survived and those that terminated):

```powershell
cargo run --bin hive_workbench -- report scenarios
```

Report written to:

```
reports/evidence/SCENARIO_EVIDENCE_REPORT.md
```

The report includes: scenario name, expected outcome, actual outcome, final score, threshold, and pass/fail status for each scenario.
