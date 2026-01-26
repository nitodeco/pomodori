use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::{AppHandle, Manager};

use super::schema::{Session, SessionRow, CREATE_SESSIONS_INDEX, CREATE_SESSIONS_TABLE};

pub struct DbManager(pub SqlitePool);

pub async fn init_database(app: &AppHandle) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    std::fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;

    let db_path = app_data_dir.join("pomodori.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(CREATE_SESSIONS_TABLE)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(CREATE_SESSIONS_INDEX)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    app.manage(DbManager(pool));

    Ok(())
}

#[tauri::command]
pub async fn db_create_session(
    app: AppHandle,
    session_type: String,
    started_at: String,
    duration_in_secs: i64,
) -> Result<i64, String> {
    let state = app.state::<DbManager>();

    let result = sqlx::query(
        "INSERT INTO sessions (session_type, started_at, duration_in_secs, completed) VALUES (?, ?, ?, 0)",
    )
    .bind(&session_type)
    .bind(&started_at)
    .bind(duration_in_secs)
    .execute(&state.0)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn db_complete_session(app: AppHandle, id: i64, ended_at: String) -> Result<(), String> {
    let state = app.state::<DbManager>();

    sqlx::query("UPDATE sessions SET ended_at = ?, completed = 1 WHERE id = ?")
        .bind(&ended_at)
        .bind(id)
        .execute(&state.0)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn db_get_sessions(
    app: AppHandle,
    from_date: Option<String>,
    to_date: Option<String>,
) -> Result<Vec<Session>, String> {
    let state = app.state::<DbManager>();

    let rows: Vec<SessionRow> = match (&from_date, &to_date) {
        (Some(from), Some(to)) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at >= ? AND started_at <= ? ORDER BY started_at DESC",
            )
            .bind(from)
            .bind(to)
            .fetch_all(&state.0)
            .await
        }
        (Some(from), None) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at >= ? ORDER BY started_at DESC",
            )
            .bind(from)
            .fetch_all(&state.0)
            .await
        }
        (None, Some(to)) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at <= ? ORDER BY started_at DESC",
            )
            .bind(to)
            .fetch_all(&state.0)
            .await
        }
        (None, None) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions ORDER BY started_at DESC",
            )
            .fetch_all(&state.0)
            .await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(Session::from).collect())
}
