# App Spec

## One-Sentence Description

OPLURIX/FounderAI is a repo-based ecological intelligence platform that combines a public marketing and product site, conservation-focused digital products, research campaign pages, operational playbooks, and an auditable AI-assisted FounderAI runtime.

## Current Stage

The project is already being built.

It is not just an idea. The repo already contains:

- a Rust FounderAI daemon and CLI
- Docker/cloud deployment files
- a public static OPLURIX site under `docs/`
- bilingual pages
- one live digital product
- product delivery packages
- campaign pages and link maps
- Netlify-compatible forms
- operations docs
- measurement templates
- AI provider routing for Ollama, Claude, and OpenAI

## Core Purpose

The platform exists to help real conservation work become visible, useful, and economically sustainable without losing scientific integrity.

It supports two connected needs:

- Public-facing OPLURIX work: audience building, product sales, lead capture, research support, services, and credibility.
- Internal FounderAI work: agent-assisted drafting, outreach, grant support, operations, approvals, and auditable run artifacts.

## Primary Users

### Conservation Communicator Buyer

You are a field biologist, graduate researcher, conservation NGO worker, or biodiversity professional. You have real experience, notebooks, observations, lessons, and project work. You know the work matters, but you do not have a repeatable way to turn it into clear public communication.

You need small, practical systems that help you publish without becoming fake, vague, or careless.

### Biodiversity Services Prospect

You are a researcher, NGO team, or conservation organization with field data, reporting pressure, training needs, or donor communication needs. You want someone who understands the field reality behind the data, not only the presentation layer.

You need credible support for analysis, communication, training, remote sensing, or proposal work.

### Research Supporter Or Collaborator

You care about Madagascar, tropical ecology, drone surveys, lemur monitoring, or conservation visibility. You may not buy a product, but you may support the ATBC travel campaign, share the work, or become a collaborator.

You need a clear story, direct support path, and proof that the work is real.

### Founder Operator

You are building OPLURIX as both a survival engine and a serious conservation platform. You need the repo to keep strategy, site, products, campaigns, delivery, and automation aligned.

You need clarity more than more ideas.

## Problems Solved

- Conservation knowledge stays hidden in notebooks and internal documents.
- Technical biodiversity work often fails to become public trust, funding, or collaboration.
- Generic AI advice does not respect scientific context or field constraints.
- Product ideas can multiply faster than verified checkout and delivery.
- Campaign links, forms, and metrics can drift without a data dictionary.
- AI agent outputs need approvals, logs, and inspectable artifacts.

## Core Jobs To Be Done

1. Help the right visitor understand OPLURIX within seconds.
2. Route cold traffic to the free ethics checklist.
3. Route warm traffic to Product 1.
4. Route hot traffic to Payhip checkout.
5. Route research-support traffic to the ATBC campaign page.
6. Capture leads through Netlify now and Kit later.
7. Keep product launch status truthful.
8. Package and deliver digital products manually until automation is verified.
9. Track weekly conversion signals.
10. Preserve FounderAI approvals and auditability.

## V1 Scope

V1 is a lightweight, trustworthy conversion and operations system.

Included:

- public OPLURIX homepage
- Product 1 page in English and French
- products index pages
- ethics checklist lead magnet in English and French
- ATBC research campaign pages in English and French
- about, projects, services, privacy, thank-you, and field-notes pages
- Netlify-compatible forms
- Payhip checkout for Product 1
- PayPal direct support link for the ATBC campaign
- manual delivery SOPs and packages
- weekly measurement scoreboard
- campaign link map
- FounderAI Rust daemon, CLI, runtime artifacts, approvals, model routing, and provider configs

## Not V1

Do not build these until the current funnel has signal:

- full SaaS account system
- custom user dashboard
- automated subscription platform
- database-first rewrite
- full CRM replacement
- active checkout for Products 2 to 10 without verified delivery
- automated email sequences unless Kit is actually connected
- autonomous external sending without approvals
- complex MRV scoring application
- institutional licensing automation
- AI-generated claims without human verification

## Public Site Information Architecture

Current public surfaces:

| Surface | Primary job |
| --- | --- |
| `docs/index.html` | Main conversion hub: Product 1, checklist, proof, contact. |
| `docs/products/index.html` | Product suite overview with clear launch-state truth. |
| `docs/products/01-expert-to-influencer-content-engine.html` | Live Product 1 sales page. |
| `docs/products/01-expert-to-influencer-content-engine-fr.html` | French Product 1 sales page. |
| `docs/ethics-checklist.html` | English free-entry lead magnet. |
| `docs/ethics-checklist-fr.html` | French free-entry lead magnet. |
| `docs/atbc-2026-drone-surveys.html` | English research support and credibility page. |
| `docs/atbc-2026-drone-surveys-fr.html` | French research support and credibility page. |
| `docs/about.html` | Founder and OPLURIX origin/context. |
| `docs/about-fr.html` | French founder and OPLURIX origin/context. |
| `docs/projects.html` | Wider project map and bigger-picture proof. |
| `docs/services-en.html` | English services conversion page. |
| `docs/services-fr.html` | French services conversion page. |
| `docs/field-notes.html` | Authority-building field notes hub. |
| `docs/thank-you.html` | Post-form routing. |
| `docs/privacy.html` | Privacy and trust. |

## Primary User Flows

### Cold Social Visitor

1. Sees LinkedIn or Facebook post.
2. Clicks checklist link with `campaign`, `surface`, `lang`, and `interest`.
3. Submits Netlify form.
4. Lands on thank-you page.
5. Gets checklist and sees Product 1 as next step.

### Warm Product Visitor

1. Arrives on Product 1 page.
2. Reads promise, proof, contents, objections, and delivery expectations.
3. Clicks Payhip checkout.
4. Receives Payhip confirmation and manual delivery path.
5. Operator records sale and feedback.

### Research Support Visitor

1. Arrives on ATBC campaign page.
2. Understands drone survey context, Madagascar sites, ATBC 2026 significance, and travel funding need.
3. Chooses direct PayPal support, Payhip purchase, or manual support route.

### FounderAI Operator

1. Drops request into inbox or runs CLI command.
2. FounderAI assembles prompt packet.
3. Provider runs through router.
4. Output, metadata, stdout, stderr, and approvals are written to runtime artifacts.
5. Human approves protected actions before external consequences.

## Success Metrics

North star:

- Turn warm attention into owned audience and Product 1 sales.

Weekly metrics:

- `checklist_signups_en`
- `checklist_signups_fr`
- `updates_or_contact_signups`
- `total_email_subscribers`
- `product_page_visits`
- `product_checkout_clicks`
- `payhip_sales`
- `revenue_usd`
- `linkedin_comments_or_dms`
- `top_objection`

FounderAI reliability metrics:

- successful `tick`
- provider reachability
- non-zero failure runs investigated
- pending approvals reviewed
- run artifacts complete

## Operating Principles

- One live product beats ten unclear offers.
- Product truth must be visible.
- The repo site is the authority layer.
- Public copy must preserve field credibility.
- Manual delivery is acceptable if it is honest and timely.
- Automation is useful only after the manual path is trustworthy.
- Every protected external action needs human approval.

