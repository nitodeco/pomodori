import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { completeSession, createSession } from "../database";
import type { SessionType, TimerStatus } from "../timer/types";

type ActiveSession = {
  id: number;
  sessionType: SessionType;
  startedAt: string;
  durationInSecs: number;
};

type SessionCompletedCallback = (session: ActiveSession) => void;

const createSessionTracker = () => {
  let maybeActiveSession: ActiveSession | null = null;
  let unlistenTick: UnlistenFn | null = null;
  let unlistenFinished: UnlistenFn | null = null;
  let unlistenStopped: UnlistenFn | null = null;
  let isInitialized = false;
  const completedCallbacks = new Set<SessionCompletedCallback>();

  const startSession = async (sessionType: SessionType, durationInSecs: number): Promise<void> => {
    if (maybeActiveSession) {
      return;
    }

    const startedAt = new Date().toISOString();
    const sessionId = await createSession(sessionType, startedAt, durationInSecs);

    maybeActiveSession = {
      id: sessionId,
      sessionType,
      startedAt,
      durationInSecs,
    };
  };

  const finishSession = async (isCompleted: boolean): Promise<void> => {
    if (!maybeActiveSession) {
      return;
    }

    if (isCompleted) {
      const endedAt = new Date().toISOString();
      await completeSession(maybeActiveSession.id, endedAt);

      for (const callback of completedCallbacks) {
        callback(maybeActiveSession);
      }
    }

    maybeActiveSession = null;
  };

  const handleTimerTick = (status: TimerStatus): void => {
    if (status.state === "running" && !maybeActiveSession) {
      startSession(status.sessionType, status.totalSecs);
    }
  };

  const handleTimerFinished = async (status: TimerStatus): Promise<void> => {
    const isCompleted = status.remainingSecs === 0;
    await finishSession(isCompleted);
  };

  const handleTimerStopped = async (): Promise<void> => {
    await finishSession(false);
  };

  const init = async (): Promise<void> => {
    if (isInitialized) {
      return;
    }

    unlistenTick = await listen<TimerStatus>("timer-tick", (event) => {
      handleTimerTick(event.payload);
    });

    unlistenFinished = await listen<TimerStatus>("timer-finished", (event) => {
      handleTimerFinished(event.payload);
    });

    unlistenStopped = await listen<TimerStatus>("timer-stopped", () => {
      handleTimerStopped();
    });

    isInitialized = true;
  };

  const onSessionCompleted = (callback: SessionCompletedCallback): (() => void) => {
    completedCallbacks.add(callback);

    return () => {
      completedCallbacks.delete(callback);
    };
  };

  const cancelActiveSession = async (): Promise<void> => {
    await finishSession(false);
  };

  const getActiveSession = (): ActiveSession | null => maybeActiveSession;

  const cleanup = (): void => {
    if (unlistenTick) {
      unlistenTick();
      unlistenTick = null;
    }

    if (unlistenFinished) {
      unlistenFinished();
      unlistenFinished = null;
    }

    if (unlistenStopped) {
      unlistenStopped();
      unlistenStopped = null;
    }

    completedCallbacks.clear();
    maybeActiveSession = null;
    isInitialized = false;
  };

  return {
    init,
    onSessionCompleted,
    cancelActiveSession,
    getActiveSession,
    cleanup,
  };
};

export const sessionTracker = createSessionTracker();
