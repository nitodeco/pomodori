import { getCurrentWindow } from "@tauri-apps/api/window";

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

window.addEventListener("DOMContentLoaded", () => {
  setupTitlebarControls();
});
