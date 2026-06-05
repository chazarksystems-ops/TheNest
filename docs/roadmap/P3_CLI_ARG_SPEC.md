# P3 CLI Argument Specification

## Purpose

Define the exact manual command-line argument parser grammar, behavior, output modes, and command semantics for the `hive_workbench` CLI tool. This ensures future implementation agents can build the local workbench interface cleanly, robustly, and without relying on heavy external dependencies or introducing forbidden async runtimes.

## Current State

The codebase currently has a simple `demo` CLI binary (`swarm_core/src/bin/demo.rs`) which accepts a single argument: either a named scenario shortcut (`healthy`, `exact`, `breach`, `below`) or a direct file path, runs that scenario, prints human-readable logs to stdout, and writes a serialized JSON receipt to `receipts/out/` if the worker terminated.

## Target Binary Decision

- **Preferred final user-facing binary:** `hive_workbench`
- **Evolve demo binary:** The existing `demo` binary (`swarm_core/src/bin/demo.rs`) should be evolved and renamed directly to `hive_workbench` (`swarm_core/src/bin/hive_workbench.rs`).
- **Binary overlap:** Do not maintain two overlapping CLIs. Evolving the demo binary completely replaces it. Any validation scripts (`validate.ps1`, `validate.sh`) referencing the `demo` binary must be updated to reference `hive_workbench`.

## Command Grammar

The workbench CLI uses the following command grammar:

```text
hive_workbench <command> [args] [--output <human|json|quiet>]
```

The `--output` flag is optional and can appear anywhere after the command name. If omitted, it defaults to `human`.

## Output Modes

- `human`: Prints formatted, readable text logs and ASCII tables designed for local human inspection.
- `json`: Prints stable, structured, machine-readable JSON representation of the command's primary results to stdout. Any log statements or errors must go to stderr to prevent polluting stdout.
- `quiet`: Minimizes output to stdout, printing only success status or essential result outcomes (e.g., exit code 0 on success, non-zero on failure).

## Commands

### list

- **Purpose:** Scan the `scenarios/` directory and list available scenarios.
- **Args:** None.
- **Output (human):** Prints a clean table containing the scenario name, file path, expected outcome (if declared in metadata), and a brief description (if declared).
- **Output (json):** Prints a JSON array of scenario metadata objects.
- **Output (quiet):** Prints the count of found scenarios (e.g., `Found 6 scenarios.`).

### run <scenario-name>

- **Purpose:** Run a named scenario using its pre-defined shortcut or local filename.
- **Args:** `<scenario-name>` (positional, required).
- **Shortcuts:**
  - `healthy` -> `scenarios/worker_survives.json`
  - `below`   -> `scenarios/worker_just_below_threshold.json`
  - `exact`   -> `scenarios/worker_exact_threshold.json`
  - `breach`  -> `scenarios/worker_threshold_breach.json`
- **Output (human):** Same as current demo: prints loading info, metrics, config, outcome (SURVIVED or TERMINATED), final suffering score, and receipt details.
- **Output (json):** Prints only the raw serializable execution output (e.g., the worker outcome payload or receipt JSON).
- **Output (quiet):** Prints only the outcome string (e.g., `SURVIVED` or `TERMINATED`).

### run-file <path>

- **Purpose:** Run a scenario from a specific local JSON file path.
- **Args:** `<path>` (positional, required).
- **Output (human/json/quiet):** Behaves identically to `run` command but loads the scenario file directly from the specified absolute or relative path.

### suite

- **Purpose:** Run all scenarios in the `scenarios/` library and present a summary.
- **Args:** None.
- **Output (human):** Prints a table showing:
  ```text
  scenario | expected | actual | score | threshold | status
  ```
- **Output (json):** Prints a JSON list of run outcomes, including name, expected outcome, actual outcome, score, and status.
- **Output (quiet):** Prints summary counts (e.g., `Suite execution completed: 6 run, 6 passed, 0 failed.`).

### validate-scenarios

*(Planned for P4)*
- **Purpose:** Parse all scenario JSON files and validate their contents and executability.
- **Args:** None.

### summarize <receipts-path>

*(Planned for P5)*
- **Purpose:** Analyze and aggregate metrics from a directory of receipt JSON files.
- **Args:** `<receipts-path>` (positional, defaults to `receipts/out`).

### report scenarios

*(Planned for P5)*
- **Purpose:** Generate a markdown scenario execution evidence report.
- **Args:** None.

### golden-preview <scenario-name>

*(Planned for P6)*
- **Purpose:** Display what a new golden receipt for the scenario would look like.
- **Args:** `<scenario-name>` (positional, required).

### regression

*(Planned for P6)*
- **Purpose:** Execute outcome validation checks and golden checks.
- **Args:** None.

## Error Handling

- Missing command: Print usage instructions to stderr and exit with code `1`.
- Unknown command: Print error message and help to stderr and exit with code `1`.
- Missing required argument (e.g. `run` with no name): Print missing argument error to stderr and exit with code `1`.
- File not found: Print file error to stderr and exit with code `2`.
- Scenario parse failure: Print deserialization errors to stderr and exit with code `3`.

## Path Rules

- Scenario files are located under `scenarios/` directory relative to the repository root.
- Workbench commands must resolve relative paths starting from the repository root.
- Generated receipts must be written to `receipts/out/` (which remains git-ignored).

## JSON Output Rules

- When `--output json` is set, only valid JSON must be printed to `stdout`.
- All warnings, informational logs, and error descriptions must be written to `stderr`.
- The printed JSON must have no trailing text or debug prints.

## Human Output Rules

- Use clear, aligned text grids or simple tables.
- Print status indicators clearly (e.g. `SURVIVED`, `TERMINATED`).
- Write receipt files using indentation for readability.

## Quiet Output Rules

- Suppress all detailed trace logging.
- Limit output to exit status codes and single-line summary strings.

## Backward Compatibility

- Scenarios that lack metadata (such as `description` or `expected_outcome`) must still load and run successfully.
- Serialization and deserialization structs must use `#[serde(default)]` or `Option<T>` for metadata fields to support older formats.

## Forbidden Work

The implementation of this CLI command surface must strictly avoid adding:
- **Async runtimes:** No Tokio, async-std, or other futures runtimes.
- **Network services:** No HTTP servers, Actix web, Axum, or TCP listening.
- **Databases:** No SQL databases (sqlite, postgres, etc.) or ORM libraries.
- **Task queues / schedulers:** No async task loop runners, background worker threads, or scheduler engines.
- **Clap dependency:** Clap or other large command line parsing frameworks are forbidden to keep compile times fast and dependencies minimal. Argument parsing must be done using standard library `std::env::args()` or a simple custom helper.

## Acceptance Criteria

- Running `cargo run --bin hive_workbench -- list` displays all scenarios.
- Running `cargo run --bin hive_workbench -- run healthy` runs the survived scenario.
- Running `cargo run --bin hive_workbench -- suite` executes the suite and prints a table.
- Output mode `--output json` is parsed and returns valid, parseable JSON.
- Validation script `validate.ps1` runs the new workbench binaries successfully.

## Future Parking Lot

- Fully interactive console UI.
- Support for auto-updating golden snapshots via CLI flags.
- CSV or HTML export formats.
