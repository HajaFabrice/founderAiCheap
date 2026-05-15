use crate::config::AppConfig;
use crate::marketing::funnel_snapshot_path;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};

const INSUFFICIENT_LIVE_SIGNAL: &str = "insufficient_live_signal";
const RUN_RETROSPECTIVE_WINDOW_DAYS: i64 = 14;
const RUN_ACTIVITY_WINDOW_DAYS: i64 = 7;

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
struct RunMetadata {
    #[serde(default)]
    run_id: String,
    #[serde(default)]
    job_id: String,
    #[serde(default)]
    started_at: Option<String>,
    #[serde(default)]
    finished_at: Option<String>,
    #[serde(default)]
    exit_code: Option<i32>,
    #[serde(default)]
    task_type: Option<String>,
    #[serde(default)]
    role_id: Option<String>,
    #[serde(default)]
    agent_id: Option<String>,
    #[serde(default)]
    provider: Option<String>,
    #[serde(default)]
    model: Option<String>,
    #[serde(default)]
    output_file: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
struct DeliveryLogRow {
    #[serde(default)]
    buyer_name: String,
    #[serde(default)]
    buyer_email: String,
    #[serde(default)]
    product: String,
    #[serde(default)]
    payment_confirmed_at: String,
    #[serde(default)]
    delivery_sent_at: String,
    #[serde(default)]
    delivery_link: String,
    #[serde(default)]
    buyer_confirmed_access: String,
    #[serde(default)]
    download_confirmed: String,
    #[serde(default)]
    follow_up_due_date: String,
    #[serde(default)]
    launch_segment: String,
    #[serde(default)]
    source_message: String,
    #[serde(default)]
    notes: String,
}

#[derive(Debug, Default, Deserialize)]
struct CustomerFeedbackRow {
    #[serde(default)]
    logged_at: String,
    #[serde(default)]
    source: String,
    #[serde(default)]
    contact_name: String,
    #[serde(default)]
    contact_email: String,
    #[serde(default)]
    product_or_workflow: String,
    #[serde(default)]
    feedback_stage: String,
    #[serde(default)]
    signal_type: String,
    #[serde(default)]
    sentiment: String,
    #[serde(default)]
    feedback_text: String,
    #[serde(default)]
    recommended_change: String,
    #[serde(default)]
    related_delivery_status: String,
    #[serde(default)]
    related_run_id: String,
    #[serde(default)]
    status: String,
}

#[derive(Debug, Default, Deserialize)]
struct FunnelSnapshot {
    #[serde(default)]
    signal_status: String,
    #[serde(default)]
    summary: FunnelSummary,
    #[serde(default)]
    stalled_leads: Vec<StalledLead>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
struct FunnelSummary {
    #[serde(default)]
    total_leads: usize,
    #[serde(default)]
    ready_for_human_review: usize,
    #[serde(default)]
    human_verified: usize,
    #[serde(default)]
    sent_count: usize,
    #[serde(default)]
    replied_count: usize,
    #[serde(default)]
    interested_count: usize,
    #[serde(default)]
    meeting_count: usize,
    #[serde(default)]
    proposal_count: usize,
    #[serde(default)]
    won_count: usize,
    #[serde(default)]
    lost_count: usize,
    #[serde(default)]
    response_rate: Option<f64>,
    #[serde(default)]
    interest_rate: Option<f64>,
    #[serde(default)]
    win_rate: Option<f64>,
}

#[allow(dead_code)]
#[derive(Debug, Default, Deserialize)]
struct StalledLead {
    #[serde(default)]
    lead_id: String,
    #[serde(default)]
    organization: String,
    #[serde(default)]
    current_stage: String,
    #[serde(default)]
    next_action_at: Option<String>,
    #[serde(default)]
    reason: String,
}

#[derive(Debug, Clone)]
struct RunSignal {
    run_id: String,
    job_id: String,
    finished_at: String,
    exit_code: i32,
    task_type: Option<String>,
    role_id: Option<String>,
    agent_id: Option<String>,
    provider: Option<String>,
    model: Option<String>,
    output_file: String,
}

#[derive(Debug, Default)]
struct CustomerFeedbackStats {
    total_entries: usize,
    manual_entries: usize,
    derived_delivery_entries: usize,
    positive_entries: usize,
    neutral_entries: usize,
    negative_entries: usize,
    paid_orders: usize,
    confirmed_access_count: usize,
    confirmed_download_count: usize,
    open_delivery_loops: usize,
}

pub fn improvement_root(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("improvement")
}

pub fn retrospectives_dir(runtime_dir: &Path) -> PathBuf {
    improvement_root(runtime_dir).join("retrospectives")
}

pub fn achievement_log_path(runtime_dir: &Path) -> PathBuf {
    improvement_root(runtime_dir).join("achievement_log.json")
}

pub fn customer_feedback_path(runtime_dir: &Path) -> PathBuf {
    improvement_root(runtime_dir).join("customer_feedback.json")
}

pub fn improvement_backlog_path(runtime_dir: &Path) -> PathBuf {
    improvement_root(runtime_dir).join("improvement_backlog.json")
}

pub fn latest_weekly_retrospective_path(runtime_dir: &Path) -> PathBuf {
    improvement_root(runtime_dir).join("latest_weekly_retrospective.md")
}

pub fn ensure_improvement_dirs(runtime_dir: &Path) -> Result<()> {
    for path in [
        improvement_root(runtime_dir),
        retrospectives_dir(runtime_dir),
    ] {
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create {}", path.display()))?;
    }
    Ok(())
}

fn delivery_log_path(config: &AppConfig) -> PathBuf {
    config
        .workspace_root
        .join("sales")
        .join("oplurix-first-sale")
        .join("delivery_log.csv")
}

fn customer_feedback_log_path(config: &AppConfig) -> PathBuf {
    config
        .workspace_root
        .join("sales")
        .join("customer_feedback_log.csv")
}

fn parse_boolish(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "yes" | "y" | "1" => Some(true),
        "false" | "no" | "n" | "0" => Some(false),
        _ => None,
    }
}

fn parse_rfc3339_utc(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|item| item.with_timezone(&Utc))
}

fn read_optional_csv<T: DeserializeOwned + Default>(path: &Path) -> Result<Vec<T>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut reader = ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(path)
        .with_context(|| format!("failed to open {}", path.display()))?;

    let mut rows = Vec::new();
    for record in reader.deserialize() {
        let row: T =
            record.with_context(|| format!("failed to parse CSV row in {}", path.display()))?;
        rows.push(row);
    }
    Ok(rows)
}

fn read_funnel_snapshot(config: &AppConfig) -> Result<FunnelSnapshot> {
    let path = funnel_snapshot_path(&config.runtime_dir);
    if !path.exists() {
        return Ok(FunnelSnapshot::default());
    }

    let raw =
        fs::read_to_string(&path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))
}

fn collect_run_signals(runtime_dir: &Path) -> Result<Vec<RunSignal>> {
    let runs_dir = runtime_dir.join("runs");
    if !runs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut runs = Vec::new();
    for entry in
        fs::read_dir(&runs_dir).with_context(|| format!("failed to list {}", runs_dir.display()))?
    {
        let path = entry?.path();
        if !path.is_dir() {
            continue;
        }

        let metadata_path = path.join("metadata.json");
        if !metadata_path.exists() {
            continue;
        }

        let raw = match fs::read_to_string(&metadata_path) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let metadata: RunMetadata = match serde_json::from_str(&raw) {
            Ok(value) => value,
            Err(_) => continue,
        };

        let run_id = if metadata.run_id.is_empty() {
            path.file_name()
                .and_then(|value| value.to_str())
                .unwrap_or("unknown-run")
                .to_string()
        } else {
            metadata.run_id.clone()
        };

        let finished_at = metadata
            .finished_at
            .clone()
            .or(metadata.started_at.clone())
            .unwrap_or_else(|| run_id.clone());

        runs.push(RunSignal {
            run_id,
            job_id: metadata.job_id,
            finished_at,
            exit_code: metadata.exit_code.unwrap_or(1),
            task_type: metadata.task_type,
            role_id: metadata.role_id,
            agent_id: metadata.agent_id,
            provider: metadata.provider,
            model: metadata.model,
            output_file: path.join("output.md").display().to_string(),
        });
    }

    runs.sort_by(|left, right| {
        right
            .finished_at
            .cmp(&left.finished_at)
            .then_with(|| right.run_id.cmp(&left.run_id))
    });
    Ok(runs)
}

fn human_job_label(job_id: &str) -> String {
    job_id
        .split_once("--")
        .map(|(left, _)| left)
        .unwrap_or(job_id)
        .replace('-', " ")
}

fn build_achievement_log_value(
    runs: &[RunSignal],
    delivery_rows: &[DeliveryLogRow],
    runtime_dir: &Path,
) -> Value {
    let now = Utc::now();
    let mut success_7d = 0usize;
    let mut success_30d = 0usize;
    let mut recent_successes = Vec::new();

    for run in runs.iter().filter(|run| run.exit_code == 0) {
        if let Some(finished_at) = parse_rfc3339_utc(&run.finished_at) {
            let age = now.signed_duration_since(finished_at);
            if age <= Duration::days(7) {
                success_7d += 1;
            }
            if age <= Duration::days(30) {
                success_30d += 1;
            }
        }

        if recent_successes.len() < 12 {
            recent_successes.push(json!({
                "achievement_type": "successful_run",
                "achieved_at": run.finished_at,
                "title": format!("Completed {}", human_job_label(&run.job_id)),
                "job_id": run.job_id,
                "task_type": run.task_type,
                "role_id": run.role_id,
                "agent_id": run.agent_id,
                "provider": run.provider,
                "model": run.model,
                "evidence_path": run.output_file,
            }));
        }
    }

    let paid_orders = delivery_rows
        .iter()
        .filter(|row| !row.payment_confirmed_at.trim().is_empty())
        .count();
    let confirmed_downloads = delivery_rows
        .iter()
        .filter(|row| parse_boolish(&row.download_confirmed) == Some(true))
        .count();
    let open_delivery_loops = delivery_rows
        .iter()
        .filter(|row| {
            !row.payment_confirmed_at.trim().is_empty()
                && parse_boolish(&row.download_confirmed) != Some(true)
        })
        .count();

    let delivery_achievements = delivery_rows
        .iter()
        .filter_map(|row| {
            let download_confirmed = parse_boolish(&row.download_confirmed) == Some(true);
            let access_confirmed = parse_boolish(&row.buyer_confirmed_access) == Some(true);

            if download_confirmed {
                Some(json!({
                    "achievement_type": "product_download_confirmed",
                    "achieved_at": if row.delivery_sent_at.trim().is_empty() {
                        "NEEDS_HUMAN_VERIFICATION".to_string()
                    } else {
                        row.delivery_sent_at.clone()
                    },
                    "title": format!("Confirmed download for {}", row.product),
                    "buyer_name": row.buyer_name,
                    "buyer_email": row.buyer_email,
                    "launch_segment": row.launch_segment,
                    "source_message": row.source_message,
                    "evidence_path": delivery_log_path_from_runtime(runtime_dir),
                    "notes": row.notes,
                }))
            } else if access_confirmed {
                Some(json!({
                    "achievement_type": "buyer_access_confirmed",
                    "achieved_at": if row.delivery_sent_at.trim().is_empty() {
                        "NEEDS_HUMAN_VERIFICATION".to_string()
                    } else {
                        row.delivery_sent_at.clone()
                    },
                    "title": format!("Confirmed delivery access for {}", row.product),
                    "buyer_name": row.buyer_name,
                    "buyer_email": row.buyer_email,
                    "launch_segment": row.launch_segment,
                    "source_message": row.source_message,
                    "evidence_path": delivery_log_path_from_runtime(runtime_dir),
                    "notes": row.notes,
                }))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let last_retrospective_at = runs
        .iter()
        .find(|run| run.job_id == "weekly-retrospective" && run.exit_code == 0)
        .map(|run| run.finished_at.clone());

    json!({
        "generated_at": now.to_rfc3339(),
        "purpose": "Normalized achievement log for project improvement, weekly retrospectives, and bounded backlog generation.",
        "sources": [
            format!("{}/runs/*/metadata.json", runtime_dir.display()),
            "sales/oplurix-first-sale/delivery_log.csv"
        ],
        "summary": {
            "successful_runs_total": runs.iter().filter(|run| run.exit_code == 0).count(),
            "successful_runs_last_7_days": success_7d,
            "successful_runs_last_30_days": success_30d,
            "paid_orders_logged": paid_orders,
            "confirmed_product_downloads": confirmed_downloads,
            "open_delivery_loops": open_delivery_loops,
            "last_weekly_retrospective_at": last_retrospective_at
        },
        "recent_achievements": recent_successes,
        "customer_delivery_achievements": delivery_achievements,
        "notes": [
            "Achievements can come from internal run completions or real buyer outcomes.",
            "A run completing successfully is not the same thing as a customer outcome; both are logged separately."
        ]
    })
}

fn delivery_log_path_from_runtime(runtime_dir: &Path) -> String {
    runtime_dir
        .join("..")
        .join("sales")
        .join("oplurix-first-sale")
        .join("delivery_log.csv")
        .display()
        .to_string()
}

fn infer_sentiment_from_text(text: &str) -> String {
    let lower = text.to_ascii_lowercase();
    if [
        "problem",
        "issue",
        "broken",
        "cannot",
        "can't",
        "could not",
        "didn't work",
        "did not work",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        "negative".to_string()
    } else if [
        "helpful",
        "useful",
        "worked",
        "great",
        "clear",
        "downloaded",
        "thanks",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        "positive".to_string()
    } else {
        "neutral".to_string()
    }
}

fn build_customer_feedback_value(
    feedback_rows: &[CustomerFeedbackRow],
    delivery_rows: &[DeliveryLogRow],
) -> (Value, CustomerFeedbackStats) {
    let mut stats = CustomerFeedbackStats::default();
    let mut entries = Vec::new();

    for row in feedback_rows {
        let sentiment = if row.sentiment.trim().is_empty() {
            infer_sentiment_from_text(&row.feedback_text)
        } else {
            row.sentiment.trim().to_ascii_lowercase()
        };

        stats.total_entries += 1;
        stats.manual_entries += 1;
        match sentiment.as_str() {
            "positive" => stats.positive_entries += 1,
            "negative" => stats.negative_entries += 1,
            _ => stats.neutral_entries += 1,
        }

        entries.push(json!({
            "logged_at": if row.logged_at.trim().is_empty() { "NEEDS_HUMAN_VERIFICATION".to_string() } else { row.logged_at.clone() },
            "source": if row.source.trim().is_empty() { "manual-feedback-log".to_string() } else { row.source.clone() },
            "contact_name": row.contact_name,
            "contact_email": row.contact_email,
            "product_or_workflow": row.product_or_workflow,
            "feedback_stage": row.feedback_stage,
            "signal_type": row.signal_type,
            "sentiment": sentiment,
            "feedback_text": row.feedback_text,
            "recommended_change": row.recommended_change,
            "related_delivery_status": row.related_delivery_status,
            "related_run_id": row.related_run_id,
            "status": if row.status.trim().is_empty() { "open".to_string() } else { row.status.clone() },
        }));
    }

    for row in delivery_rows {
        let payment_confirmed = !row.payment_confirmed_at.trim().is_empty();
        let access_confirmed = parse_boolish(&row.buyer_confirmed_access) == Some(true);
        let download_confirmed = parse_boolish(&row.download_confirmed) == Some(true);
        let has_notes = !row.notes.trim().is_empty();

        if payment_confirmed {
            stats.paid_orders += 1;
        }
        if access_confirmed {
            stats.confirmed_access_count += 1;
        }
        if download_confirmed {
            stats.confirmed_download_count += 1;
        }
        if payment_confirmed && !download_confirmed {
            stats.open_delivery_loops += 1;
        }

        if !(has_notes || access_confirmed || download_confirmed) {
            continue;
        }

        let feedback_text = if has_notes {
            row.notes.clone()
        } else if download_confirmed {
            "Buyer download confirmed.".to_string()
        } else if access_confirmed {
            "Buyer confirmed access to the delivery package.".to_string()
        } else {
            "Delivery interaction recorded.".to_string()
        };
        let sentiment = if download_confirmed {
            "positive".to_string()
        } else {
            infer_sentiment_from_text(&feedback_text)
        };

        stats.total_entries += 1;
        stats.derived_delivery_entries += 1;
        match sentiment.as_str() {
            "positive" => stats.positive_entries += 1,
            "negative" => stats.negative_entries += 1,
            _ => stats.neutral_entries += 1,
        }

        entries.push(json!({
            "logged_at": if row.delivery_sent_at.trim().is_empty() {
                "NEEDS_HUMAN_VERIFICATION".to_string()
            } else {
                row.delivery_sent_at.clone()
            },
            "source": "delivery_log",
            "contact_name": row.buyer_name,
            "contact_email": row.buyer_email,
            "product_or_workflow": row.product,
            "feedback_stage": if download_confirmed {
                "post-download"
            } else if access_confirmed {
                "post-delivery"
            } else {
                "delivery-follow-up"
            },
            "signal_type": if download_confirmed {
                "delivery-success"
            } else if access_confirmed {
                "delivery-access-confirmed"
            } else {
                "delivery-note"
            },
            "sentiment": sentiment,
            "feedback_text": feedback_text,
            "recommended_change": "",
            "related_delivery_status": if download_confirmed {
                "download_confirmed"
            } else if access_confirmed {
                "access_confirmed"
            } else {
                "follow_up_open"
            },
            "related_run_id": "",
            "status": "open",
        }));
    }

    entries.sort_by(|left, right| {
        right["logged_at"]
            .as_str()
            .unwrap_or_default()
            .cmp(left["logged_at"].as_str().unwrap_or_default())
    });

    (
        json!({
            "generated_at": Utc::now().to_rfc3339(),
            "purpose": "Normalized customer and operator feedback log for weekly retrospectives and product or workflow adjustments.",
            "sources": [
                "sales/customer_feedback_log.csv",
                "sales/oplurix-first-sale/delivery_log.csv"
            ],
            "summary": {
                "total_entries": stats.total_entries,
                "manual_entries": stats.manual_entries,
                "derived_delivery_entries": stats.derived_delivery_entries,
                "positive_entries": stats.positive_entries,
                "neutral_entries": stats.neutral_entries,
                "negative_entries": stats.negative_entries,
                "paid_orders": stats.paid_orders,
                "confirmed_access_count": stats.confirmed_access_count,
                "confirmed_download_count": stats.confirmed_download_count,
                "open_delivery_loops": stats.open_delivery_loops
            },
            "entries": entries,
            "notes": [
                "Feedback can come from explicit messages or from delivery confirmations and issues.",
                "Absence of feedback is itself a signal when a launch is live."
            ]
        }),
        stats,
    )
}

fn push_backlog_item(items: &mut Vec<Value>, input: BacklogItemInput<'_>) {
    items.push(json!({
        "backlog_id": input.backlog_id,
        "title": input.title,
        "priority": input.priority,
        "source_type": input.source_type,
        "source_refs": input.source_refs,
        "rationale": input.rationale,
        "recommended_owner": input.recommended_owner,
        "recommended_agent_id": input.recommended_agent_id,
        "target_area": input.target_area,
        "next_step": input.next_step,
        "status": "open"
    }));
}

struct BacklogItemInput<'a> {
    backlog_id: &'a str,
    title: &'a str,
    priority: &'a str,
    source_type: &'a str,
    source_refs: Vec<String>,
    rationale: &'a str,
    recommended_owner: &'a str,
    recommended_agent_id: &'a str,
    target_area: &'a str,
    next_step: &'a str,
}

fn build_improvement_backlog_value(
    config: &AppConfig,
    runs: &[RunSignal],
    funnel: &FunnelSnapshot,
    feedback_stats: &CustomerFeedbackStats,
) -> Value {
    let mut items = Vec::new();

    if funnel.signal_status == INSUFFICIENT_LIVE_SIGNAL || funnel.summary.sent_count == 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "generate-live-signal",
                title: "Generate the first real outreach signal before over-interpreting performance",
                priority: "high",
                source_type: "data-gap",
                source_refs: vec![funnel_snapshot_path(&config.runtime_dir).display().to_string()],
                rationale: "The funnel still lacks enough live outreach data to support strong conclusions or message tuning.",
                recommended_owner: "Founder plus Hildegard",
                recommended_agent_id: "hildegard",
                target_area: "go-to-market",
                next_step: "Human-verify 5-8 warm leads, send the first bounded outreach batch, and log every reply or non-reply honestly.",
            },
        );
    }

    if funnel.summary.ready_for_human_review > 0 && funnel.summary.sent_count == 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "verify-and-send-warm-list",
                title: "Move review-ready leads from candidate state into real human-reviewed outreach",
                priority: "high",
                source_type: "pipeline-gap",
                source_refs: vec![funnel_snapshot_path(&config.runtime_dir).display().to_string()],
                rationale: "There are leads ready for human review, but none have yet produced live signal.",
                recommended_owner: "Founder plus Anthony",
                recommended_agent_id: "anthony",
                target_area: "pipeline-activation",
                next_step: "Verify ownership and contact details for the top shortlist, then send the first warm messages instead of generating more draft-only work.",
            },
        );
    }

    if feedback_stats.paid_orders == 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "first-paid-download-proof",
                title: "Close the first paid product-download loop and record proof cleanly",
                priority: "high",
                source_type: "achievement-gap",
                source_refs: vec![delivery_log_path(config).display().to_string()],
                rationale: "The launch workspace exists, but the project still has no logged paid download outcome to learn from.",
                recommended_owner: "Founder plus OPLURIX launch workflow",
                recommended_agent_id: "juniper",
                target_area: "product-launch",
                next_step: "Push EcoR Complete through the first warm-buyer cycle, then log payment confirmation, delivery sent, access confirmation, and download confirmation in the delivery log.",
            },
        );
    }

    if feedback_stats.total_entries == 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "capture-real-feedback",
                title: "Capture explicit buyer or reviewer feedback instead of relying only on internal inference",
                priority: "high",
                source_type: "feedback-gap",
                source_refs: vec![customer_feedback_log_path(config).display().to_string()],
                rationale: "The system cannot improve from feedback that is never written down, especially after launch messages or product delivery.",
                recommended_owner: "Founder plus Juniper",
                recommended_agent_id: "juniper",
                target_area: "feedback-capture",
                next_step: "After each launch message, reply, or delivery, add one honest row to sales/customer_feedback_log.csv with the objection, reaction, or requested change.",
            },
        );
    }

    if feedback_stats.open_delivery_loops > 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "close-delivery-loop",
                title: "Close open delivery loops for paid or delivered buyers",
                priority: "high",
                source_type: "delivery-risk",
                source_refs: vec![delivery_log_path(config).display().to_string()],
                rationale: "A paid order without confirmed download is both a customer-risk and a missing learning loop.",
                recommended_owner: "Founder plus Juniper",
                recommended_agent_id: "juniper",
                target_area: "customer-success",
                next_step: "Check the delivery link, follow up manually, and record whether the buyer accessed and downloaded the package successfully.",
            },
        );
    }

    if !funnel.stalled_leads.is_empty() {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "unstall-active-leads",
                title: "Resolve stalled leads before generating new campaign complexity",
                priority: "medium",
                source_type: "pipeline-friction",
                source_refs: vec![funnel_snapshot_path(&config.runtime_dir).display().to_string()],
                rationale: "Stalled leads already contain more information than fresh cold prospects and should usually be cleared first.",
                recommended_owner: "Founder plus Perpetua",
                recommended_agent_id: "perpetua",
                target_area: "follow-up",
                next_step: "Review the stalled lead list, decide whether each lead gets a follow-up, a holding reply, or a stop condition, and log the outcome.",
            },
        );
    }

    let recent_failures = runs
        .iter()
        .filter(|run| run.exit_code != 0)
        .filter(|run| {
            parse_rfc3339_utc(&run.finished_at)
                .map(|finished_at| {
                    Utc::now().signed_duration_since(finished_at)
                        <= Duration::days(RUN_ACTIVITY_WINDOW_DAYS)
                })
                .unwrap_or(false)
        })
        .count();
    if recent_failures >= 3 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "stabilize-provider-path",
                title: "Stabilize provider reliability before trusting the automation rhythm",
                priority: "medium",
                source_type: "runtime-failure",
                source_refs: vec![config.runtime_dir.join("runs").display().to_string()],
                rationale: "Repeated failed runs in the last week mean the system is losing learning opportunities and creating noisy artifacts.",
                recommended_owner: "Founder plus Columban",
                recommended_agent_id: "columban",
                target_area: "runtime-reliability",
                next_step: "Inspect the failed run artifacts, choose the smallest reliability fix, and keep the change bounded so approvals and auditability stay intact.",
            },
        );
    }

    let retrospective_recent = runs.iter().any(|run| {
        run.job_id == "weekly-retrospective"
            && run.exit_code == 0
            && parse_rfc3339_utc(&run.finished_at)
                .map(|finished_at| {
                    Utc::now().signed_duration_since(finished_at)
                        <= Duration::days(RUN_RETROSPECTIVE_WINDOW_DAYS)
                })
                .unwrap_or(false)
    });
    if !retrospective_recent {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "protect-review-cadence",
                title: "Protect weekly retrospective cadence so lessons actually compound",
                priority: "medium",
                source_type: "process-drift",
                source_refs: vec![
                    latest_weekly_retrospective_path(&config.runtime_dir)
                        .display()
                        .to_string(),
                ],
                rationale: "Without a recent retrospective, the system can produce work but still fail to learn from it.",
                recommended_owner: "Founder plus Francis",
                recommended_agent_id: "francis",
                target_area: "governance",
                next_step: "Run the weekly retrospective, decide the top three improvements, and keep the backlog current instead of letting insight stay implicit.",
            },
        );
    }

    if feedback_stats.negative_entries > 0 {
        push_backlog_item(
            &mut items,
            BacklogItemInput {
                backlog_id: "act-on-negative-feedback",
                title: "Translate negative or friction-heavy feedback into one concrete change",
                priority: "medium",
                source_type: "feedback",
                source_refs: vec![customer_feedback_log_path(config).display().to_string()],
                rationale: "Negative feedback only creates value when it changes copy, delivery, packaging, or QA rules.",
                recommended_owner: "Founder plus Francis",
                recommended_agent_id: "francis",
                target_area: "offer-optimization",
                next_step: "Review the negative feedback entries, name the repeated friction plainly, and convert it into one bounded revision to the page, package, or follow-up flow.",
            },
        );
    }

    json!({
        "generated_at": Utc::now().to_rfc3339(),
        "purpose": "Heuristic improvement backlog generated from achievements, feedback, delivery outcomes, and funnel data.",
        "rules": [
            "This file is an internal prioritization helper, not autopilot authority.",
            "Backlog items should be reviewed in the weekly retrospective before larger system changes are made.",
            "Customer outcomes and verified feedback should outweigh internal speculation."
        ],
        "items": items
    })
}

pub fn sync_improvement_state(config: &AppConfig) -> Result<()> {
    ensure_improvement_dirs(&config.runtime_dir)?;

    let runs = collect_run_signals(&config.runtime_dir)?;
    let delivery_rows = read_optional_csv::<DeliveryLogRow>(&delivery_log_path(config))?;
    let feedback_rows =
        read_optional_csv::<CustomerFeedbackRow>(&customer_feedback_log_path(config))?;
    let funnel = read_funnel_snapshot(config)?;

    let achievement_value = build_achievement_log_value(&runs, &delivery_rows, &config.runtime_dir);
    fs::write(
        achievement_log_path(&config.runtime_dir),
        serde_json::to_string_pretty(&achievement_value)
            .context("failed to serialize achievement log")?,
    )
    .context("failed to write achievement log")?;

    let (feedback_value, feedback_stats) =
        build_customer_feedback_value(&feedback_rows, &delivery_rows);
    fs::write(
        customer_feedback_path(&config.runtime_dir),
        serde_json::to_string_pretty(&feedback_value)
            .context("failed to serialize customer feedback log")?,
    )
    .context("failed to write customer feedback log")?;

    let backlog_value = build_improvement_backlog_value(config, &runs, &funnel, &feedback_stats);
    fs::write(
        improvement_backlog_path(&config.runtime_dir),
        serde_json::to_string_pretty(&backlog_value)
            .context("failed to serialize improvement backlog")?,
    )
    .context("failed to write improvement backlog")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn customer_feedback_value_counts_manual_and_delivery_signals() {
        let feedback_rows = vec![CustomerFeedbackRow {
            logged_at: "2026-05-01T09:00:00Z".to_string(),
            source: "manual".to_string(),
            contact_name: "Buyer".to_string(),
            contact_email: "buyer@example.com".to_string(),
            product_or_workflow: "EcoR Toolkit Complete".to_string(),
            feedback_stage: "post-purchase".to_string(),
            signal_type: "objection".to_string(),
            sentiment: "negative".to_string(),
            feedback_text: "The page was helpful but the delivery instructions felt unclear."
                .to_string(),
            recommended_change: "Clarify the delivery window.".to_string(),
            related_delivery_status: "paid".to_string(),
            related_run_id: String::new(),
            status: "open".to_string(),
        }];
        let delivery_rows = vec![DeliveryLogRow {
            buyer_name: "Buyer".to_string(),
            buyer_email: "buyer@example.com".to_string(),
            product: "EcoR Toolkit Complete".to_string(),
            payment_confirmed_at: "2026-05-01T10:00:00Z".to_string(),
            delivery_sent_at: "2026-05-01T10:30:00Z".to_string(),
            delivery_link: "https://example.com".to_string(),
            buyer_confirmed_access: "true".to_string(),
            download_confirmed: "true".to_string(),
            follow_up_due_date: "2026-05-02".to_string(),
            launch_segment: "A".to_string(),
            source_message: "warm_launch".to_string(),
            notes: String::new(),
        }];

        let (value, stats) = build_customer_feedback_value(&feedback_rows, &delivery_rows);
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.manual_entries, 1);
        assert_eq!(stats.derived_delivery_entries, 1);
        assert_eq!(stats.negative_entries, 1);
        assert_eq!(stats.positive_entries, 1);
        assert_eq!(stats.confirmed_download_count, 1);
        assert_eq!(
            value["summary"]["confirmed_download_count"]
                .as_u64()
                .expect("confirmed download count"),
            1
        );
    }

    #[test]
    fn improvement_backlog_flags_live_signal_and_feedback_gaps() {
        let config = AppConfig {
            config_path: PathBuf::from("config/founderai.json"),
            workspace_root: PathBuf::from("."),
            founder_brain_path: PathBuf::from("founder-brain"),
            runtime_dir: PathBuf::from("runtime"),
            inbox_dir: PathBuf::from("inbox"),
            outbox_dir: PathBuf::from("outbox"),
            agent_roster_path: PathBuf::from("config/agents.json"),
            deadline_tracker_path: PathBuf::from("config/pio_deadlines.json"),
            poll_interval_seconds: 60,
            internet_check: Default::default(),
            worker: Default::default(),
            strategic_validation: Default::default(),
            inbox_request_defaults: Default::default(),
            team_roles: Default::default(),
            agent_profiles: Default::default(),
            model_router: Default::default(),
            offline_queue: Default::default(),
            notifier: Default::default(),
            jobs: Vec::new(),
        };
        let funnel = FunnelSnapshot {
            signal_status: INSUFFICIENT_LIVE_SIGNAL.to_string(),
            summary: FunnelSummary {
                ready_for_human_review: 3,
                sent_count: 0,
                ..Default::default()
            },
            stalled_leads: Vec::new(),
        };
        let backlog = build_improvement_backlog_value(
            &config,
            &[],
            &funnel,
            &CustomerFeedbackStats::default(),
        );

        let titles = backlog["items"]
            .as_array()
            .expect("backlog items")
            .iter()
            .filter_map(|item| item.get("backlog_id").and_then(Value::as_str))
            .collect::<Vec<_>>();
        assert!(titles.contains(&"generate-live-signal"));
        assert!(titles.contains(&"verify-and-send-warm-list"));
        assert!(titles.contains(&"first-paid-download-proof"));
        assert!(titles.contains(&"capture-real-feedback"));
    }
}
