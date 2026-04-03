#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CONFIG_PATH="${1:-${REPO_ROOT}/config/founderai.json}"
MODE="${2:-daemon}"
BIN_PATH="${FOUNDERAI_BIN_PATH:-}"

if [[ -z "${BIN_PATH}" ]]; then
  if [[ -x "${REPO_ROOT}/target/release/founderai-ollama-rust" ]]; then
    BIN_PATH="${REPO_ROOT}/target/release/founderai-ollama-rust"
  elif [[ -x "${REPO_ROOT}/target/debug/founderai-ollama-rust" ]]; then
    BIN_PATH="${REPO_ROOT}/target/debug/founderai-ollama-rust"
  else
    echo "FounderAI binary not found. Build it first with: cargo build --release" >&2
    exit 1
  fi
fi

cd "${REPO_ROOT}"

if [[ "${MODE}" == "once" || "${MODE}" == "tick" ]]; then
  exec "${BIN_PATH}" tick --config "${CONFIG_PATH}"
fi

if [[ "${MODE}" == "foreground" ]]; then
  exec "${BIN_PATH}" daemon --config "${CONFIG_PATH}"
fi

nohup "${BIN_PATH}" daemon --config "${CONFIG_PATH}" > "${REPO_ROOT}/runtime/logs/founderai-daemon.out" 2>&1 &
echo "FounderAI background launcher started."
