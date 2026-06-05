# P4 — Scenario Authoring

Scenarios are plain JSON files stored in the `scenarios/` directory. Each file describes the inputs to a single worker run and optionally declares the expected outcome.

---

## What a scenario JSON file contains

A scenario file has two top-level objects:

- **`config`** — worker configuration (thresholds and weights)
- **`metrics`** — simulated observations fed to the worker

Optional metadata fields describe the scenario for documentation, validation, and reporting purposes.

---

## Required fields

### `scenario_name`

A short identifier for the scenario. Must be unique across the scenario library. Used as the filename stem for receipt output.

```json
"scenario_name": "exact_threshold"
```

### `config`

| Field | Type | Description |
|---|---|---|
| `alpha` | float | Weight applied to context bloat |
| `beta` | float | Weight applied to error rate |
| `gamma` | float | Weight applied to coordination debt |
| `threshold` | float | Suffering score at or above which the worker terminates |

### `metrics`

| Field | Type | Description |
|---|---|---|
| `context_bloat` | float | Observed bloat ratio |
| `error_rate` | float | Observed error rate |
| `coordination_debt` | float | Observed coordination debt |

---

## Optional metadata fields

These fields are loaded with `serde(default)`. **Old scenario files without these fields load correctly** — missing metadata fields simply default to `None`.

| Field | Type | Description |
|---|---|---|
| `id` | string (UUID) | Optional stable identifier for the scenario |
| `description` | string | Human-readable explanation of what the scenario tests |
| `expected_outcome` | string | One of `"survived"`, `"terminated"`, `"invalid"` |
| `expected_score` | float | Expected final suffering score (advisory) |

### Valid `expected_outcome` values

| Value | Meaning |
|---|---|
| `"survived"` | Worker score stays below threshold; worker continues |
| `"terminated"` | Worker score meets or exceeds threshold; worker terminates |
| `"invalid"` | Scenario input is expected to fail validation |

---

## How to create a new scenario

1. Copy `scenarios/TEMPLATE.scenario.json` to a new file:
   ```powershell
   Copy-Item scenarios\TEMPLATE.scenario.json scenarios\my_scenario.json
   ```
2. Fill in `scenario_name`, `config`, and `metrics`.
3. Optionally add `description`, `expected_outcome`, and `expected_score`.
4. Validate:
   ```powershell
   cargo run --bin hive_workbench -- validate-scenarios
   ```
5. Run the scenario:
   ```powershell
   cargo run --bin hive_workbench -- run-file scenarios/my_scenario.json
   ```

---

## Validation rules

### Config validation
- `threshold` must be `>= 0`
- No `NaN` or `Infinity` values in any config field

### Metrics validation
- No negative values for `context_bloat`, `error_rate`, or `coordination_debt`
- No `NaN` or `Infinity` values in any metrics field

---

## TEMPLATE.scenario.json

```json
{
  "scenario_name": "TEMPLATE",
  "description": "Replace this with a description of what this scenario tests.",
  "expected_outcome": "survived",
  "expected_score": 0.0,
  "config": {
    "alpha": 1.0,
    "beta": 1.0,
    "gamma": 1.0,
    "threshold": 10.0
  },
  "metrics": {
    "context_bloat": 0.0,
    "error_rate": 0.0,
    "coordination_debt": 0.0
  }
}
```

> **Note:** The `id` field is omitted from the template. If you want a stable UUID for the scenario, generate one and add it manually. Scenarios without an `id` are fully valid.
