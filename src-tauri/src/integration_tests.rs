#[cfg(test)]
mod tests {
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use crate::database::{
        complete_session, create_session, get_sessions, get_stats, init_schema,
    };
    use crate::timer::{SessionType, Timer, TimerState};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(":memory:")
            .await
            .expect("Failed to create in-memory database");

        init_schema(&pool)
            .await
            .expect("Failed to initialize schema");

        pool
    }

    #[tokio::test]
    async fn complete_work_session_creates_and_completes_db_record() {
        let pool = setup_test_db().await;
        let mut timer = Timer::with_duration(SessionType::Work, 3);
        let started_at = "2025-01-15T10:00:00Z";

        timer.start();
        let session_id = create_session(
            &pool,
            "work",
            started_at,
            timer.total_secs() as i64,
        )
        .await
        .unwrap();

        timer.tick();
        timer.tick();
        timer.tick();
        assert!(timer.is_finished());

        complete_session(&pool, session_id, "2025-01-15T10:00:03Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].completed);
        assert_eq!(sessions[0].session_type, "work");
        assert_eq!(sessions[0].duration_in_secs, 3);
    }

    #[tokio::test]
    async fn pause_resume_produces_single_completed_session() {
        let pool = setup_test_db().await;
        let mut timer = Timer::with_duration(SessionType::Work, 5);
        let started_at = "2025-01-15T10:00:00Z";

        timer.start();
        let session_id = create_session(
            &pool,
            "work",
            started_at,
            timer.total_secs() as i64,
        )
        .await
        .unwrap();

        timer.tick();
        timer.tick();

        timer.pause();
        assert_eq!(timer.state(), TimerState::Paused);

        let sessions_mid = get_sessions(&pool, None, None).await.unwrap();
        assert_eq!(sessions_mid.len(), 1);
        assert!(!sessions_mid[0].completed);

        timer.resume();
        assert_eq!(timer.state(), TimerState::Running);

        timer.tick();
        timer.tick();
        timer.tick();
        assert!(timer.is_finished());

        complete_session(&pool, session_id, "2025-01-15T10:00:10Z")
            .await
            .unwrap();

        let sessions_final = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions_final.len(), 1);
        assert!(sessions_final[0].completed);
    }

    #[tokio::test]
    async fn stop_mid_session_leaves_session_incomplete() {
        let pool = setup_test_db().await;
        let mut timer = Timer::with_duration(SessionType::Work, 10);
        let started_at = "2025-01-15T10:00:00Z";

        timer.start();
        let _session_id = create_session(
            &pool,
            "work",
            started_at,
            timer.total_secs() as i64,
        )
        .await
        .unwrap();

        timer.tick();
        timer.tick();
        timer.tick();

        timer.stop();
        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 10);

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions.len(), 1);
        assert!(!sessions[0].completed);
        assert!(sessions[0].ended_at.is_none());
    }

    #[tokio::test]
    async fn session_type_switch_records_correct_types() {
        let pool = setup_test_db().await;

        let mut work_timer = Timer::with_duration(SessionType::Work, 2);
        work_timer.start();
        let work_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 2)
            .await
            .unwrap();
        work_timer.tick();
        work_timer.tick();
        complete_session(&pool, work_id, "2025-01-15T10:00:02Z")
            .await
            .unwrap();

        let mut break_timer = Timer::with_duration(SessionType::ShortBreak, 2);
        break_timer.start();
        let break_id = create_session(&pool, "short-break", "2025-01-15T10:00:02Z", 2)
            .await
            .unwrap();
        break_timer.tick();
        break_timer.tick();
        complete_session(&pool, break_id, "2025-01-15T10:00:04Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();

        assert_eq!(sessions.len(), 2);

        let work_session = sessions.iter().find(|s| s.session_type == "work").unwrap();
        let break_session = sessions
            .iter()
            .find(|s| s.session_type == "short-break")
            .unwrap();

        assert!(work_session.completed);
        assert!(break_session.completed);
    }

    #[tokio::test]
    async fn stats_aggregate_multiple_session_types() {
        let pool = setup_test_db().await;

        let work1_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        complete_session(&pool, work1_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let break1_id = create_session(&pool, "short-break", "2025-01-15T10:25:00Z", 300)
            .await
            .unwrap();
        complete_session(&pool, break1_id, "2025-01-15T10:30:00Z")
            .await
            .unwrap();

        let work2_id = create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();
        complete_session(&pool, work2_id, "2025-01-15T10:55:00Z")
            .await
            .unwrap();

        let break2_id = create_session(&pool, "short-break", "2025-01-15T10:55:00Z", 300)
            .await
            .unwrap();
        complete_session(&pool, break2_id, "2025-01-15T11:00:00Z")
            .await
            .unwrap();

        let long_break_id = create_session(&pool, "long-break", "2025-01-15T11:00:00Z", 900)
            .await
            .unwrap();
        complete_session(&pool, long_break_id, "2025-01-15T11:15:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 5);
        assert_eq!(stats.completed_sessions, 5);
        assert_eq!(stats.total_work_time_in_secs, 3000);
        assert_eq!(stats.total_break_time_in_secs, 1500);
    }

    #[tokio::test]
    async fn incomplete_sessions_excluded_from_time_stats() {
        let pool = setup_test_db().await;

        let completed_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        complete_session(&pool, completed_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let _incomplete_id = create_session(&pool, "work", "2025-01-15T10:30:00Z", 1500)
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.completed_sessions, 1);
        assert_eq!(stats.total_work_time_in_secs, 1500);
    }

    #[tokio::test]
    async fn multiple_work_sessions_accumulate_time() {
        let pool = setup_test_db().await;

        for i in 0..4 {
            let started_at = format!("2025-01-15T{:02}:00:00Z", 10 + i);
            let ended_at = format!("2025-01-15T{:02}:25:00Z", 10 + i);

            let id = create_session(&pool, "work", &started_at, 1500)
                .await
                .unwrap();
            complete_session(&pool, id, &ended_at).await.unwrap();
        }

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.completed_sessions, 4);
        assert_eq!(stats.total_work_time_in_secs, 6000);
    }

    #[tokio::test]
    async fn timer_state_transitions_reflect_in_session_handling() {
        let pool = setup_test_db().await;
        let mut timer = Timer::with_duration(SessionType::Work, 5);

        assert_eq!(timer.state(), TimerState::Idle);

        timer.start();
        assert_eq!(timer.state(), TimerState::Running);
        let session_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 5)
            .await
            .unwrap();

        timer.tick();
        timer.pause();
        assert_eq!(timer.state(), TimerState::Paused);

        let mid_sessions = get_sessions(&pool, None, None).await.unwrap();
        assert!(!mid_sessions[0].completed);

        timer.resume();
        assert_eq!(timer.state(), TimerState::Running);

        timer.tick();
        timer.tick();
        timer.tick();
        timer.tick();
        assert!(timer.is_finished());
        assert_eq!(timer.state(), TimerState::Idle);

        complete_session(&pool, session_id, "2025-01-15T10:00:10Z")
            .await
            .unwrap();

        let final_sessions = get_sessions(&pool, None, None).await.unwrap();
        assert!(final_sessions[0].completed);
    }

    #[tokio::test]
    async fn daily_stats_filter_correctly() {
        let pool = setup_test_db().await;

        let yesterday_id = create_session(&pool, "work", "2025-01-14T10:00:00Z", 1500)
            .await
            .unwrap();
        complete_session(&pool, yesterday_id, "2025-01-14T10:25:00Z")
            .await
            .unwrap();

        let today_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 1500)
            .await
            .unwrap();
        complete_session(&pool, today_id, "2025-01-15T10:25:00Z")
            .await
            .unwrap();

        let today_stats = get_stats(
            &pool,
            Some("2025-01-15T00:00:00Z"),
            Some("2025-01-15T23:59:59Z"),
        )
        .await
        .unwrap();

        assert_eq!(today_stats.total_sessions, 1);
        assert_eq!(today_stats.completed_sessions, 1);
        assert_eq!(today_stats.total_work_time_in_secs, 1500);

        let all_stats = get_stats(&pool, None, None).await.unwrap();
        assert_eq!(all_stats.total_sessions, 2);
    }

    #[tokio::test]
    async fn pomodoro_cycle_four_work_one_long_break() {
        let pool = setup_test_db().await;
        let base_time = 10;

        for i in 0..4 {
            let hour = base_time + i;
            let work_started = format!("2025-01-15T{:02}:00:00Z", hour);
            let work_ended = format!("2025-01-15T{:02}:25:00Z", hour);

            let work_id = create_session(&pool, "work", &work_started, 1500)
                .await
                .unwrap();
            complete_session(&pool, work_id, &work_ended).await.unwrap();

            if i < 3 {
                let break_started = format!("2025-01-15T{:02}:25:00Z", hour);
                let break_ended = format!("2025-01-15T{:02}:30:00Z", hour);

                let break_id = create_session(&pool, "short-break", &break_started, 300)
                    .await
                    .unwrap();
                complete_session(&pool, break_id, &break_ended).await.unwrap();
            }
        }

        let long_break_id = create_session(&pool, "long-break", "2025-01-15T13:25:00Z", 900)
            .await
            .unwrap();
        complete_session(&pool, long_break_id, "2025-01-15T13:40:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_sessions, 8);
        assert_eq!(stats.completed_sessions, 8);
        assert_eq!(stats.total_work_time_in_secs, 6000);
        assert_eq!(stats.total_break_time_in_secs, 1800);
    }

    #[tokio::test]
    async fn custom_duration_session_records_correctly() {
        let pool = setup_test_db().await;
        let custom_duration_in_secs = 3600;

        let mut timer = Timer::with_duration(SessionType::Work, custom_duration_in_secs);
        timer.start();

        let session_id = create_session(
            &pool,
            "work",
            "2025-01-15T10:00:00Z",
            custom_duration_in_secs as i64,
        )
        .await
        .unwrap();

        for _ in 0..custom_duration_in_secs {
            timer.tick();
        }
        assert!(timer.is_finished());

        complete_session(&pool, session_id, "2025-01-15T11:00:00Z")
            .await
            .unwrap();

        let stats = get_stats(&pool, None, None).await.unwrap();

        assert_eq!(stats.total_work_time_in_secs, custom_duration_in_secs as i64);
    }

    #[tokio::test]
    async fn reset_timer_during_session_allows_fresh_start() {
        let pool = setup_test_db().await;
        let mut timer = Timer::with_duration(SessionType::Work, 10);

        timer.start();
        let _first_session_id = create_session(&pool, "work", "2025-01-15T10:00:00Z", 10)
            .await
            .unwrap();

        timer.tick();
        timer.tick();
        timer.tick();

        timer.reset();
        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 10);

        timer.start();
        let second_session_id = create_session(&pool, "work", "2025-01-15T10:01:00Z", 10)
            .await
            .unwrap();

        for _ in 0..10 {
            timer.tick();
        }
        assert!(timer.is_finished());

        complete_session(&pool, second_session_id, "2025-01-15T10:01:10Z")
            .await
            .unwrap();

        let sessions = get_sessions(&pool, None, None).await.unwrap();
        let completed_count = sessions.iter().filter(|s| s.completed).count();

        assert_eq!(sessions.len(), 2);
        assert_eq!(completed_count, 1);
    }
}
