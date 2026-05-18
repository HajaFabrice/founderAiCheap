mod agents;
mod app;
mod approvals;
mod config;
mod improvement;
mod marketing;
mod model_router;
mod network;
mod notifier;
mod offline;
mod singleton;
mod state;
mod team_logging;
mod web;
mod worker;

use anyhow::Result;
use app::AutonomyApp;
use approvals::list_pending_approvals;
use clap::{Parser, Subcommand, ValueEnum};
use config::load_config;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum ApprovalPolicyArg {
    Inherit,
    Never,
    BeforeRun,
    AfterRun,
}

impl ApprovalPolicyArg {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Inherit => "inherit",
            Self::Never => "never",
            Self::BeforeRun => "before_run",
            Self::AfterRun => "after_run",
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "founderai-ollama-rust",
    about = "FounderAI autonomy daemon with switchable local and hosted providers."
)]
struct Cli {
    #[arg(long, default_value = "config/founderai.json", global = true)]
    config: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Daemon,
    Serve {
        #[arg(long, default_value = "127.0.0.1:8080")]
        listen: String,
    },
    Tick,
    Trigger {
        trigger_name: String,
    },
    Status {
        #[arg(long)]
        teams: bool,
    },
    Approvals,
    Approve {
        approval_id: String,
        #[arg(long, default_value = "")]
        notes: String,
    },
    Reject {
        approval_id: String,
        #[arg(long, default_value = "")]
        notes: String,
    },
    Request {
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: String,
        #[arg(long, value_enum, default_value = "inherit")]
        approval_policy: ApprovalPolicyArg,
        #[arg(long = "risk-tag")]
        risk_tag: Vec<String>,
        #[arg(long)]
        requires_internet: bool,
        #[arg(long)]
        role_id: Option<String>,
    },
}

fn strip_matching_quotes(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.chars().next().unwrap_or_default();
        let last = trimmed.chars().last().unwrap_or_default();
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

fn parse_env_assignment(line: &str) -> Option<(String, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let candidate = trimmed.strip_prefix("export ").unwrap_or(trimmed);
    let (key, value) = candidate.split_once('=')?;
    let key = key.trim();
    if key.is_empty() || key.contains(char::is_whitespace) {
        return None;
    }

    Some((key.to_string(), strip_matching_quotes(value)))
}

fn load_env_file_if_present(path: &Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }

    let raw = fs::read_to_string(path)?;
    for line in raw.lines() {
        let Some((key, value)) = parse_env_assignment(line) else {
            continue;
        };

        if std::env::var_os(&key).is_some() {
            continue;
        }

        // Safe here because this runs once at process startup before any worker threads exist.
        unsafe {
            std::env::set_var(key, value);
        }
    }

    Ok(())
}

fn load_local_env_files() -> Result<()> {
    let cwd = std::env::current_dir()?;
    for file_name in [".env", ".env.local"] {
        load_env_file_if_present(&cwd.join(file_name))?;
    }
    Ok(())
}

fn main() -> Result<()> {
    load_local_env_files()?;
    let cli = Cli::parse();
    let config = load_config(&cli.config)?;
    let executable_path = std::env::current_exe()?;
    let app = AutonomyApp::new(config, executable_path);

    match cli.command {
        Commands::Daemon => {
            app.daemon()?;
        }
        Commands::Serve { listen } => {
            web::serve(app, &listen)?;
        }
        Commands::Tick => {
            app.tick(false, None)?;
            println!("FounderAI tick completed.");
        }
        Commands::Trigger { trigger_name } => {
            app.tick(false, Some(&trigger_name))?;
            println!("FounderAI trigger '{trigger_name}' completed.");
        }
        Commands::Status { teams } => {
            println!("{}", app.status_text(teams)?);
        }
        Commands::Approvals => {
            let approvals = list_pending_approvals(&app.config.runtime_dir)?;
            if approvals.is_empty() {
                println!("No pending approvals.");
            } else {
                for approval in approvals {
                    println!(
                        "{} | job={} | phase={}",
                        approval.record.approval_id, approval.record.job_id, approval.record.phase
                    );
                    println!("{}", approval.record.summary);
                    println!("Summary file: {}", approval.summary_path.display());
                    println!();
                }
            }
        }
        Commands::Approve { approval_id, notes } => {
            let destination = app.approve_pending_approval(&approval_id, &notes)?;
            println!("Approval saved to {}", destination.display());
        }
        Commands::Reject { approval_id, notes } => {
            let destination = app.reject_pending_approval(&approval_id, &notes)?;
            println!("Rejection saved to {}", destination.display());
        }
        Commands::Request {
            title,
            body,
            approval_policy,
            risk_tag,
            requires_internet,
            role_id,
        } => {
            let path = app.create_request_file(
                &title,
                &body,
                approval_policy.as_str(),
                &risk_tag,
                requires_internet,
                role_id.as_deref(),
            )?;
            println!("Inbox request created at {}", path.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_env_assignment_supports_plain_values() {
        let parsed = parse_env_assignment("ANTHROPIC_API_KEY=test-value").unwrap();
        assert_eq!(parsed.0, "ANTHROPIC_API_KEY");
        assert_eq!(parsed.1, "test-value");
    }

    #[test]
    fn parse_env_assignment_strips_matching_quotes() {
        let parsed = parse_env_assignment("export FOUNDERAI_MODEL=\"claude-sonnet-4-6\"").unwrap();
        assert_eq!(parsed.0, "FOUNDERAI_MODEL");
        assert_eq!(parsed.1, "claude-sonnet-4-6");
    }
}
