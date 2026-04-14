use crate::config::{AppConfig, JobConfig, TeamRoleConfig};
use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineQueueEntry {
    pub queue_id: String,
    pub dedupe_key: String,
    pub queued_at: String,
    pub reason: String,
    pub trigger: String,
    pub job: JobConfig,
    pub role_id: Option<String>,
    pub request_source: Option<String>,
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
        "offline-entry".to_string()
    } else {
        trimmed
    }
}

fn queue_root(config: &AppConfig) -> PathBuf {
    let configured = PathBuf::from(&config.offline_queue.relative_path);
    if configured.is_absolute() {
        configured
    } else {
        let raw = config.offline_queue.relative_path.replace('\\', "/");
        if raw.starts_with("runtime/") || raw == "runtime" {
            config.workspace_root.join(configured)
        } else {
            config.runtime_dir.join(configured)
        }
    }
}

fn pending_dir(config: &AppConfig) -> PathBuf {
    queue_root(config).join("pending")
}

fn replayed_dir(config: &AppConfig) -> PathBuf {
    queue_root(config).join("replayed")
}

fn failed_dir(config: &AppConfig) -> PathBuf {
    queue_root(config).join("failed")
}

pub fn ensure_offline_queue_dirs(config: &AppConfig) -> Result<()> {
    for path in [queue_root(config), pending_dir(config), replayed_dir(config), failed_dir(config)] {
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create {}", path.display()))?;
    }
    Ok(())
}

fn dedupe_key(job: &JobConfig, role: Option<&TeamRoleConfig>, request_source: Option<&Path>) -> String {
    if let Some(request_source) = request_source {
        return format!(
            "request::{}",
            request_source
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or("request")
        );
    }

    let logical_job_id = job.job_id.split_once("--").map(|(left, _)| left).unwrap_or(&job.job_id);
    if let Some(role) = role {
        format!("job::{logical_job_id}::{}", role.role_id)
    } else {
        format!("job::{logical_job_id}")
    }
}

fn pending_json_path(config: &AppConfig, dedupe_key: &str) -> PathBuf {
    pending_dir(config).join(format!("{}.json", slugify(dedupe_key)))
}

fn pending_summary_path(config: &AppConfig, dedupe_key: &str) -> PathBuf {
    pending_dir(config).join(format!("{}.md", slugify(dedupe_key)))
}

pub fn count_pending_entries(config: &AppConfig) -> Result<usize> {
    if !pending_dir(config).exists() {
        return Ok(0);
    }

    let mut total = 0;
    for entry in fs::read_dir(pending_dir(config))
        .with_context(|| format!("failed to list {}", pending_dir(config).display()))?
    {
        let path = entry?.path();
        if path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            total += 1;
        }
    }
    Ok(total)
}

pub fn enqueue_offline_job(
    config: &AppConfig,
    job: &JobConfig,
    trigger: &str,
    role: Option<&TeamRoleConfig>,
    request_source: Option<&Path>,
    reason: &str,
) -> Result<String> {
    ensure_offline_queue_dirs(config)?;

    let dedupe_key = dedupe_key(job, role, request_source);
    let json_path = pending_json_path(config, &dedupe_key);
    if json_path.exists() {
        return Ok(dedupe_key);
    }

    let queue_id = format!(
        "{}-{}",
        Utc::now().format("%Y%m%dT%H%M%SZ"),
        slugify(&dedupe_key)
    );
    let entry = OfflineQueueEntry {
        queue_id,
        dedupe_key: dedupe_key.clone(),
        queued_at: Utc::now().to_rfc3339(),
        reason: reason.to_string(),
        trigger: trigger.to_string(),
        job: job.clone(),
        role_id: role.map(|value| value.role_id.clone()),
        request_source: request_source.map(|path| path.display().to_string()),
    };

    let payload = serde_json::to_string_pretty(&entry).context("failed to serialize offline queue entry")?;
    fs::write(&json_path, payload)
        .with_context(|| format!("failed to write {}", json_path.display()))?;

    let role_text = role
        .map(|value| format!("{} / {}", value.role_id, value.saint_name))
        .unwrap_or_else(|| "none".to_string());
    let summary = format!(
        "# Offline Queue Entry\n\n- Queue ID: {}\n- Dedupe key: {}\n- Queued at: {}\n- Reason: {}\n- Job ID: {}\n- Role: {}\n- Trigger: {}\n- Request source: {}\n\n## Prompt\n\n{}\n",
        entry.queue_id,
        entry.dedupe_key,
        entry.queued_at,
        entry.reason,
        entry.job.job_id,
        role_text,
        entry.trigger,
        entry.request_source.clone().unwrap_or_else(|| "none".to_string()),
        entry.job.prompt
    );
    let summary_path = pending_summary_path(config, &dedupe_key);
    fs::write(&summary_path, summary)
        .with_context(|| format!("failed to write {}", summary_path.display()))?;

    Ok(dedupe_key)
}

pub fn replay_pending_entries<F>(config: &AppConfig, mut replay: F) -> Result<usize>
where
    F: FnMut(&OfflineQueueEntry) -> Result<()>,
{
    ensure_offline_queue_dirs(config)?;
    let mut entries = Vec::new();
    for entry in fs::read_dir(pending_dir(config))
        .with_context(|| format!("failed to list {}", pending_dir(config).display()))?
    {
        let path = entry?.path();
        if path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            entries.push(path);
        }
    }
    entries.sort();

    let mut replayed = 0;
    for path in entries {
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let entry: OfflineQueueEntry = serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse {}", path.display()))?;
        replay(&entry)?;

        let destination = replayed_dir(config).join(
            path.file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("offline-entry.json"),
        );
        fs::rename(&path, &destination)
            .with_context(|| format!("failed to move {} to {}", path.display(), destination.display()))?;

        let summary_path = pending_summary_path(config, &entry.dedupe_key);
        if summary_path.exists() {
            let summary_destination = replayed_dir(config).join(
                summary_path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("offline-entry.md"),
            );
            fs::rename(&summary_path, &summary_destination).with_context(|| {
                format!(
                    "failed to move {} to {}",
                    summary_path.display(),
                    summary_destination.display()
                )
            })?;
        }

        replayed += 1;
    }

    Ok(replayed)
}
