# P1 Batch Stress Report

This report documents the performance and throughput of the HIVE P0/P1 primitives under varying batch sizes.

- **Date**: 2026-06-03
- **Command Used**: `cargo run --release --bin batch_stress`

## Stress Test Run Summary

Each stress run models a mix of workers where **25%** are configured to terminate (breach threshold) and **75%** survive.

| Batch Size (Workers) | Serialization | Elapsed Time | Throughput (Workers/sec) | Total Serialized Size |
| :--- | :--- | :--- | :--- | :--- |
| **100** | Off | 19.7 µs | ~5.07 M / sec | - |
| **100** | On (25 receipts) | 24.9 µs | ~4.02 M / sec | 8,475 bytes |
| **1,000** | Off | 31.5 µs | ~31.74 M / sec | - |
| **1,000** | On (250 receipts) | 158.4 µs | ~6.31 M / sec | 84,750 bytes |
| **10,000** | Off | 343.6 µs | ~29.10 M / sec | - |
| **10,000** | On (2,500 receipts) | 1.637 ms | ~6.11 M / sec | 847,500 bytes |
| **100,000** | Off | 3.3169 ms | **~30.15 M / sec** | - |
| **100,000** | On (25,000 receipts) | 15.4689 ms | **~6.46 M / sec** | 8,475,000 bytes |

## Performance Analysis & Discussion

1. **Massive Raw Throughput**:
   Without serialization, ticking a worker cohort scales linearly and processes **100,000 workers in ~3.3 ms**. This means a single thread can tick over **30 million workers per second**, showing that the core logic compiles into highly optimized CPU operations.

2. **Impact of Serde JSON Serialization**:
   Enabling JSON serialization for the 25% of workers that terminate changes the 100k cohort elapsed time from **3.3 ms to 15.5 ms**.
   - The serialization of 25,000 receipts (generating ~8.47 MB of JSON data) takes ~12.15 ms.
   - This translates to a throughput of **~2.05 million serialized receipts per second** (or ~6.46 million total worker ticks/sec).
   - This proves that JSON serialization, while extremely fast, is the main bottleneck compared to memory-only ticking.

3. **Memory Footprint**:
   Since the lifecycle is synchronous and doesn't store receipts in a persistent database or ledger (they are emitted and consumed or immediately serialized), memory allocations remain small and temporary, avoiding heap fragmentation and GC pauses.
