#!/usr/bin/env bash
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CONFIG_PATH="${1:-${REPO_ROOT}/config/founderai.smoke.json}"
ROLE_ID="${2:-B-Production}"
TITLE="${3:-No-budget smoke test}"
BODY="${4:-Write a concise founder-quality brief for how contributors should help harden FounderAI-Ollama-Rust this week. Keep it practical, anti-hype, and survival-first.}"
BIN_PATH="${REPO_ROOT}/target/release/founderai-ollama-rust"

if [[ ! -x "${BIN_PATH}" ]]; then
  echo "FounderAI binary not found. Build it first with: cargo build --release" >&2
  exit 1
fi

mkdir -p \
  "${REPO_ROOT}/inbox-smoke" \
  "${REPO_ROOT}/outbox-smoke" \
  "${REPO_ROOT}/runtime-smoke" \
  "${REPO_ROOT}/runtime-smoke/runs" \
  "${REPO_ROOT}/runtime-smoke/archived-inbox"

cd "${REPO_ROOT}"

shopt -s nullglob
pending_items=("${REPO_ROOT}/inbox-smoke"/*)
shopt -u nullglob
if [[ ${#pending_items[@]} -gt 0 ]]; then
  archive_dir="${REPO_ROOT}/runtime-smoke/archived-inbox/$(date -u +%Y%m%dT%H%M%SZ)"
  mkdir -p "${archive_dir}"
  for item in "${pending_items[@]}"; do
    mv "${item}" "${archive_dir}/"
  done
fi

started_epoch="$(date +%s)"
effective_title="${TITLE} $(date -u +%Y%m%dT%H%M%SZ)"

echo "Creating smoke request..."
"${BIN_PATH}" request --config "${CONFIG_PATH}" --title "${effective_title}" --body "${BODY}" --role-id "${ROLE_ID}"
request_exit=$?
if [[ ${request_exit} -ne 0 ]]; then
  exit ${request_exit}
fi

echo "Running isolated smoke tick..."
"${BIN_PATH}" tick --config "${CONFIG_PATH}"
tick_exit=$?

latest_run="$(find "${REPO_ROOT}/runtime-smoke/runs" -mindepth 1 -maxdepth 1 -type d -printf '%T@ %p\n' 2>/dev/null | awk -v start="${started_epoch}" '$1 >= start { $1=""; sub(/^ /,""); print }' | tail -n 1)"

if [[ -z "${latest_run}" ]]; then
  latest_run="$(find "${REPO_ROOT}/runtime-smoke/runs" -mindepth 1 -maxdepth 1 -type d -printf '%T@ %p\n' 2>/dev/null | sort -nr | head -n 1 | cut -d' ' -f2-)"
fi

if [[ -n "${latest_run}" ]]; then
  echo
  echo "Latest run folder:"
  echo "${latest_run}"
  echo
  echo "Artifacts:"
  echo "${latest_run}/prompt.md"
  echo "${latest_run}/output.md"
  echo "${latest_run}/stdout.txt"
  echo "${latest_run}/stderr.txt"
else
  echo "No smoke run folder was found under runtime-smoke/runs." >&2
fi

exit ${tick_exit}
