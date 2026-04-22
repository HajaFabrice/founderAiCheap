# Risk Register

| Risk | Why it matters | Mitigation | Signal |
| --- | --- | --- | --- |
| Grant deadline drift | Rufford and similar windows lose value if drafts slip silently | Pio tracks deadlines and surfaces alerts into `inbox/` | Deadline passes without an alert artifact |
| Equipment overreach | An M400-first narrative weakens fundability and credibility | Clare and grant prompts enforce the M350-first rule | Drafts mention the M400 as the first ask |
| Techni-Drones leverage stall | The conservation vertical and equity path can become vague talk | Keep explicit milestone reminders and Francis review notes | No written movement on the vertical or equity path |
| Approval regression | Unsafe actions could slip past review | Keep approvals explicit and test protected categories | Actions run without approval artifacts |
| Work-laptop dependence | The system remains fragile and hard to sustain | Preserve Docker, Linux deployment, and cloud migration docs | Runtime tied to one Windows machine |
| Docs drift | Strategy docs and repo behavior diverge | Maintain repo-synced mirrors and update README with runtime changes | Commands or roles mismatch between docs and code |
| Provider instability | Local and hosted inference may fail differently | Keep Ollama-first, OpenAI-ready routing and clear failure artifacts | Repeated provider failures with no clear fallback |
| Hidden cloud rewrite | Portability work mutates into a product redesign | Treat cloud work as an extension of the same daemon shape | New services appear without preserving file auditability |

## Review Cadence

- Review this register during the weekly strategy review when meaningful changes land.
- Re-rank risks after grant submissions, deployment changes, or major workflow additions.
- Open an issue for any new risk that touches safety, deadlines, or governance.
