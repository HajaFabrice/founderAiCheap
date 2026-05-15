use anyhow::{Context, Result};
use chrono::Utc;
use csv::WriterBuilder;
use serde_json::{Map, Value};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const CSV_HEADERS: [&str; 7] = [
    "Date",
    "Team",
    "Role",
    "Task",
    "Status",
    "Notes",
    "MetricValue",
];

pub struct TeamActivityEntry<'a> {
    pub team: &'a str,
    pub role: &'a str,
    pub task: &'a str,
    pub status: &'a str,
    pub notes: &'a str,
    pub metric_value: i64,
    pub extra: Option<Map<String, Value>>,
}

pub fn ensure_log_files(runtime_dir: &Path) -> Result<(PathBuf, PathBuf)> {
    let logs_dir = runtime_dir.join("logs");
    fs::create_dir_all(&logs_dir)
        .with_context(|| format!("failed to create logs dir {}", logs_dir.display()))?;

    let csv_path = logs_dir.join("team_activity.csv");
    let jsonl_path = logs_dir.join("team_activity.jsonl");

    if !csv_path.exists() {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&csv_path)
            .with_context(|| format!("failed to create {}", csv_path.display()))?;
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
        writer
            .write_record(CSV_HEADERS)
            .with_context(|| format!("failed to write {}", csv_path.display()))?;
        writer
            .flush()
            .with_context(|| format!("failed to flush {}", csv_path.display()))?;
    }

    if !jsonl_path.exists() {
        fs::write(&jsonl_path, "")
            .with_context(|| format!("failed to create {}", jsonl_path.display()))?;
    }

    Ok((csv_path, jsonl_path))
}

pub fn append_team_activity(runtime_dir: &Path, entry: TeamActivityEntry<'_>) -> Result<()> {
    let (csv_path, jsonl_path) = ensure_log_files(runtime_dir)?;
    let date_value = Utc::now().date_naive().to_string();

    let csv_file = OpenOptions::new()
        .append(true)
        .open(&csv_path)
        .with_context(|| format!("failed to open {}", csv_path.display()))?;
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(csv_file);
    writer
        .write_record([
            &date_value,
            entry.team,
            entry.role,
            entry.task,
            entry.status,
            entry.notes,
            &entry.metric_value.to_string(),
        ])
        .with_context(|| format!("failed to append {}", csv_path.display()))?;
    writer
        .flush()
        .with_context(|| format!("failed to flush {}", csv_path.display()))?;

    let mut payload = Map::new();
    payload.insert("Date".to_string(), Value::String(date_value));
    payload.insert("Team".to_string(), Value::String(entry.team.to_string()));
    payload.insert("Role".to_string(), Value::String(entry.role.to_string()));
    payload.insert("Task".to_string(), Value::String(entry.task.to_string()));
    payload.insert(
        "Status".to_string(),
        Value::String(entry.status.to_string()),
    );
    payload.insert("Notes".to_string(), Value::String(entry.notes.to_string()));
    payload.insert(
        "MetricValue".to_string(),
        Value::Number(entry.metric_value.into()),
    );
    payload.insert(
        "timestamp_utc".to_string(),
        Value::String(Utc::now().to_rfc3339()),
    );
    if let Some(extra_map) = entry.extra {
        for (key, value) in extra_map {
            payload.insert(key, value);
        }
    }

    let mut jsonl_text =
        serde_json::to_string(&payload).context("failed to serialize team activity entry")?;
    jsonl_text.push('\n');
    let mut jsonl_file = OpenOptions::new()
        .append(true)
        .open(&jsonl_path)
        .with_context(|| format!("failed to open {}", jsonl_path.display()))?;
    jsonl_file
        .write_all(jsonl_text.as_bytes())
        .with_context(|| format!("failed to append {}", jsonl_path.display()))?;

    Ok(())
}
