#!/usr/bin/env bash
set -euo pipefail

CLOUD_ROOT="${FOUNDERAI_CLOUD_ROOT:-/srv/founderai}"
RUNTIME_DIR="${CLOUD_ROOT}/runtime"
RUNS_DIR="${RUNTIME_DIR}/runs"
APPROVALS_DIR="${RUNTIME_DIR}/approvals/pending"

prompt_stats() {
  local stat_name="$1"
  if [[ ! -d "${RUNS_DIR}" ]]; then
    echo "none"
    return 0
  fi

  (grep -hR "\"${stat_name}\"" "${RUNS_DIR}" --include metadata.json 2>/dev/null || true) \
    | awk -F: '{gsub(/[^0-9]/, "", $2); if ($2 != "") {sum += $2; count += 1}} END {if (count == 0) print "none"; else printf "%.0f (%d runs)\n", sum / count, count}'
}

token_total() {
  local stat_name="$1"
  if [[ ! -d "${RUNS_DIR}" ]]; then
    echo "0"
    return 0
  fi

  (grep -hR "\"${stat_name}\"" "${RUNS_DIR}" --include metadata.json 2>/dev/null || true) \
    | awk -F: '{gsub(/[^0-9]/, "", $2); if ($2 != "") sum += $2} END {printf "%.0f\n", sum + 0}'
}

run_count() {
  if [[ ! -d "${RUNS_DIR}" ]]; then
    echo "0"
    return 0
  fi
  find "${RUNS_DIR}" -name metadata.json | wc -l | tr -d ' '
}

pending_approvals() {
  if [[ ! -d "${APPROVALS_DIR}" ]]; then
    echo "0"
    return 0
  fi
  find "${APPROVALS_DIR}" -name '*.json' | wc -l | tr -d ' '
}

echo "FounderAI weekly ops review"
echo "Date (UTC): $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "Server uptime: $(uptime -p)"
echo "Disk use:"
df -h "${CLOUD_ROOT}"
echo
echo "Pending approvals: $(pending_approvals)"
echo "Recorded runs: $(run_count)"
echo "Average prompt chars: $(prompt_stats prompt_chars)"
echo "Average prompt words: $(prompt_stats prompt_words)"
echo "Total input tokens seen in run metadata: $(token_total input_tokens)"
echo "Total output tokens seen in run metadata: $(token_total output_tokens)"
