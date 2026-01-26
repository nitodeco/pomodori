import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  createCircularProgress,
  createControlButtons,
  createKeyboardShortcuts,
  createSettingsPanel,
  createStatsDashboard,
  createTimeDisplay,
} from "./components";
import { timerStore } from "./timer";
import { sessionTracker, statsStore } from "./sessions";

const appWindow = getCurrentWindow();

const setupTitlebarControls = () => {
  document.querySelectorAll(".titlebar-button").forEach((button) => {
    button.addEventListener("click", async (event) => {
      const target = event.currentTarget as HTMLElement;
      const action = target.dataset.action;

      if (action === "close") {
        await appWindow.close();
      } else if (action === "minimize") {
        await appWindow.minimize();
      }
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
  });

  await timerStore.init();

  timerStore.subscribe((status) => {
    circularProgress.setProgress(status.progress);
    timeDisplay.setTime(status.remainingSecs);
    timeDisplay.setMode(status.sessionType);
    controlButtons.setState(status.state);
  });
};

const setupSettings = () => {
  const maybeApp = document.querySelector(".app");
  const maybeSettingsButton = document.getElementById("settings-button");

  if (!maybeApp || !maybeSettingsButton) {
    return;
  }

  const settingsPanel = createSettingsPanel({
    container: maybeApp as HTMLElement,
    onSettingsChange: () => {},
  });

  maybeSettingsButton.addEventListener("click", () => settingsPanel.open());
};

const setupStatsDashboard = () => {
  const maybeApp = document.querySelector(".app");
  const maybeStatsButton = document.getElementById("stats-button");

  if (!maybeApp || !maybeStatsButton) {
    return;
  }

  const statsDashboard = createStatsDashboard({
    container: maybeApp as HTMLElement,
  });

  maybeStatsButton.addEventListener("click", () => statsDashboard.open());
};

const setupSessionTracking = async () => {
  await sessionTracker.init();
  await statsStore.refresh();

  sessionTracker.onSessionCompleted(() => {
    statsStore.refreshTodayStats();
    statsStore.refreshAllTimeStats();
  });
};

window.addEventListener("DOMContentLoaded", () => {
  setupTitlebarControls();
  setupTimer();
  setupSettings();
  setupStatsDashboard();
  setupSessionTracking();
});
