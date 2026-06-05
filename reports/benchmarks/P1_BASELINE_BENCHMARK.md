# P1 Baseline Benchmark Report

This report documents the baseline performance of the core HIVE P0/P1 primitives.

- **Date**: 2026-06-03
- **OS**: Windows (x86_64)
- **Command Used**: `cargo bench`

## Benchmark Results

Criterion benchmarks were executed for core nociceptor functions, JSON serialization/deserialization of receipts, and worker ticking.

| Benchmark Name | Lower Bound | Estimate (Mean/Median) | Upper Bound | Notes / Details |
| :--- | :--- | :--- | :--- | :--- |
| `calculate_suffering` | 709.68 ps | **711.88 ps** | 714.59 ps | Inline numeric computation |
| `nociceptor_is_terminal` | 812.61 ps | **818.47 ps** | 826.46 ps | Simple comparison |
| `receipt_json_serialization` | 451.25 ns | **453.67 ns** | 456.56 ns | `serde_json::to_string` on `EpigeneticPayload` |
| `receipt_json_deserialization` | 290.47 ns | **291.72 ns** | 293.21 ns | `serde_json::from_str` into `EpigeneticPayload` |
| `worker_tick_survived` | 27.480 ns | **27.663 ns** | 27.862 ns | Tick resulting in survival path |
| `worker_tick_terminated` | 127.60 ns | **128.67 ns** | 129.71 ns | Tick resulting in termination & receipt construction |

## Observations & Insights

1. **Ultra-Low Compute Overhead**:
   The nociceptor's calculations are extremely fast, measuring in picoseconds. This confirms that pain scoring is a negligible component of any runtime overhead.
2. **Allocation & Receipt Construction Cost**:
    Ticking a terminated worker takes about `128.7 ns`, compared to `27.7 ns` for a surviving worker. This extra cost (~101 ns) is primarily driven by UUID generation and allocating the `EpigeneticPayload` struct with its strings.
3. **High Serialization Performance**:
   Serializing and deserializing a single receipt takes less than half a microsecond. In batch scenarios, this translates to very high throughput.
