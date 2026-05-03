# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

FounderAI is an autonomous agent daemon written in Rust. It runs a team of AI agents (named after saints) to handle conservation biology outreach, grant writing, lead nurturing, and production work for Techni-Drones Madagascar / ERIS. The system uses Ollama (local) or OpenAI as providers, with file-based approvals, inbox/outbox, and run artifacts as the primary interface.

## Build & Run Commands

```bash
# Build
cargo build --release

# Run daemon (polls every 60s)
cargo run -- daemon

# Single tick (one execution cycle)
cargo run -- tick

# Trigger a specific job by name
cargo run -- trigger <job-name>

# Web console (port 8080, local only)
cargo run -- serve --port 8080

# Check system status
cargo run -- status

# List pending approvals
cargo run -- approvals

# Approve / reject a pending item
cargo run -- approve <id>
cargo run -- reject <id>

# Submit a work request to inbox
cargo run -- request --role a-outreach --body "..."

# Use alternate config (e.g. smoke test)
cargo run -- --config config/founderai.smoke.json tick
```

Tests are in `src/improvement.rs` (inline unit tests):
```bash
cargo test
```

Smoke test with isolated workspace:
```bash
./scripts/run-smoke-test.sh       # Linux
./scripts/run-smoke-test.ps1      # Windows
```

## Architecture

### Core Loop

`main.rs` → `AutonomyApp` (app.rs) → `tick()` → dispatches jobs → `run_worker()` (worker.rs) → writes run artifacts to `runtime/runs/<run-id>/`

The daemon is a single-threaded polling loop (no async) with a file lock (`runtime/founderai.lock`) preventing duplicate instances.

### Key Modules

| Module | Responsibility |
|--------|----------------|
| `src/app.rs` | Main daemon loop, job scheduling, approval workflow, special agent orchestration |
| `src/worker.rs` | Prompt packet assembly, provider calls (Ollama/OpenAI), run artifact writing |
| `src/config.rs` | Config deserialization from `config/founderai.json` |
| `src/improvement.rs` | Continuous improvement: achievement log, customer feedback, backlog generation |
| `src/marketing.rs` | Funnel state sync, marketing brief generation |
| `src/model_router.rs` | Routes task types to primary/fallback providers |
| `src/approvals.rs` | File-based approval payload management |
| `src/agents/` | Special agents: Perpetua (nurture), Pio (deadlines), Zacchaeus (inbound leads) |
| `src/web.rs` | Private HTTP console on 127.0.0.1:8080 |

### Team Model

6 canonical lanes across 3 teams: A-Outreach, A-Production, B-Outreach, B-Production, C-Outreach, C-Production. Overlay agents (Bartholomew for grants, Pio for deadlines, Perpetua for nurture, Hildegard/Francis/Clare/Columban for specialized workflows) operate on top.

### Approval System

Work tagged with `external-send`, `publish`, `financial`, `destructive-write`, or `calendar-commitment` requires human approval. Approval policy is one of `never`, `before_run`, or `after_run`. Pending approvals are files in `runtime/approvals/`. Helper scripts are generated for Windows/Linux approval actions.

### Provider Routing

`model_router.rs` routes by task type (draft, qa_check, briefing, final_review, proposal, grant, scheduler). Ollama is the default; OpenAI or Claude can be used for expensive tasks or when Ollama is unavailable. The offline queue (`src/offline.rs`) holds jobs when the network is down.

Three providers are supported in `worker.rs`:
- `"ollama"` — local model via HTTP (`FOUNDERAI_BASE_URL`, default `http://localhost:11434`)
- `"openai"` — OpenAI Responses API (`OPENAI_API_KEY`)
- `"claude"` / `"anthropic"` — Anthropic Messages API (`ANTHROPIC_API_KEY`, default model `claude-sonnet-4-6`)

To use Claude as the online provider in `config/founderai.json`:
```json
"worker": {
  "provider": "claude",
  "base_url": "https://api.anthropic.com/v1",
  "model": "claude-sonnet-4-6",
  "api_key_env": "ANTHROPIC_API_KEY"
}
```
Or via environment: `FOUNDERAI_PROVIDER=claude FOUNDERAI_MODEL=claude-sonnet-4-6 FOUNDERAI_API_KEY_ENV=ANTHROPIC_API_KEY`

### Prompt Assembly

`worker.rs::build_prompt()` assembles a full context packet including:
- `founder-brain/` identity, knowledge, team structure, workflows, output patterns
- ERIS knowledge, Hormozi protocols, QA rubrics, risk register, governance constraints
- Agent roster from `config/agents.json` + selected agent's prompt from `founder-brain/prompts/`
- Agent-ready documents from `documents/99_Agent_Ready/` (targeted by agent_id)
- Role-specific work packets with metrics

### Run Artifacts

Each run writes to `runtime/runs/<run-id>/`:
- `prompt.md` — full prompt packet sent to provider
- `output.md` — generated content
- `metadata.json` — provider, role, agent, token counts, timing
- `stdout.txt` / `stderr.txt` — provider call details

### Configuration

Primary config: `config/founderai.json`. Cloud variant: `config/founderai.cloud.json`. Smoke/test: `config/founderai.smoke.json`. Agent definitions: `config/agents.json`. Individual agent prompts live in `founder-brain/prompts/<name>.md`.

### Continuous Improvement

`src/improvement.rs` writes three files under `runtime/improvement/`:
- `achievement_log.json` — run signals and delivery outcomes
- `customer_feedback.json` — manual feedback + delivery signals
- `improvement_backlog.json` — heuristic backlog items

### Key Directories at Runtime

```
inbox/          incoming .json request files
outbox/         completed outputs
runtime/
  runs/         per-run artifact folders
  state.json    persistent job/role state
  approvals/    pending approval payloads
  improvement/  achievement log, feedback, backlog
  marketing/    funnel snapshots, briefs
  grants/       Bartholomew grant artifacts
sales/          delivery logs (CSV), customer feedback log
```
