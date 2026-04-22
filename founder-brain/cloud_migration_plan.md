# Cloud Migration Plan

This is a migration stub, not permission to redesign FounderAI.

## Core Rule

Extend the existing Rust daemon. Do not replace the product shape, approvals, inbox/outbox workflow, or file-auditable behavior unless there is an equally inspectable replacement.

## Immediate Sequence

1. Keep the local-first daemon trustworthy.
2. Add Gmail API polling before replacing the inbox model.
3. Dockerize the Rust daemon so it can run on Railway, Fly, or a similar low-cost host.
4. Move runtime persistence to cloud storage only after the local runtime remains reliable.
5. Keep the Vercel UI as a thin layer over the same workflows and approvals.

## Target Architecture

- Rust daemon: Railway or Fly container
- Local or cloud inference: Ollama locally, OpenAI or DeepSeek where quality matters
- Persistent data: Supabase or inspectable object storage later
- Webhooks: Cloudflare Workers later
- UI: Next.js on Vercel, thin layer only

## Cost Logic

- Day 0: local-first, near-zero budget
- Early cloud: one low-cost container and free storage tiers
- Paid inference only where quality difference matters

## Non-Negotiables

- No hidden control-plane rewrite
- No removal of approval gates during migration
- No cloud move that makes the audit trail harder to inspect
- No SaaS-first rebuild before Gmail polling and Docker are proven
