# Continuous Improvement Templates

Updated: 2026-05-03

Use these templates to capture learning in a way the system can reuse.

## Template 1: Customer Feedback Log Row

```csv
logged_at,source,contact_name,contact_email,product_or_workflow,feedback_stage,signal_type,sentiment,feedback_text,recommended_change,related_delivery_status,related_run_id,status
2026-05-03T12:00:00Z,email,NEEDS_HUMAN_VERIFICATION,NEEDS_HUMAN_VERIFICATION,EcoR Toolkit Complete,post-download,usability-feedback,neutral,"The toolkit looks useful, but I was not sure which file to open first.","Clarify the Start Here note.",download_confirmed,,open
```

## Template 2: Weekly Retrospective

```text
# Weekly Retrospective

Week of:

## Achievement Signals
- [what was completed]
- [what outcome was confirmed]

## Data Signals
- sent_count:
- replied_count:
- response_rate:
- stalled_leads:
- failed_runs_last_7_days:

## Feedback Signals
- [buyer or collaborator feedback]
- [delivery friction]
- [what is still missing]

## What Worked
- [specific thing that should continue]

## What Did Not Work
- [specific thing that should change]

## What Changed Our Understanding
- [new lesson grounded in evidence]

## Backlog Priorities
1. [priority item]
2. [priority item]
3. [priority item]

## Next Three Bounded Changes
- [change 1]
- [change 2]
- [change 3]

## Founder Decision
- [the one decision that matters most this week]
```

## Template 3: Improvement Backlog Item

```json
{
  "backlog_id": "capture-real-feedback",
  "title": "Capture explicit buyer feedback after delivery",
  "priority": "high",
  "source_type": "feedback-gap",
  "source_refs": [
    "sales/customer_feedback_log.csv"
  ],
  "rationale": "The system cannot improve from feedback that is never written down.",
  "recommended_owner": "Founder plus Juniper",
  "recommended_agent_id": "juniper",
  "target_area": "feedback-capture",
  "next_step": "Add one honest row after each buyer interaction.",
  "status": "open"
}
```

## Template 4: Achievement Note

```text
Achievement:
Date:
Evidence path:
Why it matters:
What should continue:
What this does not prove yet:
```
