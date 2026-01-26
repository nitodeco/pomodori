import type { SessionType } from "../timer/types";

export type Session = {
  id: number;
  sessionType: SessionType;
  startedAt: string;
  endedAt: string | null;
  durationInSecs: number;
  completed: boolean;
};

export type SessionStats = {
  totalSessions: number;
  completedSessions: number;
  totalWorkTimeInSecs: number;
  totalBreakTimeInSecs: number;
};
