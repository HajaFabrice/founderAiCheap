use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAttempt {
    pub span_id: String,
    pub provider: String,
    pub base_url: String,
    pub model: String,
    pub started_at: String,
    pub finished_at: String,
    pub latency_ms: u128,
    pub status: String,
    pub error_class: Option<String>,
    pub prompt_hash: String,
    pub usage: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunTrace {
    pub trace_id: String,
    pub run_id: String,
    pub job_id: String,
    pub task_type: String,
    pub agent_id: Option<String>,
    pub provider_attempts: Vec<ProviderAttempt>,
}

impl RunTrace {
    pub fn new(
        run_id: String,
        job_id: String,
        task_type: String,
        agent_id: Option<String>,
    ) -> Self {
        Self {
            trace_id: run_id.clone(),
            run_id,
            job_id,
            task_type,
            agent_id,
            provider_attempts: Vec::new(),
        }
    }
}

pub fn prompt_hash(prompt: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in prompt.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

pub fn classify_provider_error(error: &str) -> String {
    let normalized = error.to_ascii_lowercase();
    if normalized.contains("credit balance is too low")
        || normalized.contains("billing")
        || normalized.contains("quota")
        || normalized.contains("insufficient_quota")
    {
        return "billing_low_credit".to_string();
    }
    if normalized.contains("timed out")
        || normalized.contains("operation timed out")
        || normalized.contains("deadline has elapsed")
    {
        return "offline_timeout".to_string();
    }
    if normalized.contains("api key")
        || normalized.contains("unauthorized")
        || normalized.contains("forbidden")
        || normalized.contains("401")
        || normalized.contains("403")
    {
        return "auth_error".to_string();
    }
    if normalized.contains("500")
        || normalized.contains("502")
        || normalized.contains("503")
        || normalized.contains("504")
        || normalized.contains("server error")
    {
        return "server_error".to_string();
    }
    if normalized.contains("failed to parse") || normalized.contains("invalid") {
        return "parse_error".to_string();
    }
    "unknown".to_string()
}

#[cfg(test)]
mod tests {
    use super::{classify_provider_error, prompt_hash};

    #[test]
    fn classifies_known_provider_failures() {
        assert_eq!(
            classify_provider_error("Your credit balance is too low to access the Anthropic API"),
            "billing_low_credit"
        );
        assert_eq!(
            classify_provider_error("failed to reach Ollama: operation timed out"),
            "offline_timeout"
        );
        assert_eq!(
            classify_provider_error("OpenAI returned HTTP 401 Unauthorized"),
            "auth_error"
        );
        assert_eq!(
            classify_provider_error("Claude API returned HTTP 503 Service Unavailable"),
            "server_error"
        );
    }

    #[test]
    fn prompt_hash_is_stable() {
        assert_eq!(prompt_hash("same prompt"), prompt_hash("same prompt"));
        assert_ne!(prompt_hash("same prompt"), prompt_hash("other prompt"));
    }
}
