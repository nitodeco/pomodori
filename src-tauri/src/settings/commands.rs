use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

use super::Settings;

const SETTINGS_STORE_PATH: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

pub fn settings_get_internal(app: &AppHandle) -> Settings {
    let store = app.store(SETTINGS_STORE_PATH).unwrap();

    store
        .get(SETTINGS_KEY)
        .and_then(|value| serde_json::from_value(value).ok())
        .unwrap_or_default()
}

#[tauri::command]
pub fn settings_get(app: AppHandle) -> Settings {
    settings_get_internal(&app)
}

#[tauri::command]
pub fn settings_update(app: AppHandle, settings: Settings) -> Settings {
    let store = app.store(SETTINGS_STORE_PATH).unwrap();
    let value = serde_json::to_value(&settings).unwrap();
    store.set(SETTINGS_KEY, value);
    store.save().unwrap();
    settings
}
