import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { TimerStatus } from "../timer/types";
import { SESSION_LABEL_BY_TYPE } from "../timer/types";
import * as trayApi from "./api";

type TrayManagerOptions = {
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStop: () => void;
  getCurrentState: () => TimerStatus["state"];
};

const formatTime = (totalSecsRemaining: number): string => {
  const minutes = Math.floor(totalSecsRemaining / 60);
  const seconds = totalSecsRemaining % 60;

  return `${String(minutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")}`;
};

const buildTooltip = (status: TimerStatus): string => {
  const sessionLabel = SESSION_LABEL_BY_TYPE[status.sessionType];
  const timeFormatted = formatTime(status.remainingSecs);

  if (status.state === "idle") {
    return "Pomodori - Idle";
  }

  if (status.state === "paused") {
    return `Pomodori - ${sessionLabel} (Paused) ${timeFormatted}`;
  }

  return `Pomodori - ${sessionLabel} ${timeFormatted}`;
};

export const createTrayManager = (options: TrayManagerOptions) => {
  const unlisteners: UnlistenFn[] = [];

  const setupEventListeners = async () => {
    const unlistenStart = await listen("tray-start", () => {
      options.onStart();
    });
    unlisteners.push(unlistenStart);

    const unlistenResume = await listen("tray-resume", () => {
      options.onResume();
    });
    unlisteners.push(unlistenResume);

    const unlistenPause = await listen("tray-pause", () => {
      options.onPause();
    });
    unlisteners.push(unlistenPause);

    const unlistenStop = await listen("tray-stop", () => {
      options.onStop();
    });
    unlisteners.push(unlistenStop);
  };

  const updateTooltip = (status: TimerStatus) => {
    const tooltip = buildTooltip(status);
    trayApi.updateTooltip(tooltip);
  };

  const init = async () => {
    await setupEventListeners();
  };

  const destroy = () => {
    for (const unlisten of unlisteners) {
      unlisten();
    }
  };

  return {
    init,
    updateTooltip,
    destroy,
  };
};
