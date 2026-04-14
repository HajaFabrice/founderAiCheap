use crate::config::JobConfig;
use crate::state::AppState;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundLeadRequest {
    pub title: String,
    pub body: String,
    pub contact_name: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub linkedin_url: Option<String>,
    pub source_channel: String,
    pub risk_tags: Vec<String>,
    pub requires_internet: bool,
    pub priority: Option<String>,
    pub notes: Option<String>,
    pub nurture_template: Option<String>,
    pub auto_nurture: bool,
}

#[derive(Debug, Deserialize)]
struct LeadInboxPayload {
    title: Option<String>,
    body: Option<String>,
    workflow: Option<String>,
    kind: Option<String>,
    lead_status: Option<String>,
    agent_id: Option<String>,
    #[serde(default)]
    risk_tags: Vec<String>,
    requires_internet: Option<bool>,
    contact_name: Option<String>,
    organization: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    linkedin_url: Option<String>,
    source_channel: Option<String>,
    priority: Option<String>,
    notes: Option<String>,
    nurture_template: Option<String>,
    auto_nurture: Option<bool>,
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
        "lead".to_string()
    } else {
        trimmed
    }
}

fn is_inbound_lead_payload(payload: &LeadInboxPayload) -> bool {
    payload
        .workflow
        .as_deref()
        .map(|value| value.eq_ignore_ascii_case("inbound-lead"))
        .unwrap_or(false)
        || payload
            .kind
            .as_deref()
            .map(|value| value.eq_ignore_ascii_case("inbound-lead"))
            .unwrap_or(false)
        || payload
            .lead_status
            .as_deref()
            .map(|value| value.eq_ignore_ascii_case("inbound"))
            .unwrap_or(false)
        || payload
            .agent_id
            .as_deref()
            .map(|value| value.eq_ignore_ascii_case("zacchaeus"))
            .unwrap_or(false)
}

pub fn collect_inbound_lead_requests(
    inbox_dir: &Path,
    state: &AppState,
) -> Result<Vec<(PathBuf, InboundLeadRequest)>> {
    let mut found = Vec::new();
    for entry in fs::read_dir(inbox_dir)
        .with_context(|| format!("failed to list {}", inbox_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if !path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            continue;
        }
        if state
            .processed_inbox_requests
            .contains_key(&path.display().to_string())
        {
            continue;
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read inbound lead {}", path.display()))?;
        let payload: LeadInboxPayload = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse inbound lead {}", path.display()))?;
        if !is_inbound_lead_payload(&payload) {
            continue;
        }

        let title = payload.title.unwrap_or_else(|| {
            let fallback = payload
                .contact_name
                .clone()
                .or(payload.organization.clone())
                .unwrap_or_else(|| {
                    path.file_stem()
                        .and_then(|value| value.to_str())
                        .unwrap_or("inbound lead")
                        .to_string()
                });
            format!("Inbound lead from {fallback}")
        });

        let lead = InboundLeadRequest {
            title,
            body: payload.body.unwrap_or_default().trim().to_string(),
            contact_name: payload.contact_name,
            organization: payload.organization,
            email: payload.email,
            phone: payload.phone,
            linkedin_url: payload.linkedin_url,
            source_channel: payload
                .source_channel
                .unwrap_or_else(|| "unknown".to_string()),
            risk_tags: payload.risk_tags,
            requires_internet: payload.requires_internet.unwrap_or(false),
            priority: payload.priority,
            notes: payload.notes,
            nurture_template: payload.nurture_template,
            auto_nurture: payload.auto_nurture.unwrap_or(true),
        };
        found.push((path, lead));
    }

    found.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(found)
}

pub fn build_zacchaeus_job(lead: &InboundLeadRequest, request_path: &Path) -> JobConfig {
    let mut risk_tags = lead.risk_tags.clone();
    if !risk_tags.iter().any(|value| value == "external-send") {
        risk_tags.push("external-send".to_string());
    }

    let mut context = Vec::new();
    if let Some(name) = &lead.contact_name {
        context.push(format!("Contact name: {name}"));
    }
    if let Some(org) = &lead.organization {
        context.push(format!("Organization: {org}"));
    }
    if let Some(email) = &lead.email {
        context.push(format!("Email: {email}"));
    }
    if let Some(phone) = &lead.phone {
        context.push(format!("Phone: {phone}"));
    }
    if let Some(linkedin_url) = &lead.linkedin_url {
        context.push(format!("LinkedIn: {linkedin_url}"));
    }
    context.push(format!("Source channel: {}", lead.source_channel));
    if let Some(priority) = &lead.priority {
        context.push(format!("Priority: {priority}"));
    }
    if let Some(notes) = &lead.notes {
        context.push(format!("Notes: {notes}"));
    }

    let body = if lead.body.is_empty() {
        "No lead body was provided. Draft the safest useful holding response and explain what information is still needed."
            .to_string()
    } else {
        lead.body.clone()
    };

    let prompt = format!(
        "You are handling an inbound lead as Zacchaeus.\n\nLead context:\n{}\n\nLead message or summary:\n{}\n\nDeliverable:\n- Draft a quick, warm, transparent response in founder voice.\n- If a full answer is not yet safe, draft a holding reply that promises human follow-up.\n- Include a recommended next step and whether Perpetua should start nurture after this response.\n- Draft only. Do not send externally.",
        context.join("\n"),
        body
    );

    JobConfig {
        job_id: format!(
            "zacchaeus-{}",
            slugify(
                request_path
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .unwrap_or("lead")
            )
        ),
        description: format!("Inbound lead response for {}", lead.title),
        enabled: true,
        triggers: vec!["inbox_request".to_string()],
        prompt,
        interval_seconds: None,
        cooldown_seconds: 0,
        requires_internet: lead.requires_internet,
        approval_policy: "after_run".to_string(),
        risk_tags,
        mode: "single".to_string(),
        team_roles: Vec::new(),
        run_at_local: None,
        weekdays: Vec::new(),
        task_label: format!("Lead response: {}", lead.title),
        metric_value: Some(1),
        task_type: Some("draft".to_string()),
        agent_id: Some("zacchaeus".to_string()),
    }
}
