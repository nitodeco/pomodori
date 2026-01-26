import { invoke } from "@tauri-apps/api/core";
import type { SessionType, TimerStatus } from "./types";

export const getStatus = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_get_status");

export const start = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_start");

export const pause = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_pause");

export const resume = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_resume");

export const stop = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_stop");

export const reset = (): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_reset");

export const setSessionType = (sessionType: SessionType): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_set_session_type", { sessionType });

export const setDuration = (durationInSecs: number): Promise<TimerStatus> =>
  invoke<TimerStatus>("timer_set_duration", { durationInSecs });
