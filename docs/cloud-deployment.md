# FounderAI Cloud Deployment

Run FounderAI 24/7, independent of your computer. This guide covers
everything from renting a VPS to the agents running while you sleep.

**Cost target:** under $10/month (VPS + API calls).
**Time to deploy:** 30–45 minutes on a fresh VPS.

---

## What You Need Before Starting

1. **A VPS** — Hetzner, DigitalOcean, or Vultr. Minimum: 2 vCPU, 4 GB RAM,
   20 GB disk. Ubuntu 24.04 LTS. Recommended: Hetzner CX23 (~€4/month).
   Payment: PayPal works on all three.
2. **An API key** — Either Anthropic (`ANTHROPIC_API_KEY`) or OpenAI
   (`OPENAI_API_KEY`). Anthropic: console.anthropic.com. OpenAI: platform.openai.com.
3. **A Cloudflare account** — Free. Needed to access the web console securely
   from a browser without exposing the server to the public internet.
4. **A domain or subdomain pointed to Cloudflare** — Can be a free subdomain
   or your own domain. Only needed if you want browser access to the console.
   The daemon runs fine without it.

---

## Step 1 — Rent and Connect to the VPS

On Hetzner Cloud (hetzner.com/cloud):
- Create project → Add server → Location: Helsinki or Nuremberg
- Image: Ubuntu 24.04
- Type: CX23 (2 vCPU, 4 GB RAM)
- SSH key: paste your public key, or use the Hetzner web console

Connect:
```bash
ssh root@<your-vps-ip>
```

---

## Step 2 — Install Docker

```bash
apt-get update && apt-get install -y ca-certificates curl git
install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg \
  -o /etc/apt/keyrings/docker.asc
chmod a+r /etc/apt/keyrings/docker.asc
echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] \
  https://download.docker.com/linux/ubuntu $(. /etc/os-release && echo "$VERSION_CODENAME") stable" \
  > /etc/apt/sources.list.d/docker.list
apt-get update
apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

Verify:
```bash
docker --version
docker compose version
```

---

## Step 3 — Clone the Repository

```bash
git clone https://github.com/<your-username>/founderAiCheap.git /opt/founderai
cd /opt/founderai
```

If the repo is private, use a personal access token:
```bash
git clone https://<token>@github.com/<your-username>/founderAiCheap.git /opt/founderai
```

---

## Step 4 — Create the Runtime Layout

```bash
cd /opt/founderai
./scripts/bootstrap-cloud-layout.sh /srv/founderai
```

This creates the file structure the daemon reads and writes:
```
/srv/founderai/
├── config/          ← founderai.cloud.json (copied from repo)
├── founder-brain/   ← identity and prompts (copied from repo)
├── documents/       ← agent-ready references (copied from repo)
├── inbox/           ← drop .json files here to give agents work
├── outbox/          ← completed outputs appear here
├── runtime/         ← runs, approvals, state, logs
└── backups/         ← automatic daily/weekly/monthly archives
```

---

## Step 5 — Configure the Environment

```bash
cd /opt/founderai
cp .env.cloud.example .env.cloud
nano .env.cloud
```

**Minimum required changes:**

For Anthropic Claude (recommended):
```
FOUNDERAI_PROVIDER=claude
FOUNDERAI_BASE_URL=https://api.anthropic.com/v1
FOUNDERAI_MODEL=claude-sonnet-4-20250514
FOUNDERAI_API_KEY_ENV=ANTHROPIC_API_KEY
ANTHROPIC_API_KEY=sk-ant-...your-key-here...
CLOUDFLARE_TUNNEL_TOKEN=...fill-in-step-6...
```

For OpenAI:
```
FOUNDERAI_PROVIDER=openai
FOUNDERAI_BASE_URL=https://api.openai.com/v1
FOUNDERAI_MODEL=gpt-5-mini
FOUNDERAI_API_KEY_ENV=OPENAI_API_KEY
OPENAI_API_KEY=sk-...your-key-here...
CLOUDFLARE_TUNNEL_TOKEN=...fill-in-step-6...
```

Set permissions so only root can read it:
```bash
chmod 600 .env.cloud
```

---

## Step 6 — Create the Cloudflare Tunnel

> Skip this step if you only want the daemon to run (no browser console).
> The daemon works without Cloudflare. Cloudflare is only for the web console.

1. Log in at **dash.cloudflare.com** → Zero Trust → Networks → Tunnels
2. Click **Create a tunnel** → name it `founderai`
3. Choose **Docker** as the connector
4. Cloudflare shows a command like:
   ```
   docker run cloudflare/cloudflared:latest tunnel --no-autoupdate run --token eyJh...
   ```
   Copy only the token value (the long string after `--token`)
5. Paste it into `.env.cloud`:
   ```
   CLOUDFLARE_TUNNEL_TOKEN=eyJh...
   ```
6. Back in Cloudflare, add a **Public Hostname**:
   - Subdomain: `founderai` (or anything you want)
   - Domain: your domain (or use a free `*.pages.dev` subdomain)
   - Service: `http://founderai-web:8080`
7. Click **Save tunnel**
8. Go to **Access → Applications → Add an application**
   - Type: Self-hosted
   - App domain: same subdomain as above
   - Policy: add your email address as the only allowed identity

---

## Step 7 — Start the Stack

```bash
cd /opt/founderai
docker compose -f docker-compose.cloud.yml up -d --build
```

First start builds the Rust binary inside Docker. Takes 3–5 minutes.

Watch the logs:
```bash
docker compose -f docker-compose.cloud.yml logs -f founderai-daemon
```

You should see the daemon start polling. Check status:
```bash
docker exec founderai-daemon \
  /app/founderai-ollama-rust status \
  --config /srv/founderai/config/founderai.cloud.json
```

---

## Step 8 — Verify Everything Works

**Container health:**
```bash
docker compose -f docker-compose.cloud.yml ps
```
All three services (`founderai-daemon`, `founderai-web`, `cloudflared`) should
show `healthy` or `running`.

**Daemon tick:**
```bash
docker exec founderai-daemon \
  /app/founderai-ollama-rust tick \
  --config /srv/founderai/config/founderai.cloud.json
```

**Browser console** (after Cloudflare Access login):
- Navigate to your tunnel hostname
- You should see the status dashboard
- Runs appear at `/runs`
- Pending approvals appear at `/approvals`

**Give the agents their first task manually:**
```bash
docker exec founderai-daemon \
  /app/founderai-ollama-rust request \
  --config /srv/founderai/config/founderai.cloud.json \
  --role a-outreach \
  --body "Draft the first OPLURIX LinkedIn post using the Expert-to-Influencer framework."
```

The output appears in `/srv/founderai/runtime/runs/` within 60 seconds.

---

## Step 9 — Set Up Automatic Backups

```bash
crontab -e
```

Add:
```
# Daily backup at 02:15
15 2 * * * cd /opt/founderai && FOUNDERAI_CLOUD_ROOT=/srv/founderai ./scripts/cloud-backup.sh >> /srv/founderai/runtime/logs/backup.log 2>&1
```

---

## Keeping the Daemon Updated

When you push changes to the repo (new prompts, config, agent updates):

```bash
ssh root@<your-vps-ip>
cd /opt/founderai
git pull
./scripts/bootstrap-cloud-layout.sh /srv/founderai   # syncs config and brain
docker compose -f docker-compose.cloud.yml up -d --build --no-deps founderai-daemon founderai-web
```

---

## Day-to-Day Operations

**See what the agents produced overnight:**
```bash
docker exec founderai-daemon \
  /app/founderai-ollama-rust status \
  --config /srv/founderai/config/founderai.cloud.json
```

**Review and approve a pending action:**
```bash
docker exec founderai-daemon \
  /app/founderai-ollama-rust approvals \
  --config /srv/founderai/config/founderai.cloud.json

docker exec founderai-daemon \
  /app/founderai-ollama-rust approve <approval-id> \
  --config /srv/founderai/config/founderai.cloud.json
```

**Drop a task into the inbox from your laptop:**

Create a file like `inbox/task-001.json`:
```json
{
  "role": "bartholomew",
  "body": "Draft the Rufford Small Grant application introduction section."
}
```
Then `scp` it to the VPS inbox:
```bash
scp task-001.json root@<vps-ip>:/srv/founderai/inbox/
```
The daemon picks it up within 60 seconds.

**Stop the stack:**
```bash
docker compose -f docker-compose.cloud.yml down
```

**Restart after a crash:**
```bash
docker compose -f docker-compose.cloud.yml up -d
```
(No `--build` needed unless code changed.)

---

## Cost Control

- **API spend**: every run writes `metadata.json` with token counts. Check:
  ```bash
  grep -r '"usage"' /srv/founderai/runtime/runs/*/metadata.json | tail -20
  ```
- **Reduce spend**: lower `poll_interval_seconds` in `founderai.cloud.json`,
  disable non-urgent jobs, or switch high-frequency tasks to a smaller model.
- **Hard stop**: `docker compose -f docker-compose.cloud.yml stop founderai-daemon`
  pauses the daemon without losing state.

---

## Recovery (If the VPS Is Lost)

1. Spin up a new VPS with the same setup (Steps 1–2)
2. Clone the repo (Step 3)
3. Download the latest backup from Backblaze B2 (if configured) or copy
   from a local backup
4. Extract into `/srv/founderai`: `tar -xzf backup.tar.gz -C /srv/founderai`
5. Re-create `.env.cloud` (not in backup — keep a copy offline)
6. `docker compose -f docker-compose.cloud.yml up -d --build`

---

## Important Notes

- The daemon still requires founder approval for any action tagged
  `external-send`, `financial`, `publish`, or `calendar-commitment`.
- Approve or reject via the browser console or SSH — the agents queue work
  but cannot bypass approval rules.
- The `.env.cloud` file contains your API key. Never commit it to git.
  Keep a copy in a password manager.
