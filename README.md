# FounderAI-Ollama-Rust

FounderAI-Ollama-Rust is a practical Rust rebuild of the FounderAI autonomy
layer. It keeps the founder-brain identity, approvals, inbox and outbox
workflow, team routing, audit-friendly artifacts, and background daemon model
while making the provider switchable between Ollama, Claude, and OpenAI and the runtime
portable across Windows and Linux.

## Mission

Build a durable, low-cost FounderAI that:

- preserves the founder-brain identity and Franciscan charter
- preserves approvals for protected actions
- preserves file-based runs, outputs, logs, and approvals
- supports the current Techni-Drones Madagascar and ERIS operating reality
- runs on Windows or Linux
- can use Claude now, with Ollama fallback and OpenAI still available when needed, without changing the product shape

## Scope And Non-Goals

Scope:

- Rust daemon and CLI
- file-based runtime structure
- 3 teams and 6 role lanes
- workflow overlays for grants, scheduling, lead response, nurture, QA, and review
- provider switching between Ollama, Claude, and OpenAI
- Windows scripts, Linux scripts, Docker, and CI
- contributor-ready docs and governance

Non-goals:

- redesigning FounderAI into a generic agent platform
- replacing audit files with hidden infrastructure
- removing approvals for convenience
- building a full SaaS control plane
- adding speculative frameworks before reliability work is done

## What Stays The Same

- `founder-brain/` remains the source of truth for founder identity, knowledge, workflows, output patterns, and team structure.
- `inbox/`, `outbox/`, and `runtime/` stay as plain inspectable folders.
- The 3-team and 6-role operating model stays intact:
  - `A-Outreach`
  - `A-Production`
  - `B-Outreach`
  - `B-Production`
  - `C-Outreach`
  - `C-Production`
- Overlay agents extend the workflow without replacing the six lanes:
  - `Bartholomew`
  - `Pio`
  - `Zacchaeus`
  - `Perpetua`
  - `Hildegard`
  - `Clare`
  - `Francis`
  - `Columban`
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
  - `grant-pipeline-review`
  - `weekly-strategy-review`
  - `internet-recovery-review`

## What Changed

- The autonomy engine now lives in Rust under `src/`.
- The worker backend is provider-driven:
  - `claude` over `https://api.anthropic.com/v1`
  - `ollama` over `http://localhost:11434`
  - `openai` over `https://api.openai.com/v1`
- The default hosted model is `claude-sonnet-4-6`.
- Provider settings can be overridden by environment variables for cloud deployment.
- Linux launch scripts, Docker assets, and GitHub Actions build verification are included.
- A private browser control surface is available through `serve` for cloud use.
- Failures are written into run artifacts instead of crashing the daemon.
- Founder context is synced to the April 2026 Techni-Drones and ERIS V4 documents.
- Deadline tracking now lives in `config/pio_deadlines.json` and surfaces through Pio-generated inbox requests.
- Grant drafting now has a dedicated `runtime/grants/` artifact path.
- Achievements, feedback, and backlog signals now sync into `runtime/improvement/`
  with a weekly retrospective artifact.

## No-Budget Delivery Stack

- GitHub for source, issues, pull requests, Actions, and Pages
- Markdown docs in `docs/` for zero-cost hosting
- Docker compose files for low-cost Linux deployment
- Claude for the default hosted provider path
- Ollama for self-hosted local inference when compute is available
- OpenAI for simpler hosted inference when local models are impractical

## Folder Layout

- `founder-brain/`: preserved founder context copied from the Python reference workspace
- `config/founderai.json`: live runtime config
- `config/founderai.example.json`: starter copy
- `config/pio_deadlines.json`: deadline tracker for grants and operational commitments
- `docs/`: public project charter, roadmap, risks, and volunteer onboarding
- `docs/cloud-deployment.md`: cheapest VPS deployment path with Cloudflare Access
- `docs/cloud-operations.md`: backup, restore, and weekly review runbook
- `documents/99_Agent_Ready/`: curated references, databases, and templates loaded into prompt packets for agent work
- `src/`: Rust autonomy engine
- `scripts/start-founderai.ps1`: hidden or background launcher for Windows
- `scripts/stop-founderai.ps1`: Windows stop helper
- `scripts/bootstrap-ollama-model.ps1`: Windows helper to pull the default Ollama model
- `scripts/start-founderai.sh`: Linux launcher
- `scripts/stop-founderai.sh`: Linux stop helper
- `scripts/bootstrap-ollama-model.sh`: Linux helper to pull the default Ollama model
- `scripts/bootstrap-cloud-layout.sh`: prepares the `/srv/founderai` root-disk layout for the VPS
- `scripts/cloud-backup.sh`: creates local and optional Backblaze-ready backup archives
- `scripts/cloud-weekly-review.sh`: prints uptime, disk use, approvals, and prompt/token summaries
- `scripts/founderai.service.example`: systemd example for Linux deployment
- `.github/workflows/build.yml`: Windows and Linux build verification
- `docker-compose.openai.yml`: Docker deployment using OpenAI
- `docker-compose.ollama.yml`: Docker deployment using Ollama
- `docker-compose.cloud.yml`: private VPS deployment with daemon, web console, and Cloudflare tunnel
- `.env.cloud.example`: starter environment file for the VPS
- `inbox/`: drop `.json`, `.md`, or `.txt` requests here
- `outbox/`: FounderAI run copies
- `runtime/`: logs, state, runs, approvals, and team outputs
- `runtime/grants/`: grant draft copies generated for Bartholomew-driven work
- `runtime/improvement/`: achievement log, customer feedback log, improvement backlog, and weekly retrospectives

## Project Docs

- Live docs: [hajafabrice.github.io/founderAiCheap](https://hajafabrice.github.io/founderAiCheap/)
- Public storefront root: `docs/index.html` for the OPLURIX Netlify or Pages launch
- [Claude Code instructions](CLAUDE.md)
- [App spec](docs/app-spec.md)
- [Data dictionary](docs/data-dictionary.md)
- [Brand brief](docs/brand-brief.md)
- [Feature backlog](docs/feature-backlog.md)
- [Integrations](docs/integrations.md)
- [Errors log](docs/errors-log.md)
- [Foundation compliance audit](docs/foundation-compliance-audit.md)
- [Project charter](docs/project-charter.md)
- [Roadmap](docs/roadmap.md)
- [Risk register](docs/risk-register.md)
- [Volunteer playbook](docs/volunteer-playbook.md)
- [Contributor backlog](docs/contributor-backlog.md)
- [GitHub admin checklist](docs/github-admin-checklist.md)
- [Provider troubleshooting](docs/provider-troubleshooting.md)
- [Cloud deployment](docs/cloud-deployment.md)
- [Cloud operations](docs/cloud-operations.md)
- [Continuous improvement](docs/continuous-improvement.md)
- [AI-assisted development safety](docs/ai-assisted-development-safety.md)
- [Independent services landing page (EN)](docs/independent-services-en.md)
- [Independent services landing page (FR)](docs/services-fr.md)
- [Independent services one-pager](docs/services-one-pager.md)
- [Dataset cleaning case study](docs/case-study-biodiversity-dataset-cleaning.md)
- [Netlify-ready OPLURIX storefront](docs/index.html)
- [Contributing guide](CONTRIBUTING.md)
- [Governance](GOVERNANCE.md)
- [Security policy](SECURITY.md)
- [Code of conduct](CODE_OF_CONDUCT.md)

## Provider Config

Claude-first mode in `config/founderai.json`:

```json
"worker": {
  "provider": "claude",
  "base_url": "https://api.anthropic.com/v1",
  "model": "claude-sonnet-4-6",
  "timeout_seconds": 600,
  "system_prompt": "You are FounderAI's autonomous provider worker. Follow the prompt packet exactly and write only the requested final deliverable.",
  "api_key_env": "ANTHROPIC_API_KEY"
}
```

Temporary Ollama mode via environment override:

```powershell
$env:FOUNDERAI_PROVIDER="ollama"
$env:FOUNDERAI_BASE_URL="http://localhost:11434"
$env:FOUNDERAI_MODEL="qwen2.5:3b-instruct"
$env:FOUNDERAI_TIMEOUT_SECONDS="900"
```

Supported environment overrides:

- `FOUNDERAI_PROVIDER`
- `FOUNDERAI_BASE_URL`
- `FOUNDERAI_MODEL`
- `FOUNDERAI_TIMEOUT_SECONDS`
- `FOUNDERAI_SYSTEM_PROMPT`
- `FOUNDERAI_API_KEY_ENV`

FounderAI now auto-loads `.env` and `.env.local` from the repo root at process startup. Use `.env.example` as a starting point and keep real API keys out of committed files.

Pio reads deadline state from `config/pio_deadlines.json` and turns due alerts
into inspectable inbox items instead of silent reminders.

## Build

Windows:

```powershell
cargo build --release
```

Linux:

```bash
cargo build --release
```

## Quality Checks

Check the static docs site for broken local links:

```powershell
.\scripts\check-docs-links.ps1
```

Optionally include local anchor validation:

```powershell
.\scripts\check-docs-links.ps1 -CheckAnchors
```

## Run

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

Local private web console:

```powershell
.\target\release\founderai-ollama-rust.exe serve --config .\config\founderai.json --listen 127.0.0.1:8080
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

## Provider Bootstrap

Windows:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\bootstrap-ollama-model.ps1
```

Linux:

```bash
./scripts/bootstrap-ollama-model.sh
```

Windows isolated smoke run:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\run-smoke-test.ps1
```

Linux isolated smoke run:

```bash
./scripts/run-smoke-test.sh
```

Claude path for local runs:

Windows:

```powershell
$env:FOUNDERAI_PROVIDER="claude"
$env:FOUNDERAI_BASE_URL="https://api.anthropic.com/v1"
$env:FOUNDERAI_MODEL="claude-sonnet-4-6"
$env:ANTHROPIC_API_KEY="your-key"
```

Linux:

```bash
export FOUNDERAI_PROVIDER="claude"
export FOUNDERAI_BASE_URL="https://api.anthropic.com/v1"
export FOUNDERAI_MODEL="claude-sonnet-4-6"
export ANTHROPIC_API_KEY="your-key"
```

If you want the OpenAI path instead, set:

Windows:

```powershell
$env:FOUNDERAI_PROVIDER="openai"
$env:FOUNDERAI_BASE_URL="https://api.openai.com/v1"
$env:FOUNDERAI_MODEL="gpt-5-mini"
$env:OPENAI_API_KEY="your-key"
```

Linux:

```bash
export FOUNDERAI_PROVIDER="openai"
export FOUNDERAI_BASE_URL="https://api.openai.com/v1"
export FOUNDERAI_MODEL="gpt-5-mini"
export OPENAI_API_KEY="your-key"
```

## Isolated Smoke Workspace

Use `config/founderai.smoke.json` when you want a bounded smoke test that does
not touch the main `inbox/`, `outbox/`, or `runtime/` folders. The smoke config
uses its own `inbox-smoke/`, `outbox-smoke/`, and `runtime-smoke/` directories
and disables periodic jobs so one test request stays isolated.

The smoke wrapper scripts:

- create one bounded request
- run one isolated `tick`
- print the latest run folder
- print the paths to `prompt.md`, `output.md`, `stdout.txt`, and `stderr.txt`

If Ollama remains too slow during acceptance, keep the committed config on
`claude` and switch only the provider env vars for a bounded fallback test.

## Docker Deployment

Claude-backed container:

```bash
export ANTHROPIC_API_KEY="your-key"
docker compose -f docker-compose.claude.yml up -d --build
```

OpenAI-backed container:

```bash
export OPENAI_API_KEY="your-key"
docker compose -f docker-compose.openai.yml up -d --build
```

Ollama-backed container stack:

```bash
docker compose -f docker-compose.ollama.yml up -d --build
docker exec -it founderai-ollama ollama pull qwen2.5:3b-instruct
```

The compose files keep `inbox/`, `outbox/`, and `runtime/` mounted as host directories so FounderAI remains file-auditable in the cloud.

Private VPS cloud stack:

```bash
cp .env.cloud.example .env.cloud
./scripts/bootstrap-cloud-layout.sh /srv/founderai
docker compose -f docker-compose.cloud.yml up -d --build
```

Cloud deployment uses:

- `config/founderai.cloud.json`
- `serve` for the browser-only control surface
- Cloudflare Tunnel plus Cloudflare Access for private ingress
- Claude as the default cloud provider
- the same file-based inbox, outbox, runtime, and approval artifacts

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

- `src/config.rs`: mirrors the Python config loader and preserves the job and role schema, with cloud-friendly provider overrides and deadline tracking config.
- `src/app.rs`: mirrors the daemon loop, job scheduling, inbox ingestion, approvals, deadline routing, state updates, and status output.
- `src/worker.rs`: mirrors prompt-packet assembly and run artifact creation, with a provider switch between Ollama and OpenAI and dedicated grant artifacts.
- `src/approvals.rs`: preserves file-based approval payloads and summaries, and emits platform-appropriate helper scripts.
- `src/state.rs`: preserves `runtime/state.json`.
- `src/agents/pio.rs`: turns tracked deadlines into bounded inbox requests.
- `src/team_logging.rs`: preserves CSV and JSONL team activity logs.
- `src/singleton.rs`: preserves the single-daemon lock behavior across Windows and Linux.
- `scripts/*.ps1` and `scripts/*.sh`: preserve the launcher pattern while making deployment transferable.

## Community Direction

This repo is now structured for a no-budget, open collaboration model:

- MIT-licensed source
- CC BY 4.0 documentation
- contributor docs and governance
- issue templates and PR checklist
- GitHub Actions CI
- Markdown docs ready for GitHub Pages
- provider choice so infrastructure can scale with budget

## Licensing

- Code, scripts, and repo automation are licensed under the MIT License. See [LICENSE](LICENSE).
- Public-facing repository documentation is licensed under CC BY 4.0. See [LICENSE-docs.md](LICENSE-docs.md).
- `founder-brain/` content remains separately governed unless a file is explicitly marked for public reuse.
