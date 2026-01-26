import { getCurrentWindow } from "@tauri-apps/api/window";
import { createCircularProgress, createTimeDisplay } from "./components";
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
  const maybeContainer = document.getElementById("timer-container");

  if (!maybeContainer) {
    return;
  }

  const circularProgress = createCircularProgress({
    container: maybeContainer,
    sizeInPx: 200,
    strokeWidthInPx: 8,
  });

  const timeDisplay = createTimeDisplay({
    container: maybeContainer,
  });

  await timerStore.init();

  timerStore.subscribe((status) => {
    circularProgress.setProgress(status.progress);
    timeDisplay.setTime(status.remainingSecs);
    timeDisplay.setMode(status.sessionType);
  });
};

window.addEventListener("DOMContentLoaded", () => {
  setupTitlebarControls();
  setupTimer();
});
