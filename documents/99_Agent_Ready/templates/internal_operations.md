# Internal Operations Templates

Updated: 2026-04-22

Source basis:
- `ERIS_AgentSystem_CodexInput_v5.0.txt`
- `ERIS_Communication_Templates_v1.0.txt`
- `01_MasterPlan_v4.0.txt`

## Template 1: Grant Task JSON

```json
{
  "request_type": "grant-draft",
  "grant_name": "NEEDS_HUMAN_VERIFICATION",
  "funder": "NEEDS_HUMAN_VERIFICATION",
  "deadline": "YYYY-MM-DD",
  "value": "NEEDS_HUMAN_VERIFICATION",
  "institutional_host": "Techni-Drones Madagascar",
  "equipment_target": [
    "DJI Matrice 350 RTK",
    "Zenmuse H30T",
    "MicaSense Altum-PT"
  ],
  "required_outputs": [
    "peer-reviewed methodology paper",
    "replicable survey protocol",
    "pilot ERIS biodiversity score"
  ],
  "notes": "Add factual links, attachments needed, and any known partner context here."
}
```

## Template 2: Bartholomew Grant Fact Block

Use this before drafting a narrative.

```text
Grant:
Deadline:
Funder:
Amount ceiling:
Eligibility:
Institutional host:
Community forest / field site:
Existing proof of concept:
- IPS Congress 2025 thermal drone survey
- [other verified proof point]

Equipment ask:
- DJI Matrice 350 RTK
- Zenmuse H30T
- MicaSense Altum-PT

Attachments needed:
- CV
- support letter
- institutional letter
- budget table

Facts needing verification:
- NEEDS_HUMAN_VERIFICATION
```

## Template 3: Pio Deadline Entry

```json
{
  "deadline_name": {
    "deadline": "YYYY-MM-DD",
    "alert_days_before": [14, 7, 3, 1],
    "assigned_to": "agent-id",
    "status": "pending",
    "notes": "Why this deadline matters."
  }
}
```

## Template 4: Hildegard Morning Briefing

```text
# Morning Briefing

Date:

## Immediate Priorities
- [Top 1]
- [Top 2]
- [Top 3]

## Pending Approvals
- [approval id]: [one-line reason]

## Deadlines Under Pressure
- [deadline]: [days remaining]

## New Inbox Signals
- [request id]: [summary]

## Risks Or Drift
- [mission / governance / execution issue]

## Recommended Founder Focus Today
- [one concrete focus block]
```

## Template 5: Francis Weekly Review Agenda

```text
# Weekly Review Agenda

1. Review submitted work, approvals, and blocked items.
2. Review grant progress, outreach progress, PhD progress, and launch outcomes.
3. Review KPI colors: green, yellow, red.
4. Review achievements, buyer or collaborator feedback, and missing evidence.
5. Review risks, mission drift, and missed deadlines.
6. Decide next three priorities for the coming week.
7. Record any system changes for Columban.
```

## Template 6: Columban Change Instruction JSON

```json
{
  "request_type": "system-change",
  "requested_by": "founder",
  "reason": "NEEDS_HUMAN_VERIFICATION",
  "target_area": "prompt|config|workflow|docs",
  "change_request": "Describe the smallest safe change.",
  "must_preserve": [
    "approval gates",
    "inspectable artifacts",
    "founder voice fidelity",
    "team routing"
  ],
  "notes": "Ambiguous changes should escalate instead of guessing."
}
```

## Template 7: Clare Review Checklist

```json
{
  "qa_pass": false,
  "flags": [
    "factual claim needs verification",
    "approval-sensitive action detected",
    "missing transparent AI signature",
    "tone drift or hype language"
  ],
  "recommended_action": "revise|approve_for_review|escalate"
}
```
