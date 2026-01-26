import { emit } from "@tauri-apps/api/event";
import { createSettingsPanel } from "./components";

const setupSettingsWindow = async () => {
  const settingsRoot = document.getElementById("settings-root");

  if (!settingsRoot) {
    return;
  }

  const settingsPanel = createSettingsPanel({
    container: settingsRoot,
    onSettingsChange: async (settings) => {
      await emit("settings-updated", settings);
    },
  });

  await settingsPanel.load();
};

window.addEventListener("DOMContentLoaded", () => {
  setupSettingsWindow();
});
