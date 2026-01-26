use serde::{Deserialize, Serialize};

use super::state::{SessionType, TimerState};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timer {
    state: TimerState,
    session_type: SessionType,
    remaining_secs: u32,
    total_secs: u32,
}

impl Timer {
    pub fn new(session_type: SessionType) -> Self {
        let duration_in_secs = session_type.default_duration_in_secs();

        Self {
            state: TimerState::Idle,
            session_type,
            remaining_secs: duration_in_secs,
            total_secs: duration_in_secs,
        }
    }

    pub fn with_duration(session_type: SessionType, duration_in_secs: u32) -> Self {
        Self {
            state: TimerState::Idle,
            session_type,
            remaining_secs: duration_in_secs,
            total_secs: duration_in_secs,
        }
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn session_type(&self) -> SessionType {
        self.session_type
    }

    pub fn remaining_secs(&self) -> u32 {
        self.remaining_secs
    }

    pub fn total_secs(&self) -> u32 {
        self.total_secs
    }

    pub fn elapsed_secs(&self) -> u32 {
        self.total_secs.saturating_sub(self.remaining_secs)
    }

    pub fn progress(&self) -> f64 {
        if self.total_secs == 0 {
            return 1.0;
        }

        self.elapsed_secs() as f64 / self.total_secs as f64
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_secs == 0
    }

    pub fn start(&mut self) -> bool {
        match self.state {
            TimerState::Idle => {
                self.state = TimerState::Running;
                true
            }
            TimerState::Running | TimerState::Paused => false,
        }
    }

    pub fn pause(&mut self) -> bool {
        match self.state {
            TimerState::Running => {
                self.state = TimerState::Paused;
                true
            }
            TimerState::Idle | TimerState::Paused => false,
        }
    }

    pub fn resume(&mut self) -> bool {
        match self.state {
            TimerState::Paused => {
                self.state = TimerState::Running;
                true
            }
            TimerState::Idle | TimerState::Running => false,
        }
    }

    pub fn stop(&mut self) -> bool {
        match self.state {
            TimerState::Running | TimerState::Paused => {
                self.state = TimerState::Idle;
                self.remaining_secs = self.total_secs;
                true
            }
            TimerState::Idle => false,
        }
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.remaining_secs = self.total_secs;
    }

    pub fn tick(&mut self) -> bool {
        if self.state != TimerState::Running {
            return false;
        }

        if self.remaining_secs > 0 {
            self.remaining_secs -= 1;
        }

        if self.remaining_secs == 0 {
            self.state = TimerState::Idle;
        }

        true
    }

    pub fn set_session_type(&mut self, session_type: SessionType) {
        let duration_in_secs = session_type.default_duration_in_secs();
        self.session_type = session_type;
        self.total_secs = duration_in_secs;
        self.remaining_secs = duration_in_secs;
        self.state = TimerState::Idle;
    }

    pub fn set_duration(&mut self, duration_in_secs: u32) {
        self.total_secs = duration_in_secs;
        self.remaining_secs = duration_in_secs;
        self.state = TimerState::Idle;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(SessionType::default())
    }
}
