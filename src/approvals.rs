use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalDecision {
    Pending,
    Approved,
    Rejected,
}

impl ApprovalDecision {
    fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub approval_id: String,
    pub job_id: String,
    pub phase: String,
    pub status: String,
    pub reason: String,
    pub summary: String,
    pub artifacts: Vec<String>,
    pub risk_tags: Vec<String>,
    pub created_at: String,
    pub decision_notes: Option<String>,
    pub decided_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PendingApproval {
    pub record: ApprovalRecord,
    pub summary_path: PathBuf,
}

pub fn approvals_root(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("approvals")
}

pub fn ensure_approval_dirs(runtime_dir: &Path) -> Result<()> {
    for name in ["pending", "approved", "rejected"] {
        fs::create_dir_all(approvals_root(runtime_dir).join(name))
            .with_context(|| format!("failed to create approval dir {}", name))?;
    }
    Ok(())
}

fn summary_path(runtime_dir: &Path, approval_id: &str) -> PathBuf {
    approvals_root(runtime_dir)
        .join("pending")
        .join(format!("{approval_id}.summary.md"))
}

fn command_path(runtime_dir: &Path, approval_id: &str, action: &str) -> PathBuf {
    let extension = if cfg!(windows) { "cmd" } else { "sh" };
    approvals_root(runtime_dir)
        .join("pending")
        .join(format!("{approval_id}.{action}.{extension}"))
}

fn quoted_windows_path(path: &Path) -> String {
    format!("\"{}\"", path.display())
}

fn quote_shell(text: &str) -> String {
    format!("'{}'", text.replace('\'', "'\"'\"'"))
}

fn approval_command(executable_path: &Path, approval_id: &str, action: &str, config_path: &Path, notes: Option<&str>) -> String {
    let mut command = if cfg!(windows) {
        format!(
            "{} {} \"{}\" --config \"{}\"",
            quoted_windows_path(executable_path),
            action,
            approval_id,
            config_path.display()
        )
    } else {
        format!(
            "{} {} {} --config {}",
            quote_shell(&executable_path.display().to_string()),
            action,
            quote_shell(approval_id),
            quote_shell(&config_path.display().to_string())
        )
    };
    if let Some(notes_text) = notes {
        if cfg!(windows) {
            command.push_str(&format!(" --notes \"{}\"", notes_text.replace('"', "'")));
        } else {
            command.push_str(&format!(" --notes {}", quote_shell(notes_text)));
        }
    }
    command
}

fn write_summary_files(
    runtime_dir: &Path,
    approval_id: &str,
    payload: &ApprovalRecord,
    config_path: &Path,
    executable_path: &Path,
) -> Result<()> {
    let approve_cmd = approval_command(executable_path, approval_id, "approve", config_path, None);
    let reject_cmd = approval_command(
        executable_path,
        approval_id,
        "reject",
        config_path,
        Some("Needs changes"),
    );
    let shell_label = if cfg!(windows) { "PowerShell" } else { "Bash" };

    let artifacts = if payload.artifacts.is_empty() {
        "- None".to_string()
    } else {
        payload
            .artifacts
            .iter()
            .map(|artifact| format!("- `{artifact}`"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let summary = format!(
        "# FounderAI Approval Summary\n\nApproval ID: `{}`\nJob ID: `{}`\nPhase: `{}`\nRisk tags: `{}`\n\n## Summary\n\n{}\n\n## Artifacts\n\n{}\n\n## One-Step Commands\n\nApprove:\n\n```{}\n{}\n```\n\nReject:\n\n```{}\n{}\n```\n",
        payload.approval_id,
        payload.job_id,
        payload.phase,
        if payload.risk_tags.is_empty() {
            "none".to_string()
        } else {
            payload.risk_tags.join(", ")
        },
        payload.summary,
        artifacts,
        shell_label.to_ascii_lowercase(),
        approve_cmd,
        shell_label.to_ascii_lowercase(),
        reject_cmd
    );

    fs::write(summary_path(runtime_dir, approval_id), summary)
        .with_context(|| format!("failed to write approval summary for {approval_id}"))?;
    let line_ending = if cfg!(windows) { "\r\n" } else { "\n" };
    let approve_content = if cfg!(windows) {
        format!("{approve_cmd}{line_ending}")
    } else {
        format!("#!/usr/bin/env bash\nset -euo pipefail\n{approve_cmd}{line_ending}")
    };
    let reject_content = if cfg!(windows) {
        format!("{reject_cmd}{line_ending}")
    } else {
        format!("#!/usr/bin/env bash\nset -euo pipefail\n{reject_cmd}{line_ending}")
    };
    fs::write(command_path(runtime_dir, approval_id, "approve"), approve_content)
        .with_context(|| format!("failed to write approve helper for {approval_id}"))?;
    fs::write(command_path(runtime_dir, approval_id, "reject"), reject_content)
        .with_context(|| format!("failed to write reject helper for {approval_id}"))?;
    Ok(())
}

pub fn create_approval_request(
    runtime_dir: &Path,
    approval_id: &str,
    job_id: &str,
    phase: &str,
    reason: &str,
    summary: &str,
    artifacts: &[String],
    risk_tags: &[String],
    config_path: &Path,
    executable_path: &Path,
) -> Result<PathBuf> {
    ensure_approval_dirs(runtime_dir)?;
    let payload = ApprovalRecord {
        approval_id: approval_id.to_string(),
        job_id: job_id.to_string(),
        phase: phase.to_string(),
        status: ApprovalDecision::Pending.as_str().to_string(),
        reason: reason.to_string(),
        summary: summary.to_string(),
        artifacts: artifacts.to_vec(),
        risk_tags: risk_tags.to_vec(),
        created_at: Utc::now().to_rfc3339(),
        decision_notes: None,
        decided_at: None,
    };
    let path = approvals_root(runtime_dir)
        .join("pending")
        .join(format!("{approval_id}.json"));
    let payload_text = serde_json::to_string_pretty(&payload).context("failed to serialize approval payload")?;
    fs::write(&path, payload_text)
        .with_context(|| format!("failed to write approval payload {}", path.display()))?;
    write_summary_files(runtime_dir, approval_id, &payload, config_path, executable_path)?;
    Ok(path)
}

pub fn list_pending_approvals(runtime_dir: &Path) -> Result<Vec<PendingApproval>> {
    ensure_approval_dirs(runtime_dir)?;
    let pending_dir = approvals_root(runtime_dir).join("pending");
    let mut approvals = Vec::new();
    for entry in fs::read_dir(&pending_dir)
        .with_context(|| format!("failed to list {}", pending_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read approval {}", path.display()))?;
        let record: ApprovalRecord = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse approval {}", path.display()))?;
        approvals.push(PendingApproval {
            summary_path: summary_path(runtime_dir, &record.approval_id),
            record,
        });
    }
    approvals.sort_by(|left, right| left.record.approval_id.cmp(&right.record.approval_id));
    Ok(approvals)
}

pub fn approval_decision(runtime_dir: &Path, approval_id: &str) -> Option<ApprovalDecision> {
    let root = approvals_root(runtime_dir);
    if root.join("approved").join(format!("{approval_id}.json")).exists() {
        return Some(ApprovalDecision::Approved);
    }
    if root.join("rejected").join(format!("{approval_id}.json")).exists() {
        return Some(ApprovalDecision::Rejected);
    }
    if root.join("pending").join(format!("{approval_id}.json")).exists() {
        return Some(ApprovalDecision::Pending);
    }
    None
}

pub fn decide_approval(runtime_dir: &Path, approval_id: &str, decision: ApprovalDecision, notes: &str) -> Result<PathBuf> {
    ensure_approval_dirs(runtime_dir)?;
    let pending_path = approvals_root(runtime_dir)
        .join("pending")
        .join(format!("{approval_id}.json"));
    let raw = fs::read_to_string(&pending_path)
        .with_context(|| format!("approval '{approval_id}' was not found in pending approvals"))?;
    let mut payload: ApprovalRecord = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse approval {approval_id}"))?;
    payload.status = decision.as_str().to_string();
    payload.decision_notes = if notes.trim().is_empty() {
        None
    } else {
        Some(notes.trim().to_string())
    };
    payload.decided_at = Some(Utc::now().to_rfc3339());

    let destination = approvals_root(runtime_dir)
        .join(decision.as_str())
        .join(format!("{approval_id}.json"));
    let payload_text = serde_json::to_string_pretty(&payload).context("failed to serialize approval decision")?;
    fs::write(&destination, payload_text)
        .with_context(|| format!("failed to write approval decision {}", destination.display()))?;

    fs::remove_file(&pending_path).ok();
    fs::remove_file(summary_path(runtime_dir, approval_id)).ok();
    fs::remove_file(command_path(runtime_dir, approval_id, "approve")).ok();
    fs::remove_file(command_path(runtime_dir, approval_id, "reject")).ok();

    Ok(destination)
}
