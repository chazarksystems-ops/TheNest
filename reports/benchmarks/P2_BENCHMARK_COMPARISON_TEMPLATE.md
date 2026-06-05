# HIVE Performance Comparison Report Template

Copy this template to record and compare new benchmarks against baseline measurements.

- **Date**: YYYY-MM-DD
- **Author/Tester**: [Name/Subagent]
- **Hardware Platform**: [OS/CPU/RAM details]

---

## 1. Microbenchmark Comparisons

Compare the median times reported by Criterion against the baseline metrics.

| Benchmark Name | P1 Baseline Median | New Median | Change (%) | Status (Pass / Inspect) |
| :--- | :--- | :--- | :--- | :--- |
| `calculate_suffering` | 711.88 ps | | | |
| `nociceptor_is_terminal` | 818.47 ps | | | |
| `receipt_json_serialization` | 453.67 ns | | | |
| `receipt_json_deserialization` | 291.72 ns | | | |
| `worker_tick_survived` | 27.66 ns | | | |
| `worker_tick_terminated` | 128.67 ns | | | |

---

## 2. Batch Stress Comparisons

Compare the cohort processing speeds.

| Batch Size / Mode | P1 Baseline Throughput | New Throughput | Change (%) | Status |
| :--- | :--- | :--- | :--- | :--- |
| **100k cohort (Memory Only)** | ~30.15 M / sec | | | |
| **100k cohort (25% Serialized)** | ~6.46 M / sec | | | |

---

## 3. Explanations & Notes

Detail any deviations, hardware anomalies, or code optimization comments below:
- 
