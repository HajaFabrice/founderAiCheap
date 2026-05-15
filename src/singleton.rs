use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct LockPayload {
    pid: u32,
}

fn pid_is_running(pid: u32) -> bool {
    #[cfg(windows)]
    {
        let output = match Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}")])
            .output()
        {
            Ok(output) => output,
            Err(_) => return false,
        };
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        combined.contains(&pid.to_string())
    }

    #[cfg(not(windows))]
    {
        let output = match Command::new("ps")
            .args(["-p", &pid.to_string(), "-o", "pid="])
            .output()
        {
            Ok(output) => output,
            Err(_) => return false,
        };
        return String::from_utf8_lossy(&output.stdout)
            .trim()
            .eq(&pid.to_string());
    }
}

#[derive(Debug)]
pub struct DaemonLock {
    path: PathBuf,
    acquired: bool,
}

impl DaemonLock {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            acquired: false,
        }
    }

    pub fn acquire(&mut self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create lock dir {}", parent.display()))?;
        }

        let payload = LockPayload {
            pid: std::process::id(),
        };
        let serialized =
            serde_json::to_string(&payload).context("failed to serialize lock payload")?;

        match OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.path)
        {
            Ok(mut file) => {
                file.write_all(serialized.as_bytes())
                    .with_context(|| format!("failed to write lock {}", self.path.display()))?;
                self.acquired = true;
                Ok(())
            }
            Err(_) => {
                let existing_pid = fs::read_to_string(&self.path)
                    .ok()
                    .and_then(|raw| serde_json::from_str::<LockPayload>(&raw).ok())
                    .map(|payload| payload.pid)
                    .unwrap_or(0);

                if existing_pid != 0 && pid_is_running(existing_pid) {
                    bail!("FounderAI daemon already running with PID {existing_pid}.");
                }

                fs::write(&self.path, serialized).with_context(|| {
                    format!("failed to refresh stale lock {}", self.path.display())
                })?;
                self.acquired = true;
                Ok(())
            }
        }
    }

    pub fn release(&mut self) {
        if self.acquired {
            fs::remove_file(&self.path).ok();
            self.acquired = false;
        }
    }
}

impl Drop for DaemonLock {
    fn drop(&mut self) {
        self.release();
    }
}
