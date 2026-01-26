use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SessionRow {
    pub id: i64,
    pub session_type: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_in_secs: i64,
    pub completed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: i64,
    pub session_type: String,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_in_secs: i64,
    pub completed: bool,
}

impl From<SessionRow> for Session {
    fn from(row: SessionRow) -> Self {
        Self {
            id: row.id,
            session_type: row.session_type,
            started_at: row.started_at,
            ended_at: row.ended_at,
            duration_in_secs: row.duration_in_secs,
            completed: row.completed != 0,
        }
    }
}

pub const CREATE_SESSIONS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_type TEXT NOT NULL,
    started_at TEXT NOT NULL,
    ended_at TEXT,
    duration_in_secs INTEGER NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0
);
";

pub const CREATE_SESSIONS_INDEX: &str = "
CREATE INDEX IF NOT EXISTS idx_sessions_started_at ON sessions(started_at);
";
