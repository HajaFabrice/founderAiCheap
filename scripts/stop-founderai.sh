#!/usr/bin/env bash
set -euo pipefail

PROCESS_NAME="${1:-founderai-ollama-rust}"

if pkill -f "${PROCESS_NAME}" >/dev/null 2>&1; then
  echo "Stopped FounderAI processes matching ${PROCESS_NAME}."
else
  echo "No FounderAI processes matched."
fi
