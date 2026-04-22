use crate::config::{AgentProfile, AppConfig, JobConfig, TeamRoleConfig, WorkerConfig};
use crate::model_router::resolve_worker;
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::blocking::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Serialize;
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct WorkerRunResult {
    pub run_id: String,
    pub started_at: String,
    pub finished_at: String,
    pub exit_code: i32,
    pub prompt_file: PathBuf,
    pub output_file: PathBuf,
    pub stdout_file: PathBuf,
    pub stderr_file: PathBuf,
    pub summary: String,
    pub team_output_file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ProviderStatus {
    pub reachable: bool,
    pub model_available: Option<bool>,
    pub detail: Option<String>,
}

fn read_founder_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| format!("[Missing context file: {}]", path.display()))
}

fn render_document_section(title: &str, path: &Path) -> String {
    format!(
        "### {title}\nSource file: {}\n\n{}",
        path.display(),
        read_founder_file(path)
    )
}

fn render_agent_ready_documents(
    config: &AppConfig,
    job: &JobConfig,
    role: Option<&TeamRoleConfig>,
) -> String {
    let root = config
        .workspace_root
        .join("documents")
        .join("99_Agent_Ready");

    let selected_agent_id = selected_agent_profile(config, job, role)
        .map(|agent| agent.id.as_str())
        .unwrap_or("default");

    let mut rendered = vec![
        format!("Document library root: {}", root.display()),
        format!("Selected agent-ready bundle for: {selected_agent_id}"),
        render_document_section("Document Source Priority", &root.join("references").join("source_priority.md")),
        render_document_section(
            "Canonical Reference Brief",
            &root.join("references").join("canonical_reference_brief.md"),
        ),
        render_document_section(
            "Operational Memory Database",
            &root.join("databases").join("operational_memory.json"),
        ),
    ];

    let targeted_sections: Vec<(&str, PathBuf)> = match selected_agent_id {
        "anthony" | "zacchaeus" | "perpetua" | "bonaventure" => vec![
            (
                "Prospect Targets Database",
                root.join("databases").join("prospect_targets.json"),
            ),
            ("Template Index", root.join("templates").join("template_index.md")),
            (
                "External Communications Templates",
                root.join("templates").join("external_communications.md"),
            ),
        ],
        "bartholomew" | "pio" | "hildegard" | "francis" | "clare" | "columban" => vec![
            (
                "Document Registry Database",
                root.join("databases").join("document_registry.json"),
            ),
            ("Template Index", root.join("templates").join("template_index.md")),
            (
                "Internal Operations Templates",
                root.join("templates").join("internal_operations.md"),
            ),
        ],
        "jacinta" | "duns-scotus" => vec![
            (
                "Document Registry Database",
                root.join("databases").join("document_registry.json"),
            ),
            ("Template Index", root.join("templates").join("template_index.md")),
            (
                "Research And Applications Templates",
                root.join("templates").join("research_and_applications.md"),
            ),
        ],
        _ => vec![
            ("Document Layer Overview", root.join("README.md")),
            (
                "Document Registry Database",
                root.join("databases").join("document_registry.json"),
            ),
            ("Template Index", root.join("templates").join("template_index.md")),
        ],
    };

    rendered.extend(
        targeted_sections
            .into_iter()
            .map(|(title, path)| render_document_section(title, &path)),
    );

    rendered.join("\n\n")
}

fn team_output_dir(runtime_dir: &Path, role: Option<&TeamRoleConfig>) -> Option<PathBuf> {
    let role = role?;
    let path = runtime_dir.join("teams").join(&role.role_id).join("outputs");
    fs::create_dir_all(&path).ok();
    Some(path)
}

fn grant_output_dir(runtime_dir: &Path) -> PathBuf {
    let path = runtime_dir.join("grants");
    fs::create_dir_all(&path).ok();
    path
}

fn selected_agent_profile<'a>(
    config: &'a AppConfig,
    job: &JobConfig,
    role: Option<&TeamRoleConfig>,
) -> Option<&'a AgentProfile> {
    if let Some(role) = role {
        if let Some(profile) = config.agent_profiles.get(&role.agent_id) {
            return Some(profile);
        }
    }

    job.agent_id
        .as_deref()
        .and_then(|agent_id| config.agent_profiles.get(agent_id))
}

fn render_agent_roster(config: &AppConfig) -> String {
    if config.agent_profiles.is_empty() {
        return "No agent roster loaded.".to_string();
    }

    config
        .agent_profiles
        .values()
        .map(|agent| {
            let role_text = agent
                .canonical_role_id
                .as_deref()
                .map(|value| format!(" mapped to {value}"))
                .unwrap_or_default();
            format!(
                "- {} ({}){} | kind={} | primary_model={} | escalation={}",
                agent.saint_name,
                agent.id,
                role_text,
                if agent.kind.is_empty() { "unspecified" } else { &agent.kind },
                if agent.primary_model.is_empty() {
                    "unspecified"
                } else {
                    &agent.primary_model
                },
                if agent.escalation_rule.is_empty() {
                    "Founder approval when uncertain."
                } else {
                    &agent.escalation_rule
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_selected_agent_context(
    config: &AppConfig,
    job: &JobConfig,
    role: Option<&TeamRoleConfig>,
) -> String {
    let Some(agent) = selected_agent_profile(config, job, role) else {
        return "No explicit ERIS agent profile resolved for this run.".to_string();
    };

    let prompt_text = agent
        .prompt_file
        .as_deref()
        .map(|relative| read_founder_file(&config.founder_brain_path.join(relative)))
        .unwrap_or_else(|| "No dedicated agent prompt file configured.".to_string());

    format!(
        "Resolved agent profile:\n- Agent ID: {}\n- Saint name: {}\n- Kind: {}\n- Canonical role: {}\n- Primary model: {}\n- Fallback model: {}\n- Client facing: {}\n- Mentor role: {}\n- Transparency note: {}\n- Escalation rule: {}\n- Job scope: {}\n\nAgent prompt:\n{}\n",
        agent.id,
        agent.saint_name,
        if agent.kind.is_empty() { "unspecified" } else { &agent.kind },
        agent
            .canonical_role_id
            .clone()
            .unwrap_or_else(|| "n/a".to_string()),
        if agent.primary_model.is_empty() {
            "unspecified"
        } else {
            &agent.primary_model
        },
        if agent.fallback_model.is_empty() {
            "unspecified"
        } else {
            &agent.fallback_model
        },
        agent.external_facing,
        agent.mentor_role,
        if agent.transparency_note.is_empty() {
            "Founder review required before external use."
        } else {
            &agent.transparency_note
        },
        if agent.escalation_rule.is_empty() {
            "Escalate to Founder when uncertain."
        } else {
            &agent.escalation_rule
        },
        if agent.job_scope.is_empty() {
            "No explicit scope provided."
        } else {
            &agent.job_scope
        },
        prompt_text
    )
}

pub fn build_prompt(
    config: &AppConfig,
    job: &JobConfig,
    trigger: &str,
    run_dir: &Path,
    request_source: Option<&Path>,
    role: Option<&TeamRoleConfig>,
    effective_risk_tags: &[String],
    resolved_approval_policy: &str,
) -> String {
    let founder_brain = &config.founder_brain_path;
    let founder_brain_overview = read_founder_file(&founder_brain.join("founder_brain.md"));
    let identity = read_founder_file(&founder_brain.join("references").join("identity.md"));
    let knowledge = read_founder_file(&founder_brain.join("references").join("knowledge-pack.md"));
    let team_structure = read_founder_file(&founder_brain.join("references").join("team-structure.md"));
    let workflows = read_founder_file(&founder_brain.join("references").join("workflows.md"));
    let patterns = read_founder_file(&founder_brain.join("references").join("output-patterns.md"));
    let cloud_migration_plan = read_founder_file(&founder_brain.join("cloud_migration_plan.md"));
    let agent_ready_documents = render_agent_ready_documents(config, job, role);
    let eris_knowledge = read_founder_file(&founder_brain.join("eris_knowledge.md"));
    let hormozi_protocols = read_founder_file(&founder_brain.join("hormozi_protocols.md"));
    let qa_rubrics = read_founder_file(&founder_brain.join("qa_rubrics.md"));
    let risk_register = read_founder_file(&founder_brain.join("risk_register.md"));
    let kpi_thresholds = read_founder_file(&founder_brain.join("kpi_thresholds.md"));
    let forbidden_patterns = read_founder_file(&founder_brain.join("forbidden_patterns.txt"));
    let governance_constraints = read_founder_file(&founder_brain.join("governance_constraints.json"));
    let strategic_roadmap = read_founder_file(&founder_brain.join("strategic_roadmap.md"));

    let request_note = request_source
        .map(|source| format!("\nSource request file: {}\n", source.display()))
        .unwrap_or_default();

    let role_note = if let Some(role) = role {
        let responsibilities = if role.responsibilities.is_empty() {
            "- None provided".to_string()
        } else {
            role.responsibilities
                .iter()
                .map(|item| format!("- {item}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        format!(
            "Role packet:\n- Role ID: {}\n- Team: {}\n- Role: {}\n- Display name: {}\n- Saint name: {}\n- Agent ID: {}\n- Daily quota: {} {}\n- Focus: {}\n- Responsibilities:\n{}\n",
            role.role_id,
            role.team,
            role.role,
            role.display_name,
            role.saint_name,
            role.agent_id,
            role.daily_quota,
            role.metric_unit,
            role.focus,
            responsibilities
        )
    } else {
        "No explicit role packet.".to_string()
    };

    format!(
        "# FounderAI Autonomous Run Packet\n\nYou are running a bounded FounderAI background cycle.\n\nNon-negotiables:\n- Stay in the founder's exact voice.\n- Protect survival-first priorities.\n- Never send, publish, spend, delete, or commit externally without explicit approval.\n- If the task touches protected categories, draft the work and stop for validation.\n- Keep the founder's Franciscan mission and anti-hype discipline intact.\n\nRun metadata:\n- Trigger: {trigger}\n- Job ID: {job_id}\n- Job description: {job_description}\n- Workspace root: {workspace_root}\n- Runtime directory: {runtime_dir}\n- Outbox directory: {outbox_dir}\n- Output target for this run: {output_target}{request_note}\n\n## Founder Brain\n\n{founder_brain_overview}\n\n## Founder Identity\n\n{identity}\n\n## Founder Knowledge Pack\n\n{knowledge}\n\n## ERIS Knowledge\n\n{eris_knowledge}\n\n## Hormozi Protocols\n\n{hormozi_protocols}\n\n## Team Structure\n\n{team_structure}\n\n## Founder Workflows\n\n{workflows}\n\n## Cloud Migration Context\n\n{cloud_migration_plan}\n\n## Agent-Ready Documents\n\n{agent_ready_documents}\n\n## Founder Output Patterns\n\n{patterns}\n\n## Strategic Roadmap\n\n{strategic_roadmap}\n\n## Risk Register\n\n{risk_register}\n\n## KPI Thresholds\n\n{kpi_thresholds}\n\n## QA Rubrics\n\n{qa_rubrics}\n\n## Forbidden Patterns\n\n{forbidden_patterns}\n\n## Governance Constraints\n\n{governance_constraints}\n\n## Agent Roster\n\n{agent_roster}\n\n## Selected Agent Context\n\n{selected_agent_context}\n\n## Team Role Context\n\n{role_note}\n\n## Requested Work\n\n{requested_work}\n\n## Strategic Validation\n\n- Protected tags for this run: {risk_tags}\n- Resolved approval policy: {approval_policy}\n- If an action would create outside consequences, stop and prepare a validation-ready draft.\n- Transparent AI signatures are mandatory for any client-facing draft.\n\n## Delivery Requirements\n\n- Write the primary output to the designated output file.\n- Keep the output concise, useful, and immediately reviewable.\n- Prefer a draft, brief, checklist, or structured note that the founder can validate quickly.\n- Never invent a concrete fact to make the draft feel complete.\n- If a date, budget, collaborator, site, contact, requirement, or institutional fact is missing, write `NEEDS_HUMAN_VERIFICATION`.\n- If this run fails QA or governance constraints, explain why clearly instead of pretending success.\n",
        trigger = trigger,
        job_id = job.job_id,
        job_description = if job.description.is_empty() {
            "n/a".to_string()
        } else {
            job.description.clone()
        },
        workspace_root = config.workspace_root.display(),
        runtime_dir = config.runtime_dir.display(),
        outbox_dir = config.outbox_dir.display(),
        output_target = run_dir.join("output.md").display(),
        request_note = request_note,
        founder_brain_overview = founder_brain_overview,
        identity = identity,
        knowledge = knowledge,
        eris_knowledge = eris_knowledge,
        hormozi_protocols = hormozi_protocols,
        team_structure = team_structure,
        workflows = workflows,
        cloud_migration_plan = cloud_migration_plan,
        agent_ready_documents = agent_ready_documents,
        patterns = patterns,
        strategic_roadmap = strategic_roadmap,
        risk_register = risk_register,
        kpi_thresholds = kpi_thresholds,
        qa_rubrics = qa_rubrics,
        forbidden_patterns = forbidden_patterns,
        governance_constraints = governance_constraints,
        agent_roster = render_agent_roster(config),
        selected_agent_context = render_selected_agent_context(config, job, role),
        role_note = role_note,
        requested_work = job.prompt,
        risk_tags = if effective_risk_tags.is_empty() {
            "none".to_string()
        } else {
            effective_risk_tags.join(", ")
        },
        approval_policy = resolved_approval_policy
    )
}

#[derive(Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    system: &'a str,
    stream: bool,
}

#[derive(Serialize)]
struct OpenAiResponsesRequest<'a> {
    model: &'a str,
    instructions: &'a str,
    input: &'a str,
}

fn normalize_base_url(base_url: &str) -> String {
    base_url.trim_end_matches('/').to_string()
}

fn build_client(worker: &WorkerConfig) -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(worker.timeout_seconds))
        .build()
        .context("failed to build HTTP client")
}

fn api_key_from_env(worker: &WorkerConfig) -> Result<String> {
    let value = env::var(&worker.api_key_env)
        .with_context(|| format!("environment variable {} is required for the OpenAI provider", worker.api_key_env))?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        anyhow::bail!("environment variable {} is empty", worker.api_key_env);
    }
    Ok(trimmed.to_string())
}

fn extract_openai_output_text(payload: &Value) -> Option<String> {
    if let Some(output_text) = payload.get("output_text").and_then(Value::as_str) {
        let trimmed = output_text.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }

    let mut chunks = Vec::new();
    for item in payload.get("output").and_then(Value::as_array).into_iter().flatten() {
        for content in item.get("content").and_then(Value::as_array).into_iter().flatten() {
            if let Some(text) = content.get("text").and_then(Value::as_str) {
                if !text.trim().is_empty() {
                    chunks.push(text.trim().to_string());
                }
            }
        }
    }

    if chunks.is_empty() {
        None
    } else {
        Some(chunks.join("\n"))
    }
}

pub fn provider_status(worker: &WorkerConfig) -> ProviderStatus {
    if worker.provider.eq_ignore_ascii_case("ollama") {
        let client = match build_client(worker) {
            Ok(client) => client,
            Err(err) => {
                return ProviderStatus {
                    reachable: false,
                    model_available: None,
                    detail: Some(err.to_string()),
                }
            }
        };

        let url = format!("{}/api/tags", normalize_base_url(&worker.base_url));
        let response = match client.get(&url).send() {
            Ok(response) => response,
            Err(err) => {
                return ProviderStatus {
                    reachable: false,
                    model_available: None,
                    detail: Some(err.to_string()),
                }
            }
        };

        let status = response.status();
        let raw = match response.text() {
            Ok(text) => text,
            Err(err) => {
                return ProviderStatus {
                    reachable: false,
                    model_available: None,
                    detail: Some(err.to_string()),
                }
            }
        };

        if !status.is_success() {
            return ProviderStatus {
                reachable: false,
                model_available: None,
                detail: Some(format!("HTTP {}: {}", status, raw)),
            };
        }

        let data: Value = match serde_json::from_str(&raw) {
            Ok(value) => value,
            Err(err) => {
                return ProviderStatus {
                    reachable: true,
                    model_available: None,
                    detail: Some(format!("invalid Ollama tags payload: {err}")),
                }
            }
        };

        let model_available = data
            .get("models")
            .and_then(Value::as_array)
            .map(|models| {
                models.iter().any(|model| {
                    model
                        .get("name")
                        .and_then(Value::as_str)
                        .map(|name| name == worker.model)
                        .unwrap_or(false)
                })
            });

        return ProviderStatus {
            reachable: true,
            model_available,
            detail: None,
        };
    }

    if worker.provider.eq_ignore_ascii_case("openai") {
        return match api_key_from_env(worker) {
            Ok(_) => ProviderStatus {
                reachable: true,
                model_available: None,
                detail: Some(format!("API key loaded from {}", worker.api_key_env)),
            },
            Err(err) => ProviderStatus {
                reachable: false,
                model_available: None,
                detail: Some(err.to_string()),
            },
        };
    }

    ProviderStatus {
        reachable: false,
        model_available: None,
        detail: Some(format!("unsupported provider '{}'", worker.provider)),
    }
}

fn call_ollama(prompt_text: &str, worker: &WorkerConfig) -> Result<String> {
    let client = build_client(worker)?;
    let url = format!("{}/api/generate", normalize_base_url(&worker.base_url));
    let request = OllamaGenerateRequest {
        model: &worker.model,
        prompt: prompt_text,
        system: &worker.system_prompt,
        stream: false,
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .with_context(|| format!("failed to reach Ollama at {}", worker.base_url))?;
    let status = response.status();
    let raw = response.text().context("failed to read Ollama response body")?;

    if !status.is_success() {
        anyhow::bail!("Ollama returned HTTP {}: {}", status, raw);
    }

    let payload: Value = serde_json::from_str(&raw).context("failed to parse Ollama response JSON")?;
    if let Some(error) = payload.get("error").and_then(Value::as_str) {
        anyhow::bail!("Ollama error: {error}");
    }
    let output = payload
        .get("response")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .ok_or_else(|| anyhow::anyhow!("Ollama returned no response text"))?;

    Ok(output)
}

fn call_openai(prompt_text: &str, worker: &WorkerConfig) -> Result<String> {
    let client = build_client(worker)?;
    let api_key = api_key_from_env(worker)?;
    let url = format!("{}/responses", normalize_base_url(&worker.base_url));
    let request = OpenAiResponsesRequest {
        model: &worker.model,
        instructions: &worker.system_prompt,
        input: prompt_text,
    };

    let response = client
        .post(&url)
        .header(AUTHORIZATION, format!("Bearer {api_key}"))
        .header(CONTENT_TYPE, "application/json")
        .json(&request)
        .send()
        .with_context(|| format!("failed to reach OpenAI at {}", worker.base_url))?;
    let status = response.status();
    let raw = response.text().context("failed to read OpenAI response body")?;

    if !status.is_success() {
        anyhow::bail!("OpenAI returned HTTP {}: {}", status, raw);
    }

    let payload: Value = serde_json::from_str(&raw).context("failed to parse OpenAI response JSON")?;
    extract_openai_output_text(&payload)
        .ok_or_else(|| anyhow::anyhow!("OpenAI returned no response text"))
}

fn call_provider(prompt_text: &str, worker: &WorkerConfig) -> Result<String> {
    if worker.provider.eq_ignore_ascii_case("ollama") {
        return call_ollama(prompt_text, worker);
    }
    if worker.provider.eq_ignore_ascii_case("openai") {
        return call_openai(prompt_text, worker);
    }
    anyhow::bail!("unsupported provider '{}'", worker.provider);
}

fn summary_from_output(output_file: &Path, stdout: &str) -> String {
    let text = fs::read_to_string(output_file).unwrap_or_else(|_| stdout.to_string());
    for line in text.lines() {
        let clean = line.trim();
        if !clean.is_empty() {
            return clean.chars().take(240).collect();
        }
    }
    "Run completed with no summary text.".to_string()
}

fn failure_output(worker: &WorkerConfig, reason: &str) -> String {
    format!(
        "# FounderAI Run Blocked\n\nProvider generation failed for this run.\n\n- Provider: {}\n- Base URL: {}\n- Model: {}\n- Reason: {}\n\n## Safe Recovery\n\n- Confirm the configured provider is reachable.\n- If using Ollama, confirm the configured model exists locally: `ollama pull {}`\n- If using OpenAI, confirm `{}` is set in the environment.\n- Re-run the FounderAI tick after the provider is healthy.\n",
        worker.provider, worker.base_url, worker.model, reason, worker.model, worker.api_key_env
    )
}

pub fn run_worker(
    config: &AppConfig,
    job: &JobConfig,
    trigger: &str,
    runtime_dir: &Path,
    request_source: Option<&Path>,
    role: Option<&TeamRoleConfig>,
    effective_risk_tags: &[String],
    resolved_approval_policy: &str,
    current_internet: bool,
) -> WorkerRunResult {
    let timestamp = Utc::now();
    let mut run_id_parts = vec![timestamp.format("%Y%m%dT%H%M%SZ").to_string(), job.job_id.clone()];
    if let Some(role) = role {
        run_id_parts.push(role.role_id.clone());
    }
    let run_id = run_id_parts.join("-");
    let run_dir = runtime_dir.join("runs").join(&run_id);
    fs::create_dir_all(&run_dir).ok();

    let prompt_file = run_dir.join("prompt.md");
    let output_file = run_dir.join("output.md");
    let stdout_file = run_dir.join("stdout.txt");
    let stderr_file = run_dir.join("stderr.txt");
    let metadata_file = run_dir.join("metadata.json");

    let prompt_text = build_prompt(
        config,
        job,
        trigger,
        &run_dir,
        request_source,
        role,
        effective_risk_tags,
        resolved_approval_policy,
    );
    fs::write(&prompt_file, &prompt_text).ok();

    let routed_worker = resolve_worker(config, job, role, current_internet);
    let team_output_file = team_output_dir(runtime_dir, role).map(|dir| dir.join(format!("{run_id}.md")));
    let grant_output_file = if routed_worker.task_type == "grant" || job.agent_id.as_deref() == Some("bartholomew") {
        Some(grant_output_dir(runtime_dir).join(format!("{run_id}.md")))
    } else {
        None
    };

    let started_at = Utc::now().to_rfc3339();
    let mut exit_code = 0;
    let mut active_worker = routed_worker.primary.clone();
    let mut stdout_text = format!(
        "Task type: {}\nRoute summary: {}\nPrimary provider: {}\nPrimary base URL: {}\nPrimary model: {}\nPrompt file: {}\nOutput file: {}\n",
        routed_worker.task_type,
        routed_worker.route_summary,
        routed_worker.primary.provider,
        routed_worker.primary.base_url,
        routed_worker.primary.model,
        prompt_file.display(),
        output_file.display()
    );
    if let Some(fallback) = &routed_worker.fallback {
        stdout_text.push_str(&format!(
            "Fallback provider: {}\nFallback base URL: {}\nFallback model: {}\n",
            fallback.provider, fallback.base_url, fallback.model
        ));
    }
    let mut stderr_text = String::new();

    let provider_result = match call_provider(&prompt_text, &routed_worker.primary) {
        Ok(output_text) => Ok(output_text),
        Err(primary_err) => {
            stderr_text.push_str(&format!("Primary worker failed: {primary_err:#}\n"));
            if let Some(fallback_worker) = &routed_worker.fallback {
                stdout_text.push_str("Attempting fallback worker.\n");
                match call_provider(&prompt_text, fallback_worker) {
                    Ok(output_text) => {
                        stdout_text.push_str("Fallback worker succeeded.\n");
                        active_worker = fallback_worker.clone();
                        Ok(output_text)
                    }
                    Err(fallback_err) => {
                        stderr_text.push_str(&format!("Fallback worker failed: {fallback_err:#}\n"));
                        Err(fallback_err)
                    }
                }
            } else {
                Err(primary_err)
            }
        }
    };

    match provider_result {
        Ok(output_text) => {
            stdout_text.push_str(&format!("Generated {} characters.\n", output_text.chars().count()));
            if fs::write(&output_file, output_text).is_err() {
                exit_code = 1;
                stderr_text.push_str("Failed to write provider output file.\n");
                let _ = fs::write(
                    &output_file,
                    failure_output(
                        &active_worker,
                        "Provider responded, but the output file could not be written.",
                    ),
                );
            }
        }
        Err(err) => {
            exit_code = 1;
            stderr_text.push_str(&format!("{err:#}\n"));
            let _ = fs::write(&output_file, failure_output(&active_worker, &err.to_string()));
        }
    }

    let _ = fs::write(&stdout_file, &stdout_text);
    let _ = fs::write(&stderr_file, &stderr_text);

    if let Some(team_output_file) = &team_output_file {
        if let Ok(output_text) = fs::read_to_string(&output_file) {
            let _ = fs::write(team_output_file, &output_text);
            if let Some(team_root) = team_output_file.parent().and_then(Path::parent) {
                let _ = fs::write(team_root.join("latest.md"), output_text);
            }
        }
    }

    if let Some(grant_output_file) = &grant_output_file {
        if let Ok(output_text) = fs::read_to_string(&output_file) {
            let _ = fs::write(grant_output_file, &output_text);
            if let Some(grants_root) = grant_output_file.parent() {
                let _ = fs::write(grants_root.join("latest.md"), output_text);
            }
        }
    }

    let finished_at = Utc::now().to_rfc3339();
    let summary = summary_from_output(&output_file, &stdout_text);

    let metadata = serde_json::json!({
        "run_id": run_id,
        "job_id": job.job_id,
        "trigger": trigger,
        "started_at": started_at,
        "finished_at": finished_at,
        "exit_code": exit_code,
        "provider": active_worker.provider,
        "model": active_worker.model,
        "task_type": routed_worker.task_type,
        "route_summary": routed_worker.route_summary,
        "request_source": request_source.map(|path| path.display().to_string()),
        "role_id": role.map(|item| item.role_id.clone()),
        "agent_id": role
            .map(|item| item.agent_id.clone())
            .or_else(|| job.agent_id.clone()),
        "team_output_file": team_output_file.as_ref().map(|path| path.display().to_string()),
        "grant_output_file": grant_output_file.as_ref().map(|path| path.display().to_string()),
    });
    if let Ok(metadata_text) = serde_json::to_string_pretty(&metadata) {
        let _ = fs::write(&metadata_file, metadata_text);
    }

    WorkerRunResult {
        run_id,
        started_at,
        finished_at,
        exit_code,
        prompt_file,
        output_file,
        stdout_file,
        stderr_file,
        summary,
        team_output_file,
    }
}
