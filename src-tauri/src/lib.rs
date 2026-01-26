mod commands;
pub mod settings;
pub mod timer;

use std::sync::Mutex;

use commands::{
    timer_get_status, timer_pause, timer_reset, timer_resume, timer_set_duration,
    timer_set_session_type, timer_start, timer_stop, TimerManager,
};
use settings::{settings_get, settings_update};
use timer::Timer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(TimerManager(Mutex::new(Timer::default())))
        .invoke_handler(tauri::generate_handler![
            timer_get_status,
            timer_start,
            timer_pause,
            timer_resume,
            timer_stop,
            timer_reset,
            timer_set_session_type,
            timer_set_duration,
            settings_get,
            settings_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
