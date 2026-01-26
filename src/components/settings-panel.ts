import type { Settings } from "../settings";
import { getSettings, updateSettings } from "../settings";

type SettingsPanelConfig = {
  container: HTMLElement;
  onSettingsChange: (settings: Settings) => void;
};

const SECS_PER_MIN = 60;

const secsToMins = (secs: number): number => Math.floor(secs / SECS_PER_MIN);
const minsToSecs = (mins: number): number => mins * SECS_PER_MIN;

const createNumberInput = (
  id: string,
  label: string,
  value: number,
  min: number,
  max: number
): HTMLDivElement => {
  const wrapper = document.createElement("div");
  wrapper.className = "settings-field";

  const labelEl = document.createElement("label");
  labelEl.htmlFor = id;
  labelEl.textContent = label;

  const input = document.createElement("input");
  input.type = "number";
  input.id = id;
  input.name = id;
  input.value = String(value);
  input.min = String(min);
  input.max = String(max);

  wrapper.appendChild(labelEl);
  wrapper.appendChild(input);

  return wrapper;
};

const createToggle = (
  id: string,
  label: string,
  isChecked: boolean
): HTMLDivElement => {
  const wrapper = document.createElement("div");
  wrapper.className = "settings-field settings-field--toggle";

  const labelEl = document.createElement("label");
  labelEl.htmlFor = id;
  labelEl.textContent = label;

  const toggle = document.createElement("input");
  toggle.type = "checkbox";
  toggle.id = id;
  toggle.name = id;
  toggle.checked = isChecked;
  toggle.className = "settings-toggle";

  wrapper.appendChild(labelEl);
  wrapper.appendChild(toggle);

  return wrapper;
};

const createSection = (title: string): HTMLDivElement => {
  const section = document.createElement("div");
  section.className = "settings-section";

  const heading = document.createElement("h3");
  heading.className = "settings-section__title";
  heading.textContent = title;
  section.appendChild(heading);

  return section;
};

export const createSettingsPanel = (config: SettingsPanelConfig) => {
  const { container, onSettingsChange } = config;
  let currentSettings: Settings | null = null;

  const overlay = document.createElement("div");
  overlay.className = "settings-overlay";
  overlay.style.display = "none";

  const panel = document.createElement("div");
  panel.className = "settings-panel";

  const header = document.createElement("div");
  header.className = "settings-header";

  const title = document.createElement("h2");
  title.className = "settings-title";
  title.textContent = "Settings";

  const closeButton = document.createElement("button");
  closeButton.className = "settings-close";
  closeButton.innerHTML = "×";
  closeButton.setAttribute("aria-label", "Close settings");

  header.appendChild(title);
  header.appendChild(closeButton);

  const content = document.createElement("div");
  content.className = "settings-content";

  const durationsSection = createSection("Durations");
  const workDuration = createNumberInput(
    "workDuration",
    "Work (minutes)",
    25,
    1,
    120
  );
  const shortBreak = createNumberInput(
    "shortBreak",
    "Short Break (minutes)",
    5,
    1,
    60
  );
  const longBreak = createNumberInput(
    "longBreak",
    "Long Break (minutes)",
    15,
    1,
    60
  );
  const sessionsUntilLong = createNumberInput(
    "sessionsUntilLong",
    "Sessions until Long Break",
    4,
    1,
    10
  );

  durationsSection.appendChild(workDuration);
  durationsSection.appendChild(shortBreak);
  durationsSection.appendChild(longBreak);
  durationsSection.appendChild(sessionsUntilLong);

  const automationSection = createSection("Automation");
  const autoStartBreaks = createToggle(
    "autoStartBreaks",
    "Auto-start Breaks",
    false
  );
  const autoStartWork = createToggle(
    "autoStartWork",
    "Auto-start Work",
    false
  );

  automationSection.appendChild(autoStartBreaks);
  automationSection.appendChild(autoStartWork);

  const notificationsSection = createSection("Notifications");
  const soundEnabled = createToggle("soundEnabled", "Sound Effects", true);
  const notificationsEnabled = createToggle(
    "notificationsEnabled",
    "Desktop Notifications",
    true
  );

  notificationsSection.appendChild(soundEnabled);
  notificationsSection.appendChild(notificationsEnabled);

  content.appendChild(durationsSection);
  content.appendChild(automationSection);
  content.appendChild(notificationsSection);

  const footer = document.createElement("div");
  footer.className = "settings-footer";

  const saveButton = document.createElement("button");
  saveButton.className = "settings-save";
  saveButton.textContent = "Save";

  footer.appendChild(saveButton);

  panel.appendChild(header);
  panel.appendChild(content);
  panel.appendChild(footer);
  overlay.appendChild(panel);
  container.appendChild(overlay);

  const getFormValues = (): Settings => {
    const workDurationInput = document.getElementById(
      "workDuration"
    ) as HTMLInputElement;
    const shortBreakInput = document.getElementById(
      "shortBreak"
    ) as HTMLInputElement;
    const longBreakInput = document.getElementById(
      "longBreak"
    ) as HTMLInputElement;
    const sessionsInput = document.getElementById(
      "sessionsUntilLong"
    ) as HTMLInputElement;
    const autoBreaksInput = document.getElementById(
      "autoStartBreaks"
    ) as HTMLInputElement;
    const autoWorkInput = document.getElementById(
      "autoStartWork"
    ) as HTMLInputElement;
    const soundInput = document.getElementById(
      "soundEnabled"
    ) as HTMLInputElement;
    const notifInput = document.getElementById(
      "notificationsEnabled"
    ) as HTMLInputElement;

    return {
      workDurationInSecs: minsToSecs(Number(workDurationInput.value)),
      shortBreakDurationInSecs: minsToSecs(Number(shortBreakInput.value)),
      longBreakDurationInSecs: minsToSecs(Number(longBreakInput.value)),
      sessionsUntilLongBreak: Number(sessionsInput.value),
      autoStartBreaks: autoBreaksInput.checked,
      autoStartWork: autoWorkInput.checked,
      soundEnabled: soundInput.checked,
      notificationsEnabled: notifInput.checked,
    };
  };

  const setFormValues = (settings: Settings) => {
    const workDurationInput = document.getElementById(
      "workDuration"
    ) as HTMLInputElement;
    const shortBreakInput = document.getElementById(
      "shortBreak"
    ) as HTMLInputElement;
    const longBreakInput = document.getElementById(
      "longBreak"
    ) as HTMLInputElement;
    const sessionsInput = document.getElementById(
      "sessionsUntilLong"
    ) as HTMLInputElement;
    const autoBreaksInput = document.getElementById(
      "autoStartBreaks"
    ) as HTMLInputElement;
    const autoWorkInput = document.getElementById(
      "autoStartWork"
    ) as HTMLInputElement;
    const soundInput = document.getElementById(
      "soundEnabled"
    ) as HTMLInputElement;
    const notifInput = document.getElementById(
      "notificationsEnabled"
    ) as HTMLInputElement;

    workDurationInput.value = String(secsToMins(settings.workDurationInSecs));
    shortBreakInput.value = String(
      secsToMins(settings.shortBreakDurationInSecs)
    );
    longBreakInput.value = String(secsToMins(settings.longBreakDurationInSecs));
    sessionsInput.value = String(settings.sessionsUntilLongBreak);
    autoBreaksInput.checked = settings.autoStartBreaks;
    autoWorkInput.checked = settings.autoStartWork;
    soundInput.checked = settings.soundEnabled;
    notifInput.checked = settings.notificationsEnabled;
  };

  const open = async () => {
    currentSettings = await getSettings();
    setFormValues(currentSettings);
    overlay.style.display = "flex";
  };

  const close = () => {
    overlay.style.display = "none";
  };

  const save = async () => {
    const newSettings = getFormValues();
    const savedSettings = await updateSettings(newSettings);
    currentSettings = savedSettings;
    onSettingsChange(savedSettings);
    close();
  };

  closeButton.addEventListener("click", close);
  overlay.addEventListener("click", (event) => {
    if (event.target === overlay) {
      close();
    }
  });
  saveButton.addEventListener("click", save);

  const destroy = () => {
    overlay.remove();
  };

  return {
    element: overlay,
    open,
    close,
    destroy,
  };
};

export type SettingsPanel = ReturnType<typeof createSettingsPanel>;
