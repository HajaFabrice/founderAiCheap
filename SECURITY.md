# Security Policy

## Supported Focus

This project prioritizes:

- approval safety for protected actions
- auditability of runs, logs, and approvals
- safe fallback behavior when providers fail
- secure handling of provider credentials through environment variables

## Report A Vulnerability

Do not open a public issue for a vulnerability that could expose credentials,
weaken approval gates, or enable unsafe actions.

Instead, contact the maintainer privately and include:

- affected file or feature
- reproduction steps
- impact
- suggested mitigation if you have one

## High-Priority Security Areas

- approval queue bypasses
- protected action execution without review
- secret leakage into logs, prompt packets, or artifacts
- unsafe file writes outside the expected runtime structure
- remote provider request handling

## Hardening Expectations

- Keep API keys in environment variables, not committed config
- Keep `runtime/` out of git
- Keep founder-brain and config readable but controlled
- Preserve safe fallback artifacts when generation fails
