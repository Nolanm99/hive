use std::{fs, path::Path};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::AgentConfig;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppConfig {
    #[serde(default = "default_agents")]
    pub agents: Vec<AgentConfig>,
    #[serde(default)]
    pub default_model: Option<String>,
}

impl AppConfig {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let text = fs::read_to_string(path.as_ref())
            .with_context(|| format!("read config {}", path.as_ref().display()))?;
        toml::from_str(&text).context("parse app config")
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            agents: default_agents(),
            default_model: None,
        }
    }
}

fn default_agents() -> Vec<AgentConfig> {
    vec![AgentConfig::codex()]
}
