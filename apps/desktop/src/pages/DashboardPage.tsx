import { SummaryCard } from "../components/cards/SummaryCard";
import { UsageDonut } from "../components/charts/UsageDonut";
import { appSections } from "../navigation";
import { useStatsStore } from "../store/useStatsStore";

const dashboardSection = appSections[0];

export function DashboardPage() {
  const totalActiveSeconds = useStatsStore((state) => state.totalActiveSeconds);
  const currentAppName = useStatsStore((state) => state.currentAppName);
  const currentWindowTitle = useStatsStore((state) => state.currentWindowTitle);
  const apps = useStatsStore((state) => state.apps);
  const summary = useStatsStore((state) => state.summary);

  return (
    <section className="dashboard-page">
      <h2>{dashboardSection.title}</h2>
      <p>{dashboardSection.description}</p>
      <div className="dashboard-grid">
        <article className="dashboard-metric">
          <span>当前软件</span>
          <strong>{currentAppName}</strong>
          <small>{currentWindowTitle}</small>
        </article>
        <article className="dashboard-metric">
          <span>活跃总时长</span>
          <strong>{Math.floor(totalActiveSeconds / 60)} 分钟</strong>
        </article>
      </div>
      <UsageDonut items={apps} />
      <SummaryCard summary={summary} />
    </section>
  );
}
