#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
CLOUD_ROOT="${1:-/srv/founderai}"

mkdir -p \
  "${CLOUD_ROOT}/backups" \
  "${CLOUD_ROOT}/config" \
  "${CLOUD_ROOT}/docs" \
  "${CLOUD_ROOT}/documents" \
  "${CLOUD_ROOT}/founder-brain" \
  "${CLOUD_ROOT}/inbox" \
  "${CLOUD_ROOT}/outbox" \
  "${CLOUD_ROOT}/runtime"

cp -a "${REPO_ROOT}/config/." "${CLOUD_ROOT}/config/"
cp -a "${REPO_ROOT}/founder-brain/." "${CLOUD_ROOT}/founder-brain/"
cp -a "${REPO_ROOT}/docs/." "${CLOUD_ROOT}/docs/"

mkdir -p "${CLOUD_ROOT}/documents/99_Agent_Ready"
cp -a "${REPO_ROOT}/documents/99_Agent_Ready/." "${CLOUD_ROOT}/documents/99_Agent_Ready/"

echo "FounderAI cloud layout prepared at ${CLOUD_ROOT}"
echo "Next steps:"
echo "1. Copy .env.cloud.example to .env.cloud and fill in ANTHROPIC_API_KEY (or OPENAI_API_KEY) plus CLOUDFLARE_TUNNEL_TOKEN."
echo "2. Review ${CLOUD_ROOT}/config/founderai.cloud.json."
echo "3. Run: docker compose -f docker-compose.cloud.yml up -d --build"
echo "4. Pull the default VPS-friendly Ollama model: docker exec founderai-ollama ollama pull qwen2.5:3b-instruct"
