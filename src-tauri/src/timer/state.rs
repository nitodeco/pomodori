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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timer_state_default_is_idle() {
        assert_eq!(TimerState::default(), TimerState::Idle);
    }

    #[test]
    fn session_type_default_is_work() {
        assert_eq!(SessionType::default(), SessionType::Work);
    }

    #[test]
    fn work_duration_is_25_minutes() {
        assert_eq!(SessionType::WORK_DURATION_IN_SECS, 25 * 60);
        assert_eq!(SessionType::Work.default_duration_in_secs(), 25 * 60);
    }

    #[test]
    fn short_break_duration_is_5_minutes() {
        assert_eq!(SessionType::SHORT_BREAK_DURATION_IN_SECS, 5 * 60);
        assert_eq!(SessionType::ShortBreak.default_duration_in_secs(), 5 * 60);
    }

    #[test]
    fn long_break_duration_is_15_minutes() {
        assert_eq!(SessionType::LONG_BREAK_DURATION_IN_SECS, 15 * 60);
        assert_eq!(SessionType::LongBreak.default_duration_in_secs(), 15 * 60);
    }
}
