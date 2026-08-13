# Agency Agents Patterns For FounderAI

Updated: 2026-08-13

Purpose: extract useful patterns from
[msitarzewski/agency-agents](https://github.com/msitarzewski/agency-agents)
for the Rust FounderAI long-term goal assistant.

Scope reviewed:

- Local clone: `C:\Users\Student\Desktop\perso\agency-agents` at
  `ebe9c99acb5c96f9468de368d8bead775387d1a7`
- [README.md](https://github.com/msitarzewski/agency-agents/blob/main/README.md)
- [divisions.json](https://github.com/msitarzewski/agency-agents/blob/main/divisions.json)
- [tools.json](https://github.com/msitarzewski/agency-agents/blob/main/tools.json)
- [scripts/convert.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/convert.sh)
- [scripts/install.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/install.sh)
- [scripts/lib.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/lib.sh)
- [scripts/lint-agents.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/lint-agents.sh)
- [scripts/check-tools.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/check-tools.sh)
- [scripts/check-divisions.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/check-divisions.sh)
- [scripts/check-runbooks.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/check-runbooks.sh)
- [scripts/check-agent-originality.sh](https://github.com/msitarzewski/agency-agents/blob/main/scripts/check-agent-originality.sh)
- [scripts/build-hermes-plugin.py](https://github.com/msitarzewski/agency-agents/blob/main/scripts/build-hermes-plugin.py)
- [NEXUS Executive Brief](https://github.com/msitarzewski/agency-agents/blob/main/strategy/EXECUTIVE-BRIEF.md)
- [NEXUS runbooks.json](https://github.com/msitarzewski/agency-agents/blob/main/strategy/runbooks.json)
- [NEXUS Handoff Templates](https://github.com/msitarzewski/agency-agents/blob/main/strategy/coordination/handoff-templates.md)
- [NEXUS Agent Activation Prompts](https://github.com/msitarzewski/agency-agents/blob/main/strategy/coordination/agent-activation-prompts.md)
- [Multi-Agent Systems Architect](https://github.com/msitarzewski/agency-agents/blob/main/engineering/engineering-multi-agent-systems-architect.md)
- [Autonomous Optimization Architect](https://github.com/msitarzewski/agency-agents/blob/main/engineering/engineering-autonomous-optimization-architect.md)
- [Reality Checker](https://github.com/msitarzewski/agency-agents/blob/main/testing/testing-reality-checker.md)
- [Evidence Collector](https://github.com/msitarzewski/agency-agents/blob/main/testing/testing-evidence-collector.md)
- [Minimal Change Engineer](https://github.com/msitarzewski/agency-agents/blob/main/engineering/engineering-minimal-change-engineer.md)
- [Rust Refactoring Specialist](https://github.com/msitarzewski/agency-agents/blob/main/engineering/engineering-rust-refactoring-specialist.md)
- [Code Reviewer](https://github.com/msitarzewski/agency-agents/blob/main/engineering/engineering-code-reviewer.md)
- [Studio Producer](https://github.com/msitarzewski/agency-agents/blob/main/project-management/project-management-studio-producer.md)

Local comparison points:

- `src/config.rs`
- `src/model_router.rs`
- `src/worker.rs`
- `src/app.rs`
- `src/state.rs`
- `src/approvals.rs`
- `src/team_logging.rs`
- `config/agents.json`
- `config/founderai.cloud.json`
- `founder-brain/qa_rubrics.md`
- `docs/ai-assisted-development-safety.md`

## Executive Summary

| Rank | Takeaway | Impact | Feasibility | First move |
| --- | --- | --- | --- | --- |
| 1 | Add trace/span metadata per run step | High | High | Extend `metadata.json` and `team_activity.jsonl` with `trace_id`, `span_id`, `provider_attempts`, latency, status, and error classification. |
| 2 | Add lazy specialist routing | High | Medium | Create an `agent_catalog` module that indexes ERIS agents, prompt files, scopes, tags, and supported task types; load only selected agent context into prompts. |
| 3 | Add evidence-based QA gates and handoffs | High | High | Make Clare-style review require concrete artifacts, typed pass/fail evidence, retry counts, and structured handoff state. |

Immediate implementation order:

1. `src/trace.rs`: typed run telemetry and provider-attempt records.
2. `src/agent_catalog.rs`: lazy specialist index from `config/agents.json` + prompt frontmatter.
3. `src/qa_gate.rs`: reusable evidence contract for `qa_check`, `final_review`, `code-change`, and `publish`.
4. `src/model_router.rs`: cost/failure counters + circuit breaker state.

## Detailed Findings

### Architecture

Information gathered:

- `agency-agents` is not a runtime framework. It is a large, structured specialist catalog.
- Agents are source `.md` files with frontmatter metadata: `name`, `description`, `color`, often `emoji` and `vibe`.
- The repo keeps division metadata in `divisions.json`, a source-of-truth registry checked by scripts.
- `convert.sh` renders the same agent source into tool-specific formats: Codex TOML, OpenCode agents, Cursor rules, Qwen agents, Kimi YAML, Osaurus skills, Hermes plugin data.
- The Hermes integration avoids preloading hundreds of agents; it exposes a small fixed tool surface: search, inspect, load, delegate.

Analysis:

- The strongest architecture idea is separation between agent source, agent registry, and runtime invocation.
- The weak part for a real daemon is that most behavior remains prompt text, not typed state transitions.
- The conversion scripts are useful because they treat agent instructions as portable content with lintable metadata.
- Lazy loading is directly relevant to FounderAI because `build_prompt()` currently injects a very large prompt packet and then appends selected agent context.

Synthesis for FounderAI:

- Keep the saint-based roster, but formalize it as an agent catalog.
- Add searchable/task-routable metadata without turning FounderAI into a generic marketplace.
- Load only common governance + selected specialist bundles into `worker::build_prompt()`.
- Move "which references this agent needs" out of a large `match selected_agent_id` block and into metadata.

Current local fit:

- `config/agents.json` already contains `id`, `kind`, `job_scope`, model preferences, escalation rules, role mappings.
- `worker.rs::render_agent_ready_documents()` is the obvious refactor point.
- `worker.rs::render_selected_agent_context()` is the obvious lazy-load point.
- `model_router.rs::infer_task_type()` is the obvious search/query routing point.

### Capabilities

Information gathered:

- Agency agents define behavior through strong identity, mission, rules, deliverables, workflow, success metrics.
- Multi-Agent Systems Architect emphasizes context contracts, topology, HITL gates, traceability, and root-cause analysis.
- Evidence Collector and Reality Checker aggressively reject unsupported readiness claims.
- Autonomous Optimization Architect promotes providers only after historical performance evidence and uses hard cost guardrails/circuit breakers.
- Minimal Change Engineer creates an explicit anti-scope-creep operating mode.
- Rust Refactoring Specialist distinguishes broad coherent refactors from unrelated churn and demands verification.

Analysis:

- The effective pattern is not "more agents"; it is sharper agent contracts.
- FounderAI already has identity and approval safety, but agents still receive broad shared context.
- The repo's "personality" content is less useful than the operational contracts behind each persona.
- For long-term assistance, strongest capabilities to copy are: bounded mandate, evidence discipline, run scoring, escalation triggers, memory of failure patterns.

Synthesis for FounderAI:

- Add per-agent `capabilities`, `forbidden_actions`, `required_evidence`, `context_bundles`, and `success_metrics`.
- Add task-phase gates: plan -> execute -> evidence -> review -> approve.
- Make Clare/Francis reviews consume structured run telemetry, not only free-text output.
- Add "skeptical default" to approval summaries: pending output is not considered good until evidence says so.

Current local fit:

- `founder-brain/qa_rubrics.md` already has universal pass/fail rules.
- `docs/ai-assisted-development-safety.md` already defines AI code-safety discipline.
- `approvals.rs` already stores artifacts, risk tags, and summary.
- Missing: typed evidence payload and a way to fail an approval because evidence is absent.

### Implementation

Information gathered:

- `scripts/lib.sh` centralizes frontmatter parsing and slug generation.
- `lint-agents.sh` checks required metadata, line endings, recommended sections, and content length.
- `convert.sh` validates tool selection, cleans generated output, processes agent directories deterministically, and supports parallel conversion for independent targets.
- `build-hermes-plugin.py` creates an on-disk JSON index and lazy search/load/delegate functions.
- The Hermes router scores search by token overlap across name, description, division, vibe, and body prefix.
- `tools.json` is the canonical registry for supported tool targets, install scope, destination templates, render format, and install kind.
- `check-tools.sh` fails if `tools.json`, `convert.sh`, and `install.sh` disagree.
- `check-divisions.sh` fails if `divisions.json`, source directories, converter/linter arrays, and CI path filters drift apart.
- `check-runbooks.sh` validates machine-readable scenario runbooks against real agent slugs.
- `check-agent-originality.sh` flags duplicated or find-replace agent bodies through entity-neutralized shingle overlap.

Analysis:

- The scripts are simple, deterministic, auditable, and repo-native.
- The lazy router is the strongest implementation pattern for Rust adaptation.
- The shell scripts are not worth copying directly into the daemon, but their invariants are.
- The Rust system should not parse ad hoc YAML using string splitting inside the hot path; use typed JSON/TOML config or frontmatter parsed during startup.
- The registry checks are more important than their Bash implementation: the system treats drift as a build failure, not an operator surprise.
- The originality check is a useful anti-bloat pattern for any growing prompt/agent library, but should be adapted to FounderAI as "duplicate mandate detection," not generic text policing.

Synthesis for FounderAI:

- Add a startup lint/validation path for agent profiles.
- Add deterministic slug generation and duplicate detection for agent IDs, role IDs, and task routes.
- Add `agent_catalog.json` or extend `config/agents.json` with context bundle IDs.
- Add tests for duplicate slugs, missing prompt files, unknown context bundle references, and invalid task routes.
- Add a `context_bundles.json` registry so prompt assembly moves from Rust match arms to declarative data.
- Add a `workflow_runbooks.json` registry for repeatable modes: `code_change`, `publish_campaign`, `grant_review`, `daily_team_orchestration`, `incident_response`.

Current local fit:

- `state.rs` is file-based and easy to extend.
- `worker.rs` already writes per-run `metadata.json`, `prompt.md`, `stdout.txt`, `stderr.txt`, `output.md`.
- `team_logging.rs` already appends JSONL, suitable for trace events.
- `model_router.rs` already centralizes provider choice.
- `config/agents.json` can become the canonical roster if validation is added.
- `config/founderai.cloud.json` already has routes and provider preferences that can be checked against the roster and runbooks.

### Best Practices

Patterns worth adopting:

- Agent-as-contract, not agent-as-avatar.
- Frontmatter/metadata first; body second.
- Single source of truth for categories/divisions.
- Single source of truth for tool/provider/context contracts.
- Generated outputs must be cleaned before regeneration to prevent stale files.
- Lazy router for large catalogs.
- Required lint before install/use.
- Evidence-first QA with explicit proof artifacts.
- Structured handoffs between phases and agents.
- Maximum retry count before escalation.
- Machine-readable scenario runbooks with named rosters and activation phases.
- Originality/overlap checks to stop prompt-library bloat.
- Provider promotion based on measured history, not model prestige.
- Circuit breakers for cost/failure velocity.
- Minimal-change mode to prevent AI overreach.

Patterns already present locally:

- File-based audit trail.
- Human approval gates.
- Offline queue.
- Provider routing with fallback.
- Prompt artifacts saved per run.
- Founder-specific governance and anti-hype rubric.
- Team/role state.

Missing or partial:

- No trace/span hierarchy.
- No provider attempt list in metadata.
- No cost limit per run.
- No circuit-breaker state.
- No searchable specialist registry.
- No typed evidence object attached to approvals.
- No typed handoff object between agents, teams, or workflow phases.
- No runbook registry for recurring multi-agent workflows.
- No lint for `config/agents.json` vs prompt files.
- No context-bundle contract outside hardcoded Rust match arms.

## Specific Recommendations

### 1. Build a lazy FounderAI specialist catalog

What `agency-agents` does well:

- Keeps many specialists in source files.
- Uses frontmatter metadata for discovery.
- Provides search/inspect/load/delegate instead of preloading all agents.

Difference from typical Rust agent systems:

- Typical Rust agent systems hardcode roles into enums or route tables.
- `agency-agents` treats agents as data with deterministic renderers.

Why it improves FounderAI:

- Smaller prompts.
- Easier agent additions.
- Less hardcoded context logic in `worker.rs`.
- Better routing from inbox requests to the right saint/role.

Implementation:

- Add `src/agent_catalog.rs`.
- Extend `AgentProfile`:

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentProfile {
    pub id: String,
    pub saint_name: String,
    pub kind: String,
    pub job_scope: String,
    pub prompt_file: Option<String>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub task_types: Vec<String>,
    #[serde(default)]
    pub context_bundles: Vec<String>,
    #[serde(default)]
    pub required_evidence: Vec<String>,
    #[serde(default)]
    pub forbidden_actions: Vec<String>,
}
```

Integration points:

- `config.rs`: config structs and validation.
- `worker.rs::render_agent_ready_documents()`: replace `match selected_agent_id` with context bundles.
- `app.rs::build_inbox_job()`: route `agent_id` via catalog when request includes capability hints.
- `model_router.rs::infer_task_type()`: use profile task types before string heuristics.

Challenge:

- Needs migration from hardcoded context bundle match to data-driven bundles.

### 2. Add trace/span metadata

What `agency-agents` does well:

- Multi-agent architecture guidance requires per-agent call logs with trace IDs, span IDs, latency, token/cost, input hash, tools, confidence, errors, model, and status.

Difference from typical Rust agent systems:

- Many Rust systems log only success/failure and stdout/stderr.
- This pattern treats every agent call as a traceable span.

Why it improves FounderAI:

- Faster root-cause analysis when a run fails or produces bad work.
- Easier cost review.
- Enables later dashboards without new infrastructure.

Implementation:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAttempt {
    pub span_id: String,
    pub provider: String,
    pub model: String,
    pub started_at: String,
    pub finished_at: String,
    pub latency_ms: u128,
    pub status: String,
    pub error_class: Option<String>,
    pub prompt_hash: String,
    pub usage: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTrace {
    pub trace_id: String,
    pub run_id: String,
    pub job_id: String,
    pub task_type: String,
    pub agent_id: Option<String>,
    pub attempts: Vec<ProviderAttempt>,
}
```

Integration points:

- `worker.rs::run_worker()`: create `trace_id`, push primary/fallback attempts.
- `team_logging.rs`: append `trace_id`, `span_id`, `status`, `error_class`.
- `app.rs::run_detail()`: expose trace in run view.
- `scripts/cloud-weekly-review.sh`: aggregate attempts, failures, avg latency.

Challenge:

- Add `sha2` or use existing hashing dependency if available; avoid new dependency unless needed.

### 3. Add evidence-based QA gates

What `agency-agents` does well:

- Evidence Collector/Reality Checker demand proof, not optimistic claims.
- Reports include commands, screenshots/test JSON, spec-vs-implementation, journey assessment, pass/fail.

Difference from typical Rust agent systems:

- Typical agent systems mark outputs done when generation succeeds.
- This pattern separates generation success from evidence-backed quality.

Why it improves FounderAI:

- Prevents "run succeeded" from meaning "work is trustworthy".
- Fits current Clare/Francis governance.
- Useful for public-site changes, code changes, publish, outreach, grant outputs.

Implementation:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub kind: String,       // command, file, screenshot, link, metric
    pub path: Option<String>,
    pub command: Option<String>,
    pub result: String,     // passed, failed, skipped
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceGate {
    pub gate_id: String,
    pub run_id: String,
    pub policy: String,
    pub status: String,     // pass, fail, insufficient
    pub evidence: Vec<EvidenceItem>,
    pub residual_risk: Vec<String>,
}
```

Integration points:

- `approvals.rs::ApprovalRecord`: add optional `evidence_gate`.
- `app.rs::approval_summary()`: include evidence status.
- `founder-brain/qa_rubrics.md`: add evidence field requirements.
- `docs/ai-assisted-development-safety.md`: require evidence gate for code-change risk tag.

Challenge:

- Some jobs are text-only. Do not require screenshots unless UI/browser output is relevant.

### 4. Add cost/failure circuit breakers to model routing

What `agency-agents` does well:

- Autonomous Optimization Architect ranks providers using historical performance and blocks runaway spend with hard limits and circuit breakers.

Difference from typical Rust agent systems:

- Many systems have fallback routing but no provider memory or cost guardrail.
- FounderAI has fallback routes, but no cost threshold/circuit state.

Why it improves FounderAI:

- Directly addresses previous Anthropic credit failure.
- Protects cheap VPS operation.
- Allows offline-first routing to improve with evidence.

Implementation:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealthState {
    pub provider: String,
    pub model: String,
    pub consecutive_failures: u32,
    pub circuit_open_until: Option<String>,
    pub avg_latency_ms: Option<f64>,
    pub estimated_cost_usd_7d: f64,
    pub last_error_class: Option<String>,
}
```

Integration points:

- `state.rs`: add `provider_health: BTreeMap<String, ProviderHealthState>`.
- `model_router.rs::resolve_worker()`: skip provider when circuit is open.
- `worker.rs::call_provider()`: classify timeout, auth, low_credit, server_error, parse_error.
- `config/founderai.cloud.json`: add `max_cost_per_run_usd`, `max_consecutive_failures`.

Challenge:

- Ollama cost is not token-billed. Track latency/failure only for offline providers.

### 5. Lint the agent roster before daemon start

What `agency-agents` does well:

- `lint-agents.sh` enforces frontmatter, line endings, and recommended sections.
- `divisions.json` is checked against directories and script arrays.

Difference from typical Rust agent systems:

- Rust code often validates compile-time types but not prompt/data files.

Why it improves FounderAI:

- Catches broken prompt paths, duplicate role IDs, missing escalation rules, missing transparency notes.
- Prevents silent degraded prompts.

Implementation:

```rust
pub fn validate_agent_profiles(config: &AppConfig) -> Result<()> {
    let mut ids = BTreeSet::new();
    for agent in config.agent_profiles.values() {
        anyhow::ensure!(ids.insert(agent.id.clone()), "duplicate agent id {}", agent.id);
        if let Some(prompt_file) = &agent.prompt_file {
            anyhow::ensure!(
                config.founder_brain_path.join(prompt_file).exists(),
                "missing prompt file for {}: {}",
                agent.id,
                prompt_file
            );
        }
        anyhow::ensure!(!agent.job_scope.trim().is_empty(), "missing job_scope for {}", agent.id);
    }
    Ok(())
}
```

Integration points:

- `config.rs::load_config()`: validate after loading.
- CLI: optional `validate-config` command.
- CI: run `cargo test` plus config smoke validation.

Challenge:

- Some current agents may need metadata completion first.

### 6. Add multi-agent topology templates

What `agency-agents` does well:

- README examples assemble small cross-functional teams per scenario.
- Studio Producer pattern aligns portfolio priorities, dependencies, and resource allocation.

Difference from typical Rust agent systems:

- Many systems use one planner and one worker.
- Agency-style topology is explicit: choose specialists per phase.

Why it improves FounderAI:

- Daily team orchestration can become predictable instead of prompt-only.
- Better long-term goal assistance: plan, produce, verify, review, then ask for approval.

Implementation:

```json
{
  "workflow_templates": {
    "public_site_change": [
      { "phase": "plan", "agent_id": "columban" },
      { "phase": "implement", "agent_id": "juniper" },
      { "phase": "evidence", "agent_id": "clare" },
      { "phase": "review", "agent_id": "francis" }
    ]
  }
}
```

Integration points:

- `config.rs`: add workflow template structs.
- `app.rs::run_single_job()`: expand certain task types into phase packets.
- `approvals.rs`: keep external actions blocked after phase completion.

Challenge:

- Avoid over-orchestration. Start with 2 templates only: `code_change`, `publish_campaign`.

### 7. Add typed runbooks, handoffs, and retry escalation

What `agency-agents` does well:

- `strategy/runbooks.json` maps scenario names to phase-aware agent rosters.
- Handoff templates define context, acceptance criteria, evidence requirements, pass/fail status, retry count, and escalation reports.
- NEXUS uses a simple rule: developer work is not complete until QA passes, and repeated failure escalates after a bounded number of attempts.

Difference from typical Rust agent systems:

- Typical systems store "job complete" and maybe an output file.
- NEXUS stores the coordination contract between agents and phases.
- The useful idea is not the large team size; it is typed continuity between steps.

Why it improves FounderAI:

- Long-term goal assistance needs memory of why a task moved forward, not only that it ran.
- FounderAI already has run artifacts and approvals, so handoffs can be added without changing the entire runtime.
- Failures become diagnosable: unclear request, bad prompt, provider failure, missing evidence, bad implementation, or approval blocked.

Implementation:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunbook {
    pub id: String,
    pub title: String,
    pub mode: String, // micro, sprint, recurring
    pub phases: Vec<WorkflowPhase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowPhase {
    pub phase_id: String,
    pub agent_id: String,
    pub activation: String, // always, as_needed, post_fix
    pub acceptance_criteria: Vec<String>,
    pub required_evidence: Vec<String>,
    pub max_attempts: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentHandoff {
    pub handoff_id: String,
    pub trace_id: String,
    pub from_agent_id: Option<String>,
    pub to_agent_id: String,
    pub phase_id: String,
    pub priority: String,
    pub current_state: String,
    pub relevant_files: Vec<String>,
    pub dependencies: Vec<String>,
    pub constraints: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub evidence_required: Vec<String>,
    pub attempt: u8,
    pub max_attempts: u8,
}
```

Integration points:

- `config.rs`: load `config/workflow_runbooks.json`.
- `app.rs::run_single_job()`: when a job matches a runbook, create phase runs instead of one monolithic prompt.
- `approvals.rs`: attach the latest `AgentHandoff` and `EvidenceGate` to approval summaries.
- `team_logging.rs`: append handoff events as JSONL for timeline reconstruction.
- `docs/operations/`: add operator runbooks for `code_change`, `public_site_publish`, `grant_pipeline_review`, and `daily_team_orchestration`.

Challenge:

- Keep FounderAI survival-first. Do not import NEXUS's 15-25-agent sprint style. Use 3-5 phase templates and only escalate to more agents when evidence shows the task is stuck.

## Code Examples & Integration Points

### Lazy context bundle selection

```rust
pub struct ContextBundle {
    pub id: String,
    pub required_files: Vec<PathBuf>,
    pub max_chars_per_file: usize,
}

pub fn render_context_for_agent(config: &AppConfig, agent_id: &str) -> Result<String> {
    let agent = config.agent_profiles.get(agent_id)
        .with_context(|| format!("unknown agent_id {agent_id}"))?;

    let mut sections = Vec::new();
    for bundle_id in &agent.context_bundles {
        let bundle = config.context_bundles.get(bundle_id)
            .with_context(|| format!("unknown context bundle {bundle_id}"))?;
        sections.push(render_bundle(config, bundle)?);
    }
    Ok(sections.join("\n\n"))
}
```

Use in:

- `worker.rs::render_agent_ready_documents()`

### Provider attempt wrapper

```rust
fn call_provider_with_trace(
    prompt: &str,
    worker: &WorkerConfig,
    trace: &mut RunTrace,
) -> Result<ProviderCallResponse> {
    let started = std::time::Instant::now();
    let span_id = uuid_like_id();
    let result = call_provider(prompt, worker);
    let latency_ms = started.elapsed().as_millis();

    trace.attempts.push(ProviderAttempt {
        span_id,
        provider: worker.provider.clone(),
        model: worker.model.clone(),
        started_at: Utc::now().to_rfc3339(),
        finished_at: Utc::now().to_rfc3339(),
        latency_ms,
        status: if result.is_ok() { "success" } else { "failure" }.to_string(),
        error_class: result.as_ref().err().map(classify_provider_error),
        prompt_hash: prompt_hash(prompt),
        usage: result.as_ref().ok().and_then(|r| r.usage.clone()),
    });

    result
}
```

Use in:

- `worker.rs::run_worker()`
- `src/state.rs`
- `scripts/cloud-weekly-review.sh`

### Evidence gate before after-run approval

```rust
fn build_evidence_gate(job: &JobConfig, result: &WorkerRunResult) -> EvidenceGate {
    let mut evidence = Vec::new();
    evidence.push(EvidenceItem {
        kind: "file".into(),
        path: Some(result.output_file.display().to_string()),
        command: None,
        result: if result.output_file.exists() { "passed" } else { "failed" }.into(),
        notes: "Primary output artifact exists.".into(),
    });

    EvidenceGate {
        gate_id: format!("{}-evidence", result.run_id),
        run_id: result.run_id.clone(),
        policy: job.approval_policy.clone(),
        status: if result.exit_code == 0 { "pass" } else { "fail" }.into(),
        evidence,
        residual_risk: Vec::new(),
    }
}
```

Use in:

- `app.rs::request_approval()`
- `approvals.rs::ApprovalRecord`

### Workflow runbook registry

```json
{
  "workflow_runbooks": [
    {
      "id": "code_change",
      "title": "Bounded Code Change",
      "mode": "micro",
      "phases": [
        {
          "phase_id": "plan",
          "agent_id": "columban",
          "activation": "always",
          "acceptance_criteria": [
            "Scope is bounded to the requested change",
            "Risk tags are identified before editing"
          ],
          "required_evidence": ["files_inspected", "planned_verification"],
          "max_attempts": 1
        },
        {
          "phase_id": "implement",
          "agent_id": "columban",
          "activation": "always",
          "acceptance_criteria": [
            "Patch changes only necessary files",
            "No unrelated user work is reverted"
          ],
          "required_evidence": ["git_diff", "verification_commands"],
          "max_attempts": 2
        },
        {
          "phase_id": "qa",
          "agent_id": "clare",
          "activation": "always",
          "acceptance_criteria": [
            "Commands pass or failure is explicitly documented",
            "Residual risks are named"
          ],
          "required_evidence": ["test_output", "diff_check"],
          "max_attempts": 1
        }
      ]
    }
  ]
}
```

Use in:

- `config/workflow_runbooks.json`
- `src/config.rs`
- `app.rs::run_single_job()`

### Handoff event JSONL

```json
{
  "event_type": "agent_handoff",
  "trace_id": "20260813T101530Z-site-change",
  "handoff_id": "20260813T101530Z-site-change-plan-to-implement",
  "from_agent_id": "clare",
  "to_agent_id": "columban",
  "phase_id": "implement",
  "priority": "high",
  "current_state": "Plan accepted; risk tags code-change and publish are active.",
  "relevant_files": ["docs/index.html", "docs/assets/marketing-site.css"],
  "acceptance_criteria": ["Sticky bar hides at checkout", "Updates link replaces proof"],
  "evidence_required": ["git diff", "docs link check", "browser screenshot if UI changed"],
  "attempt": 1,
  "max_attempts": 2
}
```

Use in:

- `team_activity.jsonl`
- per-run `handoff.json`
- approval summaries for phase transitions

### Agent roster lint test

```rust
#[test]
fn agent_prompts_exist() {
    let config = load_config(Path::new("config/founderai.example.json")).unwrap();
    for agent in config.agent_profiles.values() {
        if let Some(prompt_file) = &agent.prompt_file {
            assert!(
                config.founder_brain_path.join(prompt_file).exists(),
                "missing prompt for {}",
                agent.id
            );
        }
    }
}
```

Use in:

- `src/config.rs` tests
- CI build workflow

## Trade-offs & What Not To Copy

| Do not copy | Why not | Better FounderAI adaptation |
| --- | --- | --- |
| 230+ generic specialist roster | Too much prompt surface; mission drift; unnecessary choices. | Keep 14 saint agents; add typed capabilities and context bundles. |
| Personality-heavy tone | FounderAI must stay founder-specific, sober, Franciscan, anti-hype. | Copy operational contracts, not voice. |
| Shell-first conversion pipeline inside runtime | Rust daemon should not depend on Bash for live routing. | Use Rust structs + startup validation; keep scripts for CI/generation only. |
| Naive token-overlap search as only router | Fine for broad catalog discovery, weak for mission-critical task routing. | Use deterministic task_type/role matching first, then fuzzy search as helper. |
| Parallel conversion style for runtime tasks | Useful for generated files, risky for stateful agent work with approvals. | Parallelize only independent read-only audits; serialize approval-sensitive runs. |
| Auto-promotion of providers without human review | Could violate budget/quality expectations. | Allow automatic demotion/circuit break; require approval before promotion. |
| Screenshot-only QA absolutism | Many FounderAI outputs are text, research, grants, operations. | Evidence must match artifact type: commands/files/metrics/screenshots/links. |
| Tool-specific generated agent installs | This repo's runtime is FounderAI, not a Codex/Cursor agent distribution. | Learn from portable metadata; do not install external agent packs into production. |
| NEXUS's large sprint teams by default | Good for agency demos, too heavy for cheap VPS, solo founder, and approval discipline. | Use micro-runbooks with 3-5 phases; expand only when blocked. |
| Originality check thresholds copied blindly | Agency measures public catalog duplication, not mission drift. | Adapt as duplicate mandate/context overlap checks for saint agents and product docs. |

## Recommended Backlog

### Batch 1: Observability

- Add `src/trace.rs`.
- Add `trace_id`, `provider_attempts`, `active_worker_reason`, `error_class`.
- Extend `cloud-weekly-review.sh`.
- Tests: provider failure classification; metadata shape.

### Batch 2: Agent Catalog

- Add `capabilities`, `task_types`, `context_bundles`, `required_evidence` to agents.
- Add context bundle registry in config.
- Replace hardcoded `render_agent_ready_documents()` match with bundle rendering.
- Tests: missing prompt, missing bundle, duplicate agent ID, unknown task type.

### Batch 3: Evidence Gates

- Add optional `EvidenceGate` to approval records.
- Generate minimum evidence for all after-run approvals.
- Require richer evidence for `code-change`, `publish`, `external-send`.
- Update web console approval card to show evidence status.

### Batch 4: Runbooks & Handoffs

- Add `config/workflow_runbooks.json`.
- Add `AgentHandoff` and per-run `handoff.json`.
- Add 3 templates: `code_change`, `public_site_publish`, `daily_team_orchestration`.
- Add max-attempt logic before escalation to Clare/Francis.
- Tests: unknown agent in runbook, missing phase evidence, max attempts exceeded.

### Batch 5: Cost Guardrails

- Add provider health state.
- Track consecutive failures and low-credit/auth/timeout error classes.
- Skip circuit-open providers in router.
- Add config limits: `max_cost_per_run_usd`, `max_failures_before_circuit_open`.

### Batch 6: Workflow Topologies

- Add two templates: `code_change`, `publish_campaign`.
- Generate phase-specific inbox jobs.
- Preserve approvals after each external-facing phase.

## Decision

Highest-leverage immediate change: trace/span metadata.

Reason:

- Minimal behavior risk.
- Fits existing run artifact model.
- Improves every later feature.
- Gives Clare/Francis better review input.
- Helps debug provider failures like low Anthropic credits and Ollama timeouts.
