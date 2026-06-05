# Future Roadmap — Parking Lot

This file captures ideas that are allowed for future development, and a permanent list of work that is out of scope for TheNest.

---

## Allowed future work

These items are consistent with TheNest's design as a local, file-based, synchronous Rust workbench. They may be picked up in future phases.

- **Richer scenario editor** — text UI or simple file watcher that reloads scenarios on save
- **More receipt report formats** — CSV export, alternate Markdown table variants
- **More benchmark comparison tooling** — diff two Criterion runs, flag regressions automatically
- **Local terminal UI for scenario review** — optional, low priority, read-only display
- **Richer worker models** — additional metric dimensions beyond bloat/error/debt
- **Optional multi-worker batch scenario runner** — run N workers against the same scenario in a single pass
- **Scenario import/export helpers** — convert between formats or validate schema versions
- **Versioned scenario files with schema evolution** — forward-compatible JSON schema with explicit version field

---

## Permanently out of scope

The following will never be part of TheNest. If a proposed change touches any of these, it must be rejected.

| Forbidden area | Reason |
|---|---|
| Scheduler or task queue | TheNest runs synchronously; no job queuing |
| HTTP server or REST API | Local tool only; no network surface |
| Database (SQLite, Postgres, embedded DB) | File-based receipts are sufficient; no DB |
| Network runtime or socket communication | No IPC, no sockets |
| Model-serving or LLM inference | Out of domain entirely |
| Distributed swarm runtime | Single-process local tool |
| Tokio or async runtime | Synchronous-only design |
| PyO3 or Python bindings | Rust-only codebase |
| CI/CD pipeline tooling | Not a CI system |
| Enterprise governance framework | Not an enterprise product |

> [!WARNING]
> If a dependency or feature pull request introduces any of the forbidden items above, reject it and reference this file.
