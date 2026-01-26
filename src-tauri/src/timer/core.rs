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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_timer_starts_idle_with_default_duration() {
        let timer = Timer::new(SessionType::Work);

        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.session_type(), SessionType::Work);
        assert_eq!(timer.remaining_secs(), 25 * 60);
        assert_eq!(timer.total_secs(), 25 * 60);
    }

    #[test]
    fn with_duration_sets_custom_duration() {
        let timer = Timer::with_duration(SessionType::Work, 600);

        assert_eq!(timer.remaining_secs(), 600);
        assert_eq!(timer.total_secs(), 600);
    }

    #[test]
    fn default_timer_is_work_session() {
        let timer = Timer::default();

        assert_eq!(timer.session_type(), SessionType::Work);
        assert_eq!(timer.state(), TimerState::Idle);
    }

    #[test]
    fn elapsed_secs_calculates_correctly() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();

        for _ in 0..30 {
            timer.tick();
        }

        assert_eq!(timer.elapsed_secs(), 30);
        assert_eq!(timer.remaining_secs(), 70);
    }

    #[test]
    fn progress_returns_zero_at_start() {
        let timer = Timer::new(SessionType::Work);

        assert!((timer.progress() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn progress_returns_half_at_midpoint() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();

        for _ in 0..50 {
            timer.tick();
        }

        assert!((timer.progress() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn progress_returns_one_when_total_is_zero() {
        let timer = Timer::with_duration(SessionType::Work, 0);

        assert!((timer.progress() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn is_finished_when_remaining_is_zero() {
        let mut timer = Timer::with_duration(SessionType::Work, 2);
        timer.start();
        timer.tick();
        timer.tick();

        assert!(timer.is_finished());
    }

    #[test]
    fn start_from_idle_transitions_to_running() {
        let mut timer = Timer::new(SessionType::Work);

        assert!(timer.start());
        assert_eq!(timer.state(), TimerState::Running);
    }

    #[test]
    fn start_from_running_returns_false() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();

        assert!(!timer.start());
    }

    #[test]
    fn start_from_paused_returns_false() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.pause();

        assert!(!timer.start());
    }

    #[test]
    fn pause_from_running_transitions_to_paused() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();

        assert!(timer.pause());
        assert_eq!(timer.state(), TimerState::Paused);
    }

    #[test]
    fn pause_from_idle_returns_false() {
        let mut timer = Timer::new(SessionType::Work);

        assert!(!timer.pause());
    }

    #[test]
    fn pause_from_paused_returns_false() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.pause();

        assert!(!timer.pause());
    }

    #[test]
    fn resume_from_paused_transitions_to_running() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.pause();

        assert!(timer.resume());
        assert_eq!(timer.state(), TimerState::Running);
    }

    #[test]
    fn resume_from_idle_returns_false() {
        let mut timer = Timer::new(SessionType::Work);

        assert!(!timer.resume());
    }

    #[test]
    fn resume_from_running_returns_false() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();

        assert!(!timer.resume());
    }

    #[test]
    fn stop_from_running_resets_to_idle() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();
        timer.tick();
        timer.tick();

        assert!(timer.stop());
        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 100);
    }

    #[test]
    fn stop_from_paused_resets_to_idle() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();
        timer.tick();
        timer.pause();

        assert!(timer.stop());
        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 100);
    }

    #[test]
    fn stop_from_idle_returns_false() {
        let mut timer = Timer::new(SessionType::Work);

        assert!(!timer.stop());
    }

    #[test]
    fn reset_restores_full_duration() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();

        for _ in 0..50 {
            timer.tick();
        }

        timer.reset();

        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 100);
    }

    #[test]
    fn tick_decrements_remaining_when_running() {
        let mut timer = Timer::with_duration(SessionType::Work, 100);
        timer.start();

        assert!(timer.tick());
        assert_eq!(timer.remaining_secs(), 99);
    }

    #[test]
    fn tick_returns_false_when_idle() {
        let mut timer = Timer::new(SessionType::Work);

        assert!(!timer.tick());
    }

    #[test]
    fn tick_returns_false_when_paused() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.pause();

        assert!(!timer.tick());
    }

    #[test]
    fn tick_transitions_to_idle_when_finished() {
        let mut timer = Timer::with_duration(SessionType::Work, 1);
        timer.start();
        timer.tick();

        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 0);
    }

    #[test]
    fn tick_does_not_go_negative() {
        let mut timer = Timer::with_duration(SessionType::Work, 1);
        timer.start();
        timer.tick();

        let remaining_after_finish = timer.remaining_secs();
        timer.start();
        timer.tick();

        assert_eq!(timer.remaining_secs(), remaining_after_finish);
    }

    #[test]
    fn set_session_type_updates_duration_and_resets() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.tick();

        timer.set_session_type(SessionType::ShortBreak);

        assert_eq!(timer.session_type(), SessionType::ShortBreak);
        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 5 * 60);
        assert_eq!(timer.total_secs(), 5 * 60);
    }

    #[test]
    fn set_duration_updates_and_resets() {
        let mut timer = Timer::new(SessionType::Work);
        timer.start();
        timer.tick();

        timer.set_duration(1800);

        assert_eq!(timer.state(), TimerState::Idle);
        assert_eq!(timer.remaining_secs(), 1800);
        assert_eq!(timer.total_secs(), 1800);
    }

    #[test]
    fn full_session_lifecycle() {
        let mut timer = Timer::with_duration(SessionType::Work, 3);

        assert_eq!(timer.state(), TimerState::Idle);

        timer.start();
        assert_eq!(timer.state(), TimerState::Running);

        timer.tick();
        assert_eq!(timer.remaining_secs(), 2);

        timer.pause();
        assert_eq!(timer.state(), TimerState::Paused);

        timer.tick();
        assert_eq!(timer.remaining_secs(), 2);

        timer.resume();
        assert_eq!(timer.state(), TimerState::Running);

        timer.tick();
        timer.tick();
        assert_eq!(timer.remaining_secs(), 0);
        assert_eq!(timer.state(), TimerState::Idle);
        assert!(timer.is_finished());
    }
}
