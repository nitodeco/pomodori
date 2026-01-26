import type { TimerState } from "../timer/types";

type ControlButtonsConfig = {
  container: HTMLElement;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStop: () => void;
};

const createButton = (
  className: string,
  label: string,
  onClick: () => void
): HTMLButtonElement => {
  const button = document.createElement("button");
  button.className = `control-button ${className}`;
  button.textContent = label;
  button.addEventListener("click", onClick);

  return button;
};

export const createControlButtons = (config: ControlButtonsConfig) => {
  const { container, onStart, onPause, onResume, onStop } = config;

  const wrapper = document.createElement("div");
  wrapper.className = "control-buttons";

  const startButton = createButton("control-button--primary", "Start", onStart);
  const pauseButton = createButton("control-button--primary", "Pause", onPause);
  const resumeButton = createButton(
    "control-button--primary",
    "Resume",
    onResume
  );
  const stopButton = createButton("control-button--secondary", "Stop", onStop);

  wrapper.appendChild(startButton);
  wrapper.appendChild(pauseButton);
  wrapper.appendChild(resumeButton);
  wrapper.appendChild(stopButton);
  container.appendChild(wrapper);

  const setState = (state: TimerState) => {
    const isIdle = state === "idle";
    const isRunning = state === "running";
    const isPaused = state === "paused";

    startButton.style.display = isIdle ? "inline-block" : "none";
    pauseButton.style.display = isRunning ? "inline-block" : "none";
    resumeButton.style.display = isPaused ? "inline-block" : "none";
    stopButton.style.display = isIdle ? "none" : "inline-block";
  };

  setState("idle");

  const destroy = () => {
    wrapper.remove();
  };

  return {
    element: wrapper,
    setState,
    destroy,
  };
};

export type ControlButtons = ReturnType<typeof createControlButtons>;
