# 08 — Data Factory Plan — Deferred P1

## Status

**DEFERRED / DESIGN ONLY**

This file captures the Grok source’s useful data-factory direction but does not authorize implementation in P0.

## P1 goal

Create a tiny data-factory smoke test that proves a Rust process can:

1. Read a small seed JSONL file.
2. Call a mocked local HTTP endpoint or preapproved local endpoint.
3. Validate output structure.
4. Write accepted records and rejected records separately.
5. Emit run receipts.

## P1 exclusions

- No million-row generation.
- No vLLM launch unless separately approved.
- No Axolotl.
- No LoRA.
- No training.
- No external API calls.
- No model downloads.

## Candidate record schema

```json
{
  "run_id": "string",
  "seed_id": "string",
  "vertical": "deductive_reasoning",
  "prompt": "string",
  "expected_answer": "string|null",
  "generated_answer": "string|null",
  "validator_status": "accepted|rejected|error",
  "validator_reason": "string",
  "source_license": "string|null",
  "generation_model": "string|null",
  "validator_model": "string|null",
  "created_unix_ms": 0
}
```

## Required P1 validation

- Five-row smoke fixture.
- One forced rejected row.
- One forced malformed row.
- Accepted/rejected JSONL files must be separate.
- Run summary must include counts.

## Router track

The suffering-aware router from the Grok source should become P2, after P1 proves receipts and basic endpoint validation. Its first version should use explicit route costs and health metrics rather than opaque model judgments.
