import { useMemo } from "react";
import type { ActivityEntry } from "../ipc";
import { colorForApp, type ResolvedTheme } from "../theme";

interface Props {
  entries: ActivityEntry[];
  theme: ResolvedTheme;
}

export function Summary({ entries, theme }: Props) {
  const isDark = theme === "dark";

  const total = useMemo(
    () => entries.reduce((sum, e) => sum + e.duration_seconds, 0),
    [entries],
  );

  if (entries.length === 0) {
    return (
      <div className="empty-state">
        <p>No activity recorded yet today.</p>
        <p className="muted">
          TimeAtlas tracks foreground apps once per second while you're active. Use your computer
          for a minute or two, then come back here.
        </p>
      </div>
    );
  }

  return (
    <div className="summary">
      <div className="summary-header">
        <span className="muted">Today, {entries.length} apps</span>
        <strong>{formatDuration(total)}</strong>
      </div>

      <ul className="summary-list">
        {entries.map((e) => {
          const pct = total > 0 ? (e.duration_seconds / total) * 100 : 0;
          const color = colorForApp(e.app_name, isDark);
          return (
            <li key={e.app_name + e.window_title} className="summary-item">
              <span className="dot" style={{ backgroundColor: color }} />
              <span className="app">{e.app_name}</span>
              <span className="bar-wrap">
                <span className="bar" style={{ width: `${pct}%`, backgroundColor: color }} />
              </span>
              <span className="duration">{formatDuration(e.duration_seconds)}</span>
              <span className="pct muted">{pct.toFixed(0)}%</span>
            </li>
          );
        })}
      </ul>
    </div>
  );
}

function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m`;
  return `${seconds}s`;
}
