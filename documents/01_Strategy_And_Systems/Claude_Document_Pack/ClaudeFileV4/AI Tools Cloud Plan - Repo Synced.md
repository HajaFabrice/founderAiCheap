# AI Tools Cloud Plan

Repo-synced operational mirror of `AI_Tools_CloudPlan_v1.0.docx`.

Updated: 2026-04-22

## Direction

The cloud plan does not replace FounderAI. It sequences the cheapest viable path to portability while preserving approvals and inspectable runtime artifacts.

## Immediate Practical Order

1. Keep the Rust daemon reliable locally.
2. Add Gmail API polling as the first cloud-adjacent improvement.
3. Dockerize the daemon.
4. Move to a low-cost host such as Railway or Fly only after the runtime is trustworthy.
5. Add cloud persistence and a thin Vercel UI later.

## Tooling Implications

- Ollama remains the low-cost local draft engine.
- OpenAI stays available for stronger hosted outputs when needed.
- Gmail API is the most useful next integration.
- Supabase and Cloudflare are later-phase persistence and webhook layers, not day-one dependencies.

## Repo Implications

- Keep local-first behavior as the default.
- Keep file-auditable behavior unless a replacement is equally inspectable.
- Treat Docker and Gmail polling as the next concrete engineering steps.
