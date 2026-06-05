# Recommended Dependency Order

Use this order unless a later slice has an obvious local dependency.

```text
09 Packaging cleanup
00 Boundary and doctrine
02 Health scoring terminology cleanup
03 Payload and receipt hardening
05 Validation scripts/tests
04 Config and profile polish
08 Docs and handoff
06 Tiny CLI/demo
07 Future runtime notes
```

## First-pass order

1. Remove `target/` from clean handoff and add `.gitignore`.
2. Add project boundary / personal-tool doctrine.
3. Add do-not-drift note.
4. Rename `latency_weight` to `coordination_debt_weight`.
5. Split `NociceptorConfig` from `WorkerHealthMetrics`.
6. Add raw metrics to `EpigeneticPayload`.
7. Add `TerminationReason` enum.
8. Add serde support.
9. Add JSON serialization test.
10. Add validation scripts.

## Important sequencing note

Do not add a CLI, runtime, scheduler, queue, database, networking, or async orchestration before the core receipt primitive is clean.

## Dispatch note

Dependency order beats filename/alphabetical order. Do not dispatch slices by directory sorting alone.
