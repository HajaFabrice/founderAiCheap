# Governance

FounderAI-Ollama-Rust uses a lightweight maintainer model.

## Roles

### Founder Steward

Responsible for the founder-brain identity, safety boundaries, approvals policy,
and strategic direction.

### Maintainers

Responsible for reviewing contributions, protecting release quality, and keeping
Windows and Linux operation healthy.

### Contributors

Anyone who improves code, docs, scripts, testing, or deployment guidance.

## Decision Rules

- Routine fixes can be merged after one maintainer review.
- Changes to approval gating, destructive actions, provider defaults, or founder voice should receive explicit review from the Founder Steward or a designated maintainer.
- If a proposal expands scope, maintainers should compare it against [docs/project-charter.md](docs/project-charter.md) before accepting it.

## Release Principles

- Reliability beats novelty
- Inspectable artifacts beat hidden automation
- Safety beats convenience
- Portable, low-cost operation beats heavy platform dependence

## When We Say No

Maintainers may reject changes that:

- Redesign the product away from the reference FounderAI shape
- Weaken approvals for protected categories
- Hide prompt packets, outputs, or audit artifacts
- Introduce unnecessary infrastructure cost or complexity

## Succession

If the project gains multiple active maintainers, they should document:

- release process
- branch protections
- label conventions
- deployment ownership
- founder-brain stewardship rules
