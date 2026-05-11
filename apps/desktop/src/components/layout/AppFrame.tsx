import { NavLink, Outlet } from "react-router-dom";
import { appSections } from "../../navigation";
import { useStatsStore } from "../../store/useStatsStore";
import { TitleBar } from "./TitleBar";

export function AppFrame() {
  const todayNote = useStatsStore((state) => state.todayNote);

  return (
    <div className="window-shell">
      <TitleBar />

      <div className="window-shell__surface">
        <aside className="sidebar">
          <div className="sidebar__panel">
            <div className="sidebar__hero">
              <p className="sidebar__eyebrow">TRACK. WATCH. REVIEW.</p>
              <h1 className="sidebar__title">TimeRecord</h1>
              <p className="sidebar__copy">本地离线记录本次开机与今日前台活跃时长。</p>
            </div>

            <nav aria-label="主导航" className="sidebar__nav">
              {appSections.map((section, index) => (
                <NavLink
                  key={section.to}
                  to={section.to}
                  end={section.end}
                  aria-label={section.label}
                  className={({ isActive }) =>
                    isActive ? "sidebar__link is-active" : "sidebar__link"
                  }
                >
                  <span className="sidebar__link-badge">
                    {String(index + 1).padStart(2, "0")}
                  </span>
                  <span className="sidebar__link-title">{section.label}</span>
                  <small className="sidebar__link-copy">{section.description}</small>
                </NavLink>
              ))}
            </nav>

            <section className="sidebar__spotlight">
              <span className="sidebar__spotlight-label">TODAY NOTE</span>
              <strong>{todayNote}</strong>
            </section>
          </div>
        </aside>

        <main className="content-shell">
          <section className="content-shell__panel">
            <Outlet />
          </section>
        </main>
      </div>
    </div>
  );
}
