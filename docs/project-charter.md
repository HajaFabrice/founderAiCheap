# Project Charter

## Problem Statement

FounderAI already works as a local Python and PowerShell system, but the durable version needs to keep the same auditable shape while supporting the current April 2026 reality: Techni-Drones Madagascar, grant pressure, ERIS concept development, and cloud portability.

## Vision

Deliver a portable FounderAI runtime that:

- preserves founder identity and Franciscan governance
- preserves the six core operating lanes
- adds workflow overlays for grant writing, deadline tracking, lead response, nurture, QA, review, and self-improvement
- keeps inbox, outbox, approvals, and runtime artifacts inspectable
- can run on Windows or Linux with Ollama-first and OpenAI-ready provider routing

## Scope

In scope:

- Rust CLI and daemon
- file-based runtime structure
- provider switching between Ollama and OpenAI
- Bartholomew grant drafting and Pio deadline tracking
- Windows launch scripts, Linux launch parity, Docker, and CI
- docs and contributor scaffolding that reflect the V4 strategy docs

Out of scope for now:

- replacing approvals with automation shortcuts
- rewriting the product into a generic agent platform
- full SaaS control-plane work
- database-first redesign before the local runtime is trustworthy

## Current Strategic Priorities

1. Protect survival-first operation.
2. Support the Techni-Drones conservation vertical.
3. Keep grant deadlines visible and actionable.
4. Preserve the ERIS long arc without drifting into hype.
5. Prepare cloud portability without breaking local auditability.

## Success Criteria

- A maintainer can run `status`, `tick`, and `daemon`.
- Deadline alerts become visible as inspectable inbox items.
- Grant drafts land in normal run artifacts plus `runtime/grants/`.
- Founder-brain prompts reflect the Techni-Drones and V4 grant context.
- Cloud migration remains an extension path, not a rewrite excuse.
