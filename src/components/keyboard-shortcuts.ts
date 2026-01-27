import type { TimerState } from "../timer/types";

type KeyboardShortcutsConfig = {
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStop: () => void;
  onReset: () => void;
  getCurrentState: () => TimerState;
  onOpenSettings?: () => void;
};

export const createKeyboardShortcuts = (config: KeyboardShortcutsConfig) => {
  const { onStart, onPause, onResume, onStop, onReset, getCurrentState, onOpenSettings } = config;

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
      return;
    }

    const key = event.code;
    const state = getCurrentState();

    if (key === "Space") {
      event.preventDefault();

      if (state === "idle") {
        onStart();
      } else if (state === "running") {
        onPause();
      } else if (state === "paused") {
        onResume();
      }

      return;
    }

    if (key === "Escape" || key === "KeyS") {
      if (state !== "idle") {
        onStop();
      }

      return;
    }

    if (key === "KeyR") {
      onReset();

      return;
    }

    if (key === "Comma" && event.metaKey && onOpenSettings) {
      event.preventDefault();
      onOpenSettings();

      return;
    }
  };

  window.addEventListener("keydown", handleKeyDown);

  const destroy = () => {
    window.removeEventListener("keydown", handleKeyDown);
  };

  return {
    destroy,
  };
};

export type KeyboardShortcuts = ReturnType<typeof createKeyboardShortcuts>;
