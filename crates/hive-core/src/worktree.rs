use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct WorktreeRef {
    pub name: String,
    pub path: PathBuf,
    pub branch: Option<String>,
}

pub struct WorktreeManager {
    repo: Repository,
}

impl WorktreeManager {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let repo = Repository::discover(path).context("discover git repository")?;
        Ok(Self { repo })
    }

    pub fn list(&self) -> Result<Vec<WorktreeRef>> {
        let names = self.repo.worktrees().context("list git worktrees")?;
        let mut worktrees = Vec::new();

        for name in names.iter().flatten() {
            let worktree = self
                .repo
                .find_worktree(name)
                .with_context(|| format!("find git worktree `{name}`"))?;
            let path = worktree.path().to_path_buf();
            worktrees.push(WorktreeRef {
                name: name.to_string(),
                path,
                branch: None,
            });
        }

        Ok(worktrees)
    }

    pub fn create(&self, name: &str, path: impl AsRef<Path>) -> Result<WorktreeRef> {
        let worktree = self
            .repo
            .worktree(name, path.as_ref(), None)
            .with_context(|| format!("create git worktree `{name}`"))?;

        Ok(WorktreeRef {
            name: name.to_string(),
            path: worktree.path().to_path_buf(),
            branch: None,
        })
    }

    pub fn prune(&self, name: &str) -> Result<()> {
        let worktree = self
            .repo
            .find_worktree(name)
            .with_context(|| format!("find git worktree `{name}`"))?;
        worktree
            .prune(None)
            .with_context(|| format!("prune git worktree `{name}`"))
    }
}
