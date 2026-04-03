# FounderAI Run Blocked

Ollama generation failed for this run.

- Provider: ollama
- Base URL: http://localhost:11434
- Model: qwen2.5:7b-instruct
- Reason: Ollama returned HTTP 404 Not Found: {"error":"model 'qwen2.5:7b-instruct' not found"}

## Safe Recovery

- Confirm Ollama is running locally.
- Confirm the configured model exists locally: `ollama pull qwen2.5:7b-instruct`
- Re-run the FounderAI tick after Ollama is healthy.
