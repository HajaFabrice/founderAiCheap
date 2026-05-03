# FounderAI Cloud Deployment

This is the cheapest reliable cloud path for FounderAI without changing the
product shape:

- one small x86 VPS
- one private browser console
- OpenAI API-first inference
- Cloudflare Tunnel plus Cloudflare Access
- the same file-auditable inbox, outbox, runtime, and approval folders

## Target Stack

- Host: Hetzner Cloud `CX23` class VPS or equivalent
- OS: Ubuntu 24.04 LTS
- Runtime: Docker Compose
- Inference: OpenAI API
- Private ingress: Cloudflare Tunnel
- Private login gate: Cloudflare Access
- Backups: local compressed archives plus optional Backblaze B2 mirror through `rclone`

## Why This Shape

- It preserves the current daemon and file artifacts instead of forcing a web-app rewrite.
- It keeps founder approvals inspectable and reversible.
- It avoids the cost of a GPU or cloud Ollama in phase 1.
- It gives you browser access without exposing FounderAI to the public internet.

## Files Added For Cloud

- `config/founderai.cloud.json`
- `.env.cloud.example`
- `docker-compose.cloud.yml`
- `scripts/bootstrap-cloud-layout.sh`
- `scripts/cloud-backup.sh`
- `scripts/cloud-weekly-review.sh`

## What The Web Console Supports

The `serve` subcommand exposes a private founder console with the same runtime
state the CLI uses:

- `GET /healthz`
- `GET /status`
- `GET /approvals`
- `POST /approvals/:id/approve`
- `POST /approvals/:id/reject`
- `POST /requests`
- `GET /runs`
- `GET /runs/:run_id`
- `GET /runs/:run_id/view`

Authentication is intentionally not inside the app. Put the whole surface
behind Cloudflare Access.

## Deployment Steps

### 1. Prepare the VPS

Install:

- Docker Engine
- Docker Compose plugin
- optional `rclone` for Backblaze B2 uploads

Then clone this repo onto the VPS.

### 2. Create the cloud runtime layout

From the repo root:

```bash
./scripts/bootstrap-cloud-layout.sh /srv/founderai
```

That creates:

- `/srv/founderai/inbox`
- `/srv/founderai/outbox`
- `/srv/founderai/runtime`
- `/srv/founderai/config`
- `/srv/founderai/founder-brain`
- `/srv/founderai/docs`
- `/srv/founderai/documents/99_Agent_Ready`
- `/srv/founderai/backups`

### 3. Fill cloud environment variables

Copy the example file:

```bash
cp .env.cloud.example .env.cloud
```

Then set at minimum:

- `OPENAI_API_KEY`
- `CLOUDFLARE_TUNNEL_TOKEN`

Keep the default OpenAI routing unless you have a cheaper current mini model you
know is supported in your account.

### 4. Review the cloud config

The cloud config is at:

- `config/founderai.cloud.json`

Key differences from local mode:

- provider default is OpenAI
- task routing is online-only
- runtime paths are absolute under `/srv/founderai`
- approval rules stay intact
- no Ollama dependency is assumed in phase 1

### 5. Start the stack

```bash
docker compose -f docker-compose.cloud.yml up -d --build
```

This starts:

- `founderai-daemon`
- `founderai-web`
- `cloudflared`

No public app port is opened. `cloudflared` makes the outbound tunnel.

### 6. Configure Cloudflare Access

In Cloudflare Zero Trust:

1. Create a tunnel for FounderAI
2. Point it at `http://founderai-web:8080`
3. Put the hostname behind an Access application
4. Limit access to your founder email identity only

This keeps the app private while still reachable from a browser.

### 7. Verify the rollout

Local checks on the VPS:

```bash
docker compose -f docker-compose.cloud.yml ps
docker compose -f docker-compose.cloud.yml logs founderai-daemon --tail=100
docker compose -f docker-compose.cloud.yml logs founderai-web --tail=100
docker compose -f docker-compose.cloud.yml logs cloudflared --tail=100
```

App checks:

```bash
docker exec founderai-daemon /app/founderai-ollama-rust status --config /srv/founderai/config/founderai.cloud.json
```

Browser checks after Cloudflare Access login:

- load `/`
- confirm status is visible
- create a request
- confirm it lands in `/srv/founderai/inbox`
- approve or reject an item
- open a run detail page

## Expected Cost Shape

Fixed infra target:

- VPS: about `$4-$5/month`
- VPS backup add-on: about `$1/month`
- Cloudflare Tunnel and Access: free at this scale
- Offsite backup storage: near zero at small archive sizes

That keeps fixed cost under the intended `$15/month` cap.

## Important Notes

- This phase is private, not public SaaS.
- The app still depends on founder review for sensitive actions.
- If OpenAI spend grows faster than expected, first reduce prompt size and
  tighten routing before considering new infrastructure.
