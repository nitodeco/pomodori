import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { TimerStatus, SessionType } from "../timer/types";
import { timerStore } from "../timer";
import { getSettings } from "../settings";

type AutoAdvanceConfig = {
  onAdvance?: (nextSessionType: SessionType, isAutoStarted: boolean) => void;
};

const getNextSessionType = (
  currentSessionType: SessionType,
  completedWorkSessions: number,
  sessionsUntilLongBreak: number
): SessionType => {
  if (currentSessionType === "work") {
    const isLongBreakDue = completedWorkSessions >= sessionsUntilLongBreak;

    return isLongBreakDue ? "longBreak" : "shortBreak";
  }

  return "work";
};

const getDurationForSessionType = (
  sessionType: SessionType,
  settings: {
    workDurationInSecs: number;
    shortBreakDurationInSecs: number;
    longBreakDurationInSecs: number;
  }
): number => {
  const DURATION_IN_SECS_BY_SESSION_TYPE: Record<SessionType, number> = {
    work: settings.workDurationInSecs,
    shortBreak: settings.shortBreakDurationInSecs,
    longBreak: settings.longBreakDurationInSecs,
  };

  return DURATION_IN_SECS_BY_SESSION_TYPE[sessionType];
};

export const createAutoAdvance = (config: AutoAdvanceConfig = {}) => {
  let unlistenFinished: UnlistenFn | null = null;
  let completedWorkSessions = 0;
  let isInitialized = false;

  const handleTimerFinished = async (status: TimerStatus): Promise<void> => {
    if (status.remainingSecs !== 0) {
      return;
    }

    const settings = await getSettings();
    const isWorkSessionCompleted = status.sessionType === "work";
    const isLongBreakCompleted = status.sessionType === "longBreak";

    if (isWorkSessionCompleted) {
      completedWorkSessions += 1;
    }

    if (isLongBreakCompleted) {
      completedWorkSessions = 0;
    }

    const nextSessionType = getNextSessionType(
      status.sessionType,
      completedWorkSessions,
      settings.sessionsUntilLongBreak
    );

    const nextDuration = getDurationForSessionType(nextSessionType, settings);

    await timerStore.setSessionType(nextSessionType);
    await timerStore.setDuration(nextDuration);

    const shouldAutoStart =
      (nextSessionType === "work" && settings.autoStartWork) ||
      (nextSessionType !== "work" && settings.autoStartBreaks);

    if (shouldAutoStart) {
      await timerStore.start();
    }

    config.onAdvance?.(nextSessionType, shouldAutoStart);
  };

  const init = async (): Promise<void> => {
    if (isInitialized) {
      return;
    }

    unlistenFinished = await listen<TimerStatus>("timer-finished", (event) => {
      handleTimerFinished(event.payload);
    });

    isInitialized = true;
  };

  const resetWorkSessionCount = (): void => {
    completedWorkSessions = 0;
  };

  const getCompletedWorkSessions = (): number => completedWorkSessions;

  const cleanup = (): void => {
    if (unlistenFinished) {
      unlistenFinished();
      unlistenFinished = null;
    }
    completedWorkSessions = 0;
    isInitialized = false;
  };

  return {
    init,
    resetWorkSessionCount,
    getCompletedWorkSessions,
    cleanup,
  };
};
