use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostProbe {
    pub host: String,
    pub port: u16,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_timeout_seconds() -> f64 {
    2.0
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WorkerOverride {
    pub provider: Option<String>,
    pub base_url: Option<String>,
    pub model: Option<String>,
    pub timeout_seconds: Option<u64>,
    pub system_prompt: Option<String>,
    pub api_key_env: Option<String>,
}

impl WorkerOverride {
    pub fn apply_to(&self, base: &WorkerConfig) -> WorkerConfig {
        WorkerConfig {
            provider: self.provider.clone().unwrap_or_else(|| base.provider.clone()),
            base_url: self.base_url.clone().unwrap_or_else(|| base.base_url.clone()),
            model: self.model.clone().unwrap_or_else(|| base.model.clone()),
            timeout_seconds: self.timeout_seconds.unwrap_or(base.timeout_seconds),
            system_prompt: self
                .system_prompt
                .clone()
                .unwrap_or_else(|| base.system_prompt.clone()),
            api_key_env: self
                .api_key_env
                .clone()
                .unwrap_or_else(|| base.api_key_env.clone()),
        }
    }
}

fn default_validation_policy() -> String {
    "after_run".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

fn default_agents_path() -> String {
    "agents.json".to_string()
}

fn default_deadlines_path() -> String {
    "pio_deadlines.json".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentProfile {
    pub id: String,
    pub saint_name: String,
    #[serde(default)]
    pub canonical_role_id: Option<String>,
    #[serde(default)]
    pub prompt_file: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub job_scope: String,
    #[serde(default)]
    pub primary_model: String,
    #[serde(default)]
    pub fallback_model: String,
    #[serde(default)]
    pub escalation_rule: String,
    #[serde(default)]
    pub external_facing: bool,
    #[serde(default)]
    pub transparency_note: String,
    #[serde(default)]
    pub mentor_role: bool,
}

fn default_router_connectivity_check_url() -> String {
    "https://1.1.1.1".to_string()
}

fn default_model_router_timeout() -> u64 {
    5
}

fn default_preferred_mode() -> String {
    "offline".to_string()
}

fn default_fallback_mode() -> String {
    "online".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskTypeRouteConfig {
    #[serde(default = "default_preferred_mode")]
    pub prefer: String,
    #[serde(default = "default_fallback_mode")]
    pub fallback: String,
    #[serde(default)]
    pub online: Option<WorkerOverride>,
    #[serde(default)]
    pub offline: Option<WorkerOverride>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelRouteConfig {
    #[serde(default)]
    pub task_types: Vec<String>,
    #[serde(default)]
    pub job_ids: Vec<String>,
    #[serde(default)]
    pub role_ids: Vec<String>,
    #[serde(default = "default_preferred_mode")]
    pub preferred_mode: String,
    #[serde(default = "default_fallback_mode")]
    pub fallback_mode: String,
    #[serde(default)]
    pub online: Option<WorkerOverride>,
    #[serde(default)]
    pub offline: Option<WorkerOverride>,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelRouterConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_router_connectivity_check_url")]
    pub connectivity_check_url: String,
    #[serde(default = "default_model_router_timeout", alias = "connectivity_timeout_seconds")]
    pub timeout_seconds: u64,
    #[serde(default)]
    pub task_types: BTreeMap<String, TaskTypeRouteConfig>,
    #[serde(default)]
    pub routes: Vec<ModelRouteConfig>,
}

impl Default for ModelRouterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            connectivity_check_url: default_router_connectivity_check_url(),
            timeout_seconds: default_model_router_timeout(),
            task_types: BTreeMap::new(),
            routes: Vec::new(),
        }
    }
}

fn default_offline_queue_path() -> String {
    "offline_queue".to_string()
}

fn default_replay_trigger() -> String {
    "internet_up".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OfflineQueueConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_offline_queue_path", alias = "path")]
    pub relative_path: String,
    #[serde(default = "default_replay_trigger", alias = "replay_on_job")]
    pub replay_trigger: String,
}

impl Default for OfflineQueueConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            relative_path: default_offline_queue_path(),
            replay_trigger: default_replay_trigger(),
        }
    }
}

fn default_slack_webhook_env() -> String {
    "FOUNDERAI_SLACK_WEBHOOK_URL".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifierConfig {
    #[serde(default = "default_false")]
    pub enabled: bool,
    #[serde(default = "default_slack_webhook_env")]
    pub slack_webhook_env: String,
}

impl Default for NotifierConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            slack_webhook_env: default_slack_webhook_env(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRoleConfig {
    pub role_id: String,
    pub team: String,
    pub role: String,
    pub display_name: String,
    pub saint_name: String,
    pub agent_id: String,
    pub daily_quota: i64,
    pub metric_unit: String,
    pub focus: String,
    pub responsibilities: Vec<String>,
    pub default_risk_tags: Vec<String>,
    pub default_approval_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub task_type: Option<String>,
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub config_path: PathBuf,
    pub workspace_root: PathBuf,
    pub founder_brain_path: PathBuf,
    pub runtime_dir: PathBuf,
    pub inbox_dir: PathBuf,
    pub outbox_dir: PathBuf,
    pub agent_roster_path: PathBuf,
    pub deadline_tracker_path: PathBuf,
    pub poll_interval_seconds: u64,
    pub internet_check: InternetCheckConfig,
    pub worker: WorkerConfig,
    pub strategic_validation: StrategicValidationConfig,
    pub inbox_request_defaults: InboxDefaults,
    pub team_roles: BTreeMap<String, TeamRoleConfig>,
    pub agent_profiles: BTreeMap<String, AgentProfile>,
    pub model_router: ModelRouterConfig,
    pub offline_queue: OfflineQueueConfig,
    pub notifier: NotifierConfig,
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
    #[serde(default = "default_agents_path")]
    agents_path: String,
    #[serde(default = "default_deadlines_path")]
    deadlines_path: String,
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
    model_router: ModelRouterConfig,
    #[serde(default)]
    offline_queue: OfflineQueueConfig,
    #[serde(default)]
    notifier: NotifierConfig,
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
    saint_name: Option<String>,
    agent_id: Option<String>,
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
    task_type: Option<String>,
    agent_id: Option<String>,
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

pub fn apply_worker_env_overrides(mut worker: WorkerConfig) -> WorkerConfig {
    worker.provider = env_override_string("FOUNDERAI_PROVIDER", worker.provider);
    worker.base_url = env_override_string("FOUNDERAI_BASE_URL", worker.base_url);
    worker.model = env_override_string("FOUNDERAI_MODEL", worker.model);
    worker.system_prompt = env_override_string("FOUNDERAI_SYSTEM_PROMPT", worker.system_prompt);
    worker.api_key_env = env_override_string("FOUNDERAI_API_KEY_ENV", worker.api_key_env);
    worker.timeout_seconds = env_override_u64("FOUNDERAI_TIMEOUT_SECONDS", worker.timeout_seconds);
    worker
}

fn default_openai_override() -> WorkerOverride {
    WorkerOverride {
        provider: Some("openai".to_string()),
        base_url: Some("https://api.openai.com/v1".to_string()),
        model: Some("gpt-5-mini".to_string()),
        timeout_seconds: Some(300),
        system_prompt: None,
        api_key_env: Some("OPENAI_API_KEY".to_string()),
    }
}

fn default_ollama_override() -> WorkerOverride {
    WorkerOverride {
        provider: Some("ollama".to_string()),
        base_url: Some("http://localhost:11434".to_string()),
        model: Some("qwen2.5:7b-instruct".to_string()),
        timeout_seconds: Some(900),
        system_prompt: None,
        api_key_env: None,
    }
}

fn inject_default_routes(router: &mut ModelRouterConfig) {
    if !router.routes.is_empty() || !router.task_types.is_empty() {
        return;
    }

    router.task_types.insert(
        "draft".to_string(),
        TaskTypeRouteConfig {
            prefer: "offline".to_string(),
            fallback: "online".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "qa_check".to_string(),
        TaskTypeRouteConfig {
            prefer: "offline".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "briefing".to_string(),
        TaskTypeRouteConfig {
            prefer: "offline".to_string(),
            fallback: "online".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "final_review".to_string(),
        TaskTypeRouteConfig {
            prefer: "online".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "proposal".to_string(),
        TaskTypeRouteConfig {
            prefer: "online".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "phd_analysis".to_string(),
        TaskTypeRouteConfig {
            prefer: "online".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "grant".to_string(),
        TaskTypeRouteConfig {
            prefer: "online".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
    router.task_types.insert(
        "scheduler".to_string(),
        TaskTypeRouteConfig {
            prefer: "offline".to_string(),
            fallback: "offline".to_string(),
            online: Some(default_openai_override()),
            offline: Some(default_ollama_override()),
        },
    );
}

fn expand_task_type_routes(router: &mut ModelRouterConfig) {
    if !router.routes.is_empty() {
        return;
    }

    for (task_type, route) in &router.task_types {
        router.routes.push(ModelRouteConfig {
            task_types: vec![task_type.clone()],
            job_ids: Vec::new(),
            role_ids: Vec::new(),
            preferred_mode: route.prefer.clone(),
            fallback_mode: route.fallback.clone(),
            online: route.online.clone().or_else(|| Some(default_openai_override())),
            offline: route.offline.clone().or_else(|| Some(default_ollama_override())),
            notes: format!("Task-type route for {task_type}"),
        });
    }
}

fn load_agent_profiles(path: &Path) -> Result<BTreeMap<String, AgentProfile>> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read agent roster {}", path.display()))?;
    let profiles: Vec<AgentProfile> = serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse agent roster {}", path.display()))?;
    let mut by_id = BTreeMap::new();
    for profile in profiles {
        by_id.insert(profile.id.clone(), profile);
    }
    Ok(by_id)
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

    let agent_roster_path = resolve_path(&base_dir, &raw.agents_path);
    let deadline_tracker_path = resolve_path(&base_dir, &raw.deadlines_path);
    let agent_profiles = load_agent_profiles(&agent_roster_path)?;

    let mut team_roles = BTreeMap::new();
    for item in raw.team_roles {
        let saint_name = item
            .saint_name
            .unwrap_or_else(|| item.display_name.clone().unwrap_or_else(|| item.id.clone()));
        let agent_id = item
            .agent_id
            .unwrap_or_else(|| saint_name.to_ascii_lowercase().replace(' ', "-"));
        let role = TeamRoleConfig {
            role_id: item.id.clone(),
            team: item.team,
            role: item.role,
            display_name: item.display_name.unwrap_or_else(|| item.id.clone()),
            saint_name,
            agent_id,
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
            task_type: item.task_type,
            agent_id: item.agent_id,
        })
        .collect();

    let worker = apply_worker_env_overrides(raw.worker);

    let mut model_router = raw.model_router;
    inject_default_routes(&mut model_router);
    expand_task_type_routes(&mut model_router);

    Ok(AppConfig {
        config_path: absolute_path.clone(),
        workspace_root: resolve_path(&base_dir, &raw.workspace_root),
        founder_brain_path: resolve_path(&base_dir, &raw.founder_brain_path),
        runtime_dir: resolve_path(&base_dir, &raw.runtime_dir),
        inbox_dir: resolve_path(&base_dir, &raw.inbox_dir),
        outbox_dir: resolve_path(&base_dir, &raw.outbox_dir),
        agent_roster_path,
        deadline_tracker_path,
        poll_interval_seconds: raw.poll_interval_seconds,
        internet_check: raw.internet_check,
        worker,
        strategic_validation: raw.strategic_validation,
        inbox_request_defaults: raw.inbox_request_defaults,
        team_roles,
        agent_profiles,
        model_router,
        offline_queue: raw.offline_queue,
        notifier: raw.notifier,
        jobs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_dir() -> PathBuf {
        let mut dir = std::env::temp_dir();
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        dir.push(format!("founderai-config-test-{stamp}"));
        dir
    }

    #[test]
    fn load_config_reads_agent_roster_and_defaults() {
        let root = unique_test_dir();
        fs::create_dir_all(root.join("config")).unwrap();
        fs::create_dir_all(root.join("founder-brain")).unwrap();
        fs::create_dir_all(root.join("runtime")).unwrap();
        fs::create_dir_all(root.join("inbox")).unwrap();
        fs::create_dir_all(root.join("outbox")).unwrap();

        let agents = r#"
[
  {
    "id": "anthony",
    "saint_name": "Anthony",
    "canonical_role_id": "A-Outreach",
    "kind": "team-role",
    "job_scope": "Draft outreach safely."
  }
]
"#;
        fs::write(root.join("config").join("agents.json"), agents).unwrap();

        let config_text = r#"
{
  "workspace_root": "..",
  "founder_brain_path": "../founder-brain",
  "runtime_dir": "../runtime",
  "inbox_dir": "../inbox",
  "outbox_dir": "../outbox",
  "team_roles": [
    {
      "id": "A-Outreach",
      "team": "A",
      "role": "Outreach",
      "display_name": "Outreach Specialist A",
      "saint_name": "Anthony",
      "agent_id": "anthony",
      "daily_quota": 3,
      "metric_unit": "prospects",
      "focus": "Cash flow",
      "responsibilities": ["Draft outreach"],
      "default_approval_policy": "inherit"
    }
  ],
  "jobs": [
    {
      "id": "startup-focus-brief",
      "prompt": "Build the first brief",
      "task_type": "briefing",
      "agent_id": "hildegard"
    }
  ]
}
"#;
        let config_path = root.join("config").join("founderai.json");
        fs::write(&config_path, config_text).unwrap();

        let config = load_config(&config_path).unwrap();
        assert_eq!(config.agent_profiles.len(), 1);
        assert!(config.model_router.enabled);
        assert!(config.model_router.task_types.contains_key("draft"));
        assert_eq!(config.offline_queue.relative_path, "offline_queue");
        assert_eq!(config.jobs[0].task_type.as_deref(), Some("briefing"));
        assert_eq!(config.jobs[0].agent_id.as_deref(), Some("hildegard"));
        assert_eq!(
            config.team_roles.get("A-Outreach").unwrap().saint_name,
            "Anthony"
        );

        let _ = fs::remove_dir_all(root);
    }
}
