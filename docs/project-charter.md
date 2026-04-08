# Project Charter

## Problem Statement

The original FounderAI system works in Python and PowerShell on Windows, but it
is tied too closely to a single machine and a single provider path. This project
rebuilds the autonomy layer in Rust so the system remains inspectable, portable,
and reliable while preserving the founder-brain identity and safety model.

## Vision

Deliver a portable FounderAI runtime that can run on Windows or Linux, talk to
either Ollama or OpenAI, and preserve the same founder-brain operating model:

- founder identity and Franciscan charter
- 3 teams and 6 roles
- inbox and outbox workflow
- approval queue for protected actions
- file-based runs, logs, and outputs

## Scope

In scope:

- Rust CLI and daemon
- file-based runtime structure
- provider switching between Ollama and OpenAI
- Windows launch scripts and Linux launch parity
- public docs, onboarding guides, and contributor scaffolding
- low-cost deployment guidance

Out of scope for now:

- multi-tenant SaaS hosting
- heavy web dashboard work
- speculative plugin frameworks
- replacing file artifacts with a database
- weakening approval gates for convenience
- rewriting founder voice into a generic assistant persona

## Objectives

1. Ship a reliable binary that can run `status`, `tick`, and `daemon`.
2. Preserve approvals, team routing, and inspectable artifacts.
3. Support both Ollama and OpenAI through config and environment variables.
4. Make the repo contributor-ready with docs, templates, and governance.
5. Keep deployment feasible on a small personal server or other low-cost host.

## Key Deliverables

- release-capable Rust application
- Docker and script-based deployment paths
- GitHub CI
- public documentation
- issue and PR templates
- governance, conduct, and security policies

## Success Criteria

- A maintainer can build the app from a clean checkout.
- A contributor can understand scope and non-goals from the repo.
- A deployer can choose Ollama or OpenAI without changing product behavior.
- FounderAI still pauses protected actions behind approval review.
