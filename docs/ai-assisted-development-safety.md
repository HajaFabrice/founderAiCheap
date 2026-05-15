# AI-Assisted Development Safety

Updated: 2026-05-11

FounderAI can benefit from AI-assisted coding without accepting vibecoding
failure modes.

This document defines the minimum bar for any AI-assisted code change that
touches the repo.

## Core Rule

AI may accelerate drafting, exploration, and refactoring.

AI may not become the owner of the codebase.

Every merged change must remain:

- understandable by a human on the project
- reviewable without trusting the model
- testable with concrete commands
- bounded in scope and blast radius
- honest about what was and was not verified

## Main Risks We Are Mitigating

- security flaws from copied or weak public patterns
- black-box code that no one on the team can debug safely
- maintenance drag from structurally inconsistent changes
- skill atrophy and shallow understanding
- organizational dependency on code nobody can explain

## Required Workflow

1. Understand before editing.
   - Name the entrypoint, the data flow, and the touched invariants before
     changing behavior.
2. Keep changes bounded.
   - Prefer small patches over sweeping rewrites.
   - Do not combine refactors, feature work, and formatting churn unless there
     is a concrete reason.
3. Check security-sensitive boundaries.
   - Review input handling, file paths, approval bypass risk, secrets,
     serialization, and external calls.
4. Verify behavior concretely.
   - Run the narrowest useful test commands and record them in the PR.
   - If something was not verified, say so plainly.
5. Preserve ownership.
   - The contributor must be able to explain the final control flow and why the
     change is safe.

## Non-Negotiables For AI-Generated Code

- No merge of code the reviewer cannot explain at a high level.
- No merge of security-sensitive code without explicit boundary review.
- No merge of generated code that adds hidden infrastructure, unexplained
  dependencies, or unbounded indirection.
- No merge of changes that weaken approval gates, auditability, or founder
  governance.
- No claim that “the AI probably handled it.”

## Minimum Review Questions

- What exact behavior changed?
- Where does untrusted input enter?
- What prevents this from bypassing approvals or leaking secrets?
- What test would fail if this broke?
- Who on the team could debug this six weeks from now?

## Extra Care Areas In This Repo

- `src/approvals.rs`
- `src/app.rs`
- `src/web.rs`
- provider routing and environment loading
- runtime artifact writing
- any change that touches `founder-brain/` governance or QA files

## Escalation Rule

If a change is security-sensitive, structurally confusing, or hard to verify,
pause and reduce scope before proceeding.
