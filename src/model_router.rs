use crate::config::{AppConfig, JobConfig, ModelRouteConfig, TeamRoleConfig, WorkerConfig};

#[derive(Debug, Clone)]
pub struct RoutedWorker {
    pub primary: WorkerConfig,
    pub fallback: Option<WorkerConfig>,
    pub task_type: String,
    pub route_summary: String,
}

fn logical_job_id(job_id: &str) -> &str {
    job_id.split_once("--").map(|(left, _)| left).unwrap_or(job_id)
}

fn normalize(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn route_matches(
    route: &ModelRouteConfig,
    task_type: &str,
    job_id: &str,
    role_id: Option<&str>,
) -> bool {
    let task_ok = route.task_types.is_empty()
        || route
            .task_types
            .iter()
            .any(|value| normalize(value) == task_type);
    let job_ok = route.job_ids.is_empty()
        || route
            .job_ids
            .iter()
            .any(|value| normalize(value) == normalize(job_id));
    let role_ok = route.role_ids.is_empty()
        || role_id.map(|role_id| {
            route
                .role_ids
                .iter()
                .any(|value| normalize(value) == normalize(role_id))
        }) == Some(true);
    task_ok && job_ok && role_ok
}

fn worker_for_mode(
    config: &AppConfig,
    route: &ModelRouteConfig,
    mode: &str,
) -> WorkerConfig {
    match normalize(mode).as_str() {
        "online" => route
            .online
            .as_ref()
            .map(|override_config| override_config.apply_to(&config.worker))
            .unwrap_or_else(|| config.worker.clone()),
        "offline" => route
            .offline
            .as_ref()
            .map(|override_config| override_config.apply_to(&config.worker))
            .unwrap_or_else(|| config.worker.clone()),
        _ => config.worker.clone(),
    }
}

pub fn infer_task_type(job: &JobConfig, role: Option<&TeamRoleConfig>) -> String {
    if let Some(task_type) = &job.task_type {
        return normalize(task_type);
    }

    let job_id = normalize(logical_job_id(&job.job_id));
    let description = normalize(&job.description);
    let prompt = normalize(&job.prompt);

    if job_id.contains("brief")
        || job_id.contains("orchestration")
        || description.contains("brief")
        || prompt.contains("brief")
    {
        return "briefing".to_string();
    }

    if job_id.contains("review") || description.contains("review") {
        return "final_review".to_string();
    }

    if job_id.contains("grant")
        || description.contains("grant")
        || prompt.contains("grant")
        || prompt.contains("rufford")
        || prompt.contains("russell train")
    {
        return "grant".to_string();
    }

    if job_id.contains("deadline")
        || job_id.contains("scheduler")
        || description.contains("deadline")
        || prompt.contains("deadline")
        || prompt.contains("schedule")
    {
        return "scheduler".to_string();
    }

    if job_id.contains("phd")
        || prompt.contains("literature")
        || prompt.contains("manuscript")
        || prompt.contains("fieldwork")
    {
        return "phd_analysis".to_string();
    }

    if job_id.contains("proposal")
        || prompt.contains("proposal")
        || prompt.contains("methodology")
        || prompt.contains("whitepaper")
    {
        return "proposal".to_string();
    }

    if role
        .map(|role| role.role.eq_ignore_ascii_case("Outreach"))
        .unwrap_or(false)
        || job_id.contains("outreach")
    {
        return "draft".to_string();
    }

    if role
        .map(|role| role.role.eq_ignore_ascii_case("Production"))
        .unwrap_or(false)
    {
        return "proposal".to_string();
    }

    "draft".to_string()
}

pub fn resolve_worker(
    config: &AppConfig,
    job: &JobConfig,
    role: Option<&TeamRoleConfig>,
    current_internet: bool,
) -> RoutedWorker {
    let task_type = infer_task_type(job, role);
    if !config.model_router.enabled {
        return RoutedWorker {
            primary: config.worker.clone(),
            fallback: None,
            task_type,
            route_summary: "Model router disabled; using base worker.".to_string(),
        };
    }

    let role_id = role.map(|value| value.role_id.as_str());
    let job_id = logical_job_id(&job.job_id);

    let Some(route) = config
        .model_router
        .routes
        .iter()
        .find(|route| route_matches(route, &task_type, job_id, role_id))
    else {
        return RoutedWorker {
            primary: config.worker.clone(),
            fallback: None,
            task_type,
            route_summary: "No matching model route; using base worker.".to_string(),
        };
    };

    let preferred_mode = normalize(&route.preferred_mode);
    let fallback_mode = normalize(&route.fallback_mode);
    let should_flip = preferred_mode == "online" && !current_internet && fallback_mode != "none";
    let primary_mode = if should_flip {
        fallback_mode.as_str()
    } else {
        preferred_mode.as_str()
    };
    let primary = worker_for_mode(config, route, primary_mode);

    let secondary = if fallback_mode == "none" {
        None
    } else if should_flip {
        None
    } else if fallback_mode == preferred_mode {
        None
    } else {
        Some(worker_for_mode(config, route, &fallback_mode))
    };

    let route_summary = if should_flip {
        format!(
            "Task type '{task_type}' matched router notes '{}'; internet was unavailable so '{}' mode was promoted.",
            route.notes,
            fallback_mode
        )
    } else {
        format!(
            "Task type '{task_type}' matched router notes '{}'; preferred '{}' mode selected.",
            route.notes,
            preferred_mode
        )
    };

    RoutedWorker {
        primary,
        fallback: secondary,
        task_type,
        route_summary,
    }
}
