# P2 Benchmark Regression Guidance

Performance optimization is key to HIVE primitives. However, benchmark execution on local developer machines is inherently noisy due to background processes, OS thermal throttling, and hardware differences. 

Therefore, performance benchmarks are **advisory** and are excluded from default test failures (i.e., `cargo test`).

---

## 1. Execution Workflow

To perform a regression analysis when modifying `swarm_core` internals:

1. **Check Out / Save Baseline**: Ensure you have a record of the reference results from [P1_BASELINE_BENCHMARK.md](file:///C:/Users/cheez/Downloads/TheNest/reports/benchmarks/P1_BASELINE_BENCHMARK.md).
2. **Run Benchmarks**: Run the Criterion suite on your local branch:
   ```powershell
   cargo bench
   ```
3. **Inspect Estimates**: Criterion prints estimates to stdout and saves detailed outputs at `target/criterion/<benchmark_name>/new/estimates.json`.
4. **Fill Comparison Template**: Copy the template from [P2_BENCHMARK_COMPARISON_TEMPLATE.md](file:///C:/Users/cheez/Downloads/TheNest/reports/benchmarks/P2_BENCHMARK_COMPARISON_TEMPLATE.md) into a new run report (e.g., `reports/benchmarks/P2_COMPARISON_RUN_YYYYMMDD.md`) and fill in the values.

---

## 2. Interpreting Performance Changes

When comparing median times:

- **< 5% Variance**: Statistically insignificant; noise or micro-variations.
- **5% to 15% Increase**: Alert threshold. Investigate memory allocations (e.g., redundant copying or UUID calls) or changes to the scoring equation math.
- **> 15% Increase**: Regression warning. Verify changes in the hot execution paths (especially in `CattleWorker::tick` or `Nociceptor::calculate_suffering`).

---

## 3. Hot Paths to Protect

- **`calculate_suffering`**: Must remain an inlineable, zero-allocation math routine.
- **`CattleWorker::tick` (Survival Path)**: Must remain under `30 ns`. Do not introduce heap allocations or formatting in the non-apoptosis path.
- **Receipt Serialization**: Keep the JSON structure flat and fields short. Do not add nested custom structs without performance verification.
