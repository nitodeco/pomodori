import { invoke } from "@tauri-apps/api/core";

export const updateTooltip = (tooltip: string): Promise<void> =>
  invoke("tray_update_tooltip", { tooltip });
