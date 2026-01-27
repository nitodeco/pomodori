import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { createTickSound, type TickSound } from "../audio";
import { getSettings } from "../settings";
import type { SessionType, TimerState, TimerStatus } from "../timer/types";

type SoundManagerConfig = {
  onlyDuringWork?: boolean;
};

const getNextSessionType = (currentSessionType: SessionType): SessionType => {
  if (currentSessionType === "work") {
    return "shortBreak";
  }

  return "work";
};

const isAutoStartEnabled = async (nextSessionType: SessionType): Promise<boolean> => {
  const settings = await getSettings();

  if (nextSessionType === "work") {
    return settings.autoStartWork;
  }

  return settings.autoStartBreaks;
};

export const createSoundManager = (config: SoundManagerConfig = {}) => {
  const { onlyDuringWork = false } = config;

  const alertSound: TickSound = createTickSound();
  let unlistenTick: UnlistenFn | null = null;
  let unlistenFinished: UnlistenFn | null = null;
  let isEnabled = true;
  let previousState: TimerState | null = null;

  const refreshSettings = async () => {
    const settings = await getSettings();
    isEnabled = settings.soundEnabled;
  };

  const shouldPlaySound = (sessionType: SessionType): boolean => {
    if (!isEnabled) {
      return false;
    }

    if (onlyDuringWork && sessionType !== "work") {
      return false;
    }

    return true;
  };

  const handleStateTransition = (status: TimerStatus) => {
    const currentState = status.state;

    if (previousState === null) {
      previousState = currentState;

      return;
    }

    if (previousState === currentState) {
      return;
    }

    const hasStateChanged = previousState !== currentState;
    const isTransition =
      (previousState === "idle" && currentState === "running") ||
      (previousState === "running" && currentState === "paused") ||
      (previousState === "paused" && currentState === "running") ||
      (previousState === "running" && currentState === "idle") ||
      (previousState === "paused" && currentState === "idle");

    if (hasStateChanged && isTransition && shouldPlaySound(status.sessionType)) {
      alertSound.play();
    }

    previousState = currentState;
  };

  const handleTimerFinished = async (status: TimerStatus) => {
    if (!shouldPlaySound(status.sessionType)) {
      return;
    }

    const nextSessionType = getNextSessionType(status.sessionType);
    const autoStartEnabled = await isAutoStartEnabled(nextSessionType);

    if (!autoStartEnabled) {
      alertSound.play();
    }
  };

  const init = async () => {
    await refreshSettings();

    unlistenTick = await listen<TimerStatus>("timer-tick", (event) => {
      handleStateTransition(event.payload);
    });

    unlistenFinished = await listen<TimerStatus>("timer-finished", (event) => {
      handleTimerFinished(event.payload);
    });
  };

  const setEnabled = (enabled: boolean) => {
    isEnabled = enabled;
  };

  const destroy = () => {
    if (unlistenTick) {
      unlistenTick();
      unlistenTick = null;
    }

    if (unlistenFinished) {
      unlistenFinished();
      unlistenFinished = null;
    }

    alertSound.destroy();
  };

  return {
    init,
    setEnabled,
    refreshSettings,
    destroy,
  };
};

export type SoundManager = ReturnType<typeof createSoundManager>;
