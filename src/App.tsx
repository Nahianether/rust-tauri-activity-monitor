import { useCallback, useEffect, useState } from "react";

import { Summary } from "./components/Summary";
import { Timeline } from "./components/Timeline";
import { SettingsPanel } from "./components/Settings";
import {
  getSettings,
  getTodaySummary,
  getTodayTimeline,
  setTracking,
  type ActivityEntry,
  type Settings,
  type TimelineSegment,
} from "./ipc";
import { useResolvedTheme } from "./theme";

type Tab = "timeline" | "summary" | "settings";

const REFRESH_INTERVAL_MS = 5000;

function App() {
  const [tab, setTab] = useState<Tab>("timeline");
  const [summary, setSummary] = useState<ActivityEntry[]>([]);
  const [timeline, setTimeline] = useState<TimelineSegment[]>([]);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [bootError, setBootError] = useState<string | null>(null);

  const resolved = useResolvedTheme(settings?.theme ?? "auto");

  const refresh = useCallback(async () => {
    try {
      const [s, t] = await Promise.all([getTodaySummary(), getTodayTimeline()]);
      setSummary(s);
      setTimeline(t);
    } catch (e) {
      // Silent: backend may not have finished initialization yet on first paint.
      console.warn("refresh failed", e);
    }
  }, []);

  // Initial settings load + recurring refresh.
  useEffect(() => {
    getSettings()
      .then(setSettings)
      .catch((e) => setBootError(String(e)));
    refresh();
    const id = setInterval(refresh, REFRESH_INTERVAL_MS);
    return () => clearInterval(id);
  }, [refresh]);

  const togglePause = async () => {
    if (!settings) return;
    const next = !settings.tracking_enabled;
    await setTracking(next);
    setSettings({ ...settings, tracking_enabled: next });
  };

  if (bootError) {
    return (
      <div className="app-shell">
        <div className="boot-error">
          <h2>TimeAtlas failed to start</h2>
          <pre>{bootError}</pre>
        </div>
      </div>
    );
  }

  return (
    <div className="app-shell">
      <header className="header">
        <div className="brand">
          <span className="brand-mark">⏱</span>
          <h1>TimeAtlas</h1>
        </div>

        <div className="status">
          <span
            className={`pulse${settings?.tracking_enabled ? "" : " pulse-off"}`}
            aria-hidden
          />
          <span className="status-text">
            {settings?.tracking_enabled ? "Tracking" : "Paused"}
          </span>
          <button
            type="button"
            className="ghost"
            onClick={togglePause}
            disabled={!settings}
          >
            {settings?.tracking_enabled ? "Pause" : "Resume"}
          </button>
        </div>
      </header>

      <nav className="tabs">
        <TabButton id="timeline" active={tab} setActive={setTab}>
          Timeline
        </TabButton>
        <TabButton id="summary" active={tab} setActive={setTab}>
          Summary
        </TabButton>
        <TabButton id="settings" active={tab} setActive={setTab}>
          Settings
        </TabButton>
      </nav>

      <main className="content">
        {tab === "timeline" && <Timeline segments={timeline} theme={resolved} />}
        {tab === "summary" && <Summary entries={summary} theme={resolved} />}
        {tab === "settings" && settings && (
          <SettingsPanel initial={settings} onChange={setSettings} />
        )}
        {tab === "settings" && !settings && <p className="muted">Loading settings…</p>}
      </main>
    </div>
  );
}

interface TabButtonProps {
  id: Tab;
  active: Tab;
  setActive: (t: Tab) => void;
  children: React.ReactNode;
}

function TabButton({ id, active, setActive, children }: TabButtonProps) {
  return (
    <button
      type="button"
      className={`tab${id === active ? " tab-active" : ""}`}
      onClick={() => setActive(id)}
    >
      {children}
    </button>
  );
}

export default App;
