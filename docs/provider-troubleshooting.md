# Provider Troubleshooting

FounderAI can report a provider as reachable while generation still fails. That
usually means the health check succeeded but the actual generation request was
too slow, the model was not loaded cleanly, or the provider stalled on a large
prompt packet.

Recent hardening added two things worth checking first:

- every run now records `Prompt size` in `stdout.txt` and `prompt_chars` /
  `prompt_words` in `metadata.json`
- oversized JSON references are compacted before prompt injection, so prompt
  growth should be more controlled across outreach and planning runs

## Quick Checks

1. Run `status` and confirm the intended provider and model.
2. If using Ollama, run `ollama list` and confirm `qwen2.5:7b-instruct` is installed.
3. If using OpenAI, confirm `OPENAI_API_KEY` is loaded in the environment.
4. Inspect the latest run folder:
   - `stderr.txt`
   - `stdout.txt`
   - `output.md`
   - `metadata.json`

5. If you are using a runtime override such as `FOUNDERAI_TIMEOUT_SECONDS`,
   confirm it is visible in `stdout.txt` as `Primary timeout seconds` or
   `Fallback timeout seconds`.

## If Ollama Is Reachable But Generation Times Out

- Increase `worker.timeout_seconds`
- Or set `FOUNDERAI_TIMEOUT_SECONDS` for a temporary global override during a
  smoke run
- Retry with the isolated smoke config
- Confirm the machine has enough RAM for the chosen model
- Try a direct local prompt with `ollama run qwen2.5:7b-instruct`
- If the host is too slow, switch temporarily to the OpenAI provider

## Recommended Timeout Baselines

- normal local runtime: `900`
- isolated smoke workspace: `1800`
- remote OpenAI provider: `300` to `900`, depending on network conditions

## Smoke Workspace

Use `config/founderai.smoke.json` when you want to test generation without
touching the main `inbox/`, `outbox/`, or `runtime/` folders.

The smoke config uses:

- `inbox-smoke/`
- `outbox-smoke/`
- `runtime-smoke/`

This keeps debugging bounded and prevents old inbox items from dominating a
single verification run.

Smoke wrapper scripts:

- Windows: `scripts/run-smoke-test.ps1`
- Linux: `scripts/run-smoke-test.sh`
