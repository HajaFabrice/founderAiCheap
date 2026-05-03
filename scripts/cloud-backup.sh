#!/usr/bin/env bash
set -euo pipefail

CLOUD_ROOT="${FOUNDERAI_CLOUD_ROOT:-/srv/founderai}"
BACKUP_ROOT="${FOUNDERAI_BACKUP_ROOT:-${CLOUD_ROOT}/backups}"
REMOTE_ROOT="${FOUNDERAI_BACKUP_REMOTE:-}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
ARCHIVE_NAME="founderai-${STAMP}.tar.gz"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

prune_local_dir() {
  local target_dir="$1"
  local keep_count="$2"
  mapfile -t archives < <(find "${target_dir}" -maxdepth 1 -type f -name 'founderai-*.tar.gz' | sort -r)
  if (( ${#archives[@]} <= keep_count )); then
    return 0
  fi
  for archive in "${archives[@]:keep_count}"; do
    rm -f "${archive}"
  done
}

prune_remote_dir() {
  local remote_dir="$1"
  local keep_count="$2"
  command -v rclone >/dev/null 2>&1 || return 0
  [[ -n "${REMOTE_ROOT}" ]] || return 0

  mapfile -t archives < <((rclone lsf --files-only "${REMOTE_ROOT%/}/${remote_dir}" 2>/dev/null || true) | grep '^founderai-.*\.tar\.gz$' || true)
  if (( ${#archives[@]} > 1 )); then
    mapfile -t archives < <(printf '%s\n' "${archives[@]}" | sort -r)
  fi
  if (( ${#archives[@]} <= keep_count )); then
    return 0
  fi
  for archive in "${archives[@]:keep_count}"; do
    rclone deletefile "${REMOTE_ROOT%/}/${remote_dir}/${archive}"
  done
}

mkdir -p "${BACKUP_ROOT}/daily" "${BACKUP_ROOT}/weekly" "${BACKUP_ROOT}/monthly"

tar -czf "${TMP_DIR}/${ARCHIVE_NAME}" \
  -C "${CLOUD_ROOT}" \
  runtime \
  config \
  founder-brain \
  docs \
  documents/99_Agent_Ready

cp "${TMP_DIR}/${ARCHIVE_NAME}" "${BACKUP_ROOT}/daily/${ARCHIVE_NAME}"

if [[ "$(date -u +%u)" == "7" ]]; then
  cp "${TMP_DIR}/${ARCHIVE_NAME}" "${BACKUP_ROOT}/weekly/${ARCHIVE_NAME}"
fi

if [[ "$(date -u +%d)" == "01" ]]; then
  cp "${TMP_DIR}/${ARCHIVE_NAME}" "${BACKUP_ROOT}/monthly/${ARCHIVE_NAME}"
fi

prune_local_dir "${BACKUP_ROOT}/daily" 7
prune_local_dir "${BACKUP_ROOT}/weekly" 4
prune_local_dir "${BACKUP_ROOT}/monthly" 3

if command -v rclone >/dev/null 2>&1 && [[ -n "${REMOTE_ROOT}" ]]; then
  rclone copyto "${BACKUP_ROOT}/daily/${ARCHIVE_NAME}" "${REMOTE_ROOT%/}/daily/${ARCHIVE_NAME}"
  if [[ -f "${BACKUP_ROOT}/weekly/${ARCHIVE_NAME}" ]]; then
    rclone copyto "${BACKUP_ROOT}/weekly/${ARCHIVE_NAME}" "${REMOTE_ROOT%/}/weekly/${ARCHIVE_NAME}"
  fi
  if [[ -f "${BACKUP_ROOT}/monthly/${ARCHIVE_NAME}" ]]; then
    rclone copyto "${BACKUP_ROOT}/monthly/${ARCHIVE_NAME}" "${REMOTE_ROOT%/}/monthly/${ARCHIVE_NAME}"
  fi

  prune_remote_dir "daily" 7
  prune_remote_dir "weekly" 4
  prune_remote_dir "monthly" 3
fi

echo "FounderAI backup created:"
echo "  ${BACKUP_ROOT}/daily/${ARCHIVE_NAME}"
if [[ -f "${BACKUP_ROOT}/weekly/${ARCHIVE_NAME}" ]]; then
  echo "  ${BACKUP_ROOT}/weekly/${ARCHIVE_NAME}"
fi
if [[ -f "${BACKUP_ROOT}/monthly/${ARCHIVE_NAME}" ]]; then
  echo "  ${BACKUP_ROOT}/monthly/${ARCHIVE_NAME}"
fi
if [[ -n "${REMOTE_ROOT}" ]]; then
  echo "Remote mirror: ${REMOTE_ROOT}"
fi
