# Personal Tool Development Doctrine

This project is for fast development of a personal tool.

It is not an engineer-grade production build, enterprise system, formal framework, or compliance-heavy architecture project.

The goal is to make useful progress quickly while keeping the code understandable, runnable, and easy to change.

## Working style

Subagents should:

- complete the assigned slice directly
- make practical, minimal changes
- avoid over-design
- avoid unnecessary abstractions
- avoid turning small improvements into architecture projects
- avoid asking for reviews unless there is a real blocker or major decision fork
- preserve the current project intent
- report what changed clearly when finished

## Review policy

Do not ask for review or approval for routine implementation choices.

Proceed using best judgment when the work is:

- local to the assigned slice
- reversible
- covered by basic validation
- not changing the project direction
- not adding runtime/orchestration/network/model-serving behavior
- not deleting user-authored source material

Ask for review only if:

- the slice instructions conflict
- a requested change would break existing behavior
- a major architectural fork is required
- files outside the allowed scope must be edited
- a dependency with major project impact is needed
- validation exposes a serious failure that cannot be fixed locally

## Validation expectation

Validation should be lightweight and practical.

Prefer:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

Do not create heavy validation frameworks, formal certification layers, complex approval gates, or excessive documentation unless the slice specifically asks for them.

## Documentation expectation

Docs should be short and useful.

Write enough for a future agent or user to understand:

- what this slice changed
- how to run it
- what should not be expanded accidentally

Do not create long policy documents, enterprise-grade governance, or unnecessary review processes.

## Stop condition

Complete the assigned slice, run basic validation, provide a concise final report, and stop.

Do not ask what to do next unless there is a genuine blocker.
Do not expand into adjacent slices.
Do not turn this personal tool into a formal engineering platform.
