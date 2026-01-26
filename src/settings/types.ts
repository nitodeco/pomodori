export type Settings = {
  workDurationInSecs: number;
  shortBreakDurationInSecs: number;
  longBreakDurationInSecs: number;
  sessionsUntilLongBreak: number;
  autoStartBreaks: boolean;
  autoStartWork: boolean;
  soundEnabled: boolean;
  notificationsEnabled: boolean;
};

const DEFAULT_WORK_DURATION_IN_SECS = 25 * 60;
const DEFAULT_SHORT_BREAK_DURATION_IN_SECS = 5 * 60;
const DEFAULT_LONG_BREAK_DURATION_IN_SECS = 15 * 60;
const DEFAULT_SESSIONS_UNTIL_LONG_BREAK = 4;

export const DEFAULT_SETTINGS: Settings = {
  workDurationInSecs: DEFAULT_WORK_DURATION_IN_SECS,
  shortBreakDurationInSecs: DEFAULT_SHORT_BREAK_DURATION_IN_SECS,
  longBreakDurationInSecs: DEFAULT_LONG_BREAK_DURATION_IN_SECS,
  sessionsUntilLongBreak: DEFAULT_SESSIONS_UNTIL_LONG_BREAK,
  autoStartBreaks: false,
  autoStartWork: false,
  soundEnabled: true,
  notificationsEnabled: true,
};
