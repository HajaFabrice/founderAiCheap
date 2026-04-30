use crate::config::AppConfig;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const INSUFFICIENT_LIVE_SIGNAL: &str = "insufficient_live_signal";

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelTrackingRecord {
    pub lead_id: Option<String>,
    pub ownership_classification: Option<String>,
    pub language: Option<String>,
    pub segment: Option<String>,
    pub offer_used: Option<String>,
    pub proof_asset_used: Option<String>,
    pub current_stage: Option<String>,
    pub last_touch_at: Option<String>,
    pub next_action_at: Option<String>,
    pub outcome: Option<String>,
    pub human_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct IndependentCrmPayload {
    #[serde(default)]
    leads: Vec<IndependentLead>,
}

#[derive(Debug, Deserialize)]
struct IndependentLead {
    organization_id: String,
    organization: String,
    #[serde(default)]
    country: Option<String>,
    #[serde(default)]
    ownership_classification: Option<String>,
    #[serde(default)]
    pipeline_stage: Option<String>,
    #[serde(default)]
    recommended_language: Option<String>,
    #[serde(default)]
    recommended_entry_offer: Option<String>,
    #[serde(default)]
    focus_area: Option<String>,
    #[serde(default)]
    priority_band: Option<String>,
    #[serde(default)]
    outreach_readiness: Option<String>,
    #[serde(default)]
    fit_notes: Option<String>,
    #[serde(default)]
    dataset_evidence_found: Option<bool>,
    #[serde(default)]
    recent_report_published: Option<bool>,
    #[serde(default)]
    contact_routes: Option<ContactRoutes>,
    #[serde(default)]
    verification: Option<VerificationRecord>,
    #[serde(default)]
    funnel_tracking: Option<FunnelTrackingRecord>,
}

#[derive(Debug, Deserialize)]
struct ContactRoutes {
    #[serde(default)]
    general_email_candidate: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VerificationRecord {
    #[serde(default)]
    human_verified: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct ReviewReadyShortlistPayload {
    #[serde(default)]
    shortlist: Vec<ReviewReadyShortlistEntry>,
}

#[derive(Debug, Deserialize)]
struct ReviewReadyShortlistEntry {
    organization_id: String,
    organization: String,
    #[serde(default)]
    country: Option<String>,
    #[serde(default)]
    recommended_language: Option<String>,
    #[serde(default)]
    focus_area: Option<String>,
    #[serde(default)]
    why_fit: Option<String>,
    #[serde(default)]
    recommended_entry_offer: Option<String>,
    #[serde(default)]
    next_human_review: Vec<String>,
}

#[derive(Debug, Serialize)]
struct FunnelLeadSnapshot {
    lead_id: String,
    organization: String,
    country: Option<String>,
    ownership_classification: String,
    language: String,
    segment: String,
    offer_used: Option<String>,
    suggested_offer: Option<String>,
    proof_asset_used: Option<String>,
    current_stage: String,
    last_touch_at: Option<String>,
    next_action_at: Option<String>,
    outcome: Option<String>,
    human_verified: bool,
    priority_band: String,
    outreach_readiness: String,
    has_email_candidate: bool,
    fit_notes: Option<String>,
}

#[derive(Debug, Serialize)]
struct LanguagePerformance {
    sent_count: usize,
    replied_count: usize,
    interest_count: usize,
    response_rate: Option<f64>,
}

#[derive(Debug, Serialize)]
struct ShortlistScorecardEntry {
    lead_id: String,
    organization: String,
    country: Option<String>,
    segment: String,
    recommended_language: String,
    recommended_offer: Option<String>,
    recommended_proof_asset: String,
    score: i64,
    readiness: String,
    rationale: Vec<String>,
    next_human_review: Vec<String>,
}

pub fn marketing_root(runtime_dir: &Path) -> PathBuf {
    runtime_dir.join("marketing")
}

pub fn marketing_briefs_dir(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("briefs")
}

pub fn funnel_reviews_dir(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("reviews")
}

pub fn funnel_snapshot_path(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("independent_funnel.json")
}

pub fn shortlist_scorecard_path(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("review_ready_shortlist_scorecard.json")
}

pub fn latest_marketing_brief_path(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("latest_marketing_brief.md")
}

pub fn latest_funnel_review_path(runtime_dir: &Path) -> PathBuf {
    marketing_root(runtime_dir).join("latest_funnel_review.md")
}

pub fn ensure_marketing_dirs(runtime_dir: &Path) -> Result<()> {
    for path in [
        marketing_root(runtime_dir),
        marketing_briefs_dir(runtime_dir),
        funnel_reviews_dir(runtime_dir),
    ] {
        fs::create_dir_all(&path)
            .with_context(|| format!("failed to create {}", path.display()))?;
    }
    Ok(())
}

fn crm_path(config: &AppConfig) -> PathBuf {
    config
        .workspace_root
        .join("documents")
        .join("99_Agent_Ready")
        .join("databases")
        .join("independent_crm.json")
}

fn shortlist_path(config: &AppConfig) -> PathBuf {
    config
        .workspace_root
        .join("documents")
        .join("99_Agent_Ready")
        .join("databases")
        .join("review_ready_outreach_shortlist.json")
}

fn parse_due_date(value: &str) -> Option<NaiveDate> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|date| date.date_naive())
        .or_else(|| NaiveDate::parse_from_str(value, "%Y-%m-%d").ok())
}

fn increment_count(map: &mut BTreeMap<String, usize>, key: String) {
    *map.entry(key).or_insert(0) += 1;
}

fn normalize_tracking(lead: &IndependentLead) -> FunnelTrackingRecord {
    let default_segment = lead
        .focus_area
        .clone()
        .unwrap_or_else(|| "general-biodiversity-data-support".to_string());
    let default_language = lead
        .recommended_language
        .clone()
        .unwrap_or_else(|| "english".to_string());
    let default_stage = lead
        .pipeline_stage
        .clone()
        .unwrap_or_else(|| "candidate".to_string());
    let default_owner = lead
        .ownership_classification
        .clone()
        .unwrap_or_else(|| "independent_candidate".to_string());
    let default_human_verified = lead
        .verification
        .as_ref()
        .and_then(|item| item.human_verified)
        .unwrap_or(false);

    let tracking = lead.funnel_tracking.clone().unwrap_or_default();
    FunnelTrackingRecord {
        lead_id: tracking
            .lead_id
            .or_else(|| Some(lead.organization_id.clone())),
        ownership_classification: tracking
            .ownership_classification
            .or_else(|| Some(default_owner)),
        language: tracking.language.or_else(|| Some(default_language)),
        segment: tracking.segment.or_else(|| Some(default_segment)),
        offer_used: tracking.offer_used,
        proof_asset_used: tracking.proof_asset_used,
        current_stage: tracking.current_stage.or_else(|| Some(default_stage)),
        last_touch_at: tracking.last_touch_at,
        next_action_at: tracking.next_action_at,
        outcome: tracking.outcome,
        human_verified: tracking
            .human_verified
            .or(Some(default_human_verified)),
    }
}

fn current_stage(tracking: &FunnelTrackingRecord) -> String {
    tracking
        .current_stage
        .clone()
        .unwrap_or_else(|| "candidate".to_string())
}

fn stage_implies_sent(stage: &str) -> bool {
    matches!(
        stage,
        "contacted"
            | "follow_up_due"
            | "reply_received"
            | "scope_clarification"
            | "meeting_booked"
            | "proposal_sent"
            | "won"
            | "lost"
            | "paused"
    )
}

fn stage_implies_reply(stage: &str) -> bool {
    matches!(
        stage,
        "reply_received"
            | "scope_clarification"
            | "meeting_booked"
            | "proposal_sent"
            | "won"
            | "lost"
            | "paused"
    )
}

fn stage_implies_interest(stage: &str) -> bool {
    matches!(stage, "scope_clarification" | "meeting_booked" | "proposal_sent" | "won")
}

fn stage_implies_proposal(stage: &str) -> bool {
    matches!(stage, "proposal_sent" | "won" | "lost")
}

fn build_funnel_snapshot(crm: &IndependentCrmPayload) -> serde_json::Value {
    let mut stage_counts = BTreeMap::new();
    let mut language_buckets: BTreeMap<String, (usize, usize, usize)> = BTreeMap::new();
    let mut segment_reply_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut proof_reply_counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut lead_snapshots = Vec::new();
    let mut stalled_leads = Vec::new();

    let mut ready_for_review = 0usize;
    let mut human_verified = 0usize;
    let mut sent_count = 0usize;
    let mut replied_count = 0usize;
    let mut interest_count = 0usize;
    let mut meeting_count = 0usize;
    let mut proposal_count = 0usize;
    let mut won_count = 0usize;
    let mut lost_count = 0usize;

    let today = Utc::now().date_naive();

    for lead in &crm.leads {
        let tracking = normalize_tracking(lead);
        let stage = current_stage(&tracking);
        let language = tracking
            .language
            .clone()
            .unwrap_or_else(|| "english".to_string());
        let segment = tracking
            .segment
            .clone()
            .unwrap_or_else(|| "general-biodiversity-data-support".to_string());
        let ownership = tracking
            .ownership_classification
            .clone()
            .unwrap_or_else(|| "independent_candidate".to_string());
        let human_verified_flag = tracking.human_verified.unwrap_or(false);

        increment_count(&mut stage_counts, stage.clone());
        if lead.outreach_readiness.as_deref() == Some("ready_for_human_review") {
            ready_for_review += 1;
        }
        if human_verified_flag {
            human_verified += 1;
        }

        let bucket = language_buckets.entry(language.clone()).or_insert((0, 0, 0));

        if stage_implies_sent(&stage) {
            sent_count += 1;
            bucket.0 += 1;
        }
        if stage_implies_reply(&stage) {
            replied_count += 1;
            bucket.1 += 1;
            increment_count(&mut segment_reply_counts, segment.clone());
            if let Some(proof_asset) = tracking.proof_asset_used.clone() {
                increment_count(&mut proof_reply_counts, proof_asset);
            }
        }
        if stage_implies_interest(&stage) {
            interest_count += 1;
            bucket.2 += 1;
        }
        if stage == "meeting_booked" || tracking.outcome.as_deref() == Some("meeting-booked") {
            meeting_count += 1;
        }
        if stage_implies_proposal(&stage) {
            proposal_count += 1;
        }
        if stage == "won" {
            won_count += 1;
        }
        if stage == "lost" {
            lost_count += 1;
        }

        let due_now = tracking
            .next_action_at
            .as_deref()
            .and_then(parse_due_date)
            .map(|date| date <= today)
            .unwrap_or(false);
        if stage == "follow_up_due" || due_now {
            stalled_leads.push(json!({
                "lead_id": tracking.lead_id.clone().unwrap_or_else(|| lead.organization_id.clone()),
                "organization": lead.organization,
                "current_stage": stage,
                "next_action_at": tracking.next_action_at,
                "reason": if stage == "follow_up_due" { "follow-up due" } else { "next action date reached" },
            }));
        }

        lead_snapshots.push(FunnelLeadSnapshot {
            lead_id: tracking
                .lead_id
                .clone()
                .unwrap_or_else(|| lead.organization_id.clone()),
            organization: lead.organization.clone(),
            country: lead.country.clone(),
            ownership_classification: ownership,
            language,
            segment,
            offer_used: tracking.offer_used.clone(),
            suggested_offer: lead.recommended_entry_offer.clone(),
            proof_asset_used: tracking.proof_asset_used.clone(),
            current_stage: stage,
            last_touch_at: tracking.last_touch_at.clone(),
            next_action_at: tracking.next_action_at.clone(),
            outcome: tracking.outcome.clone(),
            human_verified: human_verified_flag,
            priority_band: lead
                .priority_band
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            outreach_readiness: lead
                .outreach_readiness
                .clone()
                .unwrap_or_else(|| "research_more".to_string()),
            has_email_candidate: lead
                .contact_routes
                .as_ref()
                .and_then(|routes| routes.general_email_candidate.as_ref())
                .is_some(),
            fit_notes: lead.fit_notes.clone(),
        });
    }

    let response_rate = if sent_count > 0 {
        Some(replied_count as f64 / sent_count as f64)
    } else {
        None
    };
    let interest_rate = if sent_count > 0 {
        Some(interest_count as f64 / sent_count as f64)
    } else {
        None
    };
    let win_rate = if proposal_count > 0 {
        Some(won_count as f64 / proposal_count as f64)
    } else {
        None
    };

    let signal_status = if sent_count == 0 {
        INSUFFICIENT_LIVE_SIGNAL.to_string()
    } else {
        "live_signal_available".to_string()
    };

    let language_performance: BTreeMap<String, LanguagePerformance> = language_buckets
        .into_iter()
        .map(|(language, (sent, replied, interested))| {
            let rate = if sent > 0 {
                Some(replied as f64 / sent as f64)
            } else {
                None
            };
            (
                language,
                LanguagePerformance {
                    sent_count: sent,
                    replied_count: replied,
                    interest_count: interested,
                    response_rate: rate,
                },
            )
        })
        .collect();

    let best_segment = if replied_count == 0 {
        INSUFFICIENT_LIVE_SIGNAL.to_string()
    } else {
        segment_reply_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(segment, _)| segment)
            .unwrap_or_else(|| INSUFFICIENT_LIVE_SIGNAL.to_string())
    };

    let best_proof_asset = if proof_reply_counts.is_empty() {
        INSUFFICIENT_LIVE_SIGNAL.to_string()
    } else {
        proof_reply_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(asset, _)| asset)
            .unwrap_or_else(|| INSUFFICIENT_LIVE_SIGNAL.to_string())
    };

    json!({
        "generated_at": Utc::now().to_rfc3339(),
        "purpose": "Live funnel snapshot for the independent, non-Techni outreach engine.",
        "signal_status": signal_status,
        "funnel_schema": {
            "required_fields": [
                "lead_id",
                "ownership_classification",
                "language",
                "segment",
                "offer_used",
                "proof_asset_used",
                "current_stage",
                "last_touch_at",
                "next_action_at",
                "outcome",
                "human_verified"
            ]
        },
        "summary": {
            "total_leads": lead_snapshots.len(),
            "ready_for_human_review": ready_for_review,
            "human_verified": human_verified,
            "sent_count": sent_count,
            "replied_count": replied_count,
            "interested_count": interest_count,
            "meeting_count": meeting_count,
            "proposal_count": proposal_count,
            "won_count": won_count,
            "lost_count": lost_count,
            "response_rate": response_rate,
            "interest_rate": interest_rate,
            "win_rate": win_rate
        },
        "stage_counts": stage_counts,
        "language_performance": language_performance,
        "best_performing_segment": best_segment,
        "best_performing_proof_asset": best_proof_asset,
        "stalled_leads": stalled_leads,
        "notes": [
            "If sent_count is 0, the funnel does not contain enough live signal for strong performance claims.",
            "This snapshot supports strategist and analyst overlays, but does not bypass human verification or approval."
        ],
        "leads": lead_snapshots
    })
}

fn select_proof_asset(recommended_language: &str) -> String {
    if recommended_language.contains("french") {
        "biodiversity-dataset-cleaning-sample-fr".to_string()
    } else {
        "biodiversity-dataset-cleaning-sample-en".to_string()
    }
}

fn build_shortlist_scorecard(
    crm: &IndependentCrmPayload,
    shortlist: &ReviewReadyShortlistPayload,
) -> serde_json::Value {
    let mut by_id: BTreeMap<&str, &IndependentLead> = BTreeMap::new();
    for lead in &crm.leads {
        by_id.insert(lead.organization_id.as_str(), lead);
    }

    let entries = shortlist
        .shortlist
        .iter()
        .map(|item| {
            let crm_lead = by_id.get(item.organization_id.as_str()).copied();
            let mut score = 0i64;
            let mut rationale = Vec::new();
            let readiness = crm_lead
                .and_then(|lead| lead.outreach_readiness.clone())
                .unwrap_or_else(|| "research_more".to_string());
            let recommended_language = item
                .recommended_language
                .clone()
                .or_else(|| crm_lead.and_then(|lead| lead.recommended_language.clone()))
                .unwrap_or_else(|| "english".to_string());
            let segment = crm_lead
                .and_then(|lead| lead.focus_area.clone())
                .or_else(|| item.focus_area.clone())
                .unwrap_or_else(|| "general-biodiversity-data-support".to_string());

            if crm_lead
                .and_then(|lead| lead.ownership_classification.as_deref())
                == Some("independent_candidate")
            {
                score += 25;
                rationale.push("Independent candidate classification present.".to_string());
            }
            if readiness == "ready_for_human_review" {
                score += 20;
                rationale.push("Lead is already marked ready for human review.".to_string());
            } else if readiness == "secondary_queue" {
                score += 10;
                rationale.push("Lead is in the secondary queue and may still be usable.".to_string());
            }
            if crm_lead
                .and_then(|lead| lead.contact_routes.as_ref())
                .and_then(|routes| routes.general_email_candidate.as_ref())
                .is_some()
            {
                score += 15;
                rationale.push("An email candidate exists for follow-up research.".to_string());
            }
            if crm_lead.and_then(|lead| lead.dataset_evidence_found).unwrap_or(false) {
                score += 10;
                rationale.push("Dataset or reporting evidence was found.".to_string());
            }
            if crm_lead
                .and_then(|lead| lead.recent_report_published)
                .unwrap_or(false)
            {
                score += 10;
                rationale.push("A recent report appears to exist for personalization.".to_string());
            }
            match crm_lead
                .and_then(|lead| lead.priority_band.as_deref())
                .unwrap_or("unknown")
            {
                "high" => {
                    score += 10;
                    rationale.push("Priority band is high.".to_string());
                }
                "medium" => {
                    score += 5;
                    rationale.push("Priority band is medium.".to_string());
                }
                _ => {}
            }
            if !recommended_language.contains("needs_human_choice") {
                score += 5;
                rationale.push("Message language is already clear.".to_string());
            }
            if segment.to_ascii_lowercase().contains("biodiversity")
                || segment.to_ascii_lowercase().contains("conservation")
                || segment.to_ascii_lowercase().contains("research")
            {
                score += 5;
                rationale.push("Segment is tightly aligned with the current offer.".to_string());
            }

            ShortlistScorecardEntry {
                lead_id: item.organization_id.clone(),
                organization: item.organization.clone(),
                country: item.country.clone(),
                segment,
                recommended_language: recommended_language.clone(),
                recommended_offer: item
                    .recommended_entry_offer
                    .clone()
                    .or_else(|| crm_lead.and_then(|lead| lead.recommended_entry_offer.clone())),
                recommended_proof_asset: select_proof_asset(&recommended_language),
                score,
                readiness,
                rationale: if let Some(why_fit) = &item.why_fit {
                    let mut combined = vec![why_fit.clone()];
                    combined.extend(rationale);
                    combined
                } else {
                    rationale
                },
                next_human_review: item.next_human_review.clone(),
            }
        })
        .collect::<Vec<_>>();

    json!({
        "generated_at": Utc::now().to_rfc3339(),
        "purpose": "Heuristic scorecard for the review-ready independent outreach shortlist.",
        "scoring_model": {
            "max_score": 100,
            "notes": [
                "This score is a prioritization helper, not a verified truth signal.",
                "Human ownership review and contact verification remain mandatory before send."
            ]
        },
        "shortlist": entries
    })
}

pub fn sync_marketing_state(config: &AppConfig) -> Result<()> {
    ensure_marketing_dirs(&config.runtime_dir)?;

    let crm_path = crm_path(config);
    if !crm_path.exists() {
        return Ok(());
    }

    let crm_raw = fs::read_to_string(&crm_path)
        .with_context(|| format!("failed to read {}", crm_path.display()))?;
    let crm: IndependentCrmPayload = serde_json::from_str(&crm_raw)
        .with_context(|| format!("failed to parse {}", crm_path.display()))?;

    let snapshot = build_funnel_snapshot(&crm);
    let snapshot_text = serde_json::to_string_pretty(&snapshot).context("failed to serialize marketing funnel snapshot")?;
    fs::write(funnel_snapshot_path(&config.runtime_dir), snapshot_text)
        .context("failed to write marketing funnel snapshot")?;

    let shortlist_path = shortlist_path(config);
    if shortlist_path.exists() {
        let shortlist_raw = fs::read_to_string(&shortlist_path)
            .with_context(|| format!("failed to read {}", shortlist_path.display()))?;
        let shortlist: ReviewReadyShortlistPayload = serde_json::from_str(&shortlist_raw)
            .with_context(|| format!("failed to parse {}", shortlist_path.display()))?;
        let scorecard = build_shortlist_scorecard(&crm, &shortlist);
        let scorecard_text =
            serde_json::to_string_pretty(&scorecard).context("failed to serialize shortlist scorecard")?;
        fs::write(shortlist_scorecard_path(&config.runtime_dir), scorecard_text)
            .context("failed to write shortlist scorecard")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn funnel_snapshot_reports_insufficient_live_signal_when_nothing_was_sent() {
        let crm: IndependentCrmPayload = serde_json::from_value(json!({
            "leads": [
                {
                    "organization_id": "asity-madagascar",
                    "organization": "Asity Madagascar",
                    "country": "Madagascar",
                    "ownership_classification": "independent_candidate",
                    "pipeline_stage": "candidate",
                    "recommended_language": "english_or_french_needs_human_choice",
                    "focus_area": "Biodiversity / NGO",
                    "outreach_readiness": "ready_for_human_review",
                    "priority_band": "high",
                    "verification": { "human_verified": false },
                    "contact_routes": { "general_email_candidate": "info@example.org" },
                    "funnel_tracking": {
                        "lead_id": "asity-madagascar",
                        "ownership_classification": "independent_candidate",
                        "language": "english_or_french_needs_human_choice",
                        "segment": "Biodiversity / NGO",
                        "current_stage": "candidate",
                        "human_verified": false
                    }
                }
            ]
        }))
        .expect("crm payload should parse");

        let snapshot = build_funnel_snapshot(&crm);
        assert_eq!(
            snapshot
                .get("signal_status")
                .and_then(Value::as_str)
                .expect("signal_status"),
            INSUFFICIENT_LIVE_SIGNAL
        );
        assert_eq!(
            snapshot["summary"]["sent_count"].as_u64().expect("sent_count"),
            0
        );
        assert_eq!(
            snapshot["best_performing_segment"]
                .as_str()
                .expect("best_performing_segment"),
            INSUFFICIENT_LIVE_SIGNAL
        );
    }

    #[test]
    fn shortlist_scorecard_prefers_ready_independent_leads_with_contact_routes() {
        let crm: IndependentCrmPayload = serde_json::from_value(json!({
            "leads": [
                {
                    "organization_id": "asity-madagascar",
                    "organization": "Asity Madagascar",
                    "country": "Madagascar",
                    "ownership_classification": "independent_candidate",
                    "pipeline_stage": "candidate",
                    "recommended_language": "english",
                    "recommended_entry_offer": "Free 5-20 row sample review",
                    "focus_area": "Biodiversity / NGO",
                    "priority_band": "high",
                    "outreach_readiness": "ready_for_human_review",
                    "dataset_evidence_found": true,
                    "recent_report_published": true,
                    "contact_routes": { "general_email_candidate": "info@example.org" }
                }
            ]
        }))
        .expect("crm payload should parse");

        let shortlist: ReviewReadyShortlistPayload = serde_json::from_value(json!({
            "shortlist": [
                {
                    "organization_id": "asity-madagascar",
                    "organization": "Asity Madagascar",
                    "country": "Madagascar",
                    "recommended_language": "english",
                    "focus_area": "Biodiversity / NGO",
                    "why_fit": "Strong Madagascar conservation fit.",
                    "recommended_entry_offer": "Free 5-20 row sample review",
                    "next_human_review": ["verify contact route"]
                }
            ]
        }))
        .expect("shortlist payload should parse");

        let scorecard = build_shortlist_scorecard(&crm, &shortlist);
        let first_entry = scorecard["shortlist"]
            .as_array()
            .and_then(|entries| entries.first())
            .expect("first shortlist entry");
        assert_eq!(
            first_entry["recommended_proof_asset"]
                .as_str()
                .expect("recommended proof asset"),
            "biodiversity-dataset-cleaning-sample-en"
        );
        assert!(
            first_entry["score"].as_i64().expect("score") >= 70,
            "score should reward ready independent leads with evidence and a contact route"
        );
    }
}
