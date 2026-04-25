use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AgentConfig, WorktreeRef};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Session {
    pub id: Uuid,
    pub name: String,
    pub agent_config: AgentConfig,
    pub worktree: Option<WorktreeRef>,
    pub working_dir: PathBuf,
    pub messages: Vec<Message>,
    pub baseline_commit: Option<String>,
    pub status: SessionStatus,
}

impl Session {
    pub fn new(name: impl Into<String>, agent_config: AgentConfig, working_dir: PathBuf) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            agent_config,
            worktree: None,
            working_dir,
            messages: Vec::new(),
            baseline_commit: None,
            status: SessionStatus::Idle,
        }
    }

    pub fn push_user_message(&mut self, content: impl Into<String>) {
        self.messages.push(Message::new(MessageRole::User, content));
    }

    pub fn push_agent_message(&mut self, content: impl Into<String>) {
        self.messages
            .push(Message::new(MessageRole::Agent, content));
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
}

impl Message {
    pub fn new(role: MessageRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum MessageRole {
    User,
    Agent,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum SessionStatus {
    Idle,
    Running,
    Streaming,
    Error,
}
