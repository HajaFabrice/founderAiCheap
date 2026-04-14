# ERIS Agent System

Repo-synced operational mirror of `ERIS_AgentSystem_Codex_v4_0.docx`.

Updated: 2026-04-14

## Purpose

This document translates the ERIS agent-system brief into repo-operational language. It is meant to stay readable in Git, while the original `.docx` remains preserved as source material.

## What Stays

- Keep the existing FounderAI Rust daemon and CLI.
- Keep inbox/outbox, approvals, audit trails, runtime artifacts, and file-based state.
- Keep provider switching and local-first operation.
- Keep the 6-role team structure as the core workflow engine.

## What Extends

- Add saint names and named agent identities to the existing six roles.
- Add workflow-specialist agents: Zacchaeus, Perpetua, Hildegard, Clare, Francis, and Columban.
- Add model routing by task type.
- Add offline queueing with replay after connectivity returns.
- Add founder-brain knowledge files for ERIS, governance, QA, KPIs, risks, and Hormozi rules.

## Canonical Agent Mapping

| Role or Workflow | Agent |
| --- | --- |
| A-Outreach | Anthony |
| A-Production | Juniper |
| B-Outreach | Bonaventure |
| B-Production | Bernardine |
| C-Outreach | Jacinta |
| C-Production | Duns Scotus |
| Lead response | Zacchaeus |
| Follow-up sequences | Perpetua |
| Briefings and notes | Hildegard |
| QA gate | Clare |
| Weekly review | Francis |
| Self-improvement | Columban |

## Governance Rules

- No autonomous external send.
- Client-facing AI drafts require transparent disclosure.
- Governance files and approval gates are immutable safety rails.
- Mission drift toward extractive or generic off-scope work is disallowed.
- The 06:00 formation rhythm remains a non-negotiable anchor.

## Immediate Repo Deliverables

1. `config/agents.json` contains the live agent roster.
2. `config/founderai.json` and smoke config include saint identities, model routing, and offline queue settings.
3. `founder-brain/` contains ERIS knowledge, governance constraints, QA rubrics, KPI thresholds, risk register, roadmap, forbidden patterns, and prompt files.
4. Worker prompt packets load these files directly during runs.

## Phased Implementation Logic

### Phase 1

- Wire saint identities and prompts.
- Add ERIS knowledge assets.
- Keep existing jobs working.

### Phase 2

- Deepen offline and nurture behavior.
- Add workflow-specific agents when their operational loops are ready.

### Phase 3

- Strengthen QA, weekly review, and self-improvement.
- Expand toward UI and integrations only after the core daemon remains reliable.

## Sync Note

When implementation details conflict with higher-level strategy, the master plan governs strategy and this ERIS agent-system brief governs implementation shape.
