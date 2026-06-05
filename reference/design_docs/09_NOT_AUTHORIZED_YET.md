# 09 — Not Authorized Yet

The following are intentionally captured but not authorized in P0.

## Model runtime operations

- Launching vLLM.
- Launching TensorRT-LLM.
- Launching llama.cpp or mistral-rs model runtime.
- Loading local 70B/8B models.
- Starting multi-endpoint model services.
- Creating `launch_hive.sh` as an executable runtime script.

## Training operations

- Axolotl config execution.
- QLoRA training.
- DeepSpeed training.
- LoRA merge.
- Hugging Face model download.
- Synthetic dataset generation at scale.

## Observability stack

- Prometheus.
- Grafana.
- Docker Compose.
- TUI dashboard.
- Metrics servers.

## Data operations

- Million-row data generation.
- External API calls.
- Dataset publication.
- Training on generated reasoning traces without verifier-backed schema.

## File-system operations

- Delete/cleanup/archive/quarantine of unrelated files.
- Modifying existing project repos.
- Moving files outside the new P0 workspace.

## Why these are excluded

The source files contain useful future concepts, but P0 must first prove the local Rust lifecycle spine. Later packets can authorize these tracks with separate validation gates.
