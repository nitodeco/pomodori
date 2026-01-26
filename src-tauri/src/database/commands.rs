use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::{AppHandle, Manager};

use super::schema::{
    Session, SessionRow, SessionStats, StatsRow, CREATE_SESSIONS_INDEX, CREATE_SESSIONS_TABLE,
};

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

    init_schema(&pool).await?;

    app.manage(DbManager(pool));

    Ok(())
}

pub async fn init_schema(pool: &SqlitePool) -> Result<(), String> {
    sqlx::query(CREATE_SESSIONS_TABLE)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(CREATE_SESSIONS_INDEX)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn create_session(
    pool: &SqlitePool,
    session_type: &str,
    started_at: &str,
    duration_in_secs: i64,
) -> Result<i64, String> {
    let result = sqlx::query(
        "INSERT INTO sessions (session_type, started_at, duration_in_secs, completed) VALUES (?, ?, ?, 0)",
    )
    .bind(session_type)
    .bind(started_at)
    .bind(duration_in_secs)
    .execute(pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.last_insert_rowid())
}

#[tauri::command]
pub async fn db_create_session(
    app: AppHandle,
    session_type: String,
    started_at: String,
    duration_in_secs: i64,
) -> Result<i64, String> {
    let state = app.state::<DbManager>();
    create_session(&state.0, &session_type, &started_at, duration_in_secs).await
}

pub async fn complete_session(pool: &SqlitePool, id: i64, ended_at: &str) -> Result<(), String> {
    sqlx::query("UPDATE sessions SET ended_at = ?, completed = 1 WHERE id = ?")
        .bind(ended_at)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn db_complete_session(app: AppHandle, id: i64, ended_at: String) -> Result<(), String> {
    let state = app.state::<DbManager>();
    complete_session(&state.0, id, &ended_at).await
}

pub async fn get_sessions(
    pool: &SqlitePool,
    from_date: Option<&str>,
    to_date: Option<&str>,
) -> Result<Vec<Session>, String> {
    let rows: Vec<SessionRow> = match (from_date, to_date) {
        (Some(from), Some(to)) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at >= ? AND started_at <= ? ORDER BY started_at DESC",
            )
            .bind(from)
            .bind(to)
            .fetch_all(pool)
            .await
        }
        (Some(from), None) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at >= ? ORDER BY started_at DESC",
            )
            .bind(from)
            .fetch_all(pool)
            .await
        }
        (None, Some(to)) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions WHERE started_at <= ? ORDER BY started_at DESC",
            )
            .bind(to)
            .fetch_all(pool)
            .await
        }
        (None, None) => {
            sqlx::query_as::<_, SessionRow>(
                "SELECT id, session_type, started_at, ended_at, duration_in_secs, completed FROM sessions ORDER BY started_at DESC",
            )
            .fetch_all(pool)
            .await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(Session::from).collect())
}

#[tauri::command]
pub async fn db_get_sessions(
    app: AppHandle,
    from_date: Option<String>,
    to_date: Option<String>,
) -> Result<Vec<Session>, String> {
    let state = app.state::<DbManager>();
    get_sessions(
        &state.0,
        from_date.as_deref(),
        to_date.as_deref(),
    )
    .await
}

pub async fn get_stats(
    pool: &SqlitePool,
    from_date: Option<&str>,
    to_date: Option<&str>,
) -> Result<SessionStats, String> {
    let base_query = "
        SELECT
            COUNT(*) as total_sessions,
            SUM(CASE WHEN completed = 1 THEN 1 ELSE 0 END) as completed_sessions,
            COALESCE(SUM(CASE WHEN session_type = 'work' AND completed = 1 THEN duration_in_secs ELSE 0 END), 0) as total_work_time_in_secs,
            COALESCE(SUM(CASE WHEN session_type != 'work' AND completed = 1 THEN duration_in_secs ELSE 0 END), 0) as total_break_time_in_secs
        FROM sessions
    ";

    let row: StatsRow = match (from_date, to_date) {
        (Some(from), Some(to)) => {
            let query = format!("{} WHERE started_at >= ? AND started_at <= ?", base_query);
            sqlx::query_as::<_, StatsRow>(&query)
                .bind(from)
                .bind(to)
                .fetch_one(pool)
                .await
        }
        (Some(from), None) => {
            let query = format!("{} WHERE started_at >= ?", base_query);
            sqlx::query_as::<_, StatsRow>(&query)
                .bind(from)
                .fetch_one(pool)
                .await
        }
        (None, Some(to)) => {
            let query = format!("{} WHERE started_at <= ?", base_query);
            sqlx::query_as::<_, StatsRow>(&query)
                .bind(to)
                .fetch_one(pool)
                .await
        }
        (None, None) => {
            sqlx::query_as::<_, StatsRow>(base_query)
                .fetch_one(pool)
                .await
        }
    }
    .map_err(|e| e.to_string())?;

    Ok(SessionStats::from(row))
}

#[tauri::command]
pub async fn db_get_stats(
    app: AppHandle,
    from_date: Option<String>,
    to_date: Option<String>,
) -> Result<SessionStats, String> {
    let state = app.state::<DbManager>();
    get_stats(
        &state.0,
        from_date.as_deref(),
        to_date.as_deref(),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create in-memory database");

        init_schema(&pool).await.expect("Failed to initialize schema");

        pool
    }

    #[tokio::test]
    async fn create_session_returns_valid_id() {
        let pool = setup_test_db().await;

        let id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        assert!(id > 0);
    }

    #[tokio::test]
    async fn create_session_increments_id() {
        let pool = setup_test_db().await;

        let first_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        let second_id = create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();

        assert_eq!(second_id, first_id + 1);
    }

    #[tokio::test]
    async fn get_sessions_returns_empty_for_new_db() {
        let pool = setup_test_db().await;

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert!(sessions.is_empty());
    }

    #[tokio::test]
    async fn get_sessions_returns_created_sessions() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions.len(), 2);
    }

    #[tokio::test]
    async fn get_sessions_orders_by_started_at_descending() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions[0].started_at, "2025-01-15T10:25:00Z");
        assert_eq!(sessions[1].started_at, "2025-01-15T10:00:00Z");
    }

    #[tokio::test]
    async fn get_sessions_filters_by_from_date() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        let sessions = get_sessions(&pool, Some("2025-01-15T00:00:00Z"), None)
            .await
            .unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].started_at, "2025-01-15T10:00:00Z");
    }

    #[tokio::test]
    async fn get_sessions_filters_by_to_date() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, Some("2025-01-14T23:59:59Z"))
            .await
            .unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].started_at, "2025-01-14T10:00:00Z");
    }

    #[tokio::test]
    async fn get_sessions_filters_by_date_range() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-13T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        let sessions = get_sessions(
            &pool,
            Some("2025-01-14T00:00:00Z"),
            Some("2025-01-14T23:59:59Z"),
        )
        .await
        .unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].started_at, "2025-01-14T10:00:00Z");
    }

    #[tokio::test]
    async fn created_session_starts_incomplete() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert!(!sessions[0].completed);
        assert!(sessions[0].ended_at.is_none());
    }

    #[tokio::test]
    async fn complete_session_marks_as_completed() {
        let pool = setup_test_db().await;
        let id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert!(sessions[0].completed);
        assert_eq!(sessions[0].ended_at, Some("2025-01-15T10:25:00Z".to_string()));
    }

    #[tokio::test]
    async fn complete_session_only_affects_specified_id() {
        let pool = setup_test_db().await;
        let first_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        let second_id = create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, first_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();
        let first_session = sessions.iter().find(|s| s.id == first_id).unwrap();
        let second_session = sessions.iter().find(|s| s.id == second_id).unwrap();

        assert!(first_session.completed);
        assert!(!second_session.completed);
    }

    #[tokio::test]
    async fn get_stats_returns_zero_for_empty_db() {
        let pool = setup_test_db().await;

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.completed_sessions, 0);
        assert_eq!(stats.total_work_time_in_secs, 0);
        assert_eq!(stats.total_break_time_in_secs, 0);
    }

    #[tokio::test]
    async fn get_stats_counts_total_sessions() {
        let pool = setup_test_db().await;
        create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 2);
    }

    #[tokio::test]
    async fn get_stats_counts_completed_sessions() {
        let pool = setup_test_db().await;
        let completed_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, completed_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.completed_sessions, 1);
    }

    #[tokio::test]
    async fn get_stats_sums_work_time_for_completed_sessions() {
        let pool = setup_test_db().await;
        let first_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        let second_id = create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();
        create_session(&pool, "work", "2025-01-15T11:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, first_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, second_id, "2025-01-15T10:55:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_work_time_in_secs, 3000);
    }

    #[tokio::test]
    async fn get_stats_sums_break_time_separately() {
        let pool = setup_test_db().await;
        let work_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        let break_id = create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();

        complete_session(&pool, work_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, break_id, "2025-01-15T10:30:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_work_time_in_secs, 1500);
        assert_eq!(stats.total_break_time_in_secs, 300);
    }

    #[tokio::test]
    async fn get_stats_includes_long_break_in_break_time() {
        let pool = setup_test_db().await;
        let short_break_id = create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();
        let long_break_id = create_session(&pool, "long-break", "2025-01-15T12:00:00Z", 900)
            .await
            .unwrap();

        complete_session(&pool, short_break_id, "2025-01-15T10:30:00Z")
            .await
            .unwrap();
        complete_session(&pool, long_break_id, "2025-01-15T12:15:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_break_time_in_secs, 1200);
    }

    #[tokio::test]
    async fn get_stats_filters_by_from_date() {
        let pool = setup_test_db().await;
        let old_id = create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        let new_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, old_id, "2025-01-14T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, new_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, Some("2025-01-15T00:00:00Z"), None)
            .await
            .unwrap();

        assert_eq!(stats.total_sessions, 1);
        assert_eq!(stats.completed_sessions, 1);
    }

    #[tokio::test]
    async fn get_stats_filters_by_to_date() {
        let pool = setup_test_db().await;
        let old_id = create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        let new_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, old_id, "2025-01-14T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, new_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, Some("2025-01-14T23:59:59Z"))
            .await
            .unwrap();

        assert_eq!(stats.total_sessions, 1);
        assert_eq!(stats.completed_sessions, 1);
    }

    #[tokio::test]
    async fn get_stats_filters_by_date_range() {
        let pool = setup_test_db().await;
        let first_id = create_session(&pool, "work", "2025-01-13T10:00:00Z", 1500)
            .await
            .unwrap();
        let second_id = create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        let third_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, first_id, "2025-01-13T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, second_id, "2025-01-14T10:25:00Z")
            .await
            .unwrap();
        complete_session(&pool, third_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let stats = get_stats(
            &pool,
            Some("2025-01-14T00:00:00Z"),
            Some("2025-01-14T23:59:59Z"),
        )
        .await
        .unwrap();

        assert_eq!(stats.total_sessions, 1);
        assert_eq!(stats.completed_sessions, 1);
        assert_eq!(stats.total_work_time_in_secs, 1500);
    }

    #[tokio::test]
    async fn session_row_to_session_converts_completed_correctly() {
        let pool = setup_test_db().await;
        let id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        let sessions_before = get_sessions(&pool, None, None).await.unwrap();
        assert!(!sessions_before[0].completed);

        complete_session(&pool, id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let sessions_after = get_sessions(&pool, None, None).await.unwrap();
        assert!(sessions_after[0].completed);
    }

    #[tokio::test]
    async fn session_preserves_all_fields() {
        let pool = setup_test_db().await;
        let id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();

        complete_session(&pool, id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();
        let session = &sessions[0];

        assert_eq!(session.id, id);
        assert_eq!(session.session_type, "work");
        assert_eq!(session.started_at, "2025-01-15T10:00:00Z");
        assert_eq!(session.ended_at, Some("2025-01-15T10:25:00Z".to_string()));
        assert_eq!(session.duration_in_secs, 1500);
        assert!(session.completed);
    }
}
