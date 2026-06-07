# Anti-Vibe-Coding Agentic Development Prompt

Date: 2026-06-07

Use this prompt when asking Claude, Codex, or another coding agent to audit the whole OPLURIX/FounderAI repo for issues caused by fast AI-assisted iteration, duplicated files, drift between docs and implementation, vague launch-state claims, and weak verification.

## Why The Research Matters To This Repo

The attached research argues that "vibe coding" works for prototypes but becomes dangerous when a system has agents, multiple flows, long-lived state, external integrations, and public claims. That maps directly to this repo.

Our repo is not just a static website. It contains:

- the FounderAI Rust daemon, provider routing, approvals, inbox/outbox, and runtime artifacts
- the OPLURIX public site under `docs/`
- product pages and delivery packages under `sales/`
- bilingual EN/FR surfaces
- checkout paths through Payhip and PayPal
- planned Kit, Netlify, Shopify, Cloudflare, and provider integrations
- operational docs, campaign links, measurement templates, and product truth rules

The biggest risks are practical:

- product launch status drifting across pages
- form fields not matching the data dictionary
- checkout links implying more readiness than delivery supports
- duplicate product files surviving after cleanup
- docs describing one truth while HTML or JS says another
- agent provider settings failing without clear recovery
- approvals or human review being bypassed by convenience
- no repeatable audit report after each large AI-assisted pass

The replacement for vibe coding is not slow bureaucracy. For us, it means small explicit specs, safe patches, audit trails, verification, and public truth discipline.

## The Prompt

```text
Act as a senior repo stabilization engineer, product-truth auditor, and agentic-systems safety reviewer.

You are working in the OPLURIX/FounderAI repository. Your job is to audit the entire repo for issues created by fast AI-assisted/vibe-coded development, then fix the highest-risk issues safely.

This is not a redesign request. This is a stabilization pass.

Core objective:
Make the repo more explicit, auditable, consistent, testable, and truthful without breaking working flows or deleting user work.

Repository context:
- FounderAI is a Rust agent daemon with provider routing, approvals, file-based runtime artifacts, inbox/outbox, and cloud/local deployment paths.
- OPLURIX is the public marketing/product/research site under `docs/`.
- Product delivery packages live mainly under `sales/oplurix-product-suite/`.
- Product 1 is the only live checkout unless the repo proves otherwise.
- Netlify/manual capture is live; Kit is planned unless the repo proves it is connected.
- Payhip is live for Product 1; PayPal is live for direct ATBC support.
- Shopify may be temporary; the repo site remains the authority layer.

Non-negotiables:
- Do not use destructive commands.
- Do not revert user changes you did not make.
- Do not remove files unless you can prove they are duplicates or obsolete and you document the decision.
- Do not expose or invent secrets.
- Do not claim automation, checkout, delivery, support, or monitoring is live unless the repo proves it.
- Do not ask for hidden chain-of-thought. Provide concise rationale, evidence, and decision traces instead.
- If the working tree is dirty, classify pre-existing changes before editing and avoid mixing unrelated work.

Foundation docs to read first:
1. `CLAUDE.md`
2. `docs/app-spec.md`
3. `docs/data-dictionary.md`
4. `docs/brand-brief.md`
5. `docs/feature-backlog.md`
6. `docs/integrations.md`
7. `docs/foundation-compliance-audit.md`
8. `docs/errors-log.md`
9. `docs/operations/README.md`
10. `sales/oplurix-product-suite/README.md`

Phase 1 - Inventory and truth map:
- Run `git status --short`.
- Inventory major surfaces: `src/`, `config/`, `docs/`, `docs/products/`, `docs/assets/`, `docs/operations/`, `docs/campaigns/`, `sales/`, `documents/`.
- Build a concise truth map:
  - active products
  - source-only products
  - checkout-enabled products
  - active lead forms
  - planned integrations
  - active integrations
  - protected FounderAI actions
  - runtime/provider assumptions

Phase 2 - Audit for vibe-coding failure modes:

Check for these issue classes:

P0 safety/trust issues:
- checkout links on products that are not verified live
- product pages claiming live delivery when package/delivery path is not ready
- public copy implying Kit, Shopify, Netlify, PayPal, Payhip, Cloudflare, Ollama, Claude, or OpenAI behavior that is not true
- approval-sensitive FounderAI actions that bypass human review
- committed secrets or private tokens
- destructive scripts without clear safeguards
- broken public links that affect buying, support, lead capture, or trust

P1 functional issues:
- form fields that do not match `docs/data-dictionary.md`
- campaign parameters that do not match `docs/campaign-link-map.md`
- product ids that drift between HTML, JS, delivery manifests, and docs
- EN/FR pages with contradictory statuses, prices, CTAs, or checkout language
- broken local links in `docs/`
- missing or stale references in README/project docs
- provider config mismatch between docs and `config/*.json`
- Rust tests or smoke tests failing

P2 maintainability issues:
- duplicate product packages, outdated Shopify staging files, or old checkout assets that confuse the source of truth
- dead files that are no longer referenced
- stale docs that contradict foundation docs
- repeated hardcoded URLs that should be centralized or documented
- inconsistent naming conventions
- missing audit notes after important changes
- missing tests around critical Rust modules or JS behavior

P3 polish issues:
- copy inconsistencies
- visual/nav/footer asymmetry
- unclear CTA wording
- minor typos that affect trust

Phase 3 - Evidence-first audit:
For each issue, record:
- severity
- file path
- exact evidence
- why it matters
- recommended fix
- whether you will fix now or defer

Do not fix everything at once. Fix in this order:
1. P0 safety/trust issues.
2. P1 functional issues affecting public flows.
3. P1/P2 docs-data-code drift.
4. Small P2 maintainability fixes that reduce future mistakes.
5. P3 polish only if it is adjacent to a touched file.

Phase 4 - Safe fixes:
- Use small patches.
- Keep user-facing truth conservative.
- If a field name changes, update `docs/data-dictionary.md` in the same patch.
- If product status changes, update `docs/brand-brief.md`, `docs/feature-backlog.md`, product pages, and delivery docs together.
- If an integration status changes, update `docs/integrations.md`.
- If a broken flow is found but cannot be fixed safely, add an entry to `docs/errors-log.md` or an audit report.
- If duplicate files are found, list them first; only delete when duplication is proven and the canonical path is documented.

Phase 5 - Verification:
Run the safest available checks:
- `git diff --check`
- `cargo test` if Rust code was touched or if the audit covers runtime behavior
- local/static link checks if a link checker exists
- targeted `rg` checks for old contradictory phrases
- targeted inspection of Netlify form fields
- targeted checks for public checkout URLs and product status language

If a check cannot be run, say why.

Phase 6 - Required output files:
Create or update:
- `docs/operations/vibe-coding-audit-report-YYYY-MM-DD.md`

The report must include:
- executive summary
- repo truth map
- issue table with severity, file, evidence, fix status
- changes made
- checks run
- checks not run
- remaining risks
- recommended next pass

Final response format:
Start with findings and fixes, not a generic summary.
Include exact file links.
Mention tests/checks run.
Mention any pre-existing unrelated dirty changes that were intentionally left alone.
Keep the final response concise and decision-grade.

Important principle:
The goal is not to make the repo look cleaner. The goal is to make the repo harder to lie to, harder to drift, easier to debug, and safer to operate.
```

## Recommended First Use

Run the prompt after the current foundation-doc work is committed or at least clearly staged. The audit will be cleaner if it can separate:

- foundation-doc changes
- product-package changes
- Shopify cleanup
- future anti-vibe-coding fixes

If the tree is still dirty, tell the agent to produce an audit report first and only patch P0/P1 issues that are clearly safe.
