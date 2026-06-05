# validation_runner

## WHEN TO USE

Use after implementation slices that change the Rust workspace.

## INPUTS

- changed workspace
- `docs/04_VALIDATION_POLICY.md`

## ALLOWED WORK

Run the lightweight validation commands from the workspace root:

```bash
cargo fmt --all --check
cargo check --workspace
cargo test --workspace
```

## FORBIDDEN WORK

- Do not invent passing validation.
- Do not create CI, release gates, formal verification, or large validation frameworks.
- Do not fix failures outside the assigned slice unless explicitly allowed.

## OUTPUT FORMAT

```text
VALIDATION:
- cargo fmt --all --check: PASS/FAIL/NOT RUN
- cargo check --workspace: PASS/FAIL/NOT RUN
- cargo test --workspace: PASS/FAIL/NOT RUN

FAILURE_SUMMARY:
LOCAL_FIX_ATTEMPTED:
NEEDS_REVIEW: yes/no
```

## STOP CONDITION

Stop after reporting validation results and local failure summary.
