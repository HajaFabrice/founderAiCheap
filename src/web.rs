use crate::app::{ApprovalSnapshot, AutonomyApp, RunDetailSnapshot, RunListEntry, StatusSnapshot};
use anyhow::{Context, Result};
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;

#[derive(Clone)]
struct WebState {
    app: Arc<AutonomyApp>,
}

#[derive(Debug, Deserialize, Default)]
struct StatusQuery {
    teams: Option<bool>,
}

#[derive(Debug, Deserialize, Default)]
struct RequestFormPayload {
    title: String,
    body: String,
    approval_policy: Option<String>,
    risk_tags: Option<String>,
    requires_internet: Option<String>,
    role_id: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct DecisionFormPayload {
    notes: Option<String>,
}

fn internal_error(err: anyhow::Error) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn escape_html(raw: &str) -> String {
    let mut escaped = String::with_capacity(raw.len());
    for character in raw.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn render_pending_approvals(approvals: &[ApprovalSnapshot]) -> String {
    if approvals.is_empty() {
        return "<p>No pending approvals.</p>".to_string();
    }

    approvals
        .iter()
        .map(|approval| {
            let artifacts = if approval.artifacts.is_empty() {
                "<li>None</li>".to_string()
            } else {
                approval
                    .artifacts
                    .iter()
                    .map(|artifact| format!("<li><code>{}</code></li>", escape_html(artifact)))
                    .collect::<Vec<_>>()
                    .join("")
            };
            let risk_tags = if approval.risk_tags.is_empty() {
                "none".to_string()
            } else {
                approval.risk_tags.join(", ")
            };

            format!(
                r#"<article class="card">
<h3>{approval_id}</h3>
<p><strong>Job:</strong> {job_id}<br><strong>Phase:</strong> {phase}<br><strong>Risk tags:</strong> {risk_tags}</p>
<p>{summary}</p>
<details>
  <summary>Artifacts</summary>
  <ul>{artifacts}</ul>
  <p><strong>Summary file:</strong> <code>{summary_path}</code></p>
</details>
<form method="post" class="approval-form">
  <label>Notes</label>
  <textarea name="notes" rows="3" placeholder="Optional founder note"></textarea>
  <div class="button-row">
    <button type="submit" formaction="/approvals/{approval_id}/approve">Approve</button>
    <button type="submit" formaction="/approvals/{approval_id}/reject" class="danger">Reject</button>
  </div>
</form>
</article>"#,
                approval_id = escape_html(&approval.approval_id),
                job_id = escape_html(&approval.job_id),
                phase = escape_html(&approval.phase),
                risk_tags = escape_html(&risk_tags),
                summary = escape_html(&approval.summary),
                artifacts = artifacts,
                summary_path = escape_html(&approval.summary_path),
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_recent_runs(runs: &[RunListEntry]) -> String {
    if runs.is_empty() {
        return "<p>No runs recorded yet.</p>".to_string();
    }

    runs.iter()
        .map(|run| {
            let status_class = if run.exit_code == Some(0) { "ok" } else { "warn" };
            format!(
                r#"<article class="card compact">
<h3><a href="/runs/{run_id}/view">{run_id}</a></h3>
<p><strong>Job:</strong> {job_id}<br><strong>Provider:</strong> {provider}<br><strong>Model:</strong> {model}<br><strong>Exit:</strong> <span class="{status_class}">{exit_code}</span></p>
<p><strong>Started:</strong> {started_at}<br><strong>Finished:</strong> {finished_at}</p>
<p><a href="/runs/{run_id}">Raw JSON</a></p>
</article>"#,
                run_id = escape_html(&run.run_id),
                job_id = escape_html(run.job_id.as_deref().unwrap_or("unknown")),
                provider = escape_html(run.provider.as_deref().unwrap_or("unknown")),
                model = escape_html(run.model.as_deref().unwrap_or("unknown")),
                exit_code = run
                    .exit_code
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                started_at = escape_html(run.started_at.as_deref().unwrap_or("unknown")),
                finished_at = escape_html(run.finished_at.as_deref().unwrap_or("unknown")),
                status_class = status_class,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_status(snapshot: &StatusSnapshot) -> String {
    let provider_detail = snapshot
        .provider_status
        .detail
        .as_deref()
        .unwrap_or("none");
    let jobs = snapshot
        .jobs
        .iter()
        .map(|job| {
            format!(
                "<li><strong>{}</strong> — last_run={} exit={} pending_approval={}</li>",
                escape_html(&job.job_id),
                escape_html(job.last_run_id.as_deref().unwrap_or("none")),
                job.last_exit_code
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "none".to_string()),
                escape_html(job.pending_approval_id.as_deref().unwrap_or("none")),
            )
        })
        .collect::<Vec<_>>()
        .join("");
    let roles = snapshot
        .roles
        .as_ref()
        .map(|roles| {
            roles
                .iter()
                .map(|role| {
                    format!(
                        "<li><strong>{}</strong> — last_job={} status={} metric={}</li>",
                        escape_html(&role.role_id),
                        escape_html(role.last_job_id.as_deref().unwrap_or("unknown")),
                        escape_html(role.last_status.as_deref().unwrap_or("unknown")),
                        role.last_metric_value
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "none".to_string())
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default();

    format!(
        r#"<article class="card">
<h2>Runtime Status</h2>
<p><strong>Workspace:</strong> <code>{workspace}</code><br>
<strong>Runtime:</strong> <code>{runtime}</code><br>
<strong>Provider:</strong> {provider}<br>
<strong>Model:</strong> {model}<br>
<strong>Internet:</strong> {internet}<br>
<strong>Pending approvals:</strong> {pending_approvals}<br>
<strong>Offline queue:</strong> {offline_queue}<br>
<strong>Nurture sequences:</strong> {nurture}<br>
<strong>Deadline alerts:</strong> {deadline_alerts}<br>
<strong>Provider detail:</strong> {provider_detail}</p>
<details open>
  <summary>Jobs</summary>
  <ul>{jobs}</ul>
</details>
<details>
  <summary>Teams</summary>
  <ul>{roles}</ul>
</details>
</article>"#,
        workspace = escape_html(&snapshot.workspace_root),
        runtime = escape_html(&snapshot.runtime_dir),
        provider = escape_html(&snapshot.active_provider),
        model = escape_html(&snapshot.active_model),
        internet = if snapshot.internet_available {
            "available"
        } else {
            "unavailable"
        },
        pending_approvals = snapshot.pending_approvals,
        offline_queue = snapshot.offline_queue_pending,
        nurture = snapshot.active_nurture_sequences,
        deadline_alerts = snapshot.pending_deadline_alerts,
        provider_detail = escape_html(provider_detail),
        jobs = jobs,
        roles = roles,
    )
}

fn render_dashboard(snapshot: &StatusSnapshot, approvals: &[ApprovalSnapshot], runs: &[RunListEntry], app: &AutonomyApp) -> String {
    let role_options = app
        .config
        .team_roles
        .values()
        .map(|role| {
            format!(
                r#"<option value="{role_id}">{role_id} — {display_name}</option>"#,
                role_id = escape_html(&role.role_id),
                display_name = escape_html(&role.display_name)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>FounderAI Private Console</title>
  <style>
    :root {{
      color-scheme: light;
      --bg: #f4efe5;
      --ink: #1e2a27;
      --muted: #5b6864;
      --panel: #fffdf9;
      --border: #d6c7af;
      --accent: #295c52;
      --danger: #8f3c2e;
    }}
    body {{ margin: 0; font-family: Georgia, "Times New Roman", serif; background: linear-gradient(160deg, #efe5cf, var(--bg)); color: var(--ink); }}
    main {{ max-width: 1200px; margin: 0 auto; padding: 24px; }}
    h1, h2, h3 {{ margin-top: 0; }}
    p, li, label, summary, input, textarea, select, button {{ font-size: 0.98rem; }}
    .grid {{ display: grid; gap: 20px; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); }}
    .card {{ background: var(--panel); border: 1px solid var(--border); border-radius: 18px; padding: 18px; box-shadow: 0 10px 25px rgba(41, 92, 82, 0.08); }}
    .compact {{ padding-bottom: 10px; }}
    .subtle {{ color: var(--muted); }}
    .button-row {{ display: flex; gap: 10px; flex-wrap: wrap; margin-top: 10px; }}
    button {{ background: var(--accent); color: #fff; border: 0; border-radius: 999px; padding: 10px 16px; cursor: pointer; }}
    button.danger {{ background: var(--danger); }}
    input, textarea, select {{ width: 100%; padding: 10px; border-radius: 12px; border: 1px solid var(--border); background: #fff; box-sizing: border-box; margin-top: 6px; margin-bottom: 12px; }}
    textarea {{ min-height: 120px; resize: vertical; }}
    code, pre {{ font-family: "SFMono-Regular", Consolas, monospace; }}
    .ok {{ color: #1f6b37; }}
    .warn {{ color: var(--danger); }}
    a {{ color: var(--accent); }}
  </style>
</head>
<body>
<main>
  <header class="card">
    <h1>FounderAI Private Console</h1>
    <p class="subtle">Founder-only browser control over the same inbox, approvals, and run artifacts used by the CLI daemon.</p>
    <p><a href="/status?teams=true">Status JSON</a> · <a href="/approvals">Approvals JSON</a> · <a href="/runs">Runs JSON</a> · <a href="/healthz">Health check</a></p>
  </header>

  <section class="grid">
    {status_panel}
    <article class="card">
      <h2>Create Inbox Request</h2>
      <form method="post" action="/requests">
        <label>Title</label>
        <input type="text" name="title" required placeholder="Draft follow-up for ...">
        <label>Role lane</label>
        <select name="role_id">
          <option value="">General founder request</option>
          {role_options}
        </select>
        <label>Approval policy</label>
        <select name="approval_policy">
          <option value="inherit">inherit</option>
          <option value="never">never</option>
          <option value="before_run">before_run</option>
          <option value="after_run">after_run</option>
        </select>
        <label>Risk tags (comma separated)</label>
        <input type="text" name="risk_tags" placeholder="external-send, publish">
        <label><input type="checkbox" name="requires_internet" value="true"> Requires internet</label>
        <label>Body</label>
        <textarea name="body" required placeholder="Describe the bounded work FounderAI should prepare."></textarea>
        <button type="submit">Create request</button>
      </form>
    </article>
  </section>

  <section>
    <h2>Pending Approvals</h2>
    <div class="grid">{approvals_panel}</div>
  </section>

  <section>
    <h2>Recent Runs</h2>
    <div class="grid">{runs_panel}</div>
  </section>
</main>
</body>
</html>"#,
        status_panel = render_status(snapshot),
        approvals_panel = render_pending_approvals(approvals),
        runs_panel = render_recent_runs(runs),
        role_options = role_options,
    )
}

fn render_run_detail(detail: &RunDetailSnapshot) -> String {
    let metadata_text = serde_json::to_string_pretty(&detail.metadata)
        .unwrap_or_else(|_| detail.metadata.to_string());
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{run_id} | FounderAI Run</title>
  <style>
    body {{ margin: 0; background: #f5f0e7; color: #1f2a28; font-family: Georgia, "Times New Roman", serif; }}
    main {{ max-width: 1100px; margin: 0 auto; padding: 24px; }}
    article {{ background: #fffdf9; border: 1px solid #d6c7af; border-radius: 18px; padding: 18px; margin-bottom: 18px; }}
    pre {{ white-space: pre-wrap; word-break: break-word; background: #f4efe5; padding: 14px; border-radius: 12px; overflow: auto; }}
    code {{ font-family: "SFMono-Regular", Consolas, monospace; }}
    a {{ color: #295c52; }}
  </style>
</head>
<body>
<main>
  <article>
    <h1>{run_id}</h1>
    <p><a href="/">Back to console</a> · <a href="/runs/{run_id}">Raw JSON</a></p>
    <p><strong>Prompt file:</strong> <code>{prompt_file}</code><br>
    <strong>Output file:</strong> <code>{output_file}</code><br>
    <strong>Stdout file:</strong> <code>{stdout_file}</code><br>
    <strong>Stderr file:</strong> <code>{stderr_file}</code><br>
    <strong>Metadata file:</strong> <code>{metadata_file}</code></p>
  </article>
  <article><h2>Output</h2><pre>{output}</pre></article>
  <article><h2>Prompt</h2><pre>{prompt}</pre></article>
  <article><h2>Stdout</h2><pre>{stdout}</pre></article>
  <article><h2>Stderr</h2><pre>{stderr}</pre></article>
  <article><h2>Metadata</h2><pre>{metadata}</pre></article>
</main>
</body>
</html>"#,
        run_id = escape_html(&detail.run_id),
        prompt_file = escape_html(&detail.prompt_file),
        output_file = escape_html(&detail.output_file),
        stdout_file = escape_html(&detail.stdout_file),
        stderr_file = escape_html(&detail.stderr_file),
        metadata_file = escape_html(&detail.metadata_file),
        output = escape_html(&detail.output),
        prompt = escape_html(&detail.prompt),
        stdout = escape_html(&detail.stdout),
        stderr = escape_html(&detail.stderr),
        metadata = escape_html(&metadata_text),
    )
}

async fn healthz(State(state): State<WebState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    state.app.healthcheck().map_err(internal_error)?;
    Ok(Json(json!({ "ok": true })))
}

async fn dashboard(State(state): State<WebState>) -> Result<Html<String>, (StatusCode, String)> {
    let status = state.app.status_snapshot(true).map_err(internal_error)?;
    let approvals = state.app.approval_snapshots().map_err(internal_error)?;
    let runs = state.app.recent_runs(15).map_err(internal_error)?;
    Ok(Html(render_dashboard(&status, &approvals, &runs, &state.app)))
}

async fn status_json(
    State(state): State<WebState>,
    Query(query): Query<StatusQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let snapshot = state
        .app
        .status_snapshot(query.teams.unwrap_or(false))
        .map_err(internal_error)?;
    Ok(Json(snapshot))
}

async fn approvals_json(
    State(state): State<WebState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let approvals = state.app.approval_snapshots().map_err(internal_error)?;
    Ok(Json(approvals))
}

async fn approve(
    State(state): State<WebState>,
    Path(approval_id): Path<String>,
    Form(form): Form<DecisionFormPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state
        .app
        .approve_pending_approval(&approval_id, form.notes.as_deref().unwrap_or(""))
        .map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn reject(
    State(state): State<WebState>,
    Path(approval_id): Path<String>,
    Form(form): Form<DecisionFormPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    state
        .app
        .reject_pending_approval(&approval_id, form.notes.as_deref().unwrap_or(""))
        .map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn create_request(
    State(state): State<WebState>,
    Form(form): Form<RequestFormPayload>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if form.title.trim().is_empty() || form.body.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "title and body are required".to_string()));
    }

    let risk_tags = form
        .risk_tags
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    let approval_policy = form
        .approval_policy
        .unwrap_or_else(|| "inherit".to_string());
    let requires_internet = form.requires_internet.is_some();
    state
        .app
        .create_request_file(
            form.title.trim(),
            form.body.trim(),
            &approval_policy,
            &risk_tags,
            requires_internet,
            form.role_id.as_deref().filter(|value| !value.trim().is_empty()),
        )
        .map_err(internal_error)?;
    Ok(Redirect::to("/"))
}

async fn runs_json(State(state): State<WebState>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let runs = state.app.recent_runs(50).map_err(internal_error)?;
    Ok(Json(runs))
}

async fn run_json(
    State(state): State<WebState>,
    Path(run_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let detail = state.app.run_detail(&run_id).map_err(internal_error)?;
    Ok(Json(detail))
}

async fn run_view(
    State(state): State<WebState>,
    Path(run_id): Path<String>,
) -> Result<Html<String>, (StatusCode, String)> {
    let detail = state.app.run_detail(&run_id).map_err(internal_error)?;
    Ok(Html(render_run_detail(&detail)))
}

pub async fn serve(app: AutonomyApp, listen: &str) -> Result<()> {
    let socket_addr: SocketAddr = listen
        .parse()
        .with_context(|| format!("failed to parse listen address '{listen}'"))?;
    let state = WebState {
        app: Arc::new(app),
    };

    let router = Router::new()
        .route("/", get(dashboard))
        .route("/healthz", get(healthz))
        .route("/status", get(status_json))
        .route("/approvals", get(approvals_json))
        .route("/approvals/:approval_id/approve", post(approve))
        .route("/approvals/:approval_id/reject", post(reject))
        .route("/requests", post(create_request))
        .route("/runs", get(runs_json))
        .route("/runs/:run_id", get(run_json))
        .route("/runs/:run_id/view", get(run_view))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .with_context(|| format!("failed to bind FounderAI web listener on {listen}"))?;
    axum::serve(listener, router)
        .await
        .context("FounderAI web server stopped unexpectedly")
}
