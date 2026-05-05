# Agent-Ready Document Layer

Updated: 2026-05-05

This folder is the curated, plain-text mirror of the broader `documents/`
library. Agents should use this layer first during conversations and
operations instead of relying on raw mixed-version source files.

## Selection Rules

1. If the founder explicitly attaches or points to a newer text export, that
   file becomes the canonical source for that document family.
2. If the founder supplies a newer CV or profile update directly in chat, that
   update overrides older CV mirrors for bio blocks and marketing positioning.
3. Otherwise, the highest explicit version number wins.
4. If version numbers are missing, the latest dated filename wins.
5. Older files remain archival and only fill gaps when newer files do not
   contain the needed detail.
6. Every artifact here must stay inspectable as plain Markdown or JSON.

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

## Current Canonical Digital Product Sources

- `documents/oplurix-site/OPLURIX_Product_Suite_Final.md`
- `documents/oplurix-site/OPLURIX_Master_Context_Brief.md`
- `documents/oplurix-site/ExpertToInfluencer.txt`
- `documents/RDigitalProduct/EcoR_Toolkit_Product_Strategy.md`
- `documents/RDigitalProduct/Gumroad_Tier_Packages/TIER_PRODUCT_PAGE_COPY.md`

## Current Canonical Freelance Sources

- `documents/Plans/Freelance/Cleaning Summary.docx`
- `documents/Plans/Freelance/cleaned_data.csv`
- `documents/Plans/Freelance/Haja Fabrice Cv Optimized.docx`
- `documents/Plans/Freelance/Haja Fabrice CV-EnvData-FR.docx`
- `documents/Plans/Freelance/30-Day NGO Outreach & Contract Plan - Grok`
- `documents/Plans/Freelance/Client Outreach tracker 1.xlsx`
- `documents/Plans/Freelance/client outreach tracker 2.xlsx`

## Subfolders

- `references/`: short canonical briefings and source-priority rules
- `databases/`: structured operational memory and document registry files
- `templates/`: reusable copy, instructions, and JSON skeletons for agent runs

## New Agent-Ready Freelance Assets

- `references/independent_business_boundary.md`
- `references/independent_marketing_brief.md`
- `references/marketing_intelligence_overlay.md`
- `references/freelance_operating_brief.md`
- `references/agent_conversation_reference.md`
- `references/new_contact_answer_bank.md`
- `references/collaboration_charter.md`
- `references/eris_metadata_governance.md`
- `references/digital_products_sales_brief.md`
- `references/oplurix_agent_mission_map.md`
- `references/oplurix_full_product_suite.md`
- `references/oplurix_launch_matrix.md`
- `references/continuous_improvement_system.md`
- `databases/freelance_proof_assets.json`
- `databases/independent_service_catalog.json`
- `databases/independent_pipeline.json`
- `databases/independent_crm.json`
- `databases/review_ready_outreach_shortlist.json`
- `databases/founder_profile_blocks.json`
- `databases/eris_scoring_defaults.json`
- `databases/digital_products_catalog.json`
- `databases/oplurix_product_suite.json`
- `databases/oplurix_launch_matrix.json`
- `runtime/improvement/achievement_log.json`
- `runtime/improvement/customer_feedback.json`
- `runtime/improvement/improvement_backlog.json`
- `runtime/improvement/latest_weekly_retrospective.md`
- `templates/independent_freelance_templates.md`
- `templates/first_outbound_pack.md`
- `templates/ecor_first_sale_launch_pack.md`
- `templates/marketing_intelligence_templates.md`
- `templates/digital_product_sales_templates.md`
- `templates/continuous_improvement_templates.md`

## Working Rule For Agents

Use this folder for prompt-building, conversation grounding, and operational
drafting. Fall back to the raw source documents only when a needed detail is
not yet mirrored here, and prefer `NEEDS_HUMAN_VERIFICATION` over guessing.
