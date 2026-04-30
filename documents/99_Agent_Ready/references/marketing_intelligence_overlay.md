# Marketing Intelligence Overlay

Updated: 2026-04-29

Purpose:
- keep the saint roster lean while adding campaign direction and funnel learning
- improve independent, non-Techni client acquisition without changing approval gates

## Overlay Functions

### 1. Marketing Strategist

Default owner: `Hildegard`

Responsibilities:
- choose the target segment for the week
- choose the proof asset to lead with
- choose EN vs FR emphasis
- decide first-touch, follow-up, and CTA discipline
- produce a bounded weekly campaign brief for `Anthony`, `Bonaventure`, `Zacchaeus`, and `Perpetua`

Required inputs:
- `documents/99_Agent_Ready/databases/independent_crm.json`
- `documents/99_Agent_Ready/databases/review_ready_outreach_shortlist.json`
- `runtime/marketing/independent_funnel.json`
- `runtime/marketing/review_ready_shortlist_scorecard.json`
- the latest proof assets and service catalog

### 2. Growth/Data Analyst

Primary owners:
- `Juniper` for structured CRM and funnel cleanup
- `Francis` for the weekly funnel review
- `Duns Scotus` for evidence interpretation when live signals are ambiguous

Responsibilities:
- score the review-ready shortlist
- track outreach counts by stage
- compare EN vs FR response patterns
- compare offer and proof-asset performance
- identify weak segments, weak CTAs, and stalled leads

Required inputs:
- `documents/99_Agent_Ready/databases/independent_crm.json`
- `runtime/marketing/independent_funnel.json`
- `runtime/marketing/review_ready_shortlist_scorecard.json`
- the latest weekly marketing brief

## Hard Boundaries

- No new permanent copywriter agent is needed right now.
- Copywriting stays with `Anthony`, `Bonaventure`, `Zacchaeus`, `Perpetua`, and `Bernardine`.
- Strategist and analyst outputs are internal-only artifacts.
- Employer-owned or ambiguous leads must never be pushed into independent outreach by overlay logic.
- The overlay may prioritize unverified leads for review, but never treat them as send-ready.

## Decision Rules

- If `sent_count` is `0`, write `insufficient_live_signal` instead of claiming performance winners.
- If EN outreach underperforms FR for Madagascar-facing leads, flag it explicitly.
- If one proof asset keeps appearing with weak response, recommend another asset.
- If the same weak campaign pattern repeats for two weekly cycles, change the segment, proof, CTA, or language.
- If ownership is unclear, stop and route to human clarification.

## File Artifacts

- `runtime/marketing/latest_marketing_brief.md`
- `runtime/marketing/latest_funnel_review.md`
- `runtime/marketing/independent_funnel.json`
- `runtime/marketing/review_ready_shortlist_scorecard.json`

These files are inspectable, plain-text or JSON, and should remain easy to audit.
