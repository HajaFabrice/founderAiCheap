mod agents;
mod app;
mod approvals;
mod config;
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
    about = "FounderAI autonomy daemon backed by local Ollama."
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

fn main() -> Result<()> {
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
