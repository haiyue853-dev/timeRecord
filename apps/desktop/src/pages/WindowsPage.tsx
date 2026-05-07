import { useStatsStore } from "../store/useStatsStore";

export function WindowsPage() {
  const currentWindowTitle = useStatsStore((state) => state.currentWindowTitle);

  return (
    <section className="dashboard-page">
      <h2>窗口明细</h2>
      <p>当前窗口：{currentWindowTitle}</p>
    </section>
  );
}
