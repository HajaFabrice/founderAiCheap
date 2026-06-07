# Foundation Compliance Audit

Date: 2026-06-06

Purpose: make the seven foundation documents operational by checking the public site against them and recording the corrections made.

## Guardrails Checked

| Guardrail | Source | Result |
| --- | --- | --- |
| Claude Code must know where the foundation docs live. | `CLAUDE.md` | Passed. The root instructions now point to all six docs and the naming rule. |
| Foundation docs must be discoverable. | `README.md`, `docs/project-docs.html` | Fixed. README and public project docs now link to the foundation set. |
| Form fields must match the data dictionary. | `docs/data-dictionary.md` | Fixed. The homepage checklist-plus-updates form now maps to the checklist thank-you variant and the updates track. |
| Product launch status must not overclaim readiness. | `docs/brand-brief.md`, `docs/feature-backlog.md` | Fixed. Product 3 is now treated as held/source-only until its equipment facts and pricing workflow are refreshed. |
| Product catalog ids must be stable. | `docs/data-dictionary.md`, `docs/assets/storefront.js` | Fixed. Product 3 now has a storefront key with checkout disabled, and the data dictionary uses the manifest-aligned product id. |
| Integrations must not imply automation that is not live. | `docs/integrations.md` | Passed. Public pages still describe Netlify/manual capture and Payhip Product 1 checkout. |

## Corrections Made

1. Added the foundation docs to the repository `README.md`.
2. Added a Foundation docs section to `docs/project-docs.html`.
3. Added this audit file as a repeatable checkpoint.
4. Added `gear-equipment-concierge` to the storefront product map with `checkoutEnabled: false`.
5. Updated Product 3 status language from next-release candidate to held-for-refresh/source-only language.
6. Updated English and French Product 3 index copy so it no longer implies near-term checkout.
7. Updated the data dictionary product id for Product 3 to match the delivery manifest form: `oplurix_03_gear_and_equipment_concierge`.
8. Added `display_price` to the data dictionary for products with draft prices or ranges.
9. Clarified homepage checklist-plus-updates attribution with `interest_type="checklist"` and `update_track="Printable ethics checklist plus OPLURIX updates"`.
10. Updated `docs/operations/kit-field-map.csv` so the future Kit mapping matches the hybrid homepage form.

## Current Truth After Audit

- Product 1 is the only live checkout.
- Products 2 and 4 are the clean next release candidates.
- Product 3 is visible but held until specs, market pricing, and update cadence are refreshed.
- Products 5 to 10 remain source-only or draft public lanes.
- Netlify/manual capture remains the active lead path.
- Kit remains planned, not live.
- Payhip remains the active Product 1 checkout.
- PayPal remains active for direct ATBC support.

## Remaining Watch Items

- If Product 3 is revived, refresh product specs, price assumptions, and maintenance/update disclaimers before calling it a release candidate.
- If Kit is connected, update `docs/integrations.md`, `docs/data-dictionary.md`, and `docs/operations/kit-field-map.csv` in the same change.
- If Product 2 or 4 checkout goes live, update `docs/brand-brief.md`, `docs/feature-backlog.md`, `docs/data-dictionary.md`, public product pages, and delivery SOPs together.
- If the hosting base URL changes from GitHub Pages to Netlify, update `docs/campaign-link-map.md` first.
