use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use git2::{DiffFormat, DiffOptions, Oid, Repository};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct DiffSummary {
    pub files: Vec<ChangedFile>,
    pub unified_diff: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ChangedFile {
    pub path: PathBuf,
    pub status: String,
}

pub fn diff_from_baseline(
    repo_path: impl AsRef<Path>,
    baseline: Option<&str>,
) -> Result<DiffSummary> {
    let repo = Repository::discover(repo_path).context("discover git repository")?;
    let mut options = DiffOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);

    let diff = match baseline {
        Some(rev) => {
            let oid = Oid::from_str(rev).context("parse baseline oid")?;
            let commit = repo.find_commit(oid).context("find baseline commit")?;
            let tree = commit.tree().context("read baseline tree")?;
            repo.diff_tree_to_workdir_with_index(Some(&tree), Some(&mut options))
                .context("diff baseline to working tree")?
        }
        None => repo
            .diff_index_to_workdir(None, Some(&mut options))
            .context("diff index to working tree")?,
    };

    let mut files = Vec::new();
    for delta in diff.deltas() {
        let path = delta
            .new_file()
            .path()
            .or_else(|| delta.old_file().path())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("<unknown>"));
        files.push(ChangedFile {
            path,
            status: format!("{:?}", delta.status()),
        });
    }

    let mut unified = Vec::new();
    diff.print(DiffFormat::Patch, |_delta, _hunk, line| {
        unified.extend_from_slice(line.content());
        true
    })
    .context("format unified diff")?;

    Ok(DiffSummary {
        files,
        unified_diff: String::from_utf8_lossy(&unified).to_string(),
    })
}
