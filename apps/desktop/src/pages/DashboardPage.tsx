import { SummaryCard } from "../components/cards/SummaryCard";
import { UsageDonut } from "../components/charts/UsageDonut";
import { appSections } from "../navigation";
import { useStatsStore } from "../store/useStatsStore";

const dashboardSection = appSections[0];

export function DashboardPage() {
  const totalActiveSeconds = useStatsStore((state) => state.totalActiveSeconds);
  const todayActiveSeconds = useStatsStore((state) => state.todayActiveSeconds);
  const todayApps = useStatsStore((state) => state.todayApps);
  const summary = useStatsStore((state) => state.summary);
  const encouragement = useStatsStore((state) => state.encouragement);
  const summarySource = useStatsStore((state) => state.summarySource);
  const visibleTodayApps = todayApps.filter((item) => item.seconds > 0);
  const topApps = visibleTodayApps.slice(0, 3);
  const todayTopApp = topApps[0];

  return (
    <section className="dashboard-page">
      <section className="dashboard-hero dashboard-hero--compact">
        <div className="dashboard-hero__content">
          <span className="dashboard-hero__eyebrow">LIVE SESSION BOARD</span>
          <h2>{dashboardSection.title}</h2>
          <p>{dashboardSection.description}</p>

          <div className="dashboard-hero__pills">
            <span className="dashboard-pill dashboard-pill--blue">本地离线记录</span>
            <span className="dashboard-pill dashboard-pill--orange">今日维度统计</span>
            <span className="dashboard-pill dashboard-pill--ghost">鼓励语 3 小时刷新</span>
          </div>
        </div>

        <div className="dashboard-hero__preview dashboard-hero__preview--single">
          <div className="hero-preview-card hero-preview-card--accent">
            <span>本次前台活跃时长</span>
            <strong>{formatDuration(totalActiveSeconds)}</strong>
            <small>这里统计的是本次开机以来，真正位于前台且未被判定为空闲的有效活跃时长。</small>
          </div>
        </div>
      </section>

      <div className="apps-page__summary dashboard-grid--raised">
        <article className="dashboard-metric">
          <span>今日累计时长</span>
          <strong>{formatDuration(todayActiveSeconds)}</strong>
          <small>按自然日累计，今天到现在为止被记录到的前台有效活跃时长。</small>
        </article>

        <article className="dashboard-metric dashboard-metric--yellow">
          <span>今日活跃应用数</span>
          <strong>{visibleTodayApps.length}</strong>
          <small>今天真正出现在前台并累计到有效时长的应用数量。</small>
        </article>

        <article className="dashboard-metric dashboard-metric--green">
          <span>今日热门应用</span>
          <strong>{todayTopApp ? todayTopApp.appName : "等待生成"}</strong>
          <small>
            {todayTopApp
              ? `今日已累计 ${formatDuration(todayTopApp.seconds)}`
              : "开始使用电脑后，这里会自动显示今天排在第一的应用。"}
          </small>
        </article>
      </div>

      <div className="dashboard-content-grid">
        <section className="dashboard-content-card">
          <div className="dashboard-content-card__header">
            <span>TIME SPLIT</span>
            <strong>今日应用时长分布</strong>
          </div>
          <UsageDonut items={visibleTodayApps} />
        </section>

        <section className="dashboard-content-card dashboard-content-card--summary">
          <div className="dashboard-content-card__header">
            <span>SESSION NOTE</span>
            <strong>昨日小结与鼓励</strong>
          </div>
          <SummaryCard summary={summary} encouragement={encouragement} source={summarySource} />
        </section>
      </div>

      <section className="dashboard-content-card dashboard-content-card--stack">
        <div className="dashboard-content-card__header">
          <span>TOP APPS</span>
          <strong>今日热门应用排行</strong>
        </div>
        <ul className="stats-list stats-list--stacked">
          {topApps.map((app, index) => (
            <li key={app.appName} className="stats-list__item stats-list__item--rich">
              <div className="stats-list__meta">
                <span className="stats-rank">{String(index + 1).padStart(2, "0")}</span>
                <div>
                  <strong className="stats-app-name">{app.appName}</strong>
                  <small className="stats-app-copy">今天累计活跃 {formatDuration(app.seconds)}</small>
                </div>
              </div>
              <strong>{formatDuration(app.seconds)}</strong>
            </li>
          ))}
          {topApps.length === 0 ? (
            <li className="stats-list__item stats-list__item--empty">
              <strong>今天还没有记录到有效前台应用，开始使用后这里会自动出现排行。</strong>
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
