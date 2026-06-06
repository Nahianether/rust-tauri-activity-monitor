import { useEffect, useState } from "react";
import type { Theme } from "./ipc";

/// Resolved theme is always `light` or `dark` — `auto` is resolved against
/// the OS's prefers-color-scheme media query.
export type ResolvedTheme = "light" | "dark";

export function useResolvedTheme(preference: Theme): ResolvedTheme {
  const [systemDark, setSystemDark] = useState(() =>
    typeof window !== "undefined"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
      : false,
  );

  useEffect(() => {
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const onChange = (e: MediaQueryListEvent) => setSystemDark(e.matches);
    mq.addEventListener("change", onChange);
    return () => mq.removeEventListener("change", onChange);
  }, []);

  const resolved: ResolvedTheme =
    preference === "auto" ? (systemDark ? "dark" : "light") : preference;

  useEffect(() => {
    document.documentElement.dataset.theme = resolved;
  }, [resolved]);

  return resolved;
}

/// Deterministic color per app name. Stable hash + golden-ratio hue picker
/// gives well-distributed, repeatable colors with no manual palette upkeep.
export function colorForApp(appName: string, isDark: boolean): string {
  let hash = 0;
  for (let i = 0; i < appName.length; i++) {
    hash = (hash * 31 + appName.charCodeAt(i)) >>> 0;
  }
  const hue = (hash * 137.508) % 360;
  return isDark ? `hsl(${hue} 55% 60%)` : `hsl(${hue} 60% 45%)`;
}
