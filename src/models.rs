use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    New,
    InProgress,
    Completed,
    Abandoned,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::New => "NEW",
            Status::InProgress => "IN-PROGRESS",
            Status::Completed => "COMPLETED",
            Status::Abandoned => "ABANDONED",
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NEW" => Ok(Status::New),
            "IN-PROGRESS" => Ok(Status::InProgress),
            "COMPLETED" => Ok(Status::Completed),
            "ABANDONED" => Ok(Status::Abandoned),
            _ => Err(format!("invalid status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_key: String,
    pub created_at: String,
    pub description: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNote {
    pub id: i64,
    pub task_id: String,
    pub created_at: String,
    pub note: String,
}

pub fn normalize_project_key(name: &str) -> String {
    name.trim().to_lowercase()
}
