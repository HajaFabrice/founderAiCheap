use crate::config::AppConfig;
use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineConfig {
    #[serde(default)]
    pub deadline: Option<String>,
    #[serde(default)]
    pub target_submit: Option<String>,
    #[serde(default)]
    pub alert_days_before: Vec<i64>,
    #[serde(default)]
    pub assigned_to: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct DeadlineAlert {
    pub alert_key: String,
    pub deadline_id: String,
    pub assigned_to: String,
    pub title: String,
    pub body: String,
}

fn parse_date(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value.trim(), "%Y-%m-%d").ok()
}

pub fn load_deadlines(config: &AppConfig) -> Result<BTreeMap<String, DeadlineConfig>> {
    if !config.deadline_tracker_path.exists() {
        return Ok(BTreeMap::new());
    }
    let raw = fs::read_to_string(&config.deadline_tracker_path)
        .with_context(|| format!("failed to read {}", config.deadline_tracker_path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("failed to parse {}", config.deadline_tracker_path.display()))
}

pub fn collect_due_alerts(
    config: &AppConfig,
    processed_alerts: &BTreeMap<String, String>,
    now_local: DateTime<Local>,
) -> Result<Vec<DeadlineAlert>> {
    let deadlines = load_deadlines(config)?;
    let mut alerts = Vec::new();
    let today = now_local.date_naive();

    for (deadline_id, deadline) in deadlines {
        let target = match deadline
            .deadline
            .as_deref()
            .and_then(parse_date)
            .or_else(|| deadline.target_submit.as_deref().and_then(parse_date))
        {
            Some(date) => date,
            None => continue,
        };

        let days_until = (target - today).num_days();
        for alert_days in &deadline.alert_days_before {
            if days_until != *alert_days {
                continue;
            }
            let alert_key = format!("{deadline_id}::{target}::{alert_days}");
            if processed_alerts.contains_key(&alert_key) {
                continue;
            }

            let urgency = if *alert_days <= 1 {
                "HIGH"
            } else if *alert_days <= 3 {
                "MEDIUM"
            } else {
                "NORMAL"
            };
            let title = format!(
                "Pio deadline alert: {} due in {} day(s)",
                deadline_id.replace('_', " "),
                alert_days
            );
            let body = format!(
                "Deadline ID: {deadline_id}\nAssigned to: {}\nStatus: {}\nTarget date: {}\nDays remaining: {}\nUrgency: {}\n\nAction:\n- Review the deadline immediately.\n- Draft or update the required output.\n- Keep all external consequences behind approval.\n",
                if deadline.assigned_to.is_empty() {
                    "unassigned"
                } else {
                    &deadline.assigned_to
                },
                if deadline.status.is_empty() {
                    "unspecified"
                } else {
                    &deadline.status
                },
                target,
                days_until,
                urgency
            );

            alerts.push(DeadlineAlert {
                alert_key,
                deadline_id: deadline_id.clone(),
                assigned_to: deadline.assigned_to.clone(),
                title,
                body,
            });
        }
    }

    alerts.sort_by(|left, right| left.alert_key.cmp(&right.alert_key));
    Ok(alerts)
}
