# Integrations

This file tracks external services, APIs, manual channels, and local runtime dependencies for the OPLURIX/FounderAI platform.

## Integration Principles

- Keep the repo as the authority layer.
- Prefer simple, low-cost tools until traffic and sales justify complexity.
- Do not put secrets in public static files.
- Do not claim an integration is live until it has been tested end to end.
- Keep manual fallback paths visible for checkout, delivery, and support.
- Log integration failures in `docs/errors-log.md`.

## Active Public Integrations

| Integration | Current role | Status | Source of truth | Notes |
| --- | --- | --- | --- | --- |
| GitHub Pages or static `docs/` hosting | Public repo-based site | Active | `docs/` | Current public routing is based on `https://hajafabrice.github.io/founderAiCheap/`. |
| Netlify-compatible forms | Lead capture | Active/manual | HTML forms in `docs/` | Forms include hidden campaign fields and route through `docs/thank-you.html`. |
| Payhip | Product 1 checkout | Active | `docs/assets/storefront.js`, Product 1 pages | Product 1 uses direct Payhip checkout. |
| PayPal support link | ATBC direct support | Active | ATBC campaign pages | Direct support link is used for research travel support. |
| PayPal SDK | Product checkout fallback for some products | Partial | `docs/assets/storefront.js` | Public client id may exist in frontend; never place PayPal secret in frontend. |
| LinkedIn | Distribution and campaign traffic | Active/manual | `docs/campaign-link-map.md`, campaign runbooks | First-comment links should preserve tracking parameters. |
| YouTube | Research/context proof | Active/manual | ATBC campaign pages and campaign copy | Used as broader research context. |
| Email | Manual delivery and contact | Active/manual | storefront config and SOPs | Manual delivery remains acceptable if expectations are clear. |
| WhatsApp/phone | Manual support and direct payment coordination | Active/manual | public support copy where present | Keep phone-based support realistic and not overpromised. |
| Mvola/Sendwave | Direct support/payment channels | Active/manual | campaign copy | Manual confirmation required. |

## Active FounderAI Runtime Integrations

| Integration | Current role | Status | Config or path | Notes |
| --- | --- | --- | --- | --- |
| Ollama | Local/offline model provider | Active when available | `FOUNDERAI_PROVIDER=ollama`, `FOUNDERAI_BASE_URL` | Cloud compose includes an Ollama container. |
| Claude/Anthropic | Hosted model provider | Active when API key and balance exist | `ANTHROPIC_API_KEY`, `config/founderai*.json` | Provider failures must be written to run artifacts. |
| OpenAI | Optional hosted provider | Supported | `OPENAI_API_KEY` | Keep provider switching compatible. |
| Docker Compose | Cloud/local service orchestration | Active | `docker-compose.*.yml` | Cloud stack includes daemon, web, Ollama, and Cloudflare tunnel. |
| Cloudflare Tunnel | Private cloud access to web console | Configured but token-dependent | `CLOUDFLARE_TUNNEL_TOKEN` | Used to expose private services safely when token exists. |
| File runtime | Inbox, outbox, approvals, runs | Active | `inbox/`, `outbox/`, `runtime/` | This is a core design choice, not a temporary hack. |

## Planned Or Candidate Integrations

| Integration | Intended role | Activation condition | Notes |
| --- | --- | --- | --- |
| Kit | Email capture and welcome sequence | Checklist forms are stable and Kit account/form exists. | Use existing Kit blueprints in `docs/operations/`. |
| Netlify hosting | Long-term lightweight public hosting | When ready to move from GitHub Pages or Shopify. | Netlify pairs well with current static site and forms. |
| Shopify | Temporary marketing platform and possible checkout | Use only while subscription is justified. | Repo site should remain the strategic authority layer. |
| Stripe | Future checkout and webhooks | Sales volume justifies custom checkout. | Requires secure backend or hosted checkout. |
| Supabase/Postgres | Future structured storage | CSV/manual tracking becomes limiting. | Use table names from `docs/data-dictionary.md`. |
| Vercel | Future app hosting or server functions | If dynamic features are needed. | Not necessary for the current static-first funnel. |
| Analytics platform | Better traffic/event measurement | Manual scoreboard becomes insufficient. | Use reserved event names from `data-dictionary.md`. |

## Paused Or Legacy Integrations

| Integration | Current stance | Reason |
| --- | --- | --- |
| Gumroad | Legacy/reference only | Payhip is the current checkout for Product 1. |
| Full SaaS backend | Not v1 | Too much complexity before funnel signal. |
| Fully autonomous outbound sending | Not allowed without approvals | Protected actions require human review. |

## Environment Variables

| Variable | Used by | Secret? | Notes |
| --- | --- | --- | --- |
| `ANTHROPIC_API_KEY` | Claude/Anthropic provider | Yes | Never commit. |
| `OPENAI_API_KEY` | OpenAI provider | Yes | Never commit. |
| `FOUNDERAI_PROVIDER` | Runtime provider override | No | Examples: `ollama`, `claude`, `openai`. |
| `FOUNDERAI_BASE_URL` | Runtime provider base URL | No unless private | Examples: `http://localhost:11434`, `https://api.anthropic.com/v1`. |
| `FOUNDERAI_MODEL` | Runtime model override | No | Example: `qwen2.5:3b-instruct`. |
| `FOUNDERAI_TIMEOUT_SECONDS` | Runtime timeout override | No | Use longer timeouts for local models if needed. |
| `FOUNDERAI_SYSTEM_PROMPT` | Worker system prompt override | No, but treat carefully | Can alter agent behavior. |
| `FOUNDERAI_API_KEY_ENV` | Runtime API key env selector | No | Example: `ANTHROPIC_API_KEY`. |
| `CLOUDFLARE_TUNNEL_TOKEN` | Cloudflare tunnel | Yes | Required for cloudflared to stop restarting. |

## Checkout Rules

Product checkout can be called live only when:

- the public product page points to the correct checkout
- the payment path succeeds
- the buyer knows what happens after payment
- the delivery package exists
- the manual or automated delivery path is documented
- refund/support expectations are clear

Current checkout truth:

- Product 1: Payhip live.
- Products 2 to 10: checkout disabled unless founder approves a new activation after testing.

## Lead Capture Rules

Current default:

- Netlify forms capture leads.
- Hidden fields preserve language, source, campaign, CTA surface, interest type, and resource name.
- Thank-you page routes the visitor after submission.

Future Kit flow:

- Checklist forms should map to Kit forms and tags.
- General updates should map to the general OPLURIX updates form.
- Product interest should map to a product-interest form or tag.
- Do not delete Netlify fallback until Kit has been tested.

## FounderAI Provider Failure Rules

When a provider fails:

- Preserve `stdout.txt`, `stderr.txt`, `metadata.json`, and `output.md`.
- Do not hide low-credit, timeout, or unreachable-provider failures.
- Prefer offline/Ollama only when the model and network path are actually reachable.
- If a protected action is involved, keep approval gates intact even after retry.

## Manual Channels

Manual is not a failure if it is honest.

Manual channels currently include:

- email delivery
- PayPal support confirmation
- Mvola/Sendwave support coordination
- LinkedIn comments and DMs
- WhatsApp follow-up
- buyer feedback capture

When using manual channels, record:

- buyer/supporter name when appropriate
- email or contact path
- product or support route
- payment confirmation
- delivery status
- issue or feedback

