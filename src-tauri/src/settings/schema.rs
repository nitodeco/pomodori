use serde::{Deserialize, Serialize};

const DEFAULT_WORK_DURATION_IN_SECS: u32 = 25 * 60;
const DEFAULT_SHORT_BREAK_DURATION_IN_SECS: u32 = 5 * 60;
const DEFAULT_LONG_BREAK_DURATION_IN_SECS: u32 = 15 * 60;
const DEFAULT_SESSIONS_UNTIL_LONG_BREAK: u32 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub work_duration_in_secs: u32,
    pub short_break_duration_in_secs: u32,
    pub long_break_duration_in_secs: u32,
    pub sessions_until_long_break: u32,
    pub auto_start_breaks: bool,
    pub auto_start_work: bool,
    pub sound_enabled: bool,
    pub notifications_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            work_duration_in_secs: DEFAULT_WORK_DURATION_IN_SECS,
            short_break_duration_in_secs: DEFAULT_SHORT_BREAK_DURATION_IN_SECS,
            long_break_duration_in_secs: DEFAULT_LONG_BREAK_DURATION_IN_SECS,
            sessions_until_long_break: DEFAULT_SESSIONS_UNTIL_LONG_BREAK,
            auto_start_breaks: false,
            auto_start_work: false,
            sound_enabled: true,
            notifications_enabled: true,
        }
    }
}
