#!/usr/bin/env bash
set -euo pipefail

REPO="${1:-}"

resolve_repo() {
  local value="${1:-}"
  if [[ -n "${value}" ]]; then
    printf '%s' "${value}"
    return 0
  fi

  local remote
  remote="$(git config --get remote.origin.url)"
  if [[ "${remote}" =~ github\.com[:/]([^/]+/[^/.]+)(\.git)?$ ]]; then
    printf '%s' "${BASH_REMATCH[1]}"
    return 0
  fi

  echo "Could not resolve the GitHub repository from remote.origin.url." >&2
  return 1
}

if ! command -v gh >/dev/null 2>&1; then
  echo "GitHub CLI is not installed. Install it first, then run this script." >&2
  exit 1
fi

gh auth status >/dev/null
RESOLVED_REPO="$(resolve_repo "${REPO}")"

labels=(
  "good first issue|7057ff|Good entry point for a first contribution."
  "help wanted|008672|Maintainer would welcome outside help."
  "documentation|0e8a16|Docs, guides, onboarding, or public-facing explanations."
  "devops|5319e7|CI, deployment, infrastructure, or hosting work."
  "approvals|b60205|Touches approval gates or protected action review."
  "founder-voice|d93f0b|Touches founder identity, tone, or prompt fidelity."
  "provider|1d76db|Touches Ollama, OpenAI, or provider abstractions."
  "windows|0052cc|Windows-specific behavior or tooling."
  "linux|006b75|Linux-specific behavior or tooling."
  "smoke-test|fbca04|Smoke tests, validation flow, or artifact inspection."
  "needs-triage|c2e0c6|Needs maintainer review before it is categorized."
)

for row in "${labels[@]}"; do
  IFS='|' read -r name color description <<<"${row}"
  encoded_name="${name// /%20}"
  if gh api "repos/${RESOLVED_REPO}/labels/${encoded_name}" >/dev/null 2>&1; then
    gh api --method PATCH "repos/${RESOLVED_REPO}/labels/${encoded_name}" \
      -f new_name="${name}" \
      -f color="${color}" \
      -f description="${description}" >/dev/null
  else
    gh api --method POST "repos/${RESOLVED_REPO}/labels" \
      -f name="${name}" \
      -f color="${color}" \
      -f description="${description}" >/dev/null
  fi
done

existing_titles="$(gh issue list --repo "${RESOLVED_REPO}" --state all --limit 200 --json title --jq '.[].title')"

create_issue_if_missing() {
  local title="$1"
  local body="$2"
  shift 2
  if printf '%s\n' "${existing_titles}" | grep -Fxq "${title}"; then
    return 0
  fi

  local args=("issue" "create" "--repo" "${RESOLVED_REPO}" "--title" "${title}" "--body" "${body}")
  for label in "$@"; do
    args+=("--label" "${label}")
  done
  gh "${args[@]}" >/dev/null
}

create_issue_if_missing \
  "Add Rust unit tests for config parsing and provider overrides" \
  $'## Goal\n\nAdd small, deterministic tests around config parsing and environment override precedence.\n\n## Acceptance\n\n- Tests cover `config/founderai.example.json`\n- Tests cover environment overrides for provider, base URL, model, timeout, and API key env name\n- `cargo test --release` stays green\n' \
  "good first issue" "help wanted" "provider"

create_issue_if_missing \
  "Improve provider timeout logging and recovery hints" \
  $'## Goal\n\nMake slow or stalled provider generations easier to diagnose from the generated artifacts.\n\n## Acceptance\n\n- `stderr.txt` and `output.md` make timeout failures easier to understand\n- recovery guidance distinguishes slow Ollama from missing model and bad API key cases\n- the isolated smoke workflow remains unchanged\n' \
  "help wanted" "provider" "smoke-test"

create_issue_if_missing \
  "Add README screenshots or artifact examples" \
  $'## Goal\n\nAdd a few concrete examples so contributors can see what a healthy run looks like.\n\n## Acceptance\n\n- README includes one or more artifact examples or screenshots\n- examples do not leak secrets or private founder content\n- examples match the current Rust runtime behavior\n' \
  "good first issue" "documentation"

create_issue_if_missing \
  "Verify Docker deployment on a Linux host" \
  $'## Goal\n\nValidate the current Docker deployment story on a real Linux machine.\n\n## Acceptance\n\n- confirm the app builds and starts with the provided compose files\n- document any gaps or fixes needed for Ollama and OpenAI modes\n- record the exact commands and host assumptions\n' \
  "help wanted" "devops" "linux"

create_issue_if_missing \
  "Add a branchless onboarding guide for first-time contributors" \
  $'## Goal\n\nCreate a simple path for first-time contributors who want to make a small change without deep git knowledge.\n\n## Acceptance\n\n- guide is stored in `docs/`\n- it covers a docs-only or script-only first contribution path\n- it links back to the main contribution guide\n' \
  "good first issue" "documentation"

create_issue_if_missing \
  "Review founder-brain folder structure and index the references" \
  $'## Goal\n\nDocument the founder-brain reference structure so contributors understand the content without reshaping it.\n\n## Acceptance\n\n- add an index file that describes the main founder-brain reference groups\n- do not rewrite or relabel the founder-brain ideology itself\n- explain how the reference structure maps into prompt packets\n' \
  "good first issue" "documentation" "founder-voice"

echo "GitHub labels and starter issues have been synced for ${RESOLVED_REPO}."
echo "Finish the admin-only steps in docs/github-admin-checklist.md."
