import { LearningHeatmap } from "../components/charts/LearningHeatmap";
import { UsageTrend } from "../components/charts/UsageTrend";
import { useStatsStore } from "../store/useStatsStore";

export function HistoryPage() {
  const trendPoints = useStatsStore((state) => state.trendPoints);
  const weeklySummary = useStatsStore((state) => state.weeklySummary);
  const learningHeatmap = useStatsStore((state) => state.learningHeatmap);
  const heatmapCells = learningHeatmap.length
    ? learningHeatmap
    : Array.from({ length: 28 }, (_, index) => ({
        date: `0000-00-${String(index + 1).padStart(2, "0")}`,
        learningSeconds: 0,
        level: 0,
      }));
  const deltaLabel =
    weeklySummary.deltaSeconds === 0
      ? "和上周持平"
      : weeklySummary.deltaSeconds > 0
        ? `比上周多 ${formatMinutes(weeklySummary.deltaSeconds)}`
        : `比上周少 ${formatMinutes(Math.abs(weeklySummary.deltaSeconds))}`;

  return (
    <section className="dashboard-page dashboard-page--centered">
      <section className="page-hero page-hero--centered">
        <span className="page-hero__eyebrow">SESSION LINE</span>
        <h2>会话走势</h2>
        <p>这里展示的是本次开机以来真实采集到的前台活跃轨迹，同时补上最近一周对比和学习热力图。</p>
      </section>

      <section className="dashboard-content-card dashboard-content-card--centered dashboard-content-card--trend">
        <div className="dashboard-content-card__header">
          <span>TREND ANALYSIS</span>
          <strong>时长变化曲线</strong>
        </div>
        <UsageTrend points={trendPoints} />
      </section>

      <section className="history-grid history-grid--centered">
        <article className="dashboard-content-card history-card">
          <div className="dashboard-content-card__header">
            <span>WEEKLY REVIEW</span>
            <strong>最近 7 天对比</strong>
          </div>

          <div className="history-summary-grid">
            <div className="history-stat">
              <span>本周总时长</span>
              <strong>{formatHours(weeklySummary.currentWeekTotalSeconds)}</strong>
            </div>
            <div className="history-stat history-stat--accent">
              <span>学习时长</span>
              <strong>{formatHours(weeklySummary.currentWeekLearningSeconds)}</strong>
            </div>
            <div className="history-stat">
              <span>日均活跃</span>
              <strong>{formatHours(weeklySummary.currentWeekAverageSeconds)}</strong>
            </div>
            <div className="history-stat history-stat--green">
              <span>环比变化</span>
              <strong>{deltaLabel}</strong>
            </div>
          </div>

          <div className="history-best-day">
            <span>最稳的一天</span>
            <strong>{formatDate(weeklySummary.bestDay.date)}</strong>
            <small>
              当天累计 {formatHours(weeklySummary.bestDay.totalActiveSeconds)}，其中学习{" "}
              {formatHours(weeklySummary.bestDay.learningSeconds)}
            </small>
          </div>
        </article>

        <article className="dashboard-content-card history-card">
          <div className="dashboard-content-card__header">
            <span>LEARNING MAP</span>
            <strong>学习热力图</strong>
          </div>
          <LearningHeatmap cells={heatmapCells} />
        </article>
      </section>
    </section>
  );
}

function formatHours(seconds: number) {
  const hours = seconds / 3600;
  return `${hours.toFixed(hours >= 10 ? 0 : 1)} h`;
}

function formatMinutes(seconds: number) {
  return `${Math.round(seconds / 60)} 分钟`;
}

function formatDate(date: string) {
  if (!date || date.startsWith("0000-00")) {
    return "等待数据";
  }

  const [year, month, day] = date.split("-");
  return `${year}.${month}.${day}`;
}
