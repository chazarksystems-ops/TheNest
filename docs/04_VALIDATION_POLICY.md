# Lightweight Validation Policy

This project does not need engineer-grade ceremony, but it should stay runnable.

## Default validation

Run from workspace root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## When all commands pass

Report PASS and stop.

## When one command fails

Fix locally if the failure is inside the assigned slice.

Do not ask for review unless:

- the fix requires changing files outside the allowed scope
- the failure exposes a design conflict
- the failure requires a major dependency or architecture change

## When validation cannot run

Report `NOT RUN` with a short reason.

Do not invent successful validation claims.

## No heavy validation expansion

Do not create:

- formal verification frameworks
- large validation matrices
- enterprise release gates
- CI systems
- certification-style reports

unless a slice explicitly asks for it.
