import { getCurrentWindow } from "@tauri-apps/api/window";
import type { SessionStats } from "../database";
import { statsStore } from "../sessions";

type StatsDashboardConfig = {
  container: HTMLElement;
};

const SECS_PER_MIN = 60;
const MINS_PER_HOUR = 60;
const SECS_PER_HOUR = SECS_PER_MIN * MINS_PER_HOUR;

const formatTimeFromSecs = (totalSecs: number): string => {
  const hours = Math.floor(totalSecs / SECS_PER_HOUR);
  const mins = Math.floor((totalSecs % SECS_PER_HOUR) / SECS_PER_MIN);

  if (hours > 0) {
    return `${hours}h ${mins}m`;
  }

  return `${mins}m`;
};

const createStatItem = (label: string, valueId: string): HTMLDivElement => {
  const item = document.createElement("div");
  item.className = "stats-item";

  const valueEl = document.createElement("span");
  valueEl.className = "stats-item__value";
  valueEl.id = valueId;
  valueEl.textContent = "0";

  const labelEl = document.createElement("span");
  labelEl.className = "stats-item__label";
  labelEl.textContent = label;

  item.appendChild(valueEl);
  item.appendChild(labelEl);

  return item;
};

const createStatsGroup = (title: string): HTMLDivElement => {
  const group = document.createElement("div");
  group.className = "stats-group";

  const heading = document.createElement("h3");
  heading.className = "stats-group__title";
  heading.textContent = title;
  group.appendChild(heading);

  const items = document.createElement("div");
  items.className = "stats-group__items";
  group.appendChild(items);

  return group;
};

export const createStatsDashboard = (config: StatsDashboardConfig) => {
  const { container } = config;
  const appWindow = getCurrentWindow();

  const panel = document.createElement("div");
  panel.className = "stats-panel";

  const header = document.createElement("div");
  header.className = "stats-header";

  const title = document.createElement("h2");
  title.className = "stats-title";
  title.textContent = "Statistics";

  const closeButton = document.createElement("button");
  closeButton.className = "stats-close";
  closeButton.innerHTML = "×";
  closeButton.setAttribute("aria-label", "Close statistics");

  header.appendChild(title);
  header.appendChild(closeButton);

  const content = document.createElement("div");
  content.className = "stats-content";

  const todayGroup = createStatsGroup("Today");
  const todayItems = todayGroup.querySelector(".stats-group__items");

  if (!todayItems) {
    throw new Error("Stats dashboard items container missing.");
  }

  todayItems.appendChild(createStatItem("Sessions", "stats-today-sessions"));
  todayItems.appendChild(createStatItem("Focus Time", "stats-today-time"));

  const allTimeGroup = createStatsGroup("All Time");
  const allTimeItems = allTimeGroup.querySelector(".stats-group__items");

  if (!allTimeItems) {
    throw new Error("Stats dashboard items container missing.");
  }

  allTimeItems.appendChild(createStatItem("Sessions", "stats-alltime-sessions"));
  allTimeItems.appendChild(createStatItem("Focus Time", "stats-alltime-time"));

  content.appendChild(todayGroup);
  content.appendChild(allTimeGroup);

  panel.appendChild(header);
  panel.appendChild(content);
  container.appendChild(panel);

  const updateTodayStats = (stats: SessionStats): void => {
    const sessionsEl = document.getElementById("stats-today-sessions");
    const timeEl = document.getElementById("stats-today-time");

    if (sessionsEl) {
      sessionsEl.textContent = String(stats.completedSessions);
    }

    if (timeEl) {
      timeEl.textContent = formatTimeFromSecs(stats.totalWorkTimeInSecs);
    }
  };

  const updateAllTimeStats = (stats: SessionStats): void => {
    const sessionsEl = document.getElementById("stats-alltime-sessions");
    const timeEl = document.getElementById("stats-alltime-time");

    if (sessionsEl) {
      sessionsEl.textContent = String(stats.completedSessions);
    }

    if (timeEl) {
      timeEl.textContent = formatTimeFromSecs(stats.totalWorkTimeInSecs);
    }
  };

  const unsubscribeToday = statsStore.subscribeToday(updateTodayStats);
  const unsubscribeAllTime = statsStore.subscribeAllTime(updateAllTimeStats);

  const refresh = async (): Promise<void> => {
    await statsStore.refresh();
  };

  closeButton.addEventListener("click", () => {
    appWindow.close();
  });

  const handleKeyDown = (event: KeyboardEvent) => {
    if (event.code === "Escape") {
      event.preventDefault();
      event.stopPropagation();
      appWindow.close();
    }
  };

  window.addEventListener("keydown", handleKeyDown);

  const destroy = (): void => {
    window.removeEventListener("keydown", handleKeyDown);
    unsubscribeToday();
    unsubscribeAllTime();
    panel.remove();
  };

  return {
    element: panel,
    refresh,
    destroy,
  };
};

export type StatsDashboard = ReturnType<typeof createStatsDashboard>;
