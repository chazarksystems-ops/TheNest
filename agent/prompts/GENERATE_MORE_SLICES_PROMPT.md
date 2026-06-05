# Prompt To Generate More Slices

Use this when asking an agent to create additional slice files from the category map.

```text
Create additional bounded markdown slice files for the `kimi_hive_p0_test` personal Rust tool.

This is quick personal-tool development, not an engineer-grade production build.
Do not add unnecessary review gates, heavy process, or formal governance.

Project context:
The project is a P0 Rust lifecycle primitive proving worker health metrics -> threshold breach -> ownership-consuming worker termination -> receipt payload emitted.

Each slice must be self-contained and must include:
1. Mission
2. Personal tool doctrine
3. Objective
4. Relevant current files
5. Allowed edits
6. Allowed new files
7. Forbidden work
8. Required steps
9. Tests
10. Validation commands
11. Final report format
12. Stop condition

Each slice should be small enough for one coding subagent to complete without seeing the entire project.

Do not create broad tasks like "improve receipts" or "build runtime".
Create exact tasks like:
- add raw metrics to EpigeneticPayload
- add TerminationReason enum
- add JSON serialization test
- add validate.ps1 and validate.sh

Do not authorize runtime, networking, model serving, async orchestration, database, scheduler, queue, GPU logic, or unrelated architecture.
```
