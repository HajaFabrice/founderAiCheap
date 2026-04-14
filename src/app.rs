use crate::approvals::{
    approval_decision, create_approval_request, ensure_approval_dirs, list_pending_approvals, ApprovalDecision,
};
use crate::config::{AppConfig, JobConfig, TeamRoleConfig};
use crate::network::internet_is_available;
use crate::notifier::send_notification;
use crate::offline::{
    count_pending_entries, enqueue_offline_job, ensure_offline_queue_dirs, replay_pending_entries,
};
use crate::singleton::DaemonLock;
use crate::state::{load_state, save_state, AppState, ProcessedInboxRequest};
use crate::team_logging::{append_team_activity, ensure_log_files};
use crate::worker::{provider_status, run_worker, ProviderStatus, WorkerRunResult};
use anyhow::{Context, Result};
use chrono::{DateTime, Datelike, Local, NaiveTime, Utc, Weekday};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

fn local_now() -> DateTime<Local> {
    Local::now()
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
        "request".to_string()
    } else {
        trimmed
    }
}

fn display_job_label(job_label: Option<&str>, role: Option<&TeamRoleConfig>) -> Option<String> {
    let job_label = job_label?;
    if job_label.starts_with("inbox-") {
        if let Some(role) = role {
            return Some(format!("Daily packet for {}", role.role_id));
        }
    }
    Some(job_label.to_string())
}

fn parse_timestamp(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn weekday_index(value: &str) -> Option<Weekday> {
    match value.trim().to_ascii_lowercase().as_str() {
        "monday" => Some(Weekday::Mon),
        "tuesday" => Some(Weekday::Tue),
        "wednesday" => Some(Weekday::Wed),
        "thursday" => Some(Weekday::Thu),
        "friday" => Some(Weekday::Fri),
        "saturday" => Some(Weekday::Sat),
        "sunday" => Some(Weekday::Sun),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct InboxRequestPayload {
    title: Option<String>,
    body: Option<String>,
    #[serde(default)]
    risk_tags: Vec<String>,
    approval_policy: Option<String>,
    requires_internet: Option<bool>,
    role_id: Option<String>,
    task_type: Option<String>,
    agent_id: Option<String>,
}

pub struct AutonomyApp {
    pub config: AppConfig,
    state_path: PathBuf,
    log_path: PathBuf,
    lock_path: PathBuf,
    executable_path: PathBuf,
}

impl AutonomyApp {
    pub fn new(config: AppConfig, executable_path: PathBuf) -> Self {
        Self {
            state_path: config.runtime_dir.join("state.json"),
            log_path: config.runtime_dir.join("logs").join("founderai.log"),
            lock_path: config.runtime_dir.join("founderai-daemon.lock"),
            executable_path,
            config,
        }
    }

    pub fn ensure_runtime(&self) -> Result<()> {
        for path in [
            self.config.runtime_dir.clone(),
            self.config.runtime_dir.join("logs"),
            self.config.runtime_dir.join("runs"),
            self.config.runtime_dir.join("teams"),
            self.config.runtime_dir.join("briefings"),
            self.config.runtime_dir.join("nurture"),
            self.config.inbox_dir.clone(),
            self.config.outbox_dir.clone(),
        ] {
            fs::create_dir_all(&path)
                .with_context(|| format!("failed to create {}", path.display()))?;
        }

        for role in self.config.team_roles.values() {
            let team_root = self.config.runtime_dir.join("teams").join(&role.role_id);
            for subdir in ["outputs", "plans", "requests"] {
                fs::create_dir_all(team_root.join(subdir)).with_context(|| {
                    format!("failed to create {}", team_root.join(subdir).display())
                })?;
            }
        }

        fs::create_dir_all(self.config.runtime_dir.join("teams").join("daily-plans"))
            .context("failed to create daily plans dir")?;
        ensure_approval_dirs(&self.config.runtime_dir)?;
        ensure_offline_queue_dirs(&self.config)?;
        ensure_log_files(&self.config.runtime_dir)?;
        Ok(())
    }

    pub fn log(&self, message: &str) {
        let line = format!("[{}] {}\n", utc_now().to_rfc3339(), message);
        if let Some(parent) = self.log_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let existing = fs::read_to_string(&self.log_path).unwrap_or_default();
        let mut next = existing;
        next.push_str(&line);
        fs::write(&self.log_path, next).ok();
    }

    fn notify_best_effort(&self, title: &str, body: &str) {
        if let Err(err) = send_notification(&self.config.notifier, title, body) {
            self.log(&format!("Notifier failed for '{title}': {err:#}"));
        }
    }

    fn load_state_or_default(&self) -> AppState {
        match load_state(&self.state_path) {
            Ok(state) => state,
            Err(err) => {
                self.log(&format!(
                    "State file could not be loaded from {}: {err:#}. Falling back to an empty state.",
                    self.state_path.display()
                ));
                AppState::default()
            }
        }
    }

    fn effective_risk_tags(&self, job: &JobConfig, role: Option<&TeamRoleConfig>) -> Vec<String> {
        let mut merged = job.risk_tags.clone();
        if let Some(role) = role {
            merged.extend(role.default_risk_tags.clone());
        }

        let mut deduped = Vec::new();
        for item in merged {
            if !deduped.contains(&item) {
                deduped.push(item);
            }
        }
        deduped
    }

    fn resolved_approval_policy(&self, job: &JobConfig, role: Option<&TeamRoleConfig>) -> String {
        if job.approval_policy != "inherit" {
            return job.approval_policy.clone();
        }
        if let Some(role) = role {
            if role.default_approval_policy != "inherit" {
                return role.default_approval_policy.clone();
            }
        }
        let effective_risk_tags = self.effective_risk_tags(job, role);
        let requires_validation = effective_risk_tags
            .iter()
            .any(|tag| self.config.strategic_validation.always_require_tags.contains(tag));
        if requires_validation {
            return self.config.strategic_validation.default_policy.clone();
        }
        "never".to_string()
    }

    fn approval_summary(
        &self,
        job: &JobConfig,
        trigger: &str,
        result: Option<&WorkerRunResult>,
        role: Option<&TeamRoleConfig>,
    ) -> String {
        let role_text = role
            .map(|role| format!(" for {}", role.role_id))
            .unwrap_or_default();
        if let Some(result) = result {
            format!(
                "FounderAI finished '{}'{} from trigger '{}' and is waiting for your validation.\n\nSummary: {}\nOutput: {}",
                job.job_id,
                role_text,
                trigger,
                result.summary,
                result
                    .team_output_file
                    .as_ref()
                    .unwrap_or(&result.output_file)
                    .display()
            )
        } else {
            format!(
                "FounderAI wants approval before running '{}'{} from trigger '{}'.\n\nTask: {}",
                job.job_id, role_text, trigger, job.prompt
            )
        }
    }

    fn request_approval(
        &self,
        state: &mut AppState,
        job: &JobConfig,
        trigger: &str,
        phase: &str,
        role: Option<&TeamRoleConfig>,
        result: Option<&WorkerRunResult>,
    ) -> Result<String> {
        if let Some(existing) = state.ensure_job_state(&job.job_id).pending_approval_id.clone() {
            return Ok(existing);
        }

        let role_suffix = role
            .map(|role| format!("-{}", role.role_id))
            .unwrap_or_default();
        let approval_id = format!(
            "{}-{}{}-{}",
            utc_now().format("%Y%m%dT%H%M%SZ"),
            job.job_id,
            role_suffix,
            phase
        );

        let mut artifacts = Vec::new();
        if let Some(result) = result {
            if let Some(team_output_file) = &result.team_output_file {
                artifacts.push(team_output_file.display().to_string());
            }
            artifacts.push(result.output_file.display().to_string());
            artifacts.push(result.prompt_file.display().to_string());
            artifacts.push(result.stdout_file.display().to_string());
            artifacts.push(result.stderr_file.display().to_string());
        }

        let summary = self.approval_summary(job, trigger, result, role);
        create_approval_request(
            &self.config.runtime_dir,
            &approval_id,
            &job.job_id,
            phase,
            &summary,
            &summary,
            &artifacts,
            &self.effective_risk_tags(job, role),
            &self.config.config_path,
            &self.executable_path,
        )?;

        {
            let job_state = state.ensure_job_state(&job.job_id);
            job_state.pending_approval_id = Some(approval_id.clone());
            job_state.pending_approval_phase = Some(phase.to_string());
            job_state.role_id = role.map(|role| role.role_id.clone());
        }
        if let Some(role) = role {
            state.ensure_role_state(&role.role_id).pending_approval_id = Some(approval_id.clone());
        }
        self.notify_best_effort(
            "FounderAI approval required",
            &format!(
                "Approval {approval_id} is waiting on '{}' ({phase}).",
                job.job_id
            ),
        );
        Ok(approval_id)
    }

    fn clear_pending_approval(&self, state: &mut AppState, job_id: &str, role_id: Option<&str>) {
        {
            let job_state = state.ensure_job_state(job_id);
            job_state.pending_approval_id = None;
            job_state.pending_approval_phase = None;
        }
        if let Some(role_id) = role_id {
            state.ensure_role_state(role_id).pending_approval_id = None;
        }
    }

    fn refresh_after_run_approvals(&self, state: &mut AppState) {
        let pending: Vec<(String, Option<String>, String)> = state
            .jobs
            .iter()
            .filter_map(|(job_id, job_state)| {
                if job_state.pending_approval_phase.as_deref() != Some("after_run") {
                    return None;
                }
                Some((
                    job_id.clone(),
                    job_state.role_id.clone(),
                    job_state.pending_approval_id.clone()?,
                ))
            })
            .collect();

        for (job_id, role_id, approval_id) in pending {
            match approval_decision(&self.config.runtime_dir, &approval_id) {
                Some(ApprovalDecision::Approved) | Some(ApprovalDecision::Rejected) => {
                    self.clear_pending_approval(state, &job_id, role_id.as_deref());
                }
                _ => {}
            }
        }
    }

    fn consume_pending_approval(
        &self,
        state: &mut AppState,
        job: &JobConfig,
        role: Option<&TeamRoleConfig>,
    ) -> Option<String> {
        let approval_id = state.ensure_job_state(&job.job_id).pending_approval_id.clone()?;
        match approval_decision(&self.config.runtime_dir, &approval_id) {
            Some(ApprovalDecision::Pending) => Some("pending".to_string()),
            Some(ApprovalDecision::Rejected) => {
                self.log(&format!("Approval rejected for {}: {}", job.job_id, approval_id));
                self.clear_pending_approval(state, &job.job_id, role.map(|item| item.role_id.as_str()));
                Some("rejected".to_string())
            }
            Some(ApprovalDecision::Approved) => {
                let phase = state
                    .ensure_job_state(&job.job_id)
                    .pending_approval_phase
                    .clone()
                    .unwrap_or_else(|| "approved".to_string());
                self.log(&format!("Approval approved for {}: {}", job.job_id, approval_id));
                self.clear_pending_approval(state, &job.job_id, role.map(|item| item.role_id.as_str()));
                Some(phase)
            }
            None => None,
        }
    }

    fn schedule_key(&self, job: &JobConfig, now_local: DateTime<Local>) -> Option<String> {
        let run_at_local = job.run_at_local.as_ref()?;
        let scheduled_time = NaiveTime::parse_from_str(run_at_local, "%H:%M").ok()?;
        if now_local.time() < scheduled_time {
            return None;
        }

        if !job.weekdays.is_empty() {
            let allowed_days = job
                .weekdays
                .iter()
                .filter_map(|item| weekday_index(item))
                .collect::<Vec<_>>();
            if !allowed_days.contains(&now_local.weekday()) {
                return None;
            }
        }

        Some(now_local.date_naive().to_string())
    }

    fn job_is_due(
        &self,
        job: &JobConfig,
        job_state: &crate::state::JobState,
        _current_internet: bool,
        trigger: &str,
        now_local: DateTime<Local>,
    ) -> (bool, Option<String>) {
        if !job.enabled {
            return (false, None);
        }
        if trigger != "periodic" {
            return (job.triggers.iter().any(|item| item == trigger), None);
        }

        if let Some(schedule_key) = self.schedule_key(job, now_local) {
            return (job_state.last_schedule_key.as_deref() != Some(schedule_key.as_str()), Some(schedule_key));
        }

        if job.triggers.iter().any(|item| item == "periodic") {
            if let Some(interval_seconds) = job.interval_seconds {
                if let Some(last_started_at) = &job_state.last_started_at {
                    if let Some(last_time) = parse_timestamp(last_started_at) {
                        let elapsed = utc_now().signed_duration_since(last_time).num_seconds();
                        return (elapsed >= interval_seconds as i64, None);
                    }
                }
                return (true, None);
            }
        }

        (false, None)
    }

    fn record_run(
        &self,
        state: &mut AppState,
        job: &JobConfig,
        trigger: &str,
        result: &WorkerRunResult,
        role: Option<&TeamRoleConfig>,
        metric_value: i64,
        task_label: &str,
        schedule_key: Option<&str>,
    ) {
        {
            let job_state = state.ensure_job_state(&job.job_id);
            job_state.last_started_at = Some(result.started_at.clone());
            job_state.last_finished_at = Some(result.finished_at.clone());
            job_state.last_trigger = Some(trigger.to_string());
            job_state.last_run_id = Some(result.run_id.clone());
            job_state.last_exit_code = Some(result.exit_code);
            job_state.last_summary = Some(result.summary.clone());
            job_state.role_id = role.map(|role| role.role_id.clone());
            if let Some(schedule_key) = schedule_key {
                job_state.last_schedule_key = Some(schedule_key.to_string());
            }
        }

        if let Some(role) = role {
            let status = if result.exit_code == 0 { "Completed" } else { "Failed" };
            let output_path = result
                .team_output_file
                .as_ref()
                .unwrap_or(&result.output_file)
                .display()
                .to_string();
            {
                let role_state = state.ensure_role_state(&role.role_id);
                role_state.last_job_id = Some(task_label.to_string());
                role_state.last_run_id = Some(result.run_id.clone());
                role_state.last_summary = Some(result.summary.clone());
                role_state.last_status = Some(status.to_string());
                role_state.last_metric_value = Some(metric_value);
                role_state.last_output_file = Some(output_path.clone());
                role_state.last_finished_at = Some(result.finished_at.clone());
            }

            let mut extra = Map::new();
            extra.insert("role_id".to_string(), Value::String(role.role_id.clone()));
            extra.insert("run_id".to_string(), Value::String(result.run_id.clone()));
            extra.insert(
                "job_id".to_string(),
                Value::String(job.job_id.split_once("--").map(|(left, _)| left).unwrap_or(&job.job_id).to_string()),
            );
            extra.insert("output_file".to_string(), Value::String(output_path));
            if let Err(err) = append_team_activity(
                &self.config.runtime_dir,
                &role.team,
                &role.role,
                task_label,
                status,
                &result.summary,
                metric_value,
                Some(extra),
            ) {
                self.log(&format!("Failed to append team activity for {}: {err:#}", role.role_id));
            }
        }
    }

    fn load_inbox_request(
        &self,
        file_path: &Path,
    ) -> Result<(
        String,
        String,
        Vec<String>,
        String,
        bool,
        Option<String>,
        Option<String>,
        Option<String>,
    )> {
        if file_path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.eq_ignore_ascii_case("json"))
            .unwrap_or(false)
        {
            let raw = fs::read_to_string(file_path)
                .with_context(|| format!("failed to read inbox request {}", file_path.display()))?;
            let payload: InboxRequestPayload = serde_json::from_str(&raw)
                .with_context(|| format!("failed to parse inbox request {}", file_path.display()))?;
            return Ok((
                payload.title.unwrap_or_else(|| file_path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("request").to_string()),
                payload.body.unwrap_or_default().trim().to_string(),
                payload.risk_tags,
                payload
                    .approval_policy
                    .unwrap_or_else(|| self.config.inbox_request_defaults.approval_policy.clone()),
                payload
                    .requires_internet
                    .unwrap_or(self.config.inbox_request_defaults.requires_internet),
                payload.role_id,
                payload.task_type,
                payload.agent_id,
            ));
        }

        let text = fs::read_to_string(file_path)
            .with_context(|| format!("failed to read inbox request {}", file_path.display()))?;
        Ok((
            file_path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("request")
                .to_string(),
            text.trim().to_string(),
            self.config.inbox_request_defaults.risk_tags.clone(),
            self.config.inbox_request_defaults.approval_policy.clone(),
            self.config.inbox_request_defaults.requires_internet,
            None,
            None,
            None,
        ))
    }

    fn build_inbox_job(&self, file_path: &Path) -> Result<(JobConfig, Option<TeamRoleConfig>)> {
        let (title, body, risk_tags, approval_policy, requires_internet, role_id, task_type, agent_id) =
            self.load_inbox_request(file_path)?;
        let role = role_id
            .as_deref()
            .and_then(|role_id| self.config.team_roles.get(role_id))
            .cloned();
        let prompt = if body.is_empty() { title.clone() } else { body };
        let metric_value = role.as_ref().map(|role| role.daily_quota).unwrap_or(1);
        let job = JobConfig {
            job_id: format!(
                "inbox-{}",
                slugify(file_path.file_stem().and_then(|value| value.to_str()).unwrap_or("request"))
            ),
            description: format!("Inbox request from {}", file_path.file_name().and_then(|value| value.to_str()).unwrap_or("request")),
            enabled: true,
            triggers: vec!["inbox_request".to_string()],
            prompt,
            interval_seconds: None,
            cooldown_seconds: 0,
            requires_internet,
            approval_policy,
            risk_tags,
            mode: "single".to_string(),
            team_roles: Vec::new(),
            run_at_local: None,
            weekdays: Vec::new(),
            task_label: title,
            metric_value: Some(metric_value),
            task_type,
            agent_id,
        };
        Ok((job, role))
    }

    fn mark_request_processed(&self, state: &mut AppState, file_path: &Path, status: &str) {
        state.processed_inbox_requests.insert(
            file_path.display().to_string(),
            ProcessedInboxRequest {
                status: status.to_string(),
                processed_at: utc_now().to_rfc3339(),
            },
        );
    }

    fn iter_new_inbox_requests(&self, state: &AppState) -> Result<Vec<PathBuf>> {
        let mut found = Vec::new();
        for entry in fs::read_dir(&self.config.inbox_dir)
            .with_context(|| format!("failed to list {}", self.config.inbox_dir.display()))?
        {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_ascii_lowercase();
            if !["md", "txt", "json"].contains(&extension.as_str()) {
                continue;
            }
            if !state
                .processed_inbox_requests
                .contains_key(&path.display().to_string())
            {
                found.push(path);
            }
        }
        found.sort();
        Ok(found)
    }

    fn role_specific_job(&self, job: &JobConfig, role: &TeamRoleConfig) -> JobConfig {
        let mut role_job = job.clone();
        role_job.job_id = format!("{}--{}", job.job_id, role.role_id);
        role_job.prompt = format!(
            "Role assignment: {} / {} ({} / {}).\nFocus: {}\nDaily quota: {} {}.\n\n{}",
            role.display_name,
            role.saint_name,
            role.role_id,
            role.agent_id,
            role.focus,
            role.daily_quota,
            role.metric_unit,
            job.prompt
        );
        role_job.approval_policy = self.resolved_approval_policy(job, Some(role));
        role_job.risk_tags = self.effective_risk_tags(job, Some(role));
        role_job.task_label = if job.task_label.is_empty() {
            job.description.clone()
        } else {
            job.task_label.clone()
        };
        if role_job.metric_value.is_none() {
            role_job.metric_value = Some(role.daily_quota);
        }
        if role_job.agent_id.is_none() {
            role_job.agent_id = Some(role.agent_id.clone());
        }
        role_job
    }

    fn create_daily_team_requests(&self, state: &mut AppState, job: &JobConfig, schedule_key: Option<&str>) -> Result<()> {
        let date_key = schedule_key
            .map(|value| value.to_string())
            .unwrap_or_else(|| local_now().date_naive().to_string());
        let mut plan_lines = vec![format!("# Daily Team Orchestration - {date_key}"), String::new()];
        let mut created = 0;

        for role in self.config.team_roles.values() {
            let request_name = format!("{date_key}-{}-daily-plan.json", role.role_id);
            let request_path = self.config.inbox_dir.join(&request_name);
            if request_path.exists() {
                continue;
            }

            let payload = serde_json::json!({
                "title": format!("Daily packet for {}", role.role_id),
                "body": format!(
                    "Operate as {} / {}.\nTeam: {}\nRole: {}\nAgent ID: {}\nDaily quota: {} {}\nFocus: {}\nPriority tasks:\n{}\nPrepare bounded, reviewable work only.\nDo not send externally without approval.",
                    role.display_name,
                    role.saint_name,
                    role.team,
                    role.role,
                    role.agent_id,
                    role.daily_quota,
                    role.metric_unit,
                    role.focus,
                    role
                        .responsibilities
                        .iter()
                        .map(|item| format!("- {item}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                ),
                "approval_policy": "never",
                "risk_tags": Vec::<String>::new(),
                "requires_internet": false,
                "role_id": role.role_id,
                "task_type": if role.role.eq_ignore_ascii_case("Outreach") { "draft" } else { "proposal" },
                "agent_id": role.agent_id,
            });
            let payload_text = serde_json::to_string_pretty(&payload).context("failed to serialize daily request")?;
            fs::write(&request_path, &payload_text)
                .with_context(|| format!("failed to write {}", request_path.display()))?;

            let team_copy = self
                .config
                .runtime_dir
                .join("teams")
                .join(&role.role_id)
                .join("requests")
                .join(&request_name);
            fs::write(&team_copy, payload_text)
                .with_context(|| format!("failed to write {}", team_copy.display()))?;

            created += 1;
            let mut extra = Map::new();
            extra.insert("role_id".to_string(), Value::String(role.role_id.clone()));
            extra.insert("request_file".to_string(), Value::String(request_path.display().to_string()));
            if let Err(err) = append_team_activity(
                &self.config.runtime_dir,
                &role.team,
                &role.role,
                "Daily Orchestration",
                "Queued",
                &format!("Daily packet created at {}", request_name),
                role.daily_quota,
                Some(extra),
            ) {
                self.log(&format!("Failed to append orchestration activity for {}: {err:#}", role.role_id));
            }
            plan_lines.push(format!(
                "- {}: queued daily packet with quota {} {}",
                role.role_id, role.daily_quota, role.metric_unit
            ));
        }

        let plan_path = self
            .config
            .runtime_dir
            .join("teams")
            .join("daily-plans")
            .join(format!("{date_key}.md"));
        fs::write(&plan_path, format!("{}\n", plan_lines.join("\n")))
            .with_context(|| format!("failed to write {}", plan_path.display()))?;

        let now = utc_now().to_rfc3339();
        let job_state = state.ensure_job_state(&job.job_id);
        job_state.last_started_at = Some(now.clone());
        job_state.last_finished_at = Some(now);
        job_state.last_trigger = Some("periodic".to_string());
        job_state.last_run_id = Some(date_key.clone());
        job_state.last_exit_code = Some(0);
        job_state.last_summary = Some(format!("Queued {} daily team packets.", created));
        job_state.last_schedule_key = Some(date_key);
        Ok(())
    }

    fn run_single_job(
        &self,
        state: &mut AppState,
        job: &JobConfig,
        trigger: &str,
        current_internet: bool,
        now_local: DateTime<Local>,
        request_source: Option<&Path>,
        role: Option<&TeamRoleConfig>,
    ) -> Result<()> {
        {
            let job_state = state.ensure_job_state(&job.job_id);
            if job_state.logical_job_id.is_none() {
                job_state.logical_job_id =
                    Some(job.job_id.split_once("--").map(|(left, _)| left).unwrap_or(&job.job_id).to_string());
            }
            job_state.role_id = role.map(|item| item.role_id.clone());
        }

        let (is_due, schedule_key) = {
            let job_state = state.ensure_job_state(&job.job_id).clone();
            self.job_is_due(job, &job_state, current_internet, trigger, now_local)
        };
        if !is_due {
            return Ok(());
        }

        if job.requires_internet && !current_internet {
            let queue_key = enqueue_offline_job(
                &self.config,
                job,
                trigger,
                role,
                request_source,
                "Connectivity unavailable for internet-required work.",
            )?;
            self.log(&format!(
                "Queued {} into offline queue with key {}.",
                job.job_id, queue_key
            ));
            self.notify_best_effort(
                "FounderAI queued offline work",
                &format!("{} was queued because internet was unavailable.", job.job_id),
            );
            if let Some(request_source) = request_source {
                self.mark_request_processed(state, request_source, "queued_offline");
            }
            return Ok(());
        }

        let approval_phase = self.consume_pending_approval(state, job, role);
        let resolved_policy = self.resolved_approval_policy(job, role);
        if approval_phase.as_deref() == Some("pending") {
            self.log(&format!("Job {} is waiting for approval.", job.job_id));
            return Ok(());
        }
        if approval_phase.as_deref() == Some("rejected") {
            if let Some(request_source) = request_source {
                self.mark_request_processed(state, request_source, "rejected");
            }
            return Ok(());
        }
        if resolved_policy == "before_run" && approval_phase.as_deref() != Some("before_run") {
            let approval_id = self.request_approval(state, job, trigger, "before_run", role, None)?;
            self.log(&format!("Created before-run approval {} for {}.", approval_id, job.job_id));
            return Ok(());
        }

        if job.cooldown_seconds > 0 {
            if let Some(last_finished_at) = state
                .ensure_job_state(&job.job_id)
                .last_finished_at
                .clone()
                .and_then(|value| parse_timestamp(&value))
            {
                let elapsed = utc_now().signed_duration_since(last_finished_at).num_seconds();
                if elapsed < job.cooldown_seconds as i64 {
                    return Ok(());
                }
            }
        }

        self.log(&format!("Running job {} from trigger {}.", job.job_id, trigger));
        let result = run_worker(
            &self.config,
            job,
            trigger,
            &self.config.runtime_dir,
            request_source,
            role,
            &self.effective_risk_tags(job, role),
            &resolved_policy,
            current_internet,
        );

        let metric_value = job
            .metric_value
            .or_else(|| role.map(|role| role.daily_quota))
            .unwrap_or(1);
        let task_label = if job.task_label.is_empty() {
            if job.description.is_empty() {
                job.job_id.clone()
            } else {
                job.description.clone()
            }
        } else {
            job.task_label.clone()
        };
        self.record_run(
            state,
            job,
            trigger,
            &result,
            role,
            metric_value,
            &task_label,
            schedule_key.as_deref(),
        );

        let outbox_copy = self.config.outbox_dir.join(format!("{}.md", result.run_id));
        if let Ok(output_text) = fs::read_to_string(&result.output_file) {
            if let Err(err) = fs::write(&outbox_copy, output_text) {
                self.log(&format!("Failed to write outbox copy {}: {err}", outbox_copy.display()));
            }
        }

        if let Some(request_source) = request_source {
            self.mark_request_processed(state, request_source, "completed");
        }

        if resolved_policy == "after_run" {
            let approval_id = self.request_approval(state, job, trigger, "after_run", role, Some(&result))?;
            self.log(&format!("Created after-run approval {} for {}.", approval_id, job.job_id));
        }

        if result.exit_code != 0 {
            self.notify_best_effort(
                "FounderAI run failed",
                &format!(
                    "{} failed during '{}' and wrote artifacts to {}.",
                    job.job_id,
                    trigger,
                    result.output_file.display()
                ),
            );
        }

        Ok(())
    }

    fn dispatch_job(
        &self,
        state: &mut AppState,
        job: &JobConfig,
        trigger: &str,
        current_internet: bool,
        now_local: DateTime<Local>,
        request_source: Option<&Path>,
        role: Option<&TeamRoleConfig>,
    ) -> Result<()> {
        if role.is_none()
            && current_internet
            && self.config.offline_queue.enabled
            && !self
                .config
                .offline_queue
                .replay_trigger
                .eq_ignore_ascii_case("internet_up")
            && self
                .config
                .offline_queue
                .replay_trigger
                .eq_ignore_ascii_case(&job.job_id)
        {
            self.replay_offline_queue(state, current_internet, now_local)?;
        }

        if role.is_some() {
            return self.run_single_job(state, job, trigger, current_internet, now_local, request_source, role);
        }

        if job.mode == "daily_orchestration" {
            let (is_due, schedule_key) = {
                let job_state = state.ensure_job_state(&job.job_id).clone();
                self.job_is_due(job, &job_state, current_internet, trigger, now_local)
            };
            if is_due {
                self.log(&format!("Creating daily team packets for {}.", job.job_id));
                self.create_daily_team_requests(state, job, schedule_key.as_deref())?;
            }
            return Ok(());
        }

        if job.mode == "per_role" {
            let mut processed = 0;
            let mut last_role_run_id = None;
            let mut last_exit_code = None;
            let mut pending_approvals = 0;

            for role_id in &job.team_roles {
                let Some(team_role) = self.config.team_roles.get(role_id).cloned() else {
                    self.log(&format!("Configured role '{}' was not found for job {}.", role_id, job.job_id));
                    continue;
                };
                let role_job = self.role_specific_job(job, &team_role);
                let previous_run_id = state
                    .jobs
                    .get(&role_job.job_id)
                    .and_then(|job_state| job_state.last_run_id.clone());
                self.run_single_job(
                    state,
                    &role_job,
                    trigger,
                    current_internet,
                    now_local,
                    None,
                    Some(&team_role),
                )?;
                if let Some(role_state) = state.jobs.get(&role_job.job_id) {
                    if role_state.last_run_id != previous_run_id {
                        processed += 1;
                        last_role_run_id = role_state.last_run_id.clone();
                        last_exit_code = role_state.last_exit_code;
                    }
                    if role_state.pending_approval_id.is_some() {
                        pending_approvals += 1;
                    }
                }
            }

            if processed > 0 || pending_approvals > 0 {
                let parent_state = state.ensure_job_state(&job.job_id);
                parent_state.logical_job_id = Some(job.job_id.clone());
                parent_state.last_run_id = last_role_run_id;
                parent_state.last_exit_code = last_exit_code;
                parent_state.last_started_at = Some(utc_now().to_rfc3339());
                parent_state.last_finished_at = Some(utc_now().to_rfc3339());
                parent_state.last_summary = Some(format!(
                    "Processed {}/{} role packets; pending approvals: {}.",
                    processed,
                    job.team_roles.len(),
                    pending_approvals
                ));
                parent_state.pending_approval_id = None;
                parent_state.pending_approval_phase = None;
            }
            return Ok(());
        }

        self.run_single_job(state, job, trigger, current_internet, now_local, request_source, None)
    }

    fn replay_offline_queue(
        &self,
        state: &mut AppState,
        current_internet: bool,
        now_local: DateTime<Local>,
    ) -> Result<()> {
        let replayed = replay_pending_entries(&self.config, |entry| {
            let role = entry
                .role_id
                .as_deref()
                .and_then(|role_id| self.config.team_roles.get(role_id))
                .cloned();
            let request_source = entry.request_source.as_deref().map(Path::new);
            self.dispatch_job(
                state,
                &entry.job,
                &entry.trigger,
                current_internet,
                now_local,
                request_source,
                role.as_ref(),
            )
        })?;

        if replayed > 0 {
            self.log(&format!("Replayed {replayed} offline queue item(s)."));
            self.notify_best_effort(
                "FounderAI replayed queued work",
                &format!("{replayed} offline queue item(s) were replayed after connectivity returned."),
            );
        }

        Ok(())
    }

    pub fn tick(&self, include_startup: bool, manual_trigger: Option<&str>) -> Result<()> {
        self.ensure_runtime()?;
        let mut state = self.load_state_or_default();
        self.refresh_after_run_approvals(&mut state);

        let previous_internet = state.last_internet_available;
        let current_internet = internet_is_available(&self.config.internet_check);
        state.last_internet_available = Some(current_internet);
        let now_local = local_now();

        if include_startup {
            for job in &self.config.jobs {
                if let Err(err) = self.dispatch_job(&mut state, job, "startup", current_internet, now_local, None, None) {
                    self.log(&format!("Job {} failed during startup trigger: {err:#}", job.job_id));
                }
            }
        }

        if let Some(trigger) = manual_trigger {
            for job in &self.config.jobs {
                if let Err(err) = self.dispatch_job(&mut state, job, trigger, current_internet, now_local, None, None) {
                    self.log(&format!("Job {} failed during trigger {}: {err:#}", job.job_id, trigger));
                }
            }
        }

        for job in &self.config.jobs {
            if let Err(err) = self.dispatch_job(&mut state, job, "periodic", current_internet, now_local, None, None) {
                self.log(&format!("Job {} failed during periodic trigger: {err:#}", job.job_id));
            }
        }

        if previous_internet == Some(false) && current_internet {
            if self.config.offline_queue.replay_trigger.eq_ignore_ascii_case("internet_up") {
                if let Err(err) = self.replay_offline_queue(&mut state, current_internet, now_local) {
                    self.log(&format!("Offline queue replay failed: {err:#}"));
                }
            }
            for job in &self.config.jobs {
                if let Err(err) =
                    self.dispatch_job(&mut state, job, "internet_up", current_internet, now_local, None, None)
                {
                    self.log(&format!("Job {} failed during internet_up trigger: {err:#}", job.job_id));
                }
            }
        }

        for request_file in self.iter_new_inbox_requests(&state)? {
            match self.build_inbox_job(&request_file) {
                Ok((inbox_job, role)) => {
                    if let Err(err) = self.dispatch_job(
                        &mut state,
                        &inbox_job,
                        "inbox_request",
                        current_internet,
                        now_local,
                        Some(&request_file),
                        role.as_ref(),
                    ) {
                        self.log(&format!(
                            "Inbox request {} failed: {err:#}",
                            request_file.display()
                        ));
                    }
                }
                Err(err) => {
                    self.log(&format!(
                        "Inbox request {} could not be loaded: {err:#}",
                        request_file.display()
                    ));
                    self.mark_request_processed(&mut state, &request_file, "failed_to_load");
                }
            }
        }

        save_state(&self.state_path, &state)?;
        Ok(())
    }

    pub fn daemon(&self) -> Result<()> {
        self.ensure_runtime()?;
        let mut daemon_lock = DaemonLock::new(&self.lock_path);
        daemon_lock.acquire()?;
        self.log("FounderAI daemon starting.");

        if let Err(err) = self.tick(true, None) {
            self.log(&format!("Startup tick failed: {err:#}"));
        }

        loop {
            thread::sleep(Duration::from_secs(self.config.poll_interval_seconds));
            if let Err(err) = self.tick(false, None) {
                self.log(&format!("Periodic tick failed: {err:#}"));
            }
        }
    }

    fn provider_status(&self) -> ProviderStatus {
        provider_status(&self.config.worker)
    }

    pub fn status_text(&self, show_teams: bool) -> Result<String> {
        self.ensure_runtime()?;
        let mut state = self.load_state_or_default();
        state.normalize();
        let pending = list_pending_approvals(&self.config.runtime_dir)?;
        let current_internet = internet_is_available(&self.config.internet_check);
        let provider_status = self.provider_status();

        let mut lines = vec![
            format!("Config: {}", self.config.config_path.display()),
            format!("Workspace: {}", self.config.workspace_root.display()),
            format!("Runtime: {}", self.config.runtime_dir.display()),
            format!("Inbox: {}", self.config.inbox_dir.display()),
            format!("Outbox: {}", self.config.outbox_dir.display()),
            format!("Agent roster: {}", self.config.agent_roster_path.display()),
            format!("Configured agents: {}", self.config.agent_profiles.len()),
            format!("Internet status: {}", if current_internet { "available" } else { "unavailable" }),
            format!("Pending approvals: {}", pending.len()),
            format!("Offline queue pending: {}", count_pending_entries(&self.config)?),
            format!("Model router enabled: {}", self.config.model_router.enabled),
            format!("Model router routes: {}", self.config.model_router.routes.len()),
            format!("Notifier enabled: {}", self.config.notifier.enabled),
            format!("Active provider: {}", self.config.worker.provider),
            format!("Active model: {}", self.config.worker.model),
            format!("Provider reachable: {}", provider_status.reachable),
        ];
        if let Some(model_available) = provider_status.model_available {
            lines.push(format!("Configured model installed: {}", model_available));
        }
        if let Some(detail) = provider_status.detail {
            lines.push(format!("Provider detail: {}", detail));
        }

        lines.push("Jobs:".to_string());
        for job in &self.config.jobs {
            let job_state = state.ensure_job_state(&job.job_id).clone();
            lines.push(format!(
                "- {}: last_run={} exit={} pending_approval={}",
                job.job_id,
                job_state
                    .last_run_id
                    .unwrap_or_else(|| "none".to_string()),
                job_state
                    .last_exit_code
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_string()),
                job_state
                    .pending_approval_id
                    .unwrap_or_else(|| "none".to_string())
            ));
        }

        if show_teams {
            lines.push("Teams:".to_string());
            for role in self.config.team_roles.values() {
                let role_state = state.ensure_role_state(&role.role_id).clone();
                let last_job_id = display_job_label(role_state.last_job_id.as_deref(), Some(role))
                    .unwrap_or_else(|| "unknown".to_string());
                lines.push(format!(
                    "- {}: last_job={} status={} metric={} pending_approval={}",
                    role.role_id,
                    last_job_id,
                    role_state
                        .last_status
                        .unwrap_or_else(|| "unknown".to_string()),
                    role_state
                        .last_metric_value
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "none".to_string()),
                    role_state
                        .pending_approval_id
                        .unwrap_or_else(|| "none".to_string())
                ));
            }
        }

        Ok(lines.join("\n"))
    }

    pub fn create_request_file(
        &self,
        title: &str,
        body: &str,
        approval_policy: &str,
        risk_tags: &[String],
        requires_internet: bool,
        role_id: Option<&str>,
    ) -> Result<PathBuf> {
        self.ensure_runtime()?;
        let file_path = self
            .config
            .inbox_dir
            .join(format!("{}.json", slugify(title)));
        let mut payload = serde_json::json!({
            "title": title,
            "body": body,
            "approval_policy": approval_policy,
            "risk_tags": risk_tags,
            "requires_internet": requires_internet,
        });
        if let Some(role_id) = role_id {
            if let Some(object) = payload.as_object_mut() {
                object.insert("role_id".to_string(), Value::String(role_id.to_string()));
            }
        }
        let payload_text = serde_json::to_string_pretty(&payload).context("failed to serialize inbox request")?;
        fs::write(&file_path, payload_text)
            .with_context(|| format!("failed to write inbox request {}", file_path.display()))?;
        Ok(file_path)
    }
}
