use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct HostProbe {
    pub host: String,
    pub port: u16,
}

fn default_true() -> bool {
    true
}

fn default_timeout_seconds() -> f64 {
    2.0
}

#[derive(Debug, Clone, Deserialize)]
pub struct InternetCheckConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub hosts: Vec<HostProbe>,
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: f64,
}

impl Default for InternetCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hosts: Vec::new(),
            timeout_seconds: 2.0,
        }
    }
}

fn default_worker_provider() -> String {
    "ollama".to_string()
}

fn default_worker_base_url() -> String {
    "http://localhost:11434".to_string()
}

fn default_worker_model() -> String {
    "qwen2.5:7b-instruct".to_string()
}

fn default_worker_timeout() -> u64 {
    300
}

fn default_worker_system_prompt() -> String {
    "You are FounderAI's autonomous provider worker. Follow the prompt packet exactly and write only the requested final deliverable.".to_string()
}

fn default_worker_api_key_env() -> String {
    "OPENAI_API_KEY".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkerConfig {
    #[serde(default = "default_worker_provider")]
    pub provider: String,
    #[serde(default = "default_worker_base_url")]
    pub base_url: String,
    #[serde(default = "default_worker_model")]
    pub model: String,
    #[serde(default = "default_worker_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_worker_system_prompt")]
    pub system_prompt: String,
    #[serde(default = "default_worker_api_key_env")]
    pub api_key_env: String,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            provider: default_worker_provider(),
            base_url: default_worker_base_url(),
            model: default_worker_model(),
            timeout_seconds: default_worker_timeout(),
            system_prompt: default_worker_system_prompt(),
            api_key_env: default_worker_api_key_env(),
        }
    }
}

fn default_validation_policy() -> String {
    "after_run".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrategicValidationConfig {
    #[serde(default)]
    pub always_require_tags: BTreeSet<String>,
    #[serde(default = "default_validation_policy")]
    pub default_policy: String,
}

impl Default for StrategicValidationConfig {
    fn default() -> Self {
        Self {
            always_require_tags: BTreeSet::new(),
            default_policy: default_validation_policy(),
        }
    }
}

fn default_approval_policy() -> String {
    "inherit".to_string()
}

#[derive(Debug, Clone, Deserialize)]
pub struct InboxDefaults {
    #[serde(default = "default_approval_policy")]
    pub approval_policy: String,
    #[serde(default)]
    pub requires_internet: bool,
    #[serde(default)]
    pub risk_tags: Vec<String>,
}

impl Default for InboxDefaults {
    fn default() -> Self {
        Self {
            approval_policy: default_approval_policy(),
            requires_internet: false,
            risk_tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TeamRoleConfig {
    pub role_id: String,
    pub team: String,
    pub role: String,
    pub display_name: String,
    pub daily_quota: i64,
    pub metric_unit: String,
    pub focus: String,
    pub responsibilities: Vec<String>,
    pub default_risk_tags: Vec<String>,
    pub default_approval_policy: String,
}

#[derive(Debug, Clone)]
pub struct JobConfig {
    pub job_id: String,
    pub description: String,
    pub enabled: bool,
    pub triggers: Vec<String>,
    pub prompt: String,
    pub interval_seconds: Option<u64>,
    pub cooldown_seconds: u64,
    pub requires_internet: bool,
    pub approval_policy: String,
    pub risk_tags: Vec<String>,
    pub mode: String,
    pub team_roles: Vec<String>,
    pub run_at_local: Option<String>,
    pub weekdays: Vec<String>,
    pub task_label: String,
    pub metric_value: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub config_path: PathBuf,
    pub workspace_root: PathBuf,
    pub founder_brain_path: PathBuf,
    pub runtime_dir: PathBuf,
    pub inbox_dir: PathBuf,
    pub outbox_dir: PathBuf,
    pub poll_interval_seconds: u64,
    pub internet_check: InternetCheckConfig,
    pub worker: WorkerConfig,
    pub strategic_validation: StrategicValidationConfig,
    pub inbox_request_defaults: InboxDefaults,
    pub team_roles: BTreeMap<String, TeamRoleConfig>,
    pub jobs: Vec<JobConfig>,
}

fn default_poll_interval() -> u64 {
    60
}

#[derive(Debug, Deserialize)]
struct RawAppConfig {
    workspace_root: String,
    founder_brain_path: String,
    runtime_dir: String,
    inbox_dir: String,
    outbox_dir: String,
    #[serde(default = "default_poll_interval")]
    poll_interval_seconds: u64,
    #[serde(default)]
    internet_check: InternetCheckConfig,
    #[serde(default)]
    worker: WorkerConfig,
    #[serde(default)]
    strategic_validation: StrategicValidationConfig,
    #[serde(default)]
    inbox_request_defaults: InboxDefaults,
    #[serde(default)]
    team_roles: Vec<RawTeamRoleConfig>,
    #[serde(default)]
    jobs: Vec<RawJobConfig>,
}

#[derive(Debug, Deserialize)]
struct RawTeamRoleConfig {
    id: String,
    team: String,
    role: String,
    display_name: Option<String>,
    #[serde(default)]
    daily_quota: i64,
    metric_unit: Option<String>,
    focus: Option<String>,
    #[serde(default)]
    responsibilities: Vec<String>,
    #[serde(default)]
    default_risk_tags: Vec<String>,
    #[serde(default = "default_approval_policy")]
    default_approval_policy: String,
}

#[derive(Debug, Deserialize)]
struct RawJobConfig {
    id: String,
    #[serde(default)]
    description: String,
    #[serde(default = "default_true")]
    enabled: bool,
    #[serde(default)]
    triggers: Vec<String>,
    #[serde(default)]
    prompt: String,
    interval_seconds: Option<u64>,
    #[serde(default)]
    cooldown_seconds: u64,
    #[serde(default)]
    requires_internet: bool,
    #[serde(default = "default_approval_policy")]
    approval_policy: String,
    #[serde(default)]
    risk_tags: Vec<String>,
    #[serde(default = "default_single_mode")]
    mode: String,
    #[serde(default)]
    team_roles: Vec<String>,
    run_at_local: Option<String>,
    #[serde(default)]
    weekdays: Vec<String>,
    task_label: Option<String>,
    metric_value: Option<i64>,
}

fn default_single_mode() -> String {
    "single".to_string()
}

fn resolve_path(base_dir: &Path, raw_value: &str) -> PathBuf {
    let candidate = PathBuf::from(raw_value);
    let resolved = if candidate.is_absolute() {
        candidate
    } else {
        base_dir.join(candidate)
    };
    let normalized = normalize_windows_path(resolved);
    if normalized.exists() {
        normalized
            .canonicalize()
            .map(normalize_windows_path)
            .unwrap_or(normalized)
    } else {
        normalized
    }
}

fn normalize_windows_path(path: PathBuf) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(stripped) = text.strip_prefix(r"\\?\") {
        PathBuf::from(stripped)
    } else {
        path
    }
}

fn env_override_string(name: &str, current: String) -> String {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or(current)
}

fn env_override_u64(name: &str, current: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(current)
}

pub fn load_config(config_path: impl AsRef<Path>) -> Result<AppConfig> {
    let path = config_path.as_ref().to_path_buf();
    let absolute_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()
            .context("failed to read current directory")?
            .join(path)
    };
    let absolute_path = normalize_windows_path(
        absolute_path
            .canonicalize()
            .with_context(|| format!("failed to resolve config path {}", absolute_path.display()))?,
    );
    let base_dir = absolute_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));
    let raw_text = fs::read_to_string(&absolute_path)
        .with_context(|| format!("failed to read config {}", absolute_path.display()))?;
    let raw: RawAppConfig = serde_json::from_str(&raw_text)
        .with_context(|| format!("failed to parse config {}", absolute_path.display()))?;

    let mut team_roles = BTreeMap::new();
    for item in raw.team_roles {
        let role = TeamRoleConfig {
            role_id: item.id.clone(),
            team: item.team,
            role: item.role,
            display_name: item.display_name.unwrap_or_else(|| item.id.clone()),
            daily_quota: item.daily_quota,
            metric_unit: item.metric_unit.unwrap_or_else(|| "tasks".to_string()),
            focus: item.focus.unwrap_or_default(),
            responsibilities: item.responsibilities,
            default_risk_tags: item.default_risk_tags,
            default_approval_policy: item.default_approval_policy,
        };
        team_roles.insert(role.role_id.clone(), role);
    }

    let jobs = raw
        .jobs
        .into_iter()
        .map(|item| JobConfig {
            job_id: item.id.clone(),
            description: item.description,
            enabled: item.enabled,
            triggers: item.triggers,
            prompt: item.prompt.trim().to_string(),
            interval_seconds: item.interval_seconds,
            cooldown_seconds: item.cooldown_seconds,
            requires_internet: item.requires_internet,
            approval_policy: item.approval_policy,
            risk_tags: item.risk_tags,
            mode: item.mode,
            team_roles: item.team_roles,
            run_at_local: item.run_at_local,
            weekdays: item.weekdays,
            task_label: item.task_label.unwrap_or(item.id),
            metric_value: item.metric_value,
        })
        .collect();

    let mut worker = raw.worker;
    worker.provider = env_override_string("FOUNDERAI_PROVIDER", worker.provider);
    worker.base_url = env_override_string("FOUNDERAI_BASE_URL", worker.base_url);
    worker.model = env_override_string("FOUNDERAI_MODEL", worker.model);
    worker.system_prompt = env_override_string("FOUNDERAI_SYSTEM_PROMPT", worker.system_prompt);
    worker.api_key_env = env_override_string("FOUNDERAI_API_KEY_ENV", worker.api_key_env);
    worker.timeout_seconds = env_override_u64("FOUNDERAI_TIMEOUT_SECONDS", worker.timeout_seconds);

    Ok(AppConfig {
        config_path: absolute_path.clone(),
        workspace_root: resolve_path(&base_dir, &raw.workspace_root),
        founder_brain_path: resolve_path(&base_dir, &raw.founder_brain_path),
        runtime_dir: resolve_path(&base_dir, &raw.runtime_dir),
        inbox_dir: resolve_path(&base_dir, &raw.inbox_dir),
        outbox_dir: resolve_path(&base_dir, &raw.outbox_dir),
        poll_interval_seconds: raw.poll_interval_seconds,
        internet_check: raw.internet_check,
        worker,
        strategic_validation: raw.strategic_validation,
        inbox_request_defaults: raw.inbox_request_defaults,
        team_roles,
        jobs,
    })
}
