use crate::config::{Config, resolve_db_path};
use crate::project::ResolvedProject;
use crate::db::open_db;
use crate::repo::insert_task;
use crate::models::Task;

pub struct NewResult {
    pub task: Task,
}

pub fn run_new(config: &Config, project: &ResolvedProject, description: &str) -> anyhow::Result<NewResult> {
    let db_path = resolve_db_path(config)?;
    if !db_path.exists() {
        anyhow::bail!("database not initialized");
    }
    let conn = open_db(&db_path)?;
    let task = insert_task(&conn, &project.key, description)?;
    Ok(NewResult { task })
}
