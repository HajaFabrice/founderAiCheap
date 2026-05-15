# AI-Assisted Development Safety

Updated: 2026-05-11

Use this reference when a task touches code, architecture, CI, security,
runtime behavior, or self-improvement of the FounderAI system.

## Purpose

Protect the repo from vibecoding failure modes:

- insecure generated code
- changes nobody can explain
- fragile rewrites with shallow verification
- maintenance burden from inconsistent structure

## Core Rule

AI may help draft code.

AI may not replace human understanding, bounded verification, or ownership.

## Required Behaviors

- Explain the touched entrypoint and control flow before trusting the change.
- Prefer the smallest safe patch over ambitious rewrites.
- Review security boundaries explicitly:
  - input validation
  - file writes
  - approval bypass risk
  - secret handling
  - external requests
- Run the narrowest concrete verification commands available.
- State what was not verified instead of implying confidence.

## Block Conditions

Block or escalate if any of these are true:

- the contributor cannot explain the final behavior plainly
- a change touches approvals, provider auth, web request handling, or runtime
  writes without explicit boundary review
- generated code introduces new dependencies or abstractions with no clear need
- the patch mixes behavior change and broad churn without a reason
- the only justification is that the AI produced it quickly

## Repo-Specific Expectations

- Preserve approval gates.
- Preserve file-based auditability.
- Preserve founder-voice and governance constraints.
- Prefer inspectable artifacts over hidden automation.
- Treat `src/approvals.rs`, `src/app.rs`, `src/web.rs`, and provider config as
  high-scrutiny areas.

## Review Prompts For Clare And Columban

- Can I explain this code path clearly?
- Is there a simpler patch with less blast radius?
- What concrete failure would the current tests catch?
- Did we review every untrusted input boundary honestly?
- Would this still be maintainable if the model vanished tomorrow?
