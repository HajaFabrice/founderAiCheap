# Risk Register

| Risk | Why it matters | Mitigation | Signal |
| --- | --- | --- | --- |
| Single maintainer bottleneck | Progress stalls if one person is overloaded | Document setup, use issue templates, keep onboarding simple | Stale PRs and unanswered issues |
| Model availability | Ollama smoke tests fail if the model is missing on the host | Use bootstrap scripts and document provider fallback | `status` shows provider reachable but no installed model |
| Scope creep | The repo drifts into a generic agent platform | Hold changes against the charter and non-goals | Large proposals that bypass founder shape |
| Approval regression | Unsafe actions could slip past review | Keep approvals explicit and test protected categories | Actions run without approval artifacts |
| Work-laptop dependence | Project becomes hard to sustain or move | Prefer Linux VM or personal host deployment | Runtime tied to one Windows machine |
| Docs drift | Contributors cannot tell what is current | Update docs with behavior changes and review README in PRs | Commands in docs no longer match code |
| Provider lock-in | Costs or runtime constraints limit adoption | Keep both Ollama and OpenAI paths healthy | Only one provider path actually works |
| Hidden operational cost | The stack becomes too expensive or complex | Prefer static docs, GitHub Actions, small hosts, and plain files | New services appear without clear payoff |

## Review Cadence

- Review this register at least once per quarter
- Re-rank risks after any deployment or provider strategy change
- Open an issue for new risks that affect safety, cost, or portability
