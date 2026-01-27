import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import * as api from "./api";
import type { TimerStatus } from "./types";

type Subscriber = (status: TimerStatus) => void;

const DEFAULT_STATUS: TimerStatus = {
  state: "idle",
  sessionType: "work",
  remainingSecs: 25 * 60,
  totalSecs: 25 * 60,
  progress: 0,
};

const createTimerStore = () => {
  const subscribers = new Set<Subscriber>();
  let currentStatus: TimerStatus = DEFAULT_STATUS;
  let unlistenTick: UnlistenFn | null = null;
  let unlistenFinished: UnlistenFn | null = null;
  let isInitialized = false;

  const notifySubscribers = () => {
    for (const subscriber of subscribers) {
      subscriber(currentStatus);
    }
  };

  const updateStatus = (status: TimerStatus) => {
    currentStatus = status;
    notifySubscribers();
  };

  const setupEventListeners = async () => {
    if (unlistenTick || unlistenFinished) {
      return;
    }

    unlistenTick = await listen<TimerStatus>("timer-tick", (event) => {
      updateStatus(event.payload);
    });

    unlistenFinished = await listen<TimerStatus>("timer-finished", (event) => {
      updateStatus(event.payload);
    });
  };

  const init = async (): Promise<TimerStatus> => {
    if (isInitialized) {
      return currentStatus;
    }

    await setupEventListeners();
    const status = await api.getStatus();
    updateStatus(status);
    isInitialized = true;

    return status;
  };

  const getStatus = (): TimerStatus => currentStatus;

  const subscribe = (subscriber: Subscriber): (() => void) => {
    subscribers.add(subscriber);
    subscriber(currentStatus);

    return () => {
      subscribers.delete(subscriber);
    };
  };

  const start = async (): Promise<TimerStatus> => {
    const status = await api.start();
    updateStatus(status);

    return status;
  };

  const pause = async (): Promise<TimerStatus> => {
    const status = await api.pause();
    updateStatus(status);

    return status;
  };

  const resume = async (): Promise<TimerStatus> => {
    const status = await api.resume();
    updateStatus(status);

    return status;
  };

  const stop = async (): Promise<TimerStatus> => {
    const status = await api.stop();
    updateStatus(status);

    return status;
  };

  const reset = async (): Promise<TimerStatus> => {
    const status = await api.reset();
    updateStatus(status);

    return status;
  };

  const setSessionType = async (sessionType: TimerStatus["sessionType"]): Promise<TimerStatus> => {
    const status = await api.setSessionType(sessionType);
    updateStatus(status);

    return status;
  };

  const setDuration = async (durationInSecs: number): Promise<TimerStatus> => {
    const status = await api.setDuration(durationInSecs);
    updateStatus(status);

    return status;
  };

  const cleanup = () => {
    if (unlistenTick) {
      unlistenTick();
      unlistenTick = null;
    }

    if (unlistenFinished) {
      unlistenFinished();
      unlistenFinished = null;
    }

    subscribers.clear();
    isInitialized = false;
  };

  return {
    init,
    getStatus,
    subscribe,
    start,
    pause,
    resume,
    stop,
    reset,
    setSessionType,
    setDuration,
    cleanup,
  };
};

export const timerStore = createTimerStore();
