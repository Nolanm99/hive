use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
    sync::mpsc::{self, Receiver},
    thread,
};

use anyhow::{Context, Result};
use portable_pty::{Child, CommandBuilder, NativePtySystem, PtySize, PtySystem};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AgentConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl AgentConfig {
    pub fn codex() -> Self {
        Self {
            name: "Codex".to_string(),
            command: "codex".to_string(),
            args: Vec::new(),
            env: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AgentStatus {
    Starting,
    Running,
    Exited,
    Failed,
}

pub struct AgentProcess {
    pty_writer: Box<dyn Write + Send>,
    output_rx: Receiver<String>,
    child: Box<dyn Child + Send + Sync>,
    pid: Option<u32>,
    status: AgentStatus,
}

impl AgentProcess {
    pub fn spawn(config: &AgentConfig, working_dir: impl AsRef<Path>) -> Result<Self> {
        let pty_system = NativePtySystem::default();
        let pair = pty_system
            .openpty(PtySize {
                rows: 40,
                cols: 120,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("open agent PTY")?;

        let mut command = CommandBuilder::new(&config.command);
        command.cwd(working_dir.as_ref());
        for arg in &config.args {
            command.arg(arg);
        }
        for (key, value) in &config.env {
            command.env(key, value);
        }

        let child = pair
            .slave
            .spawn_command(command)
            .with_context(|| format!("spawn agent command `{}`", config.command))?;
        let pid = child.process_id();
        let mut reader = pair.master.try_clone_reader().context("clone PTY reader")?;
        let writer = pair.master.take_writer().context("take PTY writer")?;
        let (output_tx, output_rx) = mpsc::channel();

        thread::Builder::new()
            .name(format!("hive-agent-output-{}", config.name))
            .spawn(move || {
                let mut buf = [0; 4096];
                loop {
                    match reader.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            let stripped = strip_ansi_escapes::strip(&buf[..n]);
                            let text = String::from_utf8_lossy(&stripped).to_string();
                            if output_tx.send(text).is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            })
            .context("spawn PTY reader thread")?;

        Ok(Self {
            pty_writer: writer,
            output_rx,
            child,
            pid,
            status: AgentStatus::Running,
        })
    }

    pub fn send_prompt(&mut self, prompt: &str) -> Result<()> {
        self.pty_writer
            .write_all(prompt.as_bytes())
            .context("write prompt to agent")?;
        self.pty_writer.write_all(b"\n").context("write newline")?;
        self.pty_writer.flush().context("flush agent stdin")
    }

    pub fn try_recv_output(&self) -> Option<String> {
        self.output_rx.try_recv().ok()
    }

    pub fn status(&self) -> AgentStatus {
        self.status
    }

    pub fn pid(&self) -> Option<u32> {
        self.pid
    }

    pub fn try_wait(&mut self) -> Result<Option<portable_pty::ExitStatus>> {
        self.child.try_wait().context("poll agent process")
    }
}
