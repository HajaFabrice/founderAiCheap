# Contributing To FounderAI-Ollama-Rust

FounderAI-Ollama-Rust is a practical rebuild of the FounderAI autonomy layer.
The goal is not to invent a new product category. The goal is to preserve the
existing founder-brain behavior, approvals, inbox and outbox workflow, and
inspectable runtime artifacts while making the runtime portable and affordable.

## Before You Start

1. Read [README.md](README.md) for the product shape and local setup.
2. Read [docs/project-charter.md](docs/project-charter.md) for scope and non-goals.
3. Check open issues for `good first issue`, `help wanted`, `documentation`, or `devops`.
4. If your change touches approvals, founder voice, routing, or destructive actions, describe the risk clearly in the PR.

## Good First Contributions

- Improve onboarding docs or shell scripts
- Add tests around status, approvals, or config parsing
- Tighten Windows and Linux portability gaps
- Improve failure artifacts or log clarity
- Create small contributor-facing docs from existing behavior

## Workflow

1. Fork the repository or create a branch from `main`.
2. Keep changes scoped to one concern where practical.
3. Build locally with `cargo build --release`.
4. Run a quick smoke check with `status` or `tick` using a safe provider config.
5. Open a pull request with a concise summary, risk notes, and verification steps.

## Coding Expectations

- Preserve the existing FounderAI mental model.
- Prefer pragmatic code over speculative abstractions.
- Keep approval-sensitive actions gated.
- Preserve plain-file auditability for prompts, outputs, approvals, and logs.
- Keep Windows support intact while improving Linux portability.
- If you simplify something, preserve founder voice fidelity, approvals, team routing, and inspectable artifacts first.

## Documentation Expectations

- Update docs when behavior, commands, or deployment assumptions change.
- Prefer Markdown docs stored in-repo.
- Keep examples copy-pastable for Windows and Linux.

## Labels We Intend To Use

- `good first issue`: small, bounded, low-risk work
- `help wanted`: maintainer would welcome outside help
- `documentation`: docs-only or docs-heavy work
- `devops`: CI, Docker, deployment, or runtime environment work
- `founder-voice`: changes that affect tone, prompt packets, or identity fidelity
- `approvals`: changes that affect gating or protected actions

## Pull Request Checklist

- Explain the user-facing impact
- Note any approval or safety implications
- List commands you ran
- Mention anything not verified yet
- Link related issues where possible

## Communication

Use GitHub Issues for concrete tasks, bug reports, and proposals.
Use GitHub Discussions for open-ended questions, roadmap ideas, and onboarding help once Discussions is enabled.
