import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

type ActivityEntry = {
  app_name: string;
  window_title: string;
  duration_seconds: number;
};

function App() {
  const [today, setToday] = useState<ActivityEntry[]>([]);
  const [tracking, setTracking] = useState(true);

  useEffect(() => {
    const refresh = () => {
      invoke<ActivityEntry[]>("get_today_summary")
        .then(setToday)
        .catch(() => setToday([]));
    };
    refresh();
    const id = setInterval(refresh, 5000);
    return () => clearInterval(id);
  }, []);

  const toggleTracking = async () => {
    const next = !tracking;
    await invoke("set_tracking", { enabled: next });
    setTracking(next);
  };

  return (
    <div className="container">
      <header>
        <h1>TimeAtlas</h1>
        <button onClick={toggleTracking}>
          {tracking ? "Pause tracking" : "Resume tracking"}
        </button>
      </header>

      <section>
        <h2>Today</h2>
        {today.length === 0 ? (
          <p className="muted">No activity recorded yet today.</p>
        ) : (
          <ul className="activity-list">
            {today.map((e, i) => (
              <li key={i}>
                <span className="app">{e.app_name}</span>
                <span className="title">{e.window_title}</span>
                <span className="duration">
                  {formatDuration(e.duration_seconds)}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>
    </div>
  );
}

function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

export default App;
