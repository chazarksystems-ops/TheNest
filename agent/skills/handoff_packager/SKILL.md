# handoff_packager

## WHEN TO USE

Use when creating a clean source handoff or checking handoff shape.

## INPUTS

- project root
- `.gitignore`
- packaging cleanup notes or slice spec

## ALLOWED WORK

- Exclude `target/` and generated artifacts.
- Include source, docs, receipts, scripts, skills, ops, templates, prompts, and references as appropriate.
- Emit a compact inventory.

## FORBIDDEN WORK

- Do not include compiled artifacts.
- Do not inspect or rely on binaries in `target/`.
- Do not modify Rust behavior.
- Do not create a zip unless explicitly assigned.

## OUTPUT FORMAT

```text
HANDOFF_PACKAGE:
ZIP_PATH:
INCLUDED:
EXCLUDED:
WARNINGS:
```

## STOP CONDITION

Stop after the handoff inventory or package report.
