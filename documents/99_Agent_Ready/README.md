# Agent-Ready Document Layer

Updated: 2026-04-22

This folder is the curated, plain-text mirror of the broader `documents/`
library. Agents should use this layer first during conversations and
operations instead of relying on raw mixed-version source files.

## Selection Rules

1. If the founder explicitly attaches or points to a newer text export, that
   file becomes the canonical source for that document family.
2. Otherwise, the highest explicit version number wins.
3. If version numbers are missing, the latest dated filename wins.
4. Older files remain archival and only fill gaps when newer files do not
   contain the needed detail.
5. Every artifact here must stay inspectable as plain Markdown or JSON.

## Current Canonical Strategy Sources

- `C:/Users/Student/Desktop/perso/01_MasterPlan_v4.0.txt`
- `C:/Users/Student/Desktop/perso/ERIS_AgentSystem_CodexInput_v5.0.txt`
- `documents/01_Strategy_And_Systems/Claude_Document_Pack/ClaudeFileV4/AI Tools Cloud Plan - Repo Synced.md`

## Current Canonical Operations Sources

- `documents/01_Strategy_And_Systems/ERIS/ERIS_Communication_Templates_v1.0.txt`
- `documents/03_Freelance_And_Career/Outreach/Outreach Emails.docx`
- `documents/03_Freelance_And_Career/Profiles_And_Offers/service statement alternatives.docx`
- `documents/03_Freelance_And_Career/Profiles_And_Offers/Freelance Services_ Biodiversity Fieldwork & Data Analysis (Madagascar).docx`
- `documents/02_Business_And_Wealth/Roadmaps_And_Blueprints/The Billion-Scale Business Blueprint.txt`

## Subfolders

- `references/`: short canonical briefings and source-priority rules
- `databases/`: structured operational memory and document registry files
- `templates/`: reusable copy, instructions, and JSON skeletons for agent runs

## Working Rule For Agents

Use this folder for prompt-building, conversation grounding, and operational
drafting. Fall back to the raw source documents only when a needed detail is
not yet mirrored here, and prefer `NEEDS_HUMAN_VERIFICATION` over guessing.
