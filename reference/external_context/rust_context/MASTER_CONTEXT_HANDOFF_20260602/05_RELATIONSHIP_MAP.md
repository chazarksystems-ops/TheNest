# Relationship Map

## System Families

### Squeaky School Family
Squeaky School is the living archive/reference family. The recovered handoff review, doctrine decision pass, schema reconciliation pass, and schema patch proposal pass are downstream review/proposal materials around that canon boundary.

### HIVE Family
HIVE P0 is a separate implementation/build packet focused on a Rust P0 skeleton and parallel workstreams.

### SqueakyBot Family
SqueakyBot is a separate tooling/implementation repo with roadmap and design material that may intersect other implementation contexts but should not be merged into doctrine by default.

### Consus / Rust Toolbox Family
The accepted checkpoint is the baseline. The expanded tasks folder adds Phase 2 summary and future-planning direction and should be interpreted relative to the checkpoint, not above it.

## Dependency / Influence Map

```text
Squeaky School (living archive/reference)
  -> recovered handoff review
      -> doctrine decision pass
      -> schema reconciliation pass
      -> schema patch proposal pass (still blocked pending doctrine approval)

HIVE P0
  -> separate implementation/build context

SqueakyBot
  -> separate tooling/implementation context

Consus accepted checkpoint
  -> baseline for evaluating expanded tasks
Consus expanded tasks
  -> Phase 2 summary + Phase 3 planning extension
```

## Likely Join Points
- Squeaky School doctrine versus review-pass authority.
- Shared tooling and Rust-pattern comparison across HIVE, SqueakyBot, and Consus.
- Consus checkpoint-to-expanded-planning comparison.

## Areas That Must Stay Separate
- Squeaky School must remain a living reference/archive rather than an implementation repo.
- Schema patch proposal remains review-only unless doctrine approval is explicit.
- HIVE and SqueakyBot should remain separate implementation contexts unless a later human decision joins them.

## Open Questions
- Which review pass is currently accepted, if any?
- How much of HIVE P0 is phase-local versus reusable long-term baseline?
- Is SqueakyBot intended to integrate with HIVE, Consus, or neither?
- Is Consus expanded planning already ratified anywhere, or still forward-looking?
