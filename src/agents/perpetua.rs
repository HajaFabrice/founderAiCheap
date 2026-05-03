use crate::agents::zacchaeus::InboundLeadRequest;
use crate::config::{AppConfig, JobConfig};
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_TEMPLATE_ID: &str = "warm-lead-default";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStepTemplate {
    pub step_id: String,
    pub delay_days: i64,
    pub channel: String,
    pub task_type: String,
    pub approval_policy: String,
    #[serde(default)]
    pub risk_tags: Vec<String>,
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceTemplate {
    pub template_id: String,
    pub description: String,
    pub steps: Vec<SequenceStepTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SequenceTemplateStore {
    pub default_template_id: String,
    pub templates: Vec<SequenceTemplate>,
}

impl Default for SequenceTemplateStore {
    fn default() -> Self {
        Self {
            default_template_id: DEFAULT_TEMPLATE_ID.to_string(),
            templates: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceHistoryEntry {
    pub step_id: String,
    pub channel: String,
    pub completed_at: String,
    pub outcome: String,
    #[serde(default)]
    pub run_id: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveSequence {
    pub sequence_id: String,
    pub template_id: String,
    pub status: String,
    pub contact_name: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub source_channel: String,
    pub started_at: String,
    pub next_due_at: Option<String>,
    pub current_step_index: usize,
    #[serde(default)]
    pub source_request: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub last_run_id: Option<String>,
    #[serde(default)]
    pub history: Vec<SequenceHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SequenceStateFile {
    #[serde(default)]
    pub entries: Vec<ActiveSequence>,
}

#[derive(Debug, Clone)]
pub enum SequenceAction {
    Draft {
        sequence_id: String,
        step_id: String,
        channel: String,
        job: JobConfig,
    },
    PhoneFlag {
        sequence_id: String,
        step_id: String,
        title: String,
        body: String,
    },
}

#[derive(Debug, Clone)]
pub enum SequenceActionOutcome {
    Drafted {
        run_id: Option<String>,
        note: Option<String>,
    },
    Flagged {
        note: Option<String>,
    },
}

fn slugify(value: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in value.trim().chars() {
        if character.is_ascii_alphanumeric() {
            output.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    let trimmed = output.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "sequence".to_string()
    } else {
        trimmed
    }
}

fn template_path(config: &AppConfig) -> PathBuf {
    config
        .config_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("nurture_sequences.json")
}

fn runtime_state_path(config: &AppConfig) -> PathBuf {
    config.runtime_dir.join("nurture").join("sequences.json")
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn load_templates(config: &AppConfig) -> Result<SequenceTemplateStore> {
    let path = template_path(config);
    let raw =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}

fn load_runtime_state(config: &AppConfig) -> Result<SequenceStateFile> {
    let path = runtime_state_path(config);
    if !path.exists() {
        return Ok(SequenceStateFile::default());
    }
    let raw =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}

fn save_runtime_state(config: &AppConfig, state: &SequenceStateFile) -> Result<()> {
    let path = runtime_state_path(config);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    let payload =
        serde_json::to_string_pretty(state).context("failed to serialize nurture state")?;
    fs::write(&path, payload).with_context(|| format!("failed to write {}", path.display()))
}

fn lookup_template<'a>(
    templates: &'a SequenceTemplateStore,
    template_id: Option<&str>,
) -> Result<&'a SequenceTemplate> {
    let resolved_id = template_id
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(&templates.default_template_id);
    templates
        .templates
        .iter()
        .find(|template| template.template_id == resolved_id)
        .ok_or_else(|| anyhow::anyhow!("nurture template '{}' was not found", resolved_id))
}

fn step_due_at(started_at: DateTime<Utc>, step: &SequenceStepTemplate) -> DateTime<Utc> {
    started_at + Duration::days(step.delay_days)
}

fn active_dedupe_key(lead: &InboundLeadRequest) -> String {
    let seed = lead
        .email
        .clone()
        .or(lead.organization.clone())
        .or(lead.contact_name.clone())
        .unwrap_or_else(|| "lead".to_string());
    slugify(&seed)
}

pub fn ensure_nurture_files(config: &AppConfig) -> Result<()> {
    let path = runtime_state_path(config);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    if !path.exists() {
        save_runtime_state(config, &SequenceStateFile::default())?;
    }
    Ok(())
}

pub fn active_sequence_count(config: &AppConfig) -> Result<usize> {
    ensure_nurture_files(config)?;
    Ok(load_runtime_state(config)?
        .entries
        .into_iter()
        .filter(|entry| entry.status.eq_ignore_ascii_case("active"))
        .count())
}

pub fn register_sequence_from_lead(
    config: &AppConfig,
    lead: &InboundLeadRequest,
    request_source: &Path,
) -> Result<Option<String>> {
    if !lead.auto_nurture {
        return Ok(None);
    }

    ensure_nurture_files(config)?;
    let templates = load_templates(config)?;
    let template = lookup_template(&templates, lead.nurture_template.as_deref())?;
    if template.steps.is_empty() {
        return Ok(None);
    }

    let mut state = load_runtime_state(config)?;
    let request_key = request_source.display().to_string();
    if let Some(existing) = state.entries.iter().find(|entry| {
        entry.status.eq_ignore_ascii_case("active")
            && (entry.source_request.as_deref() == Some(request_key.as_str())
                || entry.sequence_id.contains(&active_dedupe_key(lead)))
    }) {
        return Ok(Some(existing.sequence_id.clone()));
    }

    let started_at = Utc::now();
    let sequence_id = format!(
        "perpetua-{}-{}",
        active_dedupe_key(lead),
        started_at.format("%Y%m%d%H%M%S")
    );
    let first_due = step_due_at(started_at, &template.steps[0]).to_rfc3339();

    state.entries.push(ActiveSequence {
        sequence_id: sequence_id.clone(),
        template_id: template.template_id.clone(),
        status: "active".to_string(),
        contact_name: lead.contact_name.clone(),
        organization: lead.organization.clone(),
        email: lead.email.clone(),
        phone: lead.phone.clone(),
        linkedin_url: lead.linkedin_url.clone(),
        source_channel: lead.source_channel.clone(),
        started_at: started_at.to_rfc3339(),
        next_due_at: Some(first_due),
        current_step_index: 0,
        source_request: Some(request_key),
        notes: lead.notes.clone(),
        last_run_id: None,
        history: Vec::new(),
    });
    save_runtime_state(config, &state)?;

    Ok(Some(sequence_id))
}

pub fn collect_due_actions(config: &AppConfig, now: DateTime<Utc>) -> Result<Vec<SequenceAction>> {
    ensure_nurture_files(config)?;
    let templates = load_templates(config)?;
    let state = load_runtime_state(config)?;
    let mut actions = Vec::new();

    for sequence in state.entries {
        if !sequence.status.eq_ignore_ascii_case("active") {
            continue;
        }
        let Some(next_due_at) = &sequence.next_due_at else {
            continue;
        };
        let Some(next_due) = parse_timestamp(next_due_at) else {
            continue;
        };
        if next_due > now {
            continue;
        }

        let template = match lookup_template(&templates, Some(&sequence.template_id)) {
            Ok(template) => template,
            Err(_) => continue,
        };
        let Some(step) = template.steps.get(sequence.current_step_index).cloned() else {
            continue;
        };

        if step.channel.eq_ignore_ascii_case("phone")
            || step.channel.eq_ignore_ascii_case("phone_flag")
        {
            let title = format!(
                "Perpetua phone follow-up flag for {}",
                sequence
                    .contact_name
                    .clone()
                    .or(sequence.organization.clone())
                    .unwrap_or_else(|| sequence.sequence_id.clone())
            );
            let body = format!(
                "Sequence ID: {}\nStep: {}\nContact: {}\nOrganization: {}\nPhone: {}\nSource channel: {}\n\nAction:\n- A phone follow-up is due.\n- Review the sequence history and decide whether to call or pause the sequence.\n- Do not treat this as an autonomous external action.\n\nPrompt hint:\n{}",
                sequence.sequence_id,
                step.step_id,
                sequence.contact_name.clone().unwrap_or_else(|| "unknown".to_string()),
                sequence.organization.clone().unwrap_or_else(|| "unknown".to_string()),
                sequence.phone.clone().unwrap_or_else(|| "unknown".to_string()),
                sequence.source_channel,
                step.prompt
            );
            actions.push(SequenceAction::PhoneFlag {
                sequence_id: sequence.sequence_id.clone(),
                step_id: step.step_id.clone(),
                title,
                body,
            });
            continue;
        }

        let mut risk_tags = step.risk_tags.clone();
        if !risk_tags.iter().any(|value| value == "external-send") {
            risk_tags.push("external-send".to_string());
        }

        let prompt = format!(
            "You are Perpetua, continuing a nurture sequence.\n\nSequence ID: {}\nCurrent step: {}\nChannel: {}\nContact name: {}\nOrganization: {}\nEmail: {}\nLinkedIn: {}\nSource channel: {}\nNotes: {}\n\nStep objective:\n{}\n\nDeliverable:\n- Draft the next respectful follow-up for approval.\n- Keep it concise, transparent, and useful.\n- If the best move is to pause or stop, state that clearly.\n- Draft only. Do not send externally.",
            sequence.sequence_id,
            step.step_id,
            step.channel,
            sequence.contact_name.clone().unwrap_or_else(|| "unknown".to_string()),
            sequence.organization.clone().unwrap_or_else(|| "unknown".to_string()),
            sequence.email.clone().unwrap_or_else(|| "unknown".to_string()),
            sequence.linkedin_url.clone().unwrap_or_else(|| "unknown".to_string()),
            sequence.source_channel,
            sequence.notes.clone().unwrap_or_else(|| "none".to_string()),
            step.prompt
        );

        actions.push(SequenceAction::Draft {
            sequence_id: sequence.sequence_id.clone(),
            step_id: step.step_id.clone(),
            channel: step.channel.clone(),
            job: JobConfig {
                job_id: format!("perpetua-{}-{}", sequence.sequence_id, step.step_id),
                description: format!("Perpetua nurture step {}", step.step_id),
                enabled: true,
                triggers: vec!["periodic".to_string()],
                prompt,
                interval_seconds: None,
                cooldown_seconds: 0,
                requires_internet: false,
                approval_policy: step.approval_policy.clone(),
                risk_tags,
                mode: "single".to_string(),
                team_roles: Vec::new(),
                run_at_local: None,
                weekdays: Vec::new(),
                task_label: format!("Perpetua {}", step.step_id),
                metric_value: Some(1),
                task_type: Some(step.task_type.clone()),
                agent_id: Some("perpetua".to_string()),
            },
        });
    }

    Ok(actions)
}

pub fn complete_action(
    config: &AppConfig,
    sequence_id: &str,
    step_id: &str,
    outcome: SequenceActionOutcome,
) -> Result<()> {
    ensure_nurture_files(config)?;
    let templates = load_templates(config)?;
    let mut state = load_runtime_state(config)?;
    let sequence = state
        .entries
        .iter_mut()
        .find(|entry| entry.sequence_id == sequence_id)
        .ok_or_else(|| anyhow::anyhow!("sequence '{}' was not found", sequence_id))?;

    let template = lookup_template(&templates, Some(&sequence.template_id))?;
    let current_step = template
        .steps
        .get(sequence.current_step_index)
        .ok_or_else(|| anyhow::anyhow!("sequence '{}' has no pending step", sequence_id))?;
    if current_step.step_id != step_id {
        anyhow::bail!(
            "sequence '{}' expected step '{}' but got '{}'",
            sequence_id,
            current_step.step_id,
            step_id
        );
    }

    let (run_id, note, outcome_label) = match outcome {
        SequenceActionOutcome::Drafted { run_id, note } => (run_id, note, "drafted".to_string()),
        SequenceActionOutcome::Flagged { note } => (None, note, "flagged".to_string()),
    };

    sequence.history.push(SequenceHistoryEntry {
        step_id: step_id.to_string(),
        channel: current_step.channel.clone(),
        completed_at: Utc::now().to_rfc3339(),
        outcome: outcome_label,
        run_id: run_id.clone(),
        note,
    });
    sequence.last_run_id = run_id;
    sequence.current_step_index += 1;
    if let Some(next_step) = template.steps.get(sequence.current_step_index) {
        let started_at = parse_timestamp(&sequence.started_at).unwrap_or_else(Utc::now);
        sequence.next_due_at = Some(step_due_at(started_at, next_step).to_rfc3339());
    } else {
        sequence.status = "completed".to_string();
        sequence.next_due_at = None;
    }

    save_runtime_state(config, &state)
}
