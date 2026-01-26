use tauri::AppHandle;

use super::update_tray_tooltip;

#[tauri::command]
pub fn tray_update_tooltip(app: AppHandle, tooltip: String) {
    update_tray_tooltip(&app, &tooltip);
}
