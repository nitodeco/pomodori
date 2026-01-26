use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerState {
    #[default]
    Idle,
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionType {
    #[default]
    Work,
    ShortBreak,
    LongBreak,
}

impl SessionType {
    pub const WORK_DURATION_IN_SECS: u32 = 25 * 60;
    pub const SHORT_BREAK_DURATION_IN_SECS: u32 = 5 * 60;
    pub const LONG_BREAK_DURATION_IN_SECS: u32 = 15 * 60;

    pub fn default_duration_in_secs(self) -> u32 {
        match self {
            Self::Work => Self::WORK_DURATION_IN_SECS,
            Self::ShortBreak => Self::SHORT_BREAK_DURATION_IN_SECS,
            Self::LongBreak => Self::LONG_BREAK_DURATION_IN_SECS,
        }
    }
}
