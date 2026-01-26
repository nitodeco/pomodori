import { SESSION_LABEL_BY_TYPE, type SessionType } from "../timer/types";

type TimeDisplayConfig = {
  container: HTMLElement;
};

const formatTime = (totalSecs: number): string => {
  const minutes = Math.floor(totalSecs / 60);
  const seconds = totalSecs % 60;
  const paddedMinutes = String(minutes).padStart(2, "0");
  const paddedSeconds = String(seconds).padStart(2, "0");

  return `${paddedMinutes}:${paddedSeconds}`;
};

export const createTimeDisplay = (config: TimeDisplayConfig) => {
  const { container } = config;

  const wrapper = document.createElement("div");
  wrapper.className = "time-display";

  const timeElement = document.createElement("div");
  timeElement.className = "time-display__time";
  timeElement.textContent = "25:00";

  const modeElement = document.createElement("div");
  modeElement.className = "time-display__mode";
  modeElement.textContent = SESSION_LABEL_BY_TYPE.work;

  wrapper.appendChild(timeElement);
  wrapper.appendChild(modeElement);
  container.appendChild(wrapper);

  const setTime = (remainingSecs: number) => {
    timeElement.textContent = formatTime(remainingSecs);
  };

  const setMode = (sessionType: SessionType) => {
    modeElement.textContent = SESSION_LABEL_BY_TYPE[sessionType];
  };

  const destroy = () => {
    wrapper.remove();
  };

  return {
    element: wrapper,
    setTime,
    setMode,
    destroy,
  };
};

export type TimeDisplay = ReturnType<typeof createTimeDisplay>;
