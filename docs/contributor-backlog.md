# Contributor Backlog

This backlog is now aligned to the V4 strategy documents and the current runtime shape.

## `good first issue`

### Add unit tests for Pio deadline parsing and alert generation

- Labels: `good first issue`, `help wanted`, `documentation`
- Goal: cover `config/pio_deadlines.json` parsing and due-alert behavior.

### Add README screenshots or artifact examples

- Labels: `good first issue`, `documentation`
- Goal: show a run folder, a grant artifact, and a status output example.

### Index the founder-brain reference files

- Labels: `good first issue`, `documentation`, `founder-voice`
- Goal: document what each founder-brain file contributes to the prompt packet.

## `help wanted`

### Verify Bartholomew on a real grant draft

- Labels: `help wanted`, `provider`
- Goal: run a bounded grant request and confirm the draft lands in `runtime/grants/`.

### Prove Zacchaeus and Perpetua end to end

- Labels: `help wanted`, `smoke-test`
- Goal: run inbound-lead and nurture-sequence smoke flows and capture the artifacts.

### Build the first Gmail polling bridge

- Labels: `help wanted`, `provider`, `devops`
- Goal: prototype Gmail API polling into inspectable inbox items without breaking the local-first workflow.

### Verify Docker deployment on a Linux host

- Labels: `help wanted`, `devops`, `linux`
- Goal: run the compose files on a real Linux machine and report any gaps.

## `maintainer-only`

### Review grant-governance checks

- Labels: `approvals`, `founder-voice`
- Goal: confirm the M350-first rule and Techni-Drones host logic stay enforced in prompts and QA.

### Plan the next cloud step without rewriting the app

- Labels: `devops`, `needs-triage`
- Goal: sequence Gmail, Docker, and deployment changes while preserving auditability.
