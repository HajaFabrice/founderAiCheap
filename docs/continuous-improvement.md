# Continuous Improvement

FounderAI now has a dedicated improvement loop that turns project outcomes into
explicit next changes without weakening approvals or auditability.

## Runtime Artifacts

- `runtime/improvement/achievement_log.json`
- `runtime/improvement/customer_feedback.json`
- `runtime/improvement/improvement_backlog.json`
- `runtime/improvement/latest_weekly_retrospective.md`

## Source Inputs

- `runtime/runs/*/metadata.json`
- `runtime/marketing/independent_funnel.json`
- `sales/oplurix-first-sale/delivery_log.csv`
- `sales/customer_feedback_log.csv`

## Weekly Flow

1. FounderAI syncs recent achievements from run metadata and delivery outcomes.
2. FounderAI normalizes feedback from the customer feedback log and delivery notes.
3. FounderAI builds a heuristic improvement backlog.
4. `Francis` produces the weekly retrospective.
5. `Hildegard`, `Juniper`, and `Columban` can use the resulting artifacts to
   adjust campaigns, cleanup work, and safe system changes.

## Manual Capture Rule

The loop only improves from feedback that is written down. Use
`sales/customer_feedback_log.csv` after:

- launch messages
- buyer replies
- delivery questions
- post-download comments
- objections that changed the sale

## Guardrails

- approval gates stay unchanged
- no automatic external sending
- no automatic system rewrites from one weak signal
- customer outcomes outweigh internal speculation
