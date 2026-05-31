# Shopify / Netlify Platform Sync

Updated: 2026-05-31

## Role Split

Shopify is the focused buying surface:

- one live offer
- Payhip-first checkout
- short conversion copy
- product cover and direct contact
- no draft catalog pressure

Netlify is the authority and continuity surface:

- bilingual pages
- research, checklist, field notes, services, and project context
- Netlify forms and thank-you flows
- broader proof paths for visitors who need trust before buying
- static fallback if the storefront moves away from Shopify

## Shared Customer Truth

- Current live product: `Expert-to-Influencer Content Engine`
- Price: `$39`
- Primary CTA: `Get Instant Access — $39`
- Checkout: Payhip
- Product framing: field notes and real conservation work made visible
- Trust framing: `Who built this`, not anxious proof stacking
- About framing: the tool came from a real field/research communication problem
- Draft/future products stay visible only as validation or roadmap context

## Route Bridge

Netlify now resolves the most important Shopify-style URLs:

- `/pages/about` -> `/about.html`
- `/pages/contact` -> `/#contact`
- `/pages/digital-delivery-policy` -> `/digital-delivery-policy.html`
- `/pages/data-sharing-opt-out` -> `/privacy.html`
- `/pages/expert-to-influencer-content-engine` -> `/products/01-expert-to-influencer-content-engine.html`
- `/products/expert-to-influencer-content-engine` -> `/products/01-expert-to-influencer-content-engine.html`
- `/collections/all` -> `/products/`

This means customer links can survive a move from Shopify to Netlify without looking broken.

## Keep In Sync

When copy changes on one platform, check these files:

- Shopify: `templates/index.json`
- Shopify: `templates/page.about.json`
- Shopify: product/page body content in Shopify admin/API
- Netlify: `docs/index.html`
- Netlify: `docs/about.html`
- Netlify: `docs/about-fr.html`
- Netlify: `docs/products/01-expert-to-influencer-content-engine.html`
- Netlify: `docs/products/01-expert-to-influencer-content-engine-fr.html`
- Netlify: `docs/assets/storefront.js`

The strongest rule: Shopify can stay lean, but Netlify must never contradict it.
