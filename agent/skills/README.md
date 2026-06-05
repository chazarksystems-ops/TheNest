# Skills Index

These skills are small task routers for low-context personal-tool development. They are not a formal agent framework.

| Skill | Purpose |
|---|---|
| `next_slice_picker` | Choose the next slice using dependency order, completed work, and blockers. |
| `slice_runner` | Run exactly one bounded slice with minimal context. |
| `drift_sentinel` | Check for forbidden runtime/orchestration/process drift. |
| `validation_runner` | Run and summarize lightweight Cargo validation for implementation slices. |
| `report_compressor` | Compress verbose reports into compact checkpoints. |
| `context_card_updater` | Keep `ops/CURRENT_CONTEXT_CARD.md` current. |
| `receipt_shaper` | Handle only receipt/payload evolution slices. |
| `nociceptor_shaper` | Handle only scoring/config/metrics cleanup slices. |
| `handoff_packager` | Keep source handoffs clean and exclude generated artifacts. |
| `slice_generator` | Create more bounded slice specs when needed. |

Use one skill with one concrete slice whenever possible.
