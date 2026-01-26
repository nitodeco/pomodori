import { listen } from "@tauri-apps/api/event";
import { createStatsDashboard } from "./components";

const setupStatsWindow = async () => {
  const statsRoot = document.getElementById("stats-root");

  if (!statsRoot) {
    return;
  }

  const statsDashboard = createStatsDashboard({
    container: statsRoot,
  });

  await statsDashboard.refresh();

  await listen("stats-refresh", () => {
    statsDashboard.refresh();
  });
};

window.addEventListener("DOMContentLoaded", () => {
  setupStatsWindow();
});
