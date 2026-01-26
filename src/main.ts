import { getCurrentWindow } from "@tauri-apps/api/window";
import {
  createCircularProgress,
  createControlButtons,
  createTimeDisplay,
} from "./components";
import { timerStore } from "./timer";

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

  await timerStore.init();

  timerStore.subscribe((status) => {
    circularProgress.setProgress(status.progress);
    timeDisplay.setTime(status.remainingSecs);
    timeDisplay.setMode(status.sessionType);
    controlButtons.setState(status.state);
  });
};

window.addEventListener("DOMContentLoaded", () => {
  setupTitlebarControls();
  setupTimer();
});
