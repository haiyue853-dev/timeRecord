import { useStatsStore } from "../store/useStatsStore";

export function AppsPage() {
  const apps = useStatsStore((state) => state.apps);
  const totalActiveSeconds = useStatsStore((state) => state.totalActiveSeconds);
  const topApp = apps[0];

  return (
    <section className="dashboard-page apps-page">
      <section className="page-hero">
        <span className="page-hero__eyebrow">APP RANK</span>
        <h2>应用统计</h2>
        <p>按软件维度统计本次开机的前台活跃时长，快速看清时间主要流向了哪里。</p>
      </section>

      <div className="apps-page__summary">
        <article className="dashboard-metric">
          <span>记录应用数</span>
          <strong>{apps.length}</strong>
          <small>本次会话中出现过的前台软件种类</small>
        </article>
        <article className="dashboard-metric dashboard-metric--yellow">
          <span>累计活跃分钟</span>
          <strong>{Math.floor(totalActiveSeconds / 60)} 分钟</strong>
          <small>仅统计位于前台且未空闲的有效时长</small>
        </article>
        <article className="dashboard-metric dashboard-metric--green">
          <span>当前第一名</span>
          <strong>{topApp ? topApp.appName : "等待生成"}</strong>
          <small>{topApp ? `${Math.floor(topApp.seconds / 60)} 分钟` : "开始使用电脑后会自动出现"}</small>
        </article>
      </div>

      <section className="list-card">
        <div className="list-card__header">
          <span>RANKED OVERVIEW</span>
          <strong>活跃时长排行</strong>
        </div>

        <ul className="stats-list stats-list--grid">
          {apps.map((app, index) => (
            <li key={app.appName} className="stats-list__item stats-list__item--rich">
              <div className="stats-list__meta">
                <span className="stats-rank">{String(index + 1).padStart(2, "0")}</span>
                <div>
                  <strong className="stats-app-name">{app.appName}</strong>
                  <small className="stats-app-copy">本次会话内被切到前台并持续活跃的累计时间</small>
                </div>
              </div>
              <strong>{Math.floor(app.seconds / 60)} 分钟</strong>
            </li>
          ))}
          {apps.length === 0 ? (
            <li className="stats-list__item stats-list__item--empty stats-list__item--full">
              <strong>还没有记录到前台应用，先开始使用电脑吧。</strong>
            </li>
          ) : null}
        </ul>
      </section>
    </section>
  );
}
