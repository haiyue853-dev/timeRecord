import { useStatsStore } from "../store/useStatsStore";

export function AppsPage() {
  const apps = useStatsStore((state) => state.apps);

  return (
    <section className="dashboard-page">
      <h2>软件统计</h2>
      <ul className="stats-list">
        {apps.map((app) => (
          <li key={app.appName} className="stats-list__item">
            <span>{app.appName}</span>
            <strong>{Math.floor(app.seconds / 60)} 分钟</strong>
          </li>
        ))}
      </ul>
    </section>
  );
}
