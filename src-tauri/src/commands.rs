use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_notification::NotificationExt;

use crate::settings::settings_get_internal;
use crate::timer::{SessionType, Timer, TimerState};

pub struct TimerManager(pub Mutex<Timer>);

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerStatus {
    state: TimerState,
    session_type: SessionType,
    remaining_secs: u32,
    total_secs: u32,
    progress: f64,
}

impl From<&Timer> for TimerStatus {
    fn from(timer: &Timer) -> Self {
        Self {
            state: timer.state(),
            session_type: timer.session_type(),
            remaining_secs: timer.remaining_secs(),
            total_secs: timer.total_secs(),
            progress: timer.progress(),
        }
    }
}

#[tauri::command]
pub fn timer_get_status(timer_manager: State<TimerManager>) -> TimerStatus {
    let timer = timer_manager.0.lock().unwrap();
    TimerStatus::from(&*timer)
}

#[tauri::command]
pub fn timer_start(app: AppHandle, timer_manager: State<TimerManager>) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();

    if timer.start() {
        let status = TimerStatus::from(&*timer);
        drop(timer);
        start_tick_loop(app);
        status
    } else {
        TimerStatus::from(&*timer)
    }
}

#[tauri::command]
pub fn timer_pause(timer_manager: State<TimerManager>) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();
    timer.pause();
    TimerStatus::from(&*timer)
}

#[tauri::command]
pub fn timer_resume(app: AppHandle, timer_manager: State<TimerManager>) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();

    if timer.resume() {
        let status = TimerStatus::from(&*timer);
        drop(timer);
        start_tick_loop(app);
        status
    } else {
        TimerStatus::from(&*timer)
    }
}

#[tauri::command]
pub fn timer_stop(app: AppHandle, timer_manager: State<TimerManager>) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();
    let was_running = timer.state() == TimerState::Running || timer.state() == TimerState::Paused;
    timer.stop();
    let status = TimerStatus::from(&*timer);

    if was_running {
        let _ = app.emit("timer-stopped", &status);
    }

    status
}

#[tauri::command]
pub fn timer_reset(timer_manager: State<TimerManager>) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();
    timer.reset();
    TimerStatus::from(&*timer)
}

#[tauri::command]
pub fn timer_set_session_type(
    session_type: SessionType,
    timer_manager: State<TimerManager>,
) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();
    timer.set_session_type(session_type);
    TimerStatus::from(&*timer)
}

#[tauri::command]
pub fn timer_set_duration(
    duration_in_secs: u32,
    timer_manager: State<TimerManager>,
) -> TimerStatus {
    let mut timer = timer_manager.0.lock().unwrap();
    timer.set_duration(duration_in_secs);
    TimerStatus::from(&*timer)
}

fn start_tick_loop(app: AppHandle) {
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(1));

        let timer_manager = app.state::<TimerManager>();
        let mut timer = timer_manager.0.lock().unwrap();

        if timer.state() != TimerState::Running {
            break;
        }

        timer.tick();
        let status = TimerStatus::from(&*timer);
        let is_finished = timer.is_finished();
        drop(timer);

        let _ = app.emit("timer-tick", &status);

        if is_finished {
            let _ = app.emit("timer-finished", &status);
            send_timer_notification(&app, status.session_type);
            break;
        }
    });
}

fn send_timer_notification(app: &AppHandle, session_type: SessionType) {
    let settings = settings_get_internal(app);

    if !settings.notifications_enabled {
        return;
    }

    let (title, body) = match session_type {
        SessionType::Work => ("Work Session Complete", "Great job! Time to take a break."),
        SessionType::ShortBreak => ("Break Over", "Ready to focus? Start your next session."),
        SessionType::LongBreak => (
            "Long Break Over",
            "Feeling refreshed? Let's get back to work.",
        ),
    };

    let _ = app.notification().builder().title(title).body(body).show();
}
