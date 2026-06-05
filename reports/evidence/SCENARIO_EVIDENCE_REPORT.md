# Scenario Evidence Report

Generated: 2026-06-05

| Scenario | Expected | Actual | Score | Threshold | Status |
|---|---|---|---|---|---|
| invalid_infinite_metric | invalid | invalid | 0.00 | 0.00 | PASS |
| invalid_negative_threshold | invalid | invalid | 0.00 | 0.00 | PASS |
| worker_exact_threshold | - | terminated | 10.00 | 10.00 | OK |
| worker_high_context_only | terminated | terminated | 6.00 | 5.00 | PASS |
| worker_high_coordination_only | terminated | terminated | 10.00 | 5.00 | PASS |
| worker_high_error_only | terminated | terminated | 10.00 | 5.00 | PASS |
| worker_invalid_nan_metric | invalid | invalid | 0.00 | 0.00 | PASS |
| worker_invalid_negative_metric | invalid | invalid | 0.00 | 0.00 | PASS |
| worker_just_below_threshold | - | survived | 9.00 | 10.00 | OK |
| worker_lenient_profile_survives | survived | survived | 3.00 | 100.00 | PASS |
| worker_strict_profile_breach | terminated | terminated | 6.00 | 3.00 | PASS |
| worker_survives | - | survived | 4.00 | 10.00 | OK |
| worker_threshold_breach | - | terminated | 12.00 | 10.00 | OK |
| worker_zero_metrics | survived | survived | 0.00 | 10.00 | PASS |

## Receipt Output Path

`receipts/out/`
