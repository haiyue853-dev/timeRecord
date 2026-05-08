import { useStatsStore } from "../store/useStatsStore";

export function WindowsPage() {
  const currentWindowTitle = useStatsStore((state) => state.currentWindowTitle);

  return (
    <section className="dashboard-page dashboard-page--centered">
      <section className="page-hero page-hero--centered">
        <span className="page-hero__eyebrow">WINDOW TRACE</span>
        <h2>窗口焦点</h2>
        <p>这里显示当前正在被记录的窗口标题。刷网课、写文档、看视频时，都能更直观地知道时间落在了哪一个具体任务上。</p>
      </section>

      <section className="focus-card focus-card--centered">
        <span className="focus-card__label">CURRENT WINDOW</span>
        <strong>{currentWindowTitle || "正在等待可记录的前台窗口标题"}</strong>
      </section>
    </section>
  );
}
