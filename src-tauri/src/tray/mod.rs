use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIcon, TrayIconBuilder},
    AppHandle, Emitter, Manager,
};

use crate::commands::TimerManager;
use crate::timer::TimerState;

pub mod commands;

const TRAY_ID: &str = "main-tray";

pub fn setup_tray(app: &AppHandle) -> Result<TrayIcon, Box<dyn std::error::Error>> {
    let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let separator = MenuItem::new(app, "", false, None::<&str>)?;
    let start = MenuItem::with_id(app, "start", "Start Timer", true, None::<&str>)?;
    let pause = MenuItem::with_id(app, "pause", "Pause Timer", true, None::<&str>)?;
    let stop = MenuItem::with_id(app, "stop", "Stop Timer", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[
            &show, &hide, &separator, &start, &pause, &stop, &separator, &quit,
        ],
    )?;

    let tray = TrayIconBuilder::with_id(TRAY_ID)
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Pomodori - Idle")
        .menu(&menu)
        .on_menu_event(handle_menu_event)
        .build(app)?;

    Ok(tray)
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "hide" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        "start" => {
            let timer_manager = app.state::<TimerManager>();
            let timer = timer_manager.0.lock().unwrap();
            let state = timer.state();
            drop(timer);

            match state {
                TimerState::Idle => {
                    let _ = app.emit("tray-start", ());
                }
                TimerState::Paused => {
                    let _ = app.emit("tray-resume", ());
                }
                _ => {}
            }
        }
        "pause" => {
            let _ = app.emit("tray-pause", ());
        }
        "stop" => {
            let _ = app.emit("tray-stop", ());
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

pub fn update_tray_tooltip(app: &AppHandle, tooltip: &str) {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_tooltip(Some(tooltip));
    }
}
