# Do Not Drift Check-List

To preserve fast personal-tool development, the following components are explicitly **FORBIDDEN** from being added to the repository:

- Tokio or async runtime (unless required as simple dev-dependency in tests, not in library)
- HTTP server (actix, axum, etc.)
- Database layer (sqlx, diesel, sqlite, postgres, etc.)
- Worker scheduler or task orchestrator
- Queue systems (celery, redis, rabbitmq, etc.)
- Swarm runtimes or actor framework (actix-actor, etc.)
- Model serving or inference integrations (vLLM, llama.cpp, mistral-rs, etc.)
- LLM calls or model API requests
- PyO3 or Python integration
- GPU logic or CUDA bindings
- Networking or distributed coordination
- Formal policy engine
- Enterprise governance docs or heavy CI/CD configuration
