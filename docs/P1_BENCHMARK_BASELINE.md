# P1 Benchmark Baseline

This document explains how to execute and analyze the benchmark suite for HIVE lifecycle primitives (`swarm_core`).

---

## 1. Prerequisites

The benchmark suite uses **Criterion**, a robust microbenchmarking library in Rust. 

- Gnuplot is optional. If missing, Criterion will use its built-in `plotters` SVG backend to generate reports automatically under `target/criterion/`.

---

## 2. Running Benchmarks

To run the complete benchmark suite:
```powershell
cargo bench
```

To run a specific benchmark module:
```powershell
# Nociceptor math & terminal checks
cargo bench --bench nociceptor_bench

# Receipt JSON serialization and deserialization
cargo bench --bench receipt_json_bench

# Worker lifecycle ticks (survival vs termination paths)
cargo bench --bench worker_tick_bench
```

---

## 3. Core Benchmark Definitions

- **`calculate_suffering`**: Evaluates the equation `(bloat * alpha) + (error * beta) + (debt * gamma)` for a given state.
- **`nociceptor_is_terminal`**: Checks whether the calculated suffering score meets or exceeds the configured threshold.
- **`receipt_json_serialization`**: Benchmarks `serde_json::to_string` converting the `EpigeneticPayload` receipt to an unformatted JSON string.
- **`receipt_json_deserialization`**: Benchmarks parsing a JSON string back into the `EpigeneticPayload` struct.
- **`worker_tick_survived`**: Measures ticking a worker that remains healthy and continues execution.
- **`worker_tick_terminated`**: Measures ticking a worker that triggers apoptosis, generates a UUID, compiles the epigenetic payload, and terminates.

---

## 4. Benchmark HTML Reports

After executing `cargo bench`, Criterion outputs visual comparison graphs and reports to the file system. You can view the results by opening `target/criterion/report/index.html` in any web browser.
