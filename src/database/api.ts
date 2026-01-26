import { invoke } from "@tauri-apps/api/core";
import type { SessionType } from "../timer/types";
import type { Session, SessionStats } from "./types";

export const createSession = (
  sessionType: SessionType,
  startedAt: string,
  durationInSecs: number
): Promise<number> =>
  invoke<number>("db_create_session", {
    sessionType,
    startedAt,
    durationInSecs,
  });

export const completeSession = (id: number, endedAt: string): Promise<void> =>
  invoke<void>("db_complete_session", { id, endedAt });

export const getSessions = (
  fromDate?: string,
  toDate?: string
): Promise<Session[]> =>
  invoke<Session[]>("db_get_sessions", { fromDate, toDate });

export const getStats = (
  fromDate?: string,
  toDate?: string
): Promise<SessionStats> =>
  invoke<SessionStats>("db_get_stats", { fromDate, toDate });
