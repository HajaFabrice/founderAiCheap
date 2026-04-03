# FounderAI-Ollama-Rust

FounderAI-Ollama-Rust is a Rust rebuild of the existing FounderAI autonomy layer. It preserves the founder-brain identity, approvals, inbox/outbox workflow, team routing, audit-friendly artifacts, and background daemon model while making the AI backend switchable between Ollama and OpenAI and the runtime portable across Windows and Linux.

## What Stays The Same

- `founder-brain/` remains the source of truth for founder identity, knowledge, workflows, output patterns, and team structure.
- `inbox/`, `outbox/`, and `runtime/` stay as plain inspectable folders.
- The 3-team / 6-role operating model stays intact:
  - `A-Outreach`
  - `A-Production`
  - `B-Outreach`
  - `B-Production`
  - `C-Outreach`
  - `C-Production`
- Approval-sensitive work still pauses behind review for:
  - `external-send`
  - `publish`
  - `financial`
  - `destructive-write`
  - `calendar-commitment`
- Periodic jobs mirror the Python reference:
  - `startup-focus-brief`
  - `daily-team-orchestration`
  - `outreach-batch`
  - `production-batch`
  - `phd-literature-engine`
  - `weekly-strategy-review`
  - `internet-recovery-review`

## What Changed

- The app logic now lives in Rust under `src/`.
- The worker backend is provider-driven:
  - `ollama` over `http://localhost:11434`
  - `openai` over `https://api.openai.com/v1`
- The default Ollama model is `qwen2.5:7b-instruct`.
- Provider settings can be overridden by environment variables for cloud deployment.
- Linux launch scripts, Docker assets, and GitHub Actions build verification are included.
- Failures are written into run artifacts instead of crashing the whole daemon.

## Folder Layout

- `founder-brain/`: preserved founder context copied from the Python reference workspace
- `config/founderai.json`: live runtime config
- `config/founderai.example.json`: starter copy
- `src/`: Rust autonomy engine
- `scripts/start-founderai.ps1`: hidden/background launcher for Windows
- `scripts/stop-founderai.ps1`: Windows stop helper
- `scripts/start-founderai.sh`: Linux/macOS launcher
- `scripts/stop-founderai.sh`: Linux/macOS stop helper
- `scripts/founderai.service.example`: systemd example for Linux deployment
- `.github/workflows/build.yml`: Windows/Linux release build verification
- `docker-compose.openai.yml`: Docker deployment using OpenAI
- `docker-compose.ollama.yml`: Docker deployment using Ollama
- `inbox/`: drop `.json`, `.md`, or `.txt` requests here
- `outbox/`: FounderAI run copies
- `runtime/`: logs, state, runs, approvals, and team outputs

## Provider Config

Ollama mode in `config/founderai.json`:

```json
"worker": {
  "provider": "ollama",
  "base_url": "http://localhost:11434",
  "model": "qwen2.5:7b-instruct",
  "timeout_seconds": 300,
  "system_prompt": "You are FounderAI's autonomous provider worker. Follow the prompt packet exactly and write only the requested final deliverable.",
  "api_key_env": "OPENAI_API_KEY"
}
```

OpenAI mode:

```json
"worker": {
  "provider": "openai",
  "base_url": "https://api.openai.com/v1",
  "model": "gpt-5-mini",
  "timeout_seconds": 300,
  "system_prompt": "You are FounderAI's autonomous provider worker. Follow the prompt packet exactly and write only the requested final deliverable.",
  "api_key_env": "OPENAI_API_KEY"
}
```

When `provider` is `openai`, keep the API key in the environment rather than the config file.

Supported environment overrides:

- `FOUNDERAI_PROVIDER`
- `FOUNDERAI_BASE_URL`
- `FOUNDERAI_MODEL`
- `FOUNDERAI_TIMEOUT_SECONDS`
- `FOUNDERAI_SYSTEM_PROMPT`
- `FOUNDERAI_API_KEY_ENV`

OpenAI key examples:

Windows:

```powershell
$env:OPENAI_API_KEY="your-key"
```

Linux:

```bash
export OPENAI_API_KEY="your-key"
```

Provider switch without editing JSON:

```powershell
$env:FOUNDERAI_PROVIDER="openai"
$env:FOUNDERAI_BASE_URL="https://api.openai.com/v1"
$env:FOUNDERAI_MODEL="gpt-5-mini"
$env:OPENAI_API_KEY="your-key"
.\target\release\founderai-ollama-rust.exe status --config .\config\founderai.json
```

## Build On Windows

1. Install the Rust toolchain if `cargo` is not available yet.
2. Make sure Ollama is installed and running locally if you want the Ollama backend.
3. Pull the Ollama model if needed:

```powershell
ollama pull qwen2.5:7b-instruct
```

4. Build:

```powershell
cargo build --release
```

## Build On Linux

1. Install Rust.
2. Clone the repo.
3. Choose a provider:
   - local Ollama on the Linux host
   - OpenAI via `OPENAI_API_KEY`
4. Build:

```bash
cargo build --release
```

## Run Commands

Windows status:

```powershell
.\target\release\founderai-ollama-rust.exe status --config .\config\founderai.json
.\target\release\founderai-ollama-rust.exe status --config .\config\founderai.json --teams
```

Linux status:

```bash
./target/release/founderai-ollama-rust status --config ./config/founderai.json
./target/release/founderai-ollama-rust status --config ./config/founderai.json --teams
```

Windows single tick:

```powershell
.\target\release\founderai-ollama-rust.exe tick --config .\config\founderai.json
```

Windows daemon:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\start-founderai.ps1
```

Linux daemon:

```bash
./scripts/start-founderai.sh ./config/founderai.json
```

Windows stop:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\stop-founderai.ps1
```

Linux stop:

```bash
./scripts/stop-founderai.sh
```

Review approvals:

```powershell
.\target\release\founderai-ollama-rust.exe approvals --config .\config\founderai.json
.\target\release\founderai-ollama-rust.exe approve <approval-id> --config .\config\founderai.json
.\target\release\founderai-ollama-rust.exe reject <approval-id> --config .\config\founderai.json --notes "why"
```

Create an inbox request:

```powershell
.\target\release\founderai-ollama-rust.exe request `
  --config .\config\founderai.json `
  --title "Draft follow-up email for SEED Madagascar" `
  --body "Use my latest offer and keep it short." `
  --risk-tag external-send
```

## Docker Deployment

OpenAI-backed container:

```bash
export OPENAI_API_KEY="your-key"
docker compose -f docker-compose.openai.yml up -d --build
```

Ollama-backed container stack:

```bash
docker compose -f docker-compose.ollama.yml up -d --build
docker exec -it founderai-ollama ollama pull qwen2.5:7b-instruct
```

The compose files keep `inbox/`, `outbox/`, and `runtime/` mounted as host directories so FounderAI remains file-auditable in the cloud.

## Cloud-Ready Direction

- Keep the repo on GitHub.
- Use GitHub Actions to prove the Rust binary builds on both Windows and Linux.
- Run the daemon on a Linux VM, VPS, or other non-work machine using either:
  - Ollama on that host
  - OpenAI via `OPENAI_API_KEY`
- Keep `founder-brain/`, `inbox/`, `outbox/`, and `runtime/` as plain files so the system stays migratable and auditable.
- Use the same binary and config shape across operating systems; only provider settings and launcher style change.

## Suggested Smoke Test

1. Make sure the selected provider is reachable.
2. Create a bounded test request:

```powershell
.\target\release\founderai-ollama-rust.exe request `
  --config .\config\founderai.json `
  --title "Founder brain context smoke test" `
  --body "Summarize the founder priorities for B-Production in a concise actionable brief." `
  --role-id B-Production
```

3. Run one tick:

```powershell
.\target\release\founderai-ollama-rust.exe tick --config .\config\founderai.json
```

4. Inspect:

- `runtime/runs/<run-id>/prompt.md`
- `runtime/runs/<run-id>/output.md`
- `runtime/runs/<run-id>/stdout.txt`
- `runtime/runs/<run-id>/stderr.txt`
- `runtime/teams/B-Production/outputs/<run-id>.md`

## Architecture Mapping To The Python Reference

- `src/config.rs`: mirrors the Python config loader and preserves the job / role schema, with cloud-friendly provider overrides.
- `src/app.rs`: mirrors the daemon loop, job scheduling, inbox ingestion, approvals, state updates, and status output.
- `src/worker.rs`: mirrors prompt-packet assembly and run artifact creation, with a config-driven provider switch between Ollama and OpenAI.
- `src/approvals.rs`: preserves file-based approval payloads and summaries, and emits platform-appropriate helper scripts.
- `src/state.rs`: preserves `runtime/state.json`.
- `src/team_logging.rs`: preserves CSV and JSONL team activity logs.
- `src/singleton.rs`: preserves the single-daemon lock behavior across Windows and Linux.
- `scripts/*.ps1` and `scripts/*.sh`: preserve the launcher pattern while making deployment transferable.
