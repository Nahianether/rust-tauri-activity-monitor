import { useState } from "react";
import type { Settings as SettingsT, Theme } from "../ipc";
import { updateSettings } from "../ipc";

interface Props {
  initial: SettingsT;
  onChange: (next: SettingsT) => void;
}

const IDLE_PRESETS: { label: string; seconds: number }[] = [
  { label: "1 min", seconds: 60 },
  { label: "3 min", seconds: 180 },
  { label: "5 min", seconds: 300 },
  { label: "10 min", seconds: 600 },
];

const THEME_OPTIONS: { label: string; value: Theme }[] = [
  { label: "Auto", value: "auto" },
  { label: "Light", value: "light" },
  { label: "Dark", value: "dark" },
];

export function SettingsPanel({ initial, onChange }: Props) {
  const [draft, setDraft] = useState<SettingsT>(initial);
  const [saving, setSaving] = useState(false);
  const [savedAt, setSavedAt] = useState<number | null>(null);

  const dirty =
    draft.idle_threshold_seconds !== initial.idle_threshold_seconds ||
    draft.theme !== initial.theme ||
    draft.tracking_enabled !== initial.tracking_enabled;

  const save = async () => {
    setSaving(true);
    try {
      const saved = await updateSettings(draft);
      onChange(saved);
      setSavedAt(Date.now());
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="settings">
      <section className="setting-row">
        <div className="setting-label">
          <strong>Idle threshold</strong>
          <p className="muted">
            If there's no keyboard or mouse input for this long, TimeAtlas pauses tracking until
            you return.
          </p>
        </div>
        <div className="setting-control">
          <div className="chip-group">
            {IDLE_PRESETS.map((p) => (
              <button
                key={p.seconds}
                type="button"
                className={`chip${draft.idle_threshold_seconds === p.seconds ? " chip-active" : ""}`}
                onClick={() => setDraft({ ...draft, idle_threshold_seconds: p.seconds })}
              >
                {p.label}
              </button>
            ))}
          </div>
        </div>
      </section>

      <section className="setting-row">
        <div className="setting-label">
          <strong>Theme</strong>
          <p className="muted">Auto follows your operating system.</p>
        </div>
        <div className="setting-control">
          <div className="chip-group">
            {THEME_OPTIONS.map((t) => (
              <button
                key={t.value}
                type="button"
                className={`chip${draft.theme === t.value ? " chip-active" : ""}`}
                onClick={() => setDraft({ ...draft, theme: t.value })}
              >
                {t.label}
              </button>
            ))}
          </div>
        </div>
      </section>

      <section className="setting-row">
        <div className="setting-label">
          <strong>Tracking</strong>
          <p className="muted">
            When off, TimeAtlas records nothing. The tracker loop runs but is a no-op.
          </p>
        </div>
        <div className="setting-control">
          <label className="toggle">
            <input
              type="checkbox"
              checked={draft.tracking_enabled}
              onChange={(e) => setDraft({ ...draft, tracking_enabled: e.target.checked })}
            />
            <span>{draft.tracking_enabled ? "Enabled" : "Paused"}</span>
          </label>
        </div>
      </section>

      <div className="settings-actions">
        <button
          type="button"
          className="primary"
          onClick={save}
          disabled={!dirty || saving}
        >
          {saving ? "Saving…" : "Save changes"}
        </button>
        {!dirty && savedAt !== null && <span className="muted small">Saved.</span>}
        {dirty && <span className="muted small">Unsaved changes.</span>}
      </div>

      <div className="settings-footer">
        <p className="muted small">
          Settings are stored locally in <code>~/.timeatlas/db.sqlite</code> alongside your
          activity data. TimeAtlas never makes outbound network requests.
        </p>
      </div>
    </div>
  );
}
