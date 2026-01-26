import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { createTickSound, type TickSound } from "../audio";
import { getSettings } from "../settings";
import type { TimerStatus } from "../timer/types";

type SoundManagerConfig = {
  onlyDuringWork?: boolean;
};

export const createSoundManager = (config: SoundManagerConfig = {}) => {
  const { onlyDuringWork = false } = config;

  const tickSound: TickSound = createTickSound();
  let unlistenTick: UnlistenFn | null = null;
  let isEnabled = true;

  const refreshSettings = async () => {
    const settings = await getSettings();
    isEnabled = settings.soundEnabled;
  };

  const handleTick = (status: TimerStatus) => {
    if (!isEnabled) {
      return;
    }

    if (status.state !== "running") {
      return;
    }

    if (onlyDuringWork && status.sessionType !== "work") {
      return;
    }

    tickSound.play();
  };

  const init = async () => {
    await refreshSettings();

    unlistenTick = await listen<TimerStatus>("timer-tick", (event) => {
      handleTick(event.payload);
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

    tickSound.destroy();
  };

  return {
    init,
    setEnabled,
    refreshSettings,
    destroy,
  };
};

export type SoundManager = ReturnType<typeof createSoundManager>;
