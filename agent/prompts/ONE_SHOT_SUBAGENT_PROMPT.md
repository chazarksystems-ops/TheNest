# One-Shot Subagent Prompt

Use this when handing one slice to an IDE agent or coding subagent.

```text
You are working on one bounded slice of the `kimi_hive_p0_test` Rust workspace.

This is a quick personal-tool development project, not an engineer-grade production build.
Do not over-engineer. Do not ask for routine reviews. Proceed with the assigned slice unless there is a true blocker, conflicting instruction, major architecture fork, or unsafe/destructive action.

Project context:
The project is a P0 Rust lifecycle primitive proving:
worker health metrics -> threshold breach -> ownership-consuming worker termination -> receipt payload emitted.

Core concepts:
- Nociceptor calculates suffering score.
- CattleWorker owns a Nociceptor.
- CattleWorker::tick(self) consumes the worker.
- WorkerOutcome::Survived(CattleWorker) returns a surviving worker.
- WorkerOutcome::Terminated(EpigeneticPayload) emits a receipt when the worker terminates.
- Apoptosis::trigger_apoptosis(self) consumes the worker and creates the payload.
- EpigeneticPayload is the termination receipt.

Your assignment is the attached slice file only.

Rules:
- Touch only allowed files.
- Create only allowed new files.
- Do not add runtime, networking, model serving, async orchestration, database, scheduler, queue, GPU logic, or unrelated architecture.
- Do not modify target/ or generated build artifacts.
- Do not continue into adjacent slices.
- Keep changes practical and minimal.

Validation:
Run from workspace root:
- cargo fmt --all --check
- cargo check --workspace
- cargo test --workspace

Final report:
Use the report format from the slice file.
```

## Scope clarification

Relevant files are context only. They are read-only unless also listed under `Allowed edits` or `Allowed new files`.
Use dependency order, not filename order, when selecting follow-up work.
