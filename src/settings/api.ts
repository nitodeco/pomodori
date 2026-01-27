import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "./types";

export const getSettings = (): Promise<Settings> => invoke<Settings>("settings_get");

export const updateSettings = (settings: Settings): Promise<Settings> =>
  invoke<Settings>("settings_update", { settings });
