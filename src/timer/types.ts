export type TimerState = "idle" | "running" | "paused";

export type SessionType = "work" | "shortBreak" | "longBreak";

export type TimerStatus = {
  state: TimerState;
  sessionType: SessionType;
  remainingSecs: number;
  totalSecs: number;
  progress: number;
};

export const SESSION_DURATION_IN_SECS_BY_TYPE: Record<SessionType, number> = {
  work: 25 * 60,
  shortBreak: 5 * 60,
  longBreak: 15 * 60,
};

export const SESSION_LABEL_BY_TYPE: Record<SessionType, string> = {
  work: "Work",
  shortBreak: "Short Break",
  longBreak: "Long Break",
};
