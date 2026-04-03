use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessedInboxRequest {
    pub status: String,
    pub processed_at: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobState {
    pub last_started_at: Option<String>,
    pub last_finished_at: Option<String>,
    pub last_trigger: Option<String>,
    pub last_run_id: Option<String>,
    pub last_exit_code: Option<i32>,
    pub last_summary: Option<String>,
    pub last_schedule_key: Option<String>,
    pub logical_job_id: Option<String>,
    pub role_id: Option<String>,
    pub pending_approval_id: Option<String>,
    pub pending_approval_phase: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoleState {
    pub last_job_id: Option<String>,
    pub last_run_id: Option<String>,
    pub last_summary: Option<String>,
    pub last_status: Option<String>,
    pub last_metric_value: Option<i64>,
    pub last_output_file: Option<String>,
    pub last_finished_at: Option<String>,
    pub pending_approval_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    pub last_internet_available: Option<bool>,
    #[serde(default)]
    pub jobs: BTreeMap<String, JobState>,
    #[serde(default)]
    pub roles: BTreeMap<String, RoleState>,
    #[serde(default)]
    pub processed_inbox_requests: BTreeMap<String, ProcessedInboxRequest>,
}

impl AppState {
    pub fn normalize(&mut self) {
        for (job_id, job_state) in self.jobs.iter_mut() {
            if job_state.logical_job_id.is_none() {
                job_state.logical_job_id = Some(job_id.split_once("--").map(|(left, _)| left).unwrap_or(job_id).to_string());
            }
        }
    }

    pub fn ensure_job_state(&mut self, job_id: &str) -> &mut JobState {
        self.jobs.entry(job_id.to_string()).or_insert_with(|| JobState {
            logical_job_id: Some(job_id.split_once("--").map(|(left, _)| left).unwrap_or(job_id).to_string()),
            ..JobState::default()
        })
    }

    pub fn ensure_role_state(&mut self, role_id: &str) -> &mut RoleState {
        self.roles.entry(role_id.to_string()).or_default()
    }
}

pub fn load_state(state_path: &Path) -> Result<AppState> {
    if !state_path.exists() {
        return Ok(AppState::default());
    }
    let raw = fs::read_to_string(state_path)
        .with_context(|| format!("failed to read state {}", state_path.display()))?;
    let mut state: AppState = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse state {}", state_path.display()))?;
    state.normalize();
    Ok(state)
}

pub fn save_state(state_path: &Path, state: &AppState) -> Result<()> {
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create state dir {}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(state).context("failed to serialize state")?;
    fs::write(state_path, text)
        .with_context(|| format!("failed to write state {}", state_path.display()))?;
    Ok(())
}
