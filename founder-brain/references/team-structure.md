# Team Structure

## Overview

FounderAI now simulates 3 parallel teams with 6 total roles. Use this structure to route work, preserve focus, and keep survival-first priorities intact.

## Team A - Data Specialist Freelance

Purpose: cash-flow engine.

### Outreach Specialist A

- Daily quota: 5 prospects.
- Focus: NGOs or field programs needing data cleaning, validation, or reporting support.
- Standard outputs: prospect notes, qualification summary, first-touch email, two follow-ups, CRM log entry.
- Handoff rule: when a lead replies or asks for scope, notify Production Specialist A.

### Production Specialist A

- Daily quota: 1 client dataset, delivery packet, or portfolio asset.
- Focus: data cleaning, validation, summary sheets, demo datasets, reusable templates, and delivery SOPs.
- Standard outputs: execution plan, cleaned dataset draft, QC notes, summary sheet, portfolio sample.
- Fallback when no client work exists: build templates, sample datasets, and delivery assets that improve conversion.

## Team B - MRV & Biodiversity Consulting

Purpose: growth engine.

### Outreach Specialist B

- Daily quota: 3 prospects.
- Focus: REDD+, donors, NGOs, institutional partners, and biodiversity programs needing MRV or ecological verification support.
- Standard outputs: partner qualification notes, proposal-style outreach, two follow-ups, CRM log entry.
- Handoff rule: when interest appears, notify Production Specialist B with the opportunity context.

### Production Specialist B

- Daily quota: 1 consulting asset, methodology draft, or proposal packet.
- Focus: methodologies, budgets, whitepapers, sample deliverables, SOPs, MRV pipeline assets, and consulting collateral.
- Standard outputs: methodology section, budget draft, sample deliverable outline, SOP, pipeline notes, case-study asset.

## Team C - MRV PhD Project

Purpose: leverage and research engine.

### Outreach Specialist C

- Daily quota: 2 prospects.
- Focus: supervisors, grants, institutional partners, and research-aligned opportunities.
- Standard outputs: supervisor or grant contact note, draft email, follow-ups, tracker log.

### Production Specialist C

- Daily quota: 5 papers or 1 bounded research asset.
- Focus: literature review, proposal sections, grant materials, budgets, fieldwork schedules, manuscript outlines, and analysis methods.
- Standard outputs: reading notes, literature matrix updates, review-type recommendation, proposal sections, manuscript structure, grant materials.
- Literature review rule: read 5 papers per day when in literature mode and record methodology, findings, and relevance.

## Shared Cadence

- Monday: team sync plus KPI review.
- Sunday: strategy review and adjustment.
- Monthly: 5-year roadmap checkpoint against survival, pilots, and ERIS institutional progress.

## Routing Rules

1. Default to Team A when the work can improve cash flow in the next 30 days.
2. Route MRV consulting, partnership, and methodology work to Team B.
3. Route PhD, literature, supervisor, and grant preparation work to Team C.
4. When a task spans teams, split it into bounded packets with explicit ownership.
5. Keep outreach and production separate unless a single packet is truly smaller and safer.

## Logging Standard

Every meaningful run should be loggable in this schema:

```csv
Date,Team,Role,Task,Status,Notes,MetricValue
2026-04-02,A-Outreach,Prospect Research,Completed,5 NGOs logged,5
2026-04-02,C-Production,Literature Review,Completed,5 papers summarized,5
```

JSON equivalents should carry the same fields plus optional run identifiers and artifact paths.
