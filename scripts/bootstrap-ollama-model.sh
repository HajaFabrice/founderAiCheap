#!/usr/bin/env bash
set -euo pipefail

MODEL="${1:-qwen2.5:3b-instruct}"

if ! command -v ollama >/dev/null 2>&1; then
  echo "Ollama is not installed or not on PATH." >&2
  exit 1
fi

if ollama list | grep -Fq "${MODEL}"; then
  echo "Model already installed: ${MODEL}"
  exit 0
fi

echo "Pulling Ollama model: ${MODEL}"
ollama pull "${MODEL}"
