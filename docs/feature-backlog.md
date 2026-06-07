# Feature Backlog

This backlog keeps the repo focused. It is not a place to collect every possible idea; it is a filter for what supports the next credible stage.

## Current North Star

Turn warm attention into owned audience and Product 1 sales.

Primary offer:

- Expert-to-Influencer Content Engine

Primary free entry:

- Conservation Content Ethics Checklist

Primary weekly metric:

- Email subscribers from conservation-relevant traffic

## Now

These are active v1 priorities.

| Priority | Feature | Why it matters | Acceptance criteria |
| --- | --- | --- | --- |
| P0 | Keep Product 1 as the main checkout path | Prevents product sprawl before sales signal. | Product 1 remains the only live checkout unless founder approves another product after activation gates. |
| P0 | Maintain clean homepage conversion path | First-time visitors need one clear next step. | Homepage clearly routes cold, warm, hot, and research-support visitors. |
| P0 | Keep Netlify forms working | Owned audience is the strongest near-term asset. | Checklist and contact/update submissions preserve canonical fields from `data-dictionary.md`. |
| P0 | Track weekly conversion metrics | Decisions need signal, not vibes. | `docs/operations/measurement-scoreboard-template.csv` can be filled every Sunday. |
| P0 | Preserve checkout and delivery truth | Trust is more valuable than temporary conversion. | No page claims instant automation unless tested; Product 1 delivery path remains documented. |
| P1 | Improve Product 1 proof and objection handling | Checkout hesitation is likely the main conversion bottleneck. | Product page has proof near CTA, "what happens after purchase", and FAQ based on real objections. |
| P1 | Keep English/French strategic symmetry | Bilingual authority is a real advantage. | EN and FR public pages have equivalent navigation, CTA logic, launch status, and footer wording. |
| P1 | Use campaign link map consistently | Attribution must survive platform changes. | Campaign links use `campaign`, `surface`, `lang`, and `interest` query parameters. |
| P1 | Maintain FounderAI run auditability | Agent work must stay inspectable. | Runs write prompt, output, metadata, stdout, and stderr. Protected actions pause for approval. |

## Next

These should follow after the current funnel is stable.

| Priority | Feature | Activation gate | Notes |
| --- | --- | --- | --- |
| P1 | Connect Kit forms and sequence | Netlify manual capture is working and checklist path is stable. | Use `docs/operations/05-kit-implementation-blueprint.md`, `06-kit-go-live-repo-checklist.md`, and `07-kit-sequence-outline.md`. |
| P1 | Publish Product 2 waitlist or checkout | Product 1 has sales signal or repeated demand for training material conversion. | Product 2 is Training-to-Quiz Generator. Checkout only after package and delivery test. |
| P1 | Publish Product 4 waitlist or checkout | Product 1 has signal and pitch-deck package is verified. | Product 4 is Biodiversity Pitch Deck Builder. |
| P2 | Refresh Product 3 before promotion | Product 2 or 4 is stable, or multiple buyers ask for field equipment decision support. | Product 3 should stay held until equipment specs, market prices, and update rules are refreshed. |
| P2 | Add stronger services intake | Service traffic appears or direct prospects ask for help. | Keep service path distinct from Product 1 path. |
| P2 | Add better analytics instrumentation | Manual weekly metrics are too slow or incomplete. | Use canonical event names from `data-dictionary.md`. |
| P2 | Create richer ATBC campaign updates | Support traffic increases or new proof becomes available. | Add updates without overclaiming monitoring or conservation outcomes. |
| P2 | Create product feedback loop | First buyers or beta readers respond. | Capture objections, testimonial snippets, and delivery issues. |

## Later

These are valuable, but not urgent.

| Feature | Why later |
| --- | --- |
| Stripe checkout | Payhip is enough for the current live product. |
| Shopify checkout migration | Useful only if Shopify remains cost-effective and delivery is cleaner than Payhip. |
| Supabase or database backend | Static files and CSVs are enough until data volume becomes painful. |
| Logged-in customer portal | Not needed for manual product delivery. |
| Automated product delivery | Useful after the manual delivery package is proven. |
| Full CRM | Kit plus CSV review is enough for the first signal cycle. |
| Public dashboard | Metrics should first prove useful internally. |
| ERIS/MRV interactive platform | Too heavy before OPLURIX has steady attention and revenue. |
| Multi-agent web UI | FounderAI CLI and private console are enough for now. |

## Explicit Non-Goals

Do not spend v1 energy on:

- launching every product because the pages exist
- redesigning the full site instead of measuring the funnel
- adding a database because it feels more professional
- hiding manual operations behind fake automation
- building a generic AI content platform
- removing approvals for convenience
- creating claims of conservation impact without evidence
- optimizing for vanity traffic over qualified conservation-relevant leads

## Product Activation Gates

A product can move from waitlist/source-only to live checkout only when all of these are true:

- Public page exists.
- Product package exists.
- ZIP or delivery asset exists.
- Delivery notes are clear.
- Price is confirmed.
- Refund/support expectations are documented.
- Checkout path is tested.
- Thank-you or delivery confirmation path is tested.
- Founder approves launch state.

## Stop Rule

Do not create a new product or new sales page until one of these is true:

- Product 1 has at least 10 sales.
- The same buyer objection appears three times.
- A specific audience asks for the same next product at least three times.

## First Build Order For Future Code Work

When a future coding session starts without a more specific instruction, prefer this order:

1. Fix broken links, forms, checkout routing, or delivery truth first.
2. Improve conversion clarity on the homepage or Product 1 page.
3. Improve measurement and attribution.
4. Improve bilingual symmetry.
5. Improve operations docs and delivery packaging.
6. Only then add new product or platform features.
