import { useStatsStore } from "../store/useStatsStore";

export function AppsPage() {
  const todayApps = useStatsStore((state) => state.todayApps).filter((item) => item.seconds > 0);
  const todayActiveSeconds = useStatsStore((state) => state.todayActiveSeconds);
  const topApp = todayApps[0];

  return (
    <section className="dashboard-page apps-page">
      <section className="page-hero">
        <span className="page-hero__eyebrow">APP RANK</span>
        <h2>应用统计</h2>
        <p>按应用维度统计今天的前台活跃时长，快速看清时间主要流向了哪里。</p>
      </section>

      <div className="apps-page__summary">
        <article className="dashboard-metric">
          <span>记录应用数</span>
          <strong>{todayApps.length}</strong>
          <small>今天真正进入前台并被记录到有效时长的应用种类。</small>
        </article>
        <article className="dashboard-metric dashboard-metric--yellow">
          <span>今日累计时长</span>
          <strong>{formatDuration(todayActiveSeconds)}</strong>
          <small>仅统计位于前台且未空闲的有效活跃时长。</small>
        </article>
        <article className="dashboard-metric dashboard-metric--green">
          <span>今日第一名</span>
          <strong>{topApp ? topApp.appName : "等待生成"}</strong>
          <small>
            {topApp ? `今日累计 ${formatDuration(topApp.seconds)}` : "开始使用电脑后会自动出现。"}
          </small>
        </article>
      </div>

      <section className="list-card">
        <div className="list-card__header">
          <span>RANKED OVERVIEW</span>
          <strong>今日活跃时长排行</strong>
        </div>

        <ul className="stats-list stats-list--grid">
          {todayApps.map((app, index) => (
            <li key={app.appName} className="stats-list__item stats-list__item--rich">
              <div className="stats-list__meta">
                <span className="stats-rank">{String(index + 1).padStart(2, "0")}</span>
                <div>
                  <strong className="stats-app-name">{app.appName}</strong>
                  <small className="stats-app-copy">今天被切到前台并持续活跃的累计时长</small>
                </div>
              </div>
              <strong>{formatDuration(app.seconds)}</strong>
            </li>
          ))}
          {todayApps.length === 0 ? (
            <li className="stats-list__item stats-list__item--empty stats-list__item--full">
              <strong>今天还没有记录到有效前台应用，先开始使用电脑吧。</strong>
            </li>
          ) : null}
        </ul>
      </section>
    </section>
  );
}

function formatDuration(seconds: number) {
  const totalMinutes = Math.floor(seconds / 60);

  if (totalMinutes <= 0) {
    return "0 分钟";
  }

  const hours = Math.floor(totalMinutes / 60);
  const minutes = totalMinutes % 60;

  if (hours === 0) {
    return `${totalMinutes} 分钟`;
  }

  if (minutes === 0) {
    return `${hours} 小时`;
  }

  return `${hours} 小时 ${minutes} 分钟`;
}
