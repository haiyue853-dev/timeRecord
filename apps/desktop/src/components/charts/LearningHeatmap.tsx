import type { HeatmapCell } from "../../types/stats";

const levelClassName = [
  "history-heatmap__cell--0",
  "history-heatmap__cell--1",
  "history-heatmap__cell--2",
  "history-heatmap__cell--3",
  "history-heatmap__cell--4",
];

export function LearningHeatmap({ cells }: { cells: HeatmapCell[] }) {
  return (
    <div className="history-heatmap">
      <div className="history-heatmap__grid">
        {cells.map((cell) => (
          <div
            key={cell.date}
            aria-label="heatmap-cell"
            className={`history-heatmap__cell ${levelClassName[cell.level] ?? levelClassName[0]}`}
            title={`${cell.date} 学习 ${Math.round(cell.learningSeconds / 60)} 分钟`}
          />
        ))}
      </div>
      <div className="history-heatmap__legend">
        <span>少</span>
        <div className="history-heatmap__legend-scale">
          {[0, 1, 2, 3, 4].map((level) => (
            <span
              key={level}
              className={`history-heatmap__legend-cell ${levelClassName[level]}`}
            />
          ))}
        </div>
        <span>多</span>
      </div>
    </div>
  );
}
