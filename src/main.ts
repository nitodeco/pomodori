import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import {
  createAutoAdvance,
  createCircularProgress,
  createControlButtons,
  createKeyboardShortcuts,
  createSoundManager,
  createTimeDisplay,
} from "./components";
import { sessionTracker, statsStore } from "./sessions";
import { timerStore } from "./timer";
import { createTrayManager } from "./tray";

const autoAdvance = createAutoAdvance();
const soundManager = createSoundManager();
const trayManager = createTrayManager({
  onStart: () => timerStore.start(),
  onPause: () => timerStore.pause(),
  onResume: () => timerStore.resume(),
  onStop: () => timerStore.stop(),
  getCurrentState: () => timerStore.getStatus().state,
});

const openSettingsWindow = async () => {
  await invoke("open_settings_window");
};

const setupContextMenu = () => {
  window.addEventListener("contextmenu", (contextMenuEvent) => {
    contextMenuEvent.preventDefault();
    invoke("show_main_context_menu", {
      position_x: contextMenuEvent.clientX,
      position_y: contextMenuEvent.clientY,
    });
  });
};

const setupTimer = async () => {
  const maybeTimerContainer = document.getElementById("timer-container");
  const maybeControlsContainer = document.getElementById("controls-container");

  if (!maybeTimerContainer || !maybeControlsContainer) {
    return;
  }

  const circularProgress = createCircularProgress({
    container: maybeTimerContainer,
    sizeInPx: 200,
    strokeWidthInPx: 8,
  });

  const timeDisplay = createTimeDisplay({
    container: maybeTimerContainer,
  });

  const controlButtons = createControlButtons({
    container: maybeControlsContainer,
    onStart: () => timerStore.start(),
    onPause: () => timerStore.pause(),
    onResume: () => timerStore.resume(),
    onStop: () => timerStore.stop(),
  });

  createKeyboardShortcuts({
    onStart: () => timerStore.start(),
    onPause: () => timerStore.pause(),
    onResume: () => timerStore.resume(),
    onStop: () => timerStore.stop(),
    onReset: () => timerStore.reset(),
    getCurrentState: () => timerStore.getStatus().state,
    onOpenSettings: () => openSettingsWindow(),
  });

  await timerStore.init();

  timerStore.subscribe((status) => {
    circularProgress.setProgress(status.progress);
    timeDisplay.setTime(status.remainingSecs);
    timeDisplay.setMode(status.sessionType);
    controlButtons.setState(status.state);
    trayManager.updateTooltip(status);
  });
};

const setupSessionTracking = async () => {
  await sessionTracker.init();
  await statsStore.refresh();

  sessionTracker.onSessionCompleted(() => {
    statsStore.refreshTodayStats();
    statsStore.refreshAllTimeStats();
    void emit("stats-refresh", null);
  });
};

const setupSounds = async () => {
  await soundManager.init();
};

const setupTray = async () => {
  await trayManager.init();
};

const setupAutoAdvance = async () => {
  await autoAdvance.init();
};

const setupSettingsSync = async () => {
  await listen("settings-updated", () => {
    soundManager.refreshSettings();
  });
};

window.addEventListener("DOMContentLoaded", () => {
  setupTimer();
  setupContextMenu();
  setupSettingsSync();
  setupSessionTracking();
  setupSounds();
  setupTray();
  setupAutoAdvance();
});
