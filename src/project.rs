use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct ProjectFile {
    pub project: String,
}

#[derive(Debug, Clone)]
pub struct ResolvedProject {
    pub name: String,
    pub key: String,
}

pub fn find_project_file(start: &Path, home: &Path) -> anyhow::Result<Option<PathBuf>> {
    let mut cur = start;
    loop {
        let candidate = cur.join(".todoer.toml");
        if candidate.exists() {
            return Ok(Some(candidate));
        }
        if cur == home {
            break;
        }
        match cur.parent() {
            Some(parent) => cur = parent,
            None => break,
        }
    }
    Ok(None)
}

pub fn load_project_name(path: &Path) -> anyhow::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    let pf: ProjectFile = toml::from_str(&contents)?;
    if pf.project.trim().is_empty() {
        anyhow::bail!("project name empty");
    }
    Ok(pf.project)
}
