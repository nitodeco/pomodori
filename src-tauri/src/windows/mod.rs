use tauri::{
    menu::{Menu, MenuItem},
    AppHandle, LogicalPosition, Manager, Position, WebviewUrl, WebviewWindowBuilder, Window,
};

const SETTINGS_WINDOW_LABEL: &str = "settings";
const STATS_WINDOW_LABEL: &str = "stats";
const MENU_OPEN_SETTINGS: &str = "open-settings";
const MENU_OPEN_STATS: &str = "open-stats";

const SETTINGS_WINDOW_URL: &str = "settings-window.html";
const STATS_WINDOW_URL: &str = "stats-window.html";

const SETTINGS_WINDOW_TITLE: &str = "Settings";
const STATS_WINDOW_TITLE: &str = "Statistics";

const SETTINGS_WINDOW_WIDTH: f64 = 360.0;
const SETTINGS_WINDOW_HEIGHT: f64 = 520.0;
const STATS_WINDOW_WIDTH: f64 = 320.0;
const STATS_WINDOW_HEIGHT: f64 = 280.0;

const MAIN_WINDOW_LABEL: &str = "main";

fn show_or_create_window(
    app: &AppHandle,
    label: &str,
    title: &str,
    url: &str,
    width: f64,
    height: f64,
) -> Result<(), String> {
    if let Some(existing_window) = app.get_webview_window(label) {
        existing_window
            .show()
            .map_err(|error| error.to_string())?;
        existing_window
            .set_focus()
            .map_err(|error| error.to_string())?;
        return Ok(());
    }

    WebviewWindowBuilder::new(app, label, WebviewUrl::App(url.into()))
        .title(title)
        .inner_size(width, height)
        .resizable(false)
        .center()
        .build()
        .map(|_| ())
        .map_err(|error| error.to_string())
}

pub fn register_main_window_menu_events(app: &AppHandle) {
    let Some(window) = app.get_webview_window(MAIN_WINDOW_LABEL) else {
        return;
    };

    let app_handle = app.clone();
    window.on_menu_event(move |_window, event| match event.id().as_ref() {
        MENU_OPEN_SETTINGS => {
            let _ = open_settings_window(app_handle.clone());
        }
        MENU_OPEN_STATS => {
            let _ = open_stats_window(app_handle.clone());
        }
        _ => {}
    });
}

fn build_main_context_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, String> {
    let settings_item = MenuItem::with_id(app, MENU_OPEN_SETTINGS, "Settings", true, None::<&str>)
        .map_err(|error| error.to_string())?;
    let stats_item = MenuItem::with_id(app, MENU_OPEN_STATS, "Statistics", true, None::<&str>)
        .map_err(|error| error.to_string())?;

    Menu::with_items(app, &[&settings_item, &stats_item]).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn show_main_context_menu(
    window: Window,
    position_x: f64,
    position_y: f64,
) -> Result<(), String> {
    let menu = build_main_context_menu(window.app_handle())?;
    let position = Position::Logical(LogicalPosition {
        x: position_x,
        y: position_y,
    });

    window
        .popup_menu_at(&menu, position)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn open_settings_window(app: AppHandle) -> Result<(), String> {
    show_or_create_window(
        &app,
        SETTINGS_WINDOW_LABEL,
        SETTINGS_WINDOW_TITLE,
        SETTINGS_WINDOW_URL,
        SETTINGS_WINDOW_WIDTH,
        SETTINGS_WINDOW_HEIGHT,
    )
}

#[tauri::command]
pub fn open_stats_window(app: AppHandle) -> Result<(), String> {
    show_or_create_window(
        &app,
        STATS_WINDOW_LABEL,
        STATS_WINDOW_TITLE,
        STATS_WINDOW_URL,
        STATS_WINDOW_WIDTH,
        STATS_WINDOW_HEIGHT,
    )
}
