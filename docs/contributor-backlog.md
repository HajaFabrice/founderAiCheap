# Contributor Backlog

This backlog is the contributor-facing slice of the no-budget rollout. It keeps
the next useful tasks visible without requiring a separate project management
tool.

The Windows and Linux smoke-test wrapper scripts are already included in this
milestone, so the remaining backlog focuses on validation, testing, docs, and
deployment hardening.

## `good first issue`

### Add Rust unit tests for config parsing and provider overrides

- Labels: `good first issue`, `help wanted`, `provider`
- Goal: add small, deterministic tests around config loading and env override precedence.

### Add README screenshots or artifact examples

- Labels: `good first issue`, `documentation`
- Goal: show the structure of a successful run folder, approval summary, and status output.

### Add a branchless onboarding guide for first-time contributors

- Labels: `good first issue`, `documentation`
- Goal: help first-time contributors submit small doc or script fixes without needing deep git knowledge.

### Review founder-brain folder structure and index the references

- Labels: `good first issue`, `documentation`, `founder-voice`
- Goal: add an inventory of the founder-brain reference files and what each one is for.

## `help wanted`

### Improve provider timeout logging and recovery hints

- Labels: `help wanted`, `provider`, `smoke-test`
- Goal: make slow Ollama generations easier to diagnose from `stderr.txt`, `stdout.txt`, and `output.md`.

### Verify Docker deployment on a Linux host

- Labels: `help wanted`, `devops`, `linux`
- Goal: run the compose files on a real Linux machine and report any gaps.

### Review and harden the isolated smoke-test wrapper scripts

- Labels: `help wanted`, `smoke-test`, `windows`, `linux`
- Goal: confirm the new wrapper scripts behave well on both platforms and improve the ergonomics if needed.

## `maintainer-only`

### Enable GitHub Pages from `main` / `docs`

- Labels: `devops`
- Goal: publish the docs tree without adding a separate docs stack.

### Enable GitHub Discussions and create the initial categories

- Labels: `needs-triage`
- Goal: use GitHub-native community workflow before adding any external chat tool.

### Apply minimal branch protection and security settings

- Labels: `needs-triage`
- Goal: require the build workflow on `main`, keep admin bypass, and enable security alerts.

### Create the `FounderAI No-Budget Board`

- Labels: `needs-triage`
- Goal: create the first GitHub Project board with the agreed workflow columns.
