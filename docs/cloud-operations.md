# FounderAI Cloud Operations

This runbook keeps the cheap VPS deployment durable without adding expensive
managed infrastructure.

## Daily Backup

Run:

```bash
FOUNDERAI_CLOUD_ROOT=/srv/founderai \
FOUNDERAI_BACKUP_REMOTE="b2:founderai-backups" \
./scripts/cloud-backup.sh
```

What it archives:

- `runtime/`
- `config/`
- `founder-brain/`
- `docs/`
- `documents/99_Agent_Ready/`

Retention:

- `7` daily archives
- `4` weekly archives
- `3` monthly archives

If `rclone` and `FOUNDERAI_BACKUP_REMOTE` are configured, the same archive set
is mirrored offsite.

## Suggested Cron

Nightly backup at `02:15` UTC:

```cron
15 2 * * * cd /path/to/repo && FOUNDERAI_CLOUD_ROOT=/srv/founderai FOUNDERAI_BACKUP_REMOTE="b2:founderai-backups" ./scripts/cloud-backup.sh >> /srv/founderai/runtime/logs/cloud-backup.log 2>&1
```

## Weekly Review

Run:

```bash
FOUNDERAI_CLOUD_ROOT=/srv/founderai ./scripts/cloud-weekly-review.sh
```

It reports:

- server uptime
- disk usage for the FounderAI root
- pending approvals
- recorded runs
- average prompt size
- aggregate token counts found in run metadata

This is the cheapest way to keep cost awareness without adding a separate
monitoring subscription.

## Recovery Checklist

If the VPS is lost or corrupted:

1. Create a fresh VPS
2. Reinstall Docker, Docker Compose, and optional `rclone`
3. Clone the repo
4. Run `./scripts/bootstrap-cloud-layout.sh /srv/founderai`
5. Download the latest backup archive from local or Backblaze B2
6. Extract it back into `/srv/founderai`
7. Restore `.env.cloud`
8. Start the stack with `docker compose -f docker-compose.cloud.yml up -d --build`
9. Confirm:
   - `status` works
   - approvals are visible
   - founder-brain files are present
   - recent runs are still inspectable

## Operational Boundaries

- Keep the app behind Cloudflare Access at all times.
- Do not expose the web console directly with an open public port.
- Do not skip approvals for convenience in cloud mode.
- Keep employer-boundary rules unchanged from local mode.
