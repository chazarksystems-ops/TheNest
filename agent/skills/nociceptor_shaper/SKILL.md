# nociceptor_shaper

## WHEN TO USE

Use for scoring, config, and health-metric cleanup slices only.

## INPUTS

- relevant `slices/SLICE_02_*.md`
- `swarm_core/src/nociceptor.rs`
- `swarm_core/src/payload.rs` if the slice explicitly allows it
- related tests in files allowed by the slice

## ALLOWED WORK

- `latency_weight` -> `coordination_debt_weight`
- `NociceptorConfig`
- `WorkerHealthMetrics`
- numeric validation
- suffering breakdown

## FORBIDDEN WORK

- Do not change scoring math unless a slice explicitly says so.
- Do not change lifecycle ownership behavior.
- Do not add runtime, serde, TOML, CLI, or persistence unless the selected slice explicitly allows it.

## OUTPUT FORMAT

Use the selected slice report format.

## STOP CONDITION

Stop after the assigned scoring/config slice.
