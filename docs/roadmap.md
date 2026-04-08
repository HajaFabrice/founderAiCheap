# Roadmap

This roadmap follows the current no-budget strategy: keep the product practical,
portable, and recruitable before adding new complexity.

## Timeline

```mermaid
gantt
    title FounderAI-Ollama-Rust No-Budget Roadmap
    dateFormat  YYYY-MM-DD
    axisFormat  %b %d

    section Foundation
    Rust runtime parity             :done, foundation1, 2026-03-28, 2026-04-06
    Repo governance and docs        :active, foundation2, 2026-04-07, 2026-04-14

    section Deployment
    Cloud deployment hardening      :deploy1, 2026-04-10, 2026-04-24
    First real provider smoke test  :crit, deploy2, 2026-04-12, 2026-04-20

    section Community
    Volunteer onboarding flow       :community1, 2026-04-14, 2026-04-28
    Issue triage and labels         :community2, 2026-04-14, 2026-04-21

    section Product
    Reliability fixes and tests     :product1, 2026-04-15, 2026-05-15
    Founder-brain content expansion :product2, 2026-04-20, 2026-05-20

    section Sustainability
    Linux host deployment playbook  :sustain1, 2026-04-18, 2026-04-30
    Quarterly health review         :sustain2, 2026-07-01, 2026-07-07
```

## Near-Term Milestones

- `v0.1`: working Rust runtime with provider switching and safe artifacts
- `v0.2`: deployment hardening, smoke-test proof, and onboarding docs
- `v0.3`: contributor-friendly issue flow, more tests, and stable cloud operation

## Maintainer Notes

- Prefer issues tagged `good first issue` and `documentation` for new contributors.
- Keep roadmap changes tied to the project charter so scope does not drift.
