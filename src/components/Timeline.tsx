import { useMemo } from "react";
import type { TimelineSegment } from "../ipc";
import { colorForApp, type ResolvedTheme } from "../theme";

interface Props {
  segments: TimelineSegment[];
  theme: ResolvedTheme;
}

const BUCKET_MINUTES = 5;
const BUCKETS_PER_DAY = (24 * 60) / BUCKET_MINUTES; // 288
const BUCKET_DURATION_SECONDS = BUCKET_MINUTES * 60; // 300

/// Renders today's activity as a 24-hour horizontal bar. Each 5-minute
/// bucket is a vertical column; if multiple apps were active in that bucket
/// the column is sub-segmented in proportion to the time spent on each.
export function Timeline({ segments, theme }: Props) {
  const isDark = theme === "dark";

  // Group segments by bucket → { bucket: { app: seconds }[] }
  const byBucket = useMemo(() => {
    const m = new Map<number, TimelineSegment[]>();
    for (const s of segments) {
      const arr = m.get(s.bucket_minute) ?? [];
      arr.push(s);
      m.set(s.bucket_minute, arr);
    }
    return m;
  }, [segments]);

  const width = BUCKETS_PER_DAY; // 1 px per bucket at base; scaled by viewBox
  const height = 60;

  const hourLabels = Array.from({ length: 25 }, (_, i) => i);
  const labelStrokeColor = isDark ? "rgba(255,255,255,0.06)" : "rgba(0,0,0,0.06)";
  const labelTextColor = isDark ? "rgba(255,255,255,0.45)" : "rgba(0,0,0,0.45)";

  return (
    <div className="timeline">
      <svg
        viewBox={`0 0 ${width} ${height + 14}`}
        preserveAspectRatio="none"
        className="timeline-svg"
        role="img"
        aria-label="Today's activity timeline by app"
      >
        {/* Hour gridlines */}
        {hourLabels.map((h) => {
          const x = (h * 60) / BUCKET_MINUTES;
          return (
            <line
              key={`grid-${h}`}
              x1={x}
              y1={0}
              x2={x}
              y2={height}
              stroke={labelStrokeColor}
              strokeWidth={0.3}
            />
          );
        })}

        {/* Bucket columns */}
        {Array.from(byBucket.entries()).map(([bucket, apps]) => {
          const total = apps.reduce((sum, a) => sum + a.duration_seconds, 0);
          const denom = Math.max(total, BUCKET_DURATION_SECONDS);
          const x = bucket / BUCKET_MINUTES;
          let y = 0;
          return apps.map((app, i) => {
            const segH = (app.duration_seconds / denom) * height;
            const rect = (
              <rect
                key={`${bucket}-${app.app_name}-${i}`}
                x={x}
                y={height - y - segH}
                width={1}
                height={segH}
                fill={colorForApp(app.app_name, isDark)}
              >
                <title>
                  {formatBucketLabel(bucket)} — {app.app_name} — {formatSeconds(app.duration_seconds)}
                </title>
              </rect>
            );
            y += segH;
            return rect;
          });
        })}

        {/* Hour labels — every 3 hours */}
        {hourLabels
          .filter((h) => h % 3 === 0 && h < 24)
          .map((h) => {
            const x = (h * 60) / BUCKET_MINUTES;
            return (
              <text
                key={`label-${h}`}
                x={x + 1}
                y={height + 10}
                fontSize={6}
                fill={labelTextColor}
                fontFamily="system-ui, sans-serif"
              >
                {String(h).padStart(2, "0")}:00
              </text>
            );
          })}
      </svg>
    </div>
  );
}

function formatBucketLabel(minute: number): string {
  const h = Math.floor(minute / 60);
  const m = minute % 60;
  return `${String(h).padStart(2, "0")}:${String(m).padStart(2, "0")}`;
}

function formatSeconds(s: number): string {
  if (s < 60) return `${s}s`;
  const m = Math.floor(s / 60);
  const remS = s % 60;
  return remS === 0 ? `${m}m` : `${m}m ${remS}s`;
}
