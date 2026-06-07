# Vibe-Coding Audit Report - 2026-06-07

Purpose: record the first repo-wide anti-vibe-coding audit after the foundation
docs were added. This report is meant to prevent repeated pass-by-pass cleanup
and give future agents a stable truth map before they edit.

## Executive Summary

The repo is moving in the right direction, but the main risk was not broken
code. The main risk was product-truth drift: several operations, campaign, and
delivery files still grouped Product 3 with Products 2 and 4 as part of the next
clean release set.

That was unsafe because Product 3 depends on equipment specs, market pricing,
and update logic that can become stale quickly. The current source of truth is:

- Product 1 is the only live checkout.
- Product 1 checkout is Payhip.
- Products 2 and 4 are the clean next release candidates.
- Product 3 is visible but held/source-only until refreshed.
- Products 5 to 10 are source-only or earlier-stage.
- Netlify/manual lead capture is active.
- Kit is planned unless a future audit proves it is connected.
- Shopify is temporary/optional and must not become the canonical truth layer.

## Working Tree Note

The working tree was already dirty before this audit. I treated unrelated
changes as pre-existing and did not revert them.

Pre-existing notable changes include:

- foundation docs added under `docs/`
- public site and product-page edits from the current improvement roadmap
- French Product 1 package work
- Shopify upload-file deletions
- an untracked French Product 1 PDF

This audit only patched high-confidence contradictions.

## Source Of Truth Used

Foundation files checked:

- `CLAUDE.md`
- `docs/app-spec.md`
- `docs/data-dictionary.md`
- `docs/brand-brief.md`
- `docs/feature-backlog.md`
- `docs/integrations.md`
- `docs/foundation-compliance-audit.md`

Operational truth used:

- no new table, field, product id, or status should be invented without checking
  `docs/data-dictionary.md`
- public copy must not imply a checkout, automation, or integration is live
  unless the repo proves it
- Product 3 must not be promoted as a clean next release until refreshed

## Issues Found And Fixed

| Severity | Issue | Risk | Fix |
|---|---|---|---|
| P1 | Product 3 was still grouped with Products 2 and 4 in campaign docs. | Public launch could promote a product that is not operationally ready. | Updated launch calendar and launch checklist to say Products 2 and 4 are next, Product 3 is held for refresh. |
| P1 | The master improvement roadmap still had a "Products 2 To 4" launch phase. | Future work could follow an outdated release order. | Reframed the phase as "Products 2 And 4 Cleanly, Refresh Product 3" and changed the activation order. |
| P1 | French Product 3 delivery metadata said `coming_soon`, `prepare_clean_release`, and `$49`. | Product metadata contradicted the English manifest and data dictionary. | Changed Product 3 FR metadata to `source_only`, `defer`, `$39`, and the canonical `oplurix_03_gear_and_equipment_concierge` id pattern. |
| P2 | French Product 3 notes called it part of the next wave. | Internal delivery notes could mislead future agents. | Reframed it as an internal preparation resource, not a launch pack. |
| P2 | French package README implied Products 2 to 4 share the same launch approval path. | Product 3 could be treated as merely pending approval instead of requiring refresh. | Split Products 2 and 4 from Product 3 in the README. |

## Files Patched In This Audit Pass

- `docs/campaigns/launch-operations-calendar.md`
- `docs/campaigns/launch-readiness-checklist.md`
- `docs/oplurix-master-improvement-roadmap.md`
- `sales/oplurix-product-suite/packages-fr/README.md`
- `sales/oplurix-product-suite/packages-fr/03-gear-equipment-concierge-fr/README.md`
- `sales/oplurix-product-suite/packages-fr/03-gear-equipment-concierge-fr/DELIVERY_MANIFEST.json`
- `sales/oplurix-product-suite/packages-fr/03-gear-equipment-concierge-fr/03_Notes/NOTES_DE_LIVRAISON.md`

## Checks Run

Product-truth scan:

- checked for stale "Products 2 to 4", "Products 2, 3, and 4", Product 3 next
  release wording, and old Product 3 id drift
- remaining hit is Product 2 French notes using "prochaine vague", which is
  valid for Product 2

Checkout scan:

- Product 1 is the only Payhip checkout surfaced in the public product pages
- product map has `checkoutEnabled: true` only for Product 1
- other product pages remain waitlist, held, or no-checkout states

Form scan:

- homepage updates form uses `oplurix-updates`
- homepage product-interest form uses `oplurix-product-interest`
- checklist EN/FR forms use `ethics-checklist-interest`
- all inspected forms include Netlify attributes, hidden `form-name`, and
  Kit-role metadata for the future migration

Local HTML link check:

- result: `LOCAL_LINK_CHECK=pass`
- scope: all `.html` files under `docs/`
- checked relative `href` and `src` targets

Secret scan:

- no actual private API key was identified
- expected placeholders were found in docs/examples, such as `your-key`,
  `sk-...your-key-here...`, and `replace-me`
- the PayPal client id in `docs/assets/storefront.js` is public client-side
  configuration, not a PayPal secret

Whitespace check:

- `git diff --check` returned no whitespace errors
- Windows CRLF warnings appeared, but they are line-ending warnings, not diff
  failures

Rust tests:

- `cargo test`
- result: 13 passed, 0 failed

## Deferred Risks

- The repo still contains broad historical material under `documents/` and
  `sales/`; not all old strategic assumptions should be treated as current.
- Some generated package READMEs contain PowerShell interpolation artifacts like
  `$(@{...}.field)`. They are ugly but not newly introduced here.
- Shopify upload duplicate cleanup is already in the working tree; this audit
  did not validate whether every removed upload file has a canonical replacement.
- Kit is still planned, not proven live. Future copy must keep saying manual or
  Netlify fallback until the actual Kit form/action is connected and tested.
- Product 3 requires a dedicated refresh pass before any public launch or paid
  checkout.

## Recommended Next Pass

1. Commit or stage the foundation-doc work separately from the audit fixes if
   possible.
2. Run a duplicate-asset cleanup pass for Shopify upload files and product PDFs.
3. Create a small canonical product-state JSON file if the site will keep
   growing.
4. Add a lightweight link-check script to the repo so local HTML checks are
   repeatable.
5. Add a "source truth" note to old `documents/` folders so future agents know
   which materials are archive/reference versus current operating doctrine.

## Current Daily Rule

If a page, package, campaign, or agent prompt says something is live, automated,
or ready to sell, it must be backed by the foundation docs and by a real route in
the repo. If not, call it visible, planned, draft, manual, source-only, or held.

