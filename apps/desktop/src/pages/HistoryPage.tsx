import { UsageTrend } from "../components/charts/UsageTrend";
import { useStatsStore } from "../store/useStatsStore";

export function HistoryPage() {
  const totalActiveSeconds = useStatsStore((state) => state.totalActiveSeconds);

  return (
    <section className="dashboard-page">
      <h2>历史趋势</h2>
      <UsageTrend
        values={[0, totalActiveSeconds / 3, totalActiveSeconds / 2, totalActiveSeconds]}
      />
    </section>
  );
}
