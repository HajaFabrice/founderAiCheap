# Continuous Improvement System

Updated: 2026-05-03

Purpose:
- turn real achievements, delivery outcomes, funnel data, and written feedback into explicit project improvements
- keep the learning loop inspectable instead of hiding it in memory or chat
- improve the project without weakening approval gates or founder voice

## Core Rule

The system should improve from:
- achievements
- operational data
- customer or collaborator feedback

It should not pretend to have learned from signals that were never captured.

## Primary Runtime Artifacts

- `runtime/improvement/achievement_log.json`
- `runtime/improvement/customer_feedback.json`
- `runtime/improvement/improvement_backlog.json`
- `runtime/improvement/latest_weekly_retrospective.md`

## Input Sources

- `runtime/runs/*/metadata.json`
- `runtime/marketing/independent_funnel.json`
- `runtime/marketing/latest_marketing_brief.md`
- `runtime/marketing/latest_funnel_review.md`
- `sales/oplurix-first-sale/delivery_log.csv`
- `sales/customer_feedback_log.csv`

## What Counts As A Useful Signal

### Achievements

- successful weekly reviews or strategist outputs
- successful production assets
- confirmed buyer delivery
- confirmed product download
- any completed milestone with inspectable evidence

### Data

- outreach counts by stage
- reply and interest rates
- stalled leads
- failed-run frequency
- delivery-loop gaps

### Feedback

- explicit buyer comments
- collaborator objections
- post-delivery friction
- repeated confusion in outreach or offers
- silence after a meaningful action, when that silence is logged honestly

## Owners

- `Hildegard`
  - keeps the strategist layer aligned with the latest retrospective
- `Francis`
  - produces the weekly retrospective
  - names red and yellow signals plainly
- `Juniper`
  - keeps CRM, delivery, and feedback records clean
- `Columban`
  - translates repeated lessons into small safe system improvements

## Hard Rules

- No claimed lesson without a traceable source.
- No autopilot system rewrite from one weak signal.
- Customer outcomes outweigh internal speculation.
- Feedback must become either:
  - a prompt change
  - a workflow change
  - a QA rule
  - a backlog item
  - or an explicit decision to hold steady
- If evidence is thin, say `insufficient_live_signal` or `feedback_not_yet_captured`.

## Weekly Loop

1. Sync achievements from run metadata and delivery logs.
2. Sync feedback from the manual feedback log and delivery confirmations.
3. Build a heuristic improvement backlog.
4. Run the weekly retrospective.
5. Use the retrospective to decide the next three bounded changes.
6. Let Columban implement only the smallest safe changes, with approvals preserved.

## Success Standard

The improvement system is working when:
- important outcomes are logged
- feedback is captured within the same week it happens
- the backlog stays short and evidence-based
- the next actions change when the evidence changes
- the same mistakes do not keep repeating unexamined
