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
  const encouragement = useStatsStore((state) => state.encouragement);
  const summarySource = useStatsStore((state) => state.summarySource);
  const topApps = apps.slice(0, 3);
  const totalMinutes = Math.floor(totalActiveSeconds / 60);

  return (
    <section className="dashboard-page">
      <section className="dashboard-hero">
        <div className="dashboard-hero__content">
          <span className="dashboard-hero__eyebrow">LIVE SESSION BOARD</span>
          <h2>{dashboardSection.title}</h2>
          <p>{dashboardSection.description}</p>

          <div className="dashboard-hero__pills">
            <span className="dashboard-pill dashboard-pill--blue">前台追踪中</span>
            <span className="dashboard-pill dashboard-pill--orange">昨日总结可用</span>
            <span className="dashboard-pill dashboard-pill--ghost">本地离线优先</span>
          </div>
        </div>

        <div className="dashboard-hero__preview">
          <div className="hero-preview-card hero-preview-card--primary">
            <span>当前前台软件</span>
            <strong>{currentAppName || "等待活动采集"}</strong>
            <small>{currentWindowTitle || "暂时还没有可记录的窗口标题"}</small>
          </div>
          <div className="hero-preview-card hero-preview-card--accent">
            <span>本次前台活跃时长</span>
            <strong>{totalMinutes} 分钟</strong>
            <small>仅统计本次开机期间位于前台且未空闲的有效时长</small>
          </div>
        </div>
      </section>

      <div className="dashboard-grid">
        <article className="dashboard-metric">
          <span>当前记录目标</span>
          <strong>{currentAppName || "等待活动采集"}</strong>
          <small>{currentWindowTitle || "窗口标题会在检测到前台活动后显示"}</small>
        </article>
        <article className="dashboard-metric">
          <span>累计活跃分钟</span>
          <strong>{totalMinutes} 分钟</strong>
          <small>如果你在刷网课，停留在课程播放器前台的时间会记到对应浏览器或播放器</small>
        </article>
        <article className="dashboard-metric dashboard-metric--yellow">
          <span>高频应用数</span>
          <strong>{apps.length}</strong>
          <small>当前会话内已经出现过的前台软件种类</small>
        </article>
        <article className="dashboard-metric dashboard-metric--green">
          <span>前三名</span>
          <strong>{topApps.length ? topApps.map((item) => item.appName).join(" / ") : "等待生成"}</strong>
          <small>最常驻留的程序会优先出现在这里</small>
        </article>
      </div>

      <div className="dashboard-content-grid">
        <section className="dashboard-content-card">
          <div className="dashboard-content-card__header">
            <span>TIME SPLIT</span>
            <strong>应用时长分布</strong>
          </div>
          <UsageDonut items={apps} />
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
          <strong>本次开机热门应用</strong>
        </div>
        <ul className="stats-list stats-list--stacked">
          {topApps.map((app, index) => (
            <li key={app.appName} className="stats-list__item stats-list__item--rich">
              <div className="stats-list__meta">
                <span className="stats-rank">{String(index + 1).padStart(2, "0")}</span>
                <div>
                  <strong className="stats-app-name">{app.appName}</strong>
                  <small className="stats-app-copy">当前会话累计 {Math.floor(app.seconds / 60)} 分钟</small>
                </div>
              </div>
              <strong>{Math.floor(app.seconds / 60)} 分钟</strong>
            </li>
          ))}
          {topApps.length === 0 ? (
            <li className="stats-list__item stats-list__item--empty">
              <strong>还没有足够数据，先开始使用电脑吧。</strong>
            </li>
          ) : null}
        </ul>
      </section>
    </section>
  );
}
