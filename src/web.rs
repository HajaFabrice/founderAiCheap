use crate::app::{ApprovalSnapshot, AutonomyApp, RunDetailSnapshot, RunListEntry, StatusSnapshot};
use anyhow::{Context, Result};
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct WebState {
    app: Arc<AutonomyApp>,
}

#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    query: BTreeMap<String, String>,
    body: Vec<u8>,
}

struct WebResponse {
    status_code: u16,
    reason_phrase: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
    extra_headers: Vec<(String, String)>,
}

fn html_response(body: String) -> WebResponse {
    WebResponse {
        status_code: 200,
        reason_phrase: "OK",
        content_type: "text/html; charset=utf-8",
        body: body.into_bytes(),
        extra_headers: Vec::new(),
    }
}

fn json_response<T: Serialize>(value: &T) -> WebResponse {
    match serde_json::to_vec_pretty(value) {
        Ok(body) => WebResponse {
            status_code: 200,
            reason_phrase: "OK",
            content_type: "application/json; charset=utf-8",
            body,
            extra_headers: Vec::new(),
        },
        Err(err) => internal_error(anyhow::anyhow!("failed to serialize JSON response: {err}")),
    }
}

fn text_response(status_code: u16, reason_phrase: &'static str, body: String) -> WebResponse {
    WebResponse {
        status_code,
        reason_phrase,
        content_type: "text/plain; charset=utf-8",
        body: body.into_bytes(),
        extra_headers: Vec::new(),
    }
}

fn redirect_response(location: &str) -> WebResponse {
    WebResponse {
        status_code: 303,
        reason_phrase: "See Other",
        content_type: "text/plain; charset=utf-8",
        body: Vec::new(),
        extra_headers: vec![("Location".to_string(), location.to_string())],
    }
}

fn bad_request(message: &str) -> WebResponse {
    text_response(400, "Bad Request", message.to_string())
}

fn not_found() -> WebResponse {
    text_response(404, "Not Found", "not found".to_string())
}

fn internal_error(err: anyhow::Error) -> WebResponse {
    text_response(500, "Internal Server Error", err.to_string())
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

fn decode_hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

fn percent_decode(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let (Some(high), Some(low)) = (
                    decode_hex_nibble(bytes[index + 1]),
                    decode_hex_nibble(bytes[index + 2]),
                ) {
                    output.push((high << 4) | low);
                    index += 3;
                } else {
                    output.push(bytes[index]);
                    index += 1;
                }
            }
            value => {
                output.push(value);
                index += 1;
            }
        }
    }

    String::from_utf8_lossy(&output).into_owned()
}

fn parse_form_encoded(raw: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for pair in raw.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        map.insert(percent_decode(key), percent_decode(value));
    }
    map
}

fn parse_request_path(raw_target: &str) -> (String, BTreeMap<String, String>) {
    let (path, query) = raw_target.split_once('?').unwrap_or((raw_target, ""));
    let query_map = if query.is_empty() {
        BTreeMap::new()
    } else {
        parse_form_encoded(query)
    };
    (path.to_string(), query_map)
}

fn read_http_request(stream: &TcpStream) -> Result<HttpRequest> {
    let cloned = stream.try_clone().context("failed to clone TCP stream")?;
    let mut reader = BufReader::new(cloned);

    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .context("failed to read request line")?;
    if request_line.trim().is_empty() {
        anyhow::bail!("received an empty request line");
    }

    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .context("request line was missing an HTTP method")?
        .to_string();
    let target = request_parts
        .next()
        .context("request line was missing a request target")?;
    let _version = request_parts
        .next()
        .context("request line was missing an HTTP version")?;

    let (path, query) = parse_request_path(target);

    let mut headers = BTreeMap::new();
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .context("failed to read HTTP headers")?;
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
        }
    }

    let content_length = headers
        .get("content-length")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = vec![0_u8; content_length];
    if content_length > 0 {
        reader
            .read_exact(&mut body)
            .context("failed to read request body")?;
    }

    Ok(HttpRequest {
        method,
        path,
        query,
        body,
    })
}

fn write_response(stream: &mut TcpStream, response: WebResponse) -> Result<()> {
    let mut headers = vec![
        format!(
            "HTTP/1.1 {} {}\r\n",
            response.status_code, response.reason_phrase
        ),
        format!("Content-Type: {}\r\n", response.content_type),
        format!("Content-Length: {}\r\n", response.body.len()),
        "Connection: close\r\n".to_string(),
    ];
    for (name, value) in response.extra_headers {
        headers.push(format!("{name}: {value}\r\n"));
    }
    headers.push("\r\n".to_string());

    for header in headers {
        stream
            .write_all(header.as_bytes())
            .context("failed to write HTTP headers")?;
    }
    stream
        .write_all(&response.body)
        .context("failed to write HTTP body")?;
    stream.flush().context("failed to flush HTTP response")?;
    Ok(())
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
    let provider_detail = snapshot.provider_status.detail.as_deref().unwrap_or("none");
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

fn render_dashboard(
    snapshot: &StatusSnapshot,
    approvals: &[ApprovalSnapshot],
    runs: &[RunListEntry],
    app: &AutonomyApp,
) -> String {
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

fn parse_bool(value: Option<&String>) -> bool {
    value
        .map(|raw| {
            matches!(
                raw.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn handle_request(request: HttpRequest, state: &WebState) -> WebResponse {
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => {
            let status = match state.app.status_snapshot(true) {
                Ok(value) => value,
                Err(err) => return internal_error(err),
            };
            let approvals = match state.app.approval_snapshots() {
                Ok(value) => value,
                Err(err) => return internal_error(err),
            };
            let runs = match state.app.recent_runs(15) {
                Ok(value) => value,
                Err(err) => return internal_error(err),
            };
            html_response(render_dashboard(&status, &approvals, &runs, &state.app))
        }
        ("GET", "/healthz") => match state.app.healthcheck() {
            Ok(_) => json_response(&json!({ "ok": true })),
            Err(err) => internal_error(err),
        },
        ("GET", "/status") => {
            let show_teams = parse_bool(request.query.get("teams"));
            match state.app.status_snapshot(show_teams) {
                Ok(snapshot) => json_response(&snapshot),
                Err(err) => internal_error(err),
            }
        }
        ("GET", "/approvals") => match state.app.approval_snapshots() {
            Ok(approvals) => json_response(&approvals),
            Err(err) => internal_error(err),
        },
        ("POST", "/requests") => {
            let form = parse_form_encoded(&String::from_utf8_lossy(&request.body));
            let title = form.get("title").map(|value| value.trim()).unwrap_or("");
            let body = form.get("body").map(|value| value.trim()).unwrap_or("");
            if title.is_empty() || body.is_empty() {
                return bad_request("title and body are required");
            }

            let risk_tags = form
                .get("risk_tags")
                .map(|raw| {
                    raw.split(',')
                        .map(str::trim)
                        .filter(|item| !item.is_empty())
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let approval_policy = form
                .get("approval_policy")
                .cloned()
                .unwrap_or_else(|| "inherit".to_string());
            let requires_internet = form.contains_key("requires_internet");
            let role_id = form
                .get("role_id")
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty());

            match state.app.create_request_file(
                title,
                body,
                &approval_policy,
                &risk_tags,
                requires_internet,
                role_id,
            ) {
                Ok(_) => redirect_response("/"),
                Err(err) => internal_error(err),
            }
        }
        ("GET", "/runs") => match state.app.recent_runs(50) {
            Ok(runs) => json_response(&runs),
            Err(err) => internal_error(err),
        },
        ("POST", path) if path.starts_with("/approvals/") => {
            let parts = path.trim_matches('/').split('/').collect::<Vec<_>>();
            if parts.len() != 3 {
                return not_found();
            }
            let approval_id = parts[1];
            let action = parts[2];
            let form = parse_form_encoded(&String::from_utf8_lossy(&request.body));
            let notes = form.get("notes").map(String::as_str).unwrap_or("");
            let result = match action {
                "approve" => state.app.approve_pending_approval(approval_id, notes),
                "reject" => state.app.reject_pending_approval(approval_id, notes),
                _ => return not_found(),
            };
            match result {
                Ok(_) => redirect_response("/"),
                Err(err) => internal_error(err),
            }
        }
        ("GET", path) if path.starts_with("/runs/") => {
            let parts = path.trim_matches('/').split('/').collect::<Vec<_>>();
            if parts.len() < 2 {
                return not_found();
            }
            let run_id = parts[1];
            match state.app.run_detail(run_id) {
                Ok(detail) if parts.len() == 2 => json_response(&detail),
                Ok(detail) if parts.len() == 3 && parts[2] == "view" => {
                    html_response(render_run_detail(&detail))
                }
                Ok(_) => not_found(),
                Err(err) => internal_error(err),
            }
        }
        _ => not_found(),
    }
}

fn handle_client(mut stream: TcpStream, state: WebState) -> Result<()> {
    stream
        .set_read_timeout(Some(Duration::from_secs(15)))
        .context("failed to set stream read timeout")?;
    stream
        .set_write_timeout(Some(Duration::from_secs(15)))
        .context("failed to set stream write timeout")?;

    let request = match read_http_request(&stream) {
        Ok(request) => request,
        Err(err) => {
            let response = bad_request(&err.to_string());
            let _ = write_response(&mut stream, response);
            return Ok(());
        }
    };
    let response = handle_request(request, &state);
    write_response(&mut stream, response)
}

pub fn serve(app: AutonomyApp, listen: &str) -> Result<()> {
    let socket_addr: SocketAddr = listen
        .parse()
        .with_context(|| format!("failed to parse listen address '{listen}'"))?;
    let listener = TcpListener::bind(socket_addr)
        .with_context(|| format!("failed to bind FounderAI web listener on {listen}"))?;
    let state = WebState { app: Arc::new(app) };

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state = state.clone();
                thread::spawn(move || {
                    if let Err(err) = handle_client(stream, state) {
                        eprintln!("FounderAI web connection error: {err:#}");
                    }
                });
            }
            Err(err) => {
                eprintln!("FounderAI web accept error: {err}");
            }
        }
    }

    Ok(())
}
