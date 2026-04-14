use crate::config::NotifierConfig;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Serialize;
use std::env;
use std::time::Duration;

#[derive(Serialize)]
struct SlackPayload<'a> {
    text: &'a str,
}

pub fn send_notification(config: &NotifierConfig, title: &str, body: &str) -> Result<bool> {
    if !config.enabled {
        return Ok(false);
    }

    let webhook_url = match env::var(&config.slack_webhook_env) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return Ok(false),
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .context("failed to build notifier HTTP client")?;
    let payload = SlackPayload {
        text: &format!("*{title}*\n{body}"),
    };
    let response = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .context("failed to send Slack notification")?;

    if !response.status().is_success() {
        anyhow::bail!("notifier returned HTTP {}", response.status());
    }

    Ok(true)
}
