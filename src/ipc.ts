import { invoke } from "@tauri-apps/api/core";

// Data shapes mirror the Rust types in src-tauri/src/{settings,tracker/storage}.rs.
// Keep these in sync if you change the backend.

export type Theme = "auto" | "light" | "dark";

export interface Settings {
  idle_threshold_seconds: number;
  theme: Theme;
  tracking_enabled: boolean;
}

export interface ActivityEntry {
  app_name: string;
  window_title: string;
  duration_seconds: number;
}

export interface TimelineSegment {
  bucket_minute: number;
  app_name: string;
  duration_seconds: number;
}

export const getTodaySummary = (): Promise<ActivityEntry[]> =>
  invoke<ActivityEntry[]>("get_today_summary");

export const getTodayTimeline = (): Promise<TimelineSegment[]> =>
  invoke<TimelineSegment[]>("get_today_timeline");

export const getSettings = (): Promise<Settings> => invoke<Settings>("get_settings");

export const updateSettings = (newSettings: Settings): Promise<Settings> =>
  invoke<Settings>("update_settings", { newSettings });

export const setTracking = (enabled: boolean): Promise<void> =>
  invoke<void>("set_tracking", { enabled });
